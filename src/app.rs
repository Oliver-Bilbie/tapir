use std::collections::HashMap;
use serde_json::Result;

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub enum KeyValuePair {
    Key,
    Value,
}

pub struct App {
    pub key_input: String,              // the currently being edited json key.
    pub value_input: String,            // the currently being edited json value.
    pub request_headers: HashMap<String, String>, // The representation of our key and value pairs with serde Serialize support
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub selected_index: u8, // the currently selected key-value pair, which will be used to determine which pair is being edited.
    pub currently_editing: Option<KeyValuePair>, // the optional state containing which of the key or value pair the user is editing. It is an option, because when the user is not directly editing a key-value pair, this will be set to `None`.
}

impl App {
    pub fn new() -> App {
        App {
            key_input: String::new(),
            value_input: String::new(),
            request_headers: HashMap::new(),
            current_screen: CurrentScreen::Main,
            selected_index: 0,
            currently_editing: None,
        }
    }

    pub fn save_key_value(&mut self) {
        self.request_headers
            .insert(self.key_input.clone(), self.value_input.clone());

        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                KeyValuePair::Key => self.currently_editing = Some(KeyValuePair::Value),
                KeyValuePair::Value => self.currently_editing = Some(KeyValuePair::Key),
            };
        } else {
            self.currently_editing = Some(KeyValuePair::Key);
        }
    }

    pub fn print_json(&self) -> Result<()> {
        let output = serde_json::to_string(&self.request_headers)?;
        println!("{}", output);
        Ok(())
    }
}
