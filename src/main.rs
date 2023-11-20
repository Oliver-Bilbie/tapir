use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod app;
mod helpers;
mod http_request;
mod ui;

use crate::{
    app::{App, CurrentScreen, KeyValuePair},
    ui::ui,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let output_value = run_app(&mut terminal, &mut app).await;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Some(output) = output_value? {
        println!("{}", output);
    }

    Ok(())
}

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<Option<String>> {
    loop {
        terminal.draw(|frame| ui::<B>(frame, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    // Navigation
                    KeyCode::Char(']') => app.increment_section(false),
                    KeyCode::Char('[') => app.increment_section(true),
                    KeyCode::Char('j') => app.increment_selection(false),
                    KeyCode::Char('k') => app.increment_selection(true),

                    // Edit values
                    KeyCode::Tab => {
                        app.current_screen = CurrentScreen::EndpointInput(app.endpoint.clone());
                    }
                    KeyCode::Char('a') => {
                        app.add_item();
                    }
                    KeyCode::Char('e') => {
                        app.edit_item();
                    }
                    KeyCode::Char('d') => {
                        app.delete_item();
                    }
                    KeyCode::Char('m') => {
                        app.increment_method(false);
                    }
                    KeyCode::Char('n') => {
                        app.increment_method(true);
                    }

                    // Functions
                    KeyCode::Enter => app.send_api_request().await,
                    KeyCode::Char('q') => {
                        return Ok(None);
                    }

                    _ => {}
                },
                CurrentScreen::Loading => match key.code {
                    KeyCode::Char('y') => return Ok(Some(":)".to_string())),
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(None);
                    }
                    _ => {}
                },
                CurrentScreen::EndpointInput(ref previous_endpoint) => match key.code {
                    KeyCode::Enter => {
                        app.current_screen = CurrentScreen::Main;
                    }
                    KeyCode::Backspace => {
                        app.endpoint.pop();
                    }
                    KeyCode::Esc => {
                        app.endpoint = previous_endpoint.clone();
                        app.current_screen = CurrentScreen::Main;
                    }
                    KeyCode::Char(value) => {
                        app.endpoint.push(value);
                    }
                    _ => {}
                },
                CurrentScreen::PairInput(ref mut input_state)
                    if key.kind == KeyEventKind::Press =>
                {
                    match key.code {
                        KeyCode::Enter => {
                            app.write_item();
                            app.current_screen = CurrentScreen::Main;
                        }
                        KeyCode::Backspace => match input_state.selected_item {
                            KeyValuePair::Key => {
                                input_state.key.pop();
                            }
                            KeyValuePair::Value => {
                                input_state.value.pop();
                            }
                        },
                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Main;
                        }
                        KeyCode::Tab => {
                            app.toggle_input_field();
                        }
                        KeyCode::Char(value) => match input_state.selected_item {
                            KeyValuePair::Key => {
                                input_state.key.push(value);
                            }
                            KeyValuePair::Value => {
                                input_state.value.push(value);
                            }
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}
