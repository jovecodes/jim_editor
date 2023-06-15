use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

pub mod jim;
pub mod mode;
pub mod mapping;
pub mod builtin_maps;

fn main() -> Result<(), Box<dyn Error>> {
    run()
}

pub fn run() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = create_jim(&mut terminal);
    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn create_jim<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    jim::Jim::new()
        .init()?
        .add_nmaps(builtin_maps::nmaps)
        .add_imaps(builtin_maps::imaps)
        .run(terminal)
}

// fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: Jim) -> io::Result<()> {
//     loop {
//         terminal.draw(|f| ui(f, &app))?;
//
//         if let Event::Key(key) = event::read()? {
//             match app.mode {
//                 InputMode::Normal => match key.code {
//                     KeyCode::Char('e') => {
//                         app.mode = InputMode::Insert;
//                     }
//                     KeyCode::Char('q') => {
//                         return Ok(());
//                     }
//                     _ => {}
//                 },
//                 InputMode::Insert => match key.code {
//                     KeyCode::Enter => {
//                         app.messages.push(app.file_contents.drain(..).collect());
//                     }
//                     KeyCode::Char(c) => {
//                         app.file_contents.push(c);
//                     }
//                     KeyCode::Backspace => {
//                         app.file_contents.pop();
//                     }
//                     KeyCode::Esc => {
//                         app.mode = InputMode::Normal;
//                     }
//                     _ => {}
//                 },
//             }
//         }
//     }
// }
//
// fn ui<B: Backend>(f: &mut Frame<B>, app: &Jim) {
//     let chunks = Layout::default()
//         .direction(Direction::Vertical)
//         .margin(2)
//         .constraints(
//             [
//                 Constraint::Length(1),
//                 Constraint::Length(3),
//                 Constraint::Min(1),
//             ]
//             .as_ref(),
//         )
//         .split(f.size());
//
//     let (msg, style) = match app.mode {
//         InputMode::Normal => (
//             vec![
//                 Span::raw("Press "),
//                 Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
//                 Span::raw(" to exit, "),
//                 Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
//                 Span::raw(" to start editing."),
//             ],
//             Style::default().add_modifier(Modifier::RAPID_BLINK),
//         ),
//         InputMode::Insert => (
//             vec![
//                 Span::raw("Press "),
//                 Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
//                 Span::raw(" to stop editing, "),
//                 Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
//                 Span::raw(" to record the message"),
//             ],
//             Style::default(),
//         ),
//     };
//     let mut text = Text::from(Spans::from(msg));
//     text.patch_style(style);
//     let help_message = Paragraph::new(text);
//     f.render_widget(help_message, chunks[0]);
//
//     let input = Paragraph::new(app.file_contents.as_ref())
//         .style(match app.mode {
//             InputMode::Normal => Style::default(),
//             InputMode::Insert => Style::default().fg(Color::Yellow),
//         })
//         .block(Block::default().borders(Borders::ALL).title("Input"));
//     f.render_widget(input, chunks[1]);
//     match app.mode {
//         InputMode::Normal =>
//             // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
//             {}
//
//         InputMode::Insert => {
//             // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
//             f.set_cursor(
//                 // Put cursor past the end of the input text
//                 1,
//                 // Move one line down, from the border to the input line
//                 chunks[1].y + 1,
//             )
//         }
//     }
//
//     let messages: Vec<ListItem> = app
//         .messages
//         .iter()
//         .enumerate()
//         .map(|(i, m)| {
//             let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
//             ListItem::new(content)
//         })
//         .collect();
//     let messages =
//         List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
//     f.render_widget(messages, chunks[2]);
// }
