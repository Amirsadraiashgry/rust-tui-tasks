mod app;
mod storage;
mod ui;

use std::io::{self, Stdout};
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, widgets::ListState, Terminal};

use app::App;

type Term = Terminal<CrosstermBackend<Stdout>>;

fn setup_terminal() -> io::Result<Term> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout))
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)
}

fn main() -> io::Result<()> {
    // در صورت panic هم ترمینال به حالت اولیه برگردد
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = restore_terminal();
        default_hook(info);
    }));

    let mut terminal = setup_terminal()?;
    let mut app = App::new(storage::default_path());
    let result = run(&mut terminal, &mut app);

    restore_terminal()?;
    terminal.show_cursor()?;
    result
}

fn run(terminal: &mut Term, app: &mut App) -> io::Result<()> {
    let mut list_state = ListState::default();
    let mut needs_draw = true;

    while !app.should_quit {
        if needs_draw {
            terminal.draw(|f| ui::draw(f, app, &mut list_state))?;
            needs_draw = false;
        }

        // poll با timeout تا CPU مصرف نشود
        if !event::poll(Duration::from_millis(100))? {
            continue;
        }

        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                handle_key(app, key.code, key.modifiers);
                needs_draw = true;
            }
            Event::Resize(_, _) => needs_draw = true,
            _ => {}
        }
    }
    Ok(())
}

fn handle_key(app: &mut App, code: KeyCode, mods: KeyModifiers) {
    if mods.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
        app.should_quit = true;
        return;
    }

    if app.input_mode {
        match code {
            KeyCode::Enter => app.submit_input(),
            KeyCode::Esc => app.cancel_input(),
            KeyCode::Backspace => {
                app.input_buffer.pop();
            }
            KeyCode::Char(c) => app.input_buffer.push(c),
            _ => {}
        }
        return;
    }

    match code {
        KeyCode::Up | KeyCode::Char('k') => app.move_up(),
        KeyCode::Down | KeyCode::Char('j') => app.move_down(),
        KeyCode::Tab => app.toggle_selected(),
        KeyCode::Char('a') => app.start_input(),
        KeyCode::Char('d') => app.delete_selected(),
        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
        _ => {}
    }
}
