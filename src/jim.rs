use crate::{mapping::Mapping, mode::Mode};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use nalgebra::Vector2;
use std::{io, path::PathBuf};
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
    nmaps: Vec<Mapping>,
    imaps: Vec<Mapping>,
}

#[derive(Debug, Default)]
pub struct JimProperties {
    pub file_contents: String,
    pub mode: Mode,
    pub cursor: Cursor,
    pub quitting: bool,
    pub buttons_pressed: Vec<KeyCode>,
}

impl JimProperties {
    pub fn use_mapping(&mut self, buttons: &Vec<KeyCode>) -> bool {
        let contains = self
            .buttons_pressed
            .windows(buttons.len())
            .any(|window| window == buttons);
        if contains {
            self.buttons_pressed.clear();
        }
        return contains;
    }

    pub fn move_cursor_down(&mut self, amount: u16) {
        for _ in 0..amount {
            self.cursor.move_down(&self.file_contents)
        }
    }

    pub fn move_cursor_up(&mut self, amount: u16) {
        for _ in 0..amount {
            self.cursor.move_up(&self.file_contents)
        }
    }

    pub fn move_cursor_right(&mut self, amount: u16) {
        for _ in 0..amount {
            self.cursor.move_right(&self.file_contents)
        }
    }

    pub fn move_cursor_left(&mut self, amount: u16) {
        for _ in 0..amount {
            self.cursor.move_left(&self.file_contents)
        }
    }

    pub fn current_line(&self) -> &str {
        &self
            .file_contents
            .lines()
            .nth(self.cursor.xy_pos.y as usize)
            .unwrap()
    }
}

impl Jim {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init(mut self) -> io::Result<Self> {
        self.load_file(self.get_workspace()?)?;

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
                }
            }
        }
        Ok(())
    }

    fn render<B: Backend>(&mut self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([Constraint::Min(0)].as_ref())
            .split(f.size());

        // let mut text = Text::from(Spans::from(self.file_contents.clone()));
        // text.patch_style(Style::default());
        // let help_message = Paragraph::new(text);
        // f.render_widget(help_message, chunks[0]);

        let text = Text::raw(
            self.properties.file_contents.clone()
                + "\n"
                + &self.properties.cursor.index.to_string()
                + "\n"
                + &self.properties.cursor.xy_pos.to_string(),
        );
        let file = Paragraph::new(text);
        f.render_widget(file, chunks[0]);

        let cursor_pos = self
            .properties
            .cursor
            .get_position(&self.properties.file_contents);
        f.set_cursor(cursor_pos.x as u16, cursor_pos.y as u16)
    }

    fn insert(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('e') => self.to_normal_mode(),
            KeyCode::Char(char) => self
                .properties
                .cursor
                .write_char_to(char, &mut self.properties.file_contents),
            KeyCode::Enter => self
                .properties
                .cursor
                .write_char_to('\n', &mut self.properties.file_contents),
            KeyCode::Backspace => self
                .properties
                .cursor
                .backspace(&mut self.properties.file_contents),
            _ => todo!(),
        }
    }

    fn to_normal_mode(&mut self) {
        self.properties.mode = Mode::Normal;
        self.properties
            .cursor
            .move_left(&self.properties.file_contents);
    }

    fn normal(&mut self, key: KeyEvent) {
        self.properties.buttons_pressed.push(key.code);
        for map in &mut self.nmaps {
            map.try_use(&mut self.properties)
        }
        if key.code == KeyCode::Char('q') {
            self.properties.quitting = true;
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

    fn load_file(&mut self, file: PathBuf) -> io::Result<()> {
        self.properties.file_contents = std::fs::read_to_string(file)?;
        Ok(())
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
}

#[derive(Debug, Default)]
pub struct Cursor {
    pub xy_pos: Vector2<usize>,
    pub index: usize,
}

impl Cursor {
    pub fn new(xy_pos: Vector2<usize>, index: usize) -> Self {
        Self { xy_pos, index }
    }

    pub fn write_to(&mut self, text_to_write: &str, file_text: &mut String) {
        for char in text_to_write.chars() {
            self.write_char_to(char, file_text);
        }
    }

    pub fn write_char_to(&mut self, char: char, file_text: &mut String) {
        file_text.insert(self.index, char);

        if char == '\n' {
            self.move_down(&file_text);
            self.move_full_left();
            self.index += self.current_line_length(&file_text).unwrap();
        } else {
            self.xy_pos.x += 1;
            self.index += 1;
        }
    }

    fn backspace(&mut self, file_contents: &mut String) {
        if file_contents.chars().nth(self.index - 1).unwrap() == '\n' {
            self.index -= 1;
            self.xy_pos.y -= 1;
            self.xy_pos.x = self.current_line_length(&file_contents).unwrap();
            self.xy_pos = self.get_position(&file_contents);
        } else {
            self.move_left(&file_contents);
        }
        file_contents.remove(self.index);
    }

    pub fn move_full_left(&mut self) {
        self.index -= self.xy_pos.x as usize;
        self.xy_pos.x = 0;
    }

    pub fn move_right(&mut self, text: &str) {
        if let Some(length) = self.current_line_length(text) {
            if self.xy_pos.x + 1 < length {
                self.index += 1;
                self.xy_pos.x += 1;
            }
        }
    }

    pub fn move_left(&mut self, text: &str) {
        if self.xy_pos.x > 0 {
            let visable_pos = self.get_position(text);
            if visable_pos.x > 0 {
                self.xy_pos = visable_pos;
                self.index -= 1;
                self.xy_pos.x -= 1;
            }
        }
    }

    pub fn move_up(&mut self, text: &str) {
        if self.xy_pos.y < 1 {
            return;
        }

        self.index -= self.get_position(text).x as usize + 1;
        self.xy_pos.y -= 1;

        if let Some(length) = self.current_line_length(text) {
            self.index -= length - self.xy_pos.x.clamp(0, length);
        } else {
            eprintln!("Could not move UP")
        }
    }

    pub fn move_down(&mut self, text: &str) {
        let lines: Vec<&str> = text.lines().collect();
        if lines.len() - 1 <= self.xy_pos.y as usize {
            return;
        }

        if let Some(length) = self.current_line_length(text) {
            self.index += length - self.get_position(text).x as usize + 1;

            self.xy_pos.y += 1;
            self.index += self.get_position(text).x as usize;
        } else {
            eprintln!("Could not move DOWN")
        }
    }

    pub fn get_position(&self, text: &str) -> Vector2<usize> {
        let length = self.current_line_length(text).unwrap();
        Vector2::new(self.xy_pos.x.clamp(0, length), self.xy_pos.y)
    }

    fn current_line_length(&self, text: &str) -> Option<usize> {
        Self::line_length(text, self.xy_pos.y as usize)
    }

    fn line_length(text: &str, pos: usize) -> Option<usize> {
        let lines: Vec<&str> = text.lines().collect();
        if let Some(line) = lines.get(pos) {
            Some(line.chars().count())
        } else {
            None
        }
    }
}
