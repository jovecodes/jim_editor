use crate::{
    cursor::Cursor,
    file::JimFile,
    mapping::{Command, Mapping},
    mode::Mode,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use nalgebra::Vector2;
use std::{
    collections::VecDeque,
    io,
    path::{Path, PathBuf},
};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    text::Text,
    widgets::Paragraph,
    Frame, Terminal,
};

#[derive(Debug, Default)]
pub struct Jim {
    properties: JimProperties,
    command: String,
    nmaps: Vec<Mapping>,
    imaps: Vec<Mapping>,
    cmaps: Vec<Command>,
}

#[derive(Debug, Default)]
pub struct JimProperties {
    pub mode: Mode,
    pub cursor: Cursor,
    pub quitting: bool,
    pub buttons_pressed: Vec<KeyCode>,
    pub buffers: Vec<JimFile>,
    pub recent_buffers: VecDeque<usize>,
    pub cant_press_maps: bool,
}

impl JimProperties {
    pub fn use_mapping(&mut self, buttons: &Vec<KeyCode>) -> bool {
        self.buttons_pressed
            .windows(buttons.len())
            .any(|window| window == buttons)
    }

    pub fn move_cursor_down(&mut self, amount: usize) {
        for _ in 0..amount {
            self.cursor.move_down(&self.buffers[self.recent_buffers[0]])
        }
    }

    pub fn move_cursor_up(&mut self, amount: usize) {
        for _ in 0..amount {
            self.cursor.move_up(&self.buffers[self.recent_buffers[0]])
        }
    }

    pub fn move_cursor_right(&mut self, amount: usize) {
        for _ in 0..amount {
            self.cursor
                .move_right(&self.buffers[self.recent_buffers[0]])
        }
    }

    pub fn move_cursor_left(&mut self, amount: usize) {
        for _ in 0..amount {
            self.cursor.move_left(&self.buffers[self.recent_buffers[0]])
        }
    }

    pub fn current_line(&self) -> &str {
        self.buffers[self.recent_buffers[0]]
            .contents
            .lines()
            .nth(self.cursor.xy_pos.y)
            .unwrap()
    }

    pub fn get_current_buffer(&self) -> &JimFile {
        &self.buffers[self.recent_buffers[0]]
    }

    pub fn get_current_buffer_contents(&self) -> &String {
        &self.get_current_buffer().contents
    }

    pub fn get_mut_current_buffer(&mut self) -> &mut JimFile {
        &mut self.buffers[self.recent_buffers[0]]
    }

    pub fn get_mut_current_buffer_contents(&mut self) -> &mut String {
        &mut self.get_mut_current_buffer().contents
    }

    pub fn write_char_to_current_buffer(&mut self, char: char) {
        self.cursor
            .write_char_to(char, &mut self.buffers[self.recent_buffers[0]]);
    }

    pub fn backspace_current_buffer(&mut self) {
        self.cursor
            .backspace(&mut self.buffers[self.recent_buffers[0]])
    }

    pub fn open_file(&mut self, path: &Path) -> io::Result<()> {
        self.buffers.push(JimFile::new(&path)?);
        self.recent_buffers.push_front(self.buffers.len() - 1);
        Ok(())
    }

    pub fn force_move_cursor_right(&mut self, amount: usize) {
        for _ in 0..amount {
            self.cursor
                .force_move_right(&self.buffers[self.recent_buffers[0]])
        }
    }

    pub fn move_cursor_full_right(&mut self) {
        self.cursor.move_full_right(&self.buffers[self.recent_buffers[0]]);
    }

    pub fn move_cursor_full_left(&mut self) {
        self.cursor.move_full_left();
    }
}

impl Jim {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init(mut self) -> io::Result<Self> {
        self.properties.open_file(self.get_workspace()?.as_path())?;

        Ok(self)
    }

    pub fn run<B: Backend>(mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            if self.properties.quitting {
                break;
            }

            terminal.draw(|f| self.render(f))?;

            if let Event::Key(key) = event::read()? {
                match self.properties.mode {
                    Mode::Normal => self.normal(key),
                    Mode::Insert => self.insert(key),
                    Mode::Command => self.command(key),
                }
            }
        }
        Ok(())
    }

    fn render<B: Backend>(&mut self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
            .split(f.size());

        let text = Text::raw(
            self.properties.get_current_buffer_contents().clone()
                + "\n"
                + &self.properties.cursor.index.to_string()
                + "\n"
                + &self.properties.cursor.xy_pos.to_string(),
        );
        let file = Paragraph::new(text);
        f.render_widget(file, chunks[0]);

        let command_paragraph = Paragraph::new(self.command.clone());
        f.render_widget(command_paragraph, chunks[1]);

        let cursor_pos = self.get_cursor_position();
        f.set_cursor(cursor_pos.x as u16, cursor_pos.y as u16)
    }

    fn insert(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('e') => self.to_normal_mode(),
            KeyCode::Char(char) => self.properties.write_char_to_current_buffer(char),
            KeyCode::Enter => self.properties.write_char_to_current_buffer('\n'),
            KeyCode::Backspace => self.properties.backspace_current_buffer(),
            _ => todo!(),
        }
    }

    fn command(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('e') => self.to_normal_mode(),
            KeyCode::Char(char) => self.write_char_to_command(char),
            KeyCode::Enter => self.run_commands(),
            KeyCode::Backspace => self.backspace_command(),
            _ => todo!(),
        }
    }

    fn normal(&mut self, key: KeyEvent) {
        self.properties.buttons_pressed.push(key.code);
        for map in &mut self.nmaps {
            map.try_use(&mut self.properties)
        }
    }

    pub fn add_nmaps(mut self, nmaps: fn() -> Vec<Mapping>) -> Self {
        self.nmaps.append(&mut (nmaps)());
        self
    }

    pub fn add_imaps(mut self, imaps: fn() -> Vec<Mapping>) -> Self {
        self.imaps.append(&mut (imaps)());
        self
    }

    pub fn add_cmaps(mut self, cmaps: fn() -> Vec<Command>) -> Self {
        self.cmaps.append(&mut (cmaps)());
        self
    }

    fn to_normal_mode(&mut self) {
        self.properties.mode = Mode::Normal;
        self.properties.move_cursor_left(1);
    }

    fn get_workspace(&self) -> io::Result<PathBuf> {
        let args: Vec<String> = std::env::args().collect();

        if args.len() > 1 {
            let path = &args[1];
            Ok(std::env::current_dir()?.join(path))
        } else {
            std::env::current_dir()
        }
    }

    fn get_cursor_position(&mut self) -> Vector2<usize> {
        self.properties
            .cursor
            .get_position(self.properties.get_current_buffer())
    }

    fn write_char_to_command(&mut self, char: char) {
        self.command.push(char);
    }

    fn run_commands(&mut self) {
        for command in &mut self.cmaps {
            command.try_use(&mut self.properties, &self.command)
        }
        self.command.clear();
        self.properties.mode = Mode::Normal;
    }

    fn backspace_command(&mut self) {
        self.command.pop();
    }
}
