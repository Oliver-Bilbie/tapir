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
    let output_value = run_app(&mut terminal, &mut app);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<Option<String>> {
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
                        if let Some(selected_index) = app.selected_index {
                            if selected_index < app.request_headers.len() as u8 - 1 {
                                app.selected_index = Some(selected_index + 1);
                            }
                        } else if app.request_headers.len() > 0 {
                            app.selected_index = Some(0);
                        }
                    }
                    KeyCode::Char('k') => {
                        if let Some(selected_index) = app.selected_index {
                            if selected_index > 0 {
                                app.selected_index = Some(selected_index - 1);
                            }
                        } else if app.request_headers.len() > 0 {
                            app.selected_index = Some(app.request_headers.len() as u8 - 1);
                        }
                    }

                    // Edit values
                    KeyCode::Char('a') => {
                        app.current_screen = CurrentScreen::Input(app::InputState {
                            mode: app::InputMode::Add,
                            selected_item: app::KeyValuePair::Key,
                            key: String::new(),
                            value: String::new(),
                        });
                    }
                    KeyCode::Char('e') => {
                        if let Some(selected_index) = app.selected_index {
                            let selected_key = app
                                .request_headers
                                .keys()
                                .nth(selected_index as usize)
                                .unwrap();
                            let selected_value = app.request_headers.get(selected_key).unwrap();
                            app.current_screen = CurrentScreen::Input(app::InputState {
                                mode: app::InputMode::Edit(selected_key.clone()),
                                selected_item: app::KeyValuePair::Key,
                                key: selected_key.clone(),
                                value: selected_value.clone(),
                            });
                        }
                    }
                    KeyCode::Char('d') => {
                        if let Some(selected_index) = app.selected_index {
                            let selected_key = app
                                .request_headers
                                .keys()
                                .nth(selected_index as usize)
                                .unwrap()
                                .clone();
                            app.delete_item(selected_key);
                        }
                    }

                    // Functions
                    KeyCode::Enter => {
                        app.current_screen = CurrentScreen::Submit;
                    }
                    KeyCode::Char('q') => {
                        return Ok(None);
                    }

                    _ => {}
                },
                CurrentScreen::Submit => match key.code {
                    KeyCode::Char('y') => {
                        let output = app.get_json_output();
                        match output {
                            Ok(output) => return Ok(Some(output)),
                            Err(err) => {
                                // TODO: Impliment error screen
                                // app.current_screen = CurrentScreen::Error(err);
                                return Err(err.into());
                            }
                        }
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(None);
                    }
                    _ => {}
                },
                CurrentScreen::Input(ref mut input_state) if key.kind == KeyEventKind::Press => {
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
