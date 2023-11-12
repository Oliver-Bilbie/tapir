use serde_json::Result;
use std::collections::HashMap;

pub enum InputMode {
    Add,
    Edit(String), // Containing the key of the entry to edit
}

pub struct InputState {
    pub mode: InputMode,
    pub selected_item: KeyValuePair,
    pub key: String,
    pub value: String,
}

pub enum CurrentScreen {
    Main,
    Input(InputState),
    Submit,
}

pub enum KeyValuePair {
    Key,
    Value,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub request_headers: HashMap<String, String>,
    pub selected_index: Option<u8>,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            request_headers: HashMap::new(),
            selected_index: None,
        }
    }

    pub fn write_item(&mut self) {
        match self.current_screen {
            CurrentScreen::Input(ref mut input_state) => {
                match input_state.mode {
                    InputMode::Add => {
                        self.request_headers
                            .insert(input_state.key.clone(), input_state.value.clone());
                    }
                    InputMode::Edit(ref key) => {
                        if input_state.key != *key {
                            self.request_headers.remove(key);
                        }
                        self.request_headers
                            .insert(input_state.key.clone(), input_state.value.clone());
                    }
                };
            }
            _ => {}
        }
    }

    pub fn delete_item(&mut self, key: String) {
        match self.current_screen {
            CurrentScreen::Main => {
                self.request_headers.remove(&key);
            }
            _ => {}
        }
    }

    pub fn toggle_input_field(&mut self) {
        match self.current_screen {
            CurrentScreen::Input(ref mut input_state) => {
                match input_state.selected_item {
                    KeyValuePair::Key => input_state.selected_item = KeyValuePair::Value,
                    KeyValuePair::Value => input_state.selected_item = KeyValuePair::Key,
                };
            }
            _ => {}
        };
    }

    pub fn get_json_output(&self) -> Result<String> {
        let output = serde_json::to_string(&self.request_headers)?;
        Ok(output)
    }
}
