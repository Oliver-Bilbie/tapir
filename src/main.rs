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
mod ui;
use crate::{
    app::{App, CurrentScreen, KeyValuePair},
    ui::ui,
};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let save_values = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // TODO: Actually save the values. For now, just print them.
    match save_values {
        Ok(true) => {
            app.print_json()?;
        }
        Ok(false) => {
            println!("Exiting without saving");
        }
        Err(_) => {
            // TODO: Impliment recovery file
            println!("Saving to recovery file");
        }
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
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
                    KeyCode::Char('j') => {
                        if app.selected_index < app.request_headers.len() as u8 - 1 {
                            app.selected_index += 1;
                        }
                    }
                    KeyCode::Char('k') => {
                        if app.selected_index > 0 {
                            app.selected_index -= 1;
                        }
                    }

                    // CRUD
                    KeyCode::Char('a') => {
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = Some(KeyValuePair::Key);
                    }
                    KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = app.request_headers.keys().nth(app.selected_index as usize).map(|_| KeyValuePair::Value);
                    }

                    // Functions
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    _ => {}
                },
                CurrentScreen::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                KeyValuePair::Key => {
                                    app.currently_editing = Some(KeyValuePair::Value);
                                }
                                KeyValuePair::Value => {
                                    app.save_key_value();
                                    app.current_screen = CurrentScreen::Main;
                                }
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                KeyValuePair::Key => {
                                    app.key_input.pop();
                                }
                                KeyValuePair::Value => {
                                    app.value_input.pop();
                                }
                            }
                        }
                    }
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                        app.currently_editing = None;
                    }
                    KeyCode::Tab => {
                        app.toggle_editing();
                    }
                    KeyCode::Char(value) => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                KeyValuePair::Key => {
                                    app.key_input.push(value);
                                }
                                KeyValuePair::Value => {
                                    app.value_input.push(value);
                                }
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
