use crate::helpers::evaluate_new_index;
use crate::http_request::HttpMethod;
use std::collections::HashMap;

pub enum PairInputMode {
    Add,
    Edit(String), // Containing the key of the entry to edit
}

pub struct PairInputState {
    pub mode: PairInputMode,
    pub selected_item: KeyValuePair,
    pub key: String,
    pub value: String,
}

pub enum Section {
    RequestBody(Option<u8>),
    RequestHeaders(Option<u8>),
    ResponseBody(Option<u8>),
    ResponseHeaders(Option<u8>),
}

impl ToString for Section {
    fn to_string(&self) -> String {
        match self {
            Section::RequestBody(_) => "Request Body",
            Section::RequestHeaders(_) => "Request Headers",
            Section::ResponseBody(_) => "Response Body",
            Section::ResponseHeaders(_) => "Response Headers",
        }
        .to_string()
    }
}

pub struct SectionValues {
    pub request_body: HashMap<String, String>,
    pub request_headers: HashMap<String, String>,
    pub response_body: HashMap<String, String>,
    pub response_headers: HashMap<String, String>,
}

pub enum CurrentScreen {
    Main,
    EndpointInput(String), // value before edit
    PairInput(PairInputState),
    // TODO: Add a loading screen
    Loading,
}

pub enum KeyValuePair {
    Key,
    Value,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub endpoint: String,
    pub method: HttpMethod,
    pub section_values: SectionValues,
    pub selected_section: Section,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            endpoint: String::new(),
            method: HttpMethod::GET,
            section_values: SectionValues {
                request_body: HashMap::new(),
                request_headers: HashMap::new(),
                response_body: HashMap::new(),
                response_headers: HashMap::new(),
            },
            selected_section: Section::RequestBody(None),
        }
    }

    pub fn increment_method(&mut self, reverse: bool) {
        match self.method {
            HttpMethod::GET => {
                match reverse {
                    false => {
                        self.method = HttpMethod::POST;
                    }
                    true => {
                        self.method = HttpMethod::DELETE;
                    }
                };
            }
            HttpMethod::POST => {
                match reverse {
                    false => {
                        self.method = HttpMethod::PUT;
                    }
                    true => {
                        self.method = HttpMethod::GET;
                    }
                };
            }
            HttpMethod::PUT => {
                match reverse {
                    false => {
                        self.method = HttpMethod::PATCH;
                    }
                    true => {
                        self.method = HttpMethod::POST;
                    }
                };
            }
            HttpMethod::PATCH => {
                match reverse {
                    false => {
                        self.method = HttpMethod::DELETE;
                    }
                    true => {
                        self.method = HttpMethod::PUT;
                    }
                };
            }
            HttpMethod::DELETE => {
                match reverse {
                    false => {
                        self.method = HttpMethod::GET;
                    }
                    true => {
                        self.method = HttpMethod::PATCH;
                    }
                };
            }
        }
    }

    pub fn increment_selection(&mut self, reverse: bool) {
        match self.current_screen {
            CurrentScreen::Main => {
                match self.selected_section {
                    Section::RequestBody(ref index) => {
                        let selected_section_length = self.section_values.request_body.len() as u8;
                        let selected_index = index.clone();
                        let new_index =
                            evaluate_new_index(selected_index, selected_section_length, reverse);
                        self.selected_section = Section::RequestBody(Some(new_index));
                    }
                    Section::RequestHeaders(ref index) => {
                        let selected_section_length =
                            self.section_values.request_headers.len() as u8;
                        let selected_index = index.clone();
                        let new_index =
                            evaluate_new_index(selected_index, selected_section_length, reverse);
                        self.selected_section = Section::RequestHeaders(Some(new_index));
                    }
                    Section::ResponseBody(ref index) => {
                        let selected_section_length = self.section_values.response_body.len() as u8;
                        let selected_index = index.clone();
                        let new_index =
                            evaluate_new_index(selected_index, selected_section_length, reverse);
                        self.selected_section = Section::ResponseBody(Some(new_index));
                    }
                    Section::ResponseHeaders(ref index) => {
                        let selected_section_length =
                            self.section_values.response_headers.len() as u8;
                        let selected_index = index.clone();
                        let new_index =
                            evaluate_new_index(selected_index, selected_section_length, reverse);
                        self.selected_section = Section::ResponseHeaders(Some(new_index));
                    }
                };
            }
            _ => return,
        }
    }

    pub fn add_item(&mut self) {
        match self.current_screen {
            CurrentScreen::Main => match self.selected_section {
                Section::RequestBody(_) => {
                    self.current_screen = CurrentScreen::PairInput(PairInputState {
                        mode: PairInputMode::Add,
                        selected_item: KeyValuePair::Key,
                        key: String::new(),
                        value: String::new(),
                    });
                }
                Section::RequestHeaders(_) => {
                    self.current_screen = CurrentScreen::PairInput(PairInputState {
                        mode: PairInputMode::Add,
                        selected_item: KeyValuePair::Key,
                        key: String::new(),
                        value: String::new(),
                    });
                }
                _ => return,
            },
            _ => return,
        }
    }

    pub fn edit_item(&mut self) {
        let selected_index = match self.selected_section {
            Section::RequestBody(ref index) => index.clone(),
            Section::RequestHeaders(ref index) => index.clone(),
            _ => return,
        };

        match selected_index {
            Some(edit_index) => {
                let edit_key = self
                    .section_values
                    .request_body
                    .keys()
                    .nth(edit_index as usize)
                    .unwrap()
                    .clone();
                let edit_value = self
                    .section_values
                    .request_body
                    .get(&edit_key)
                    .unwrap()
                    .clone();
                self.current_screen = CurrentScreen::PairInput(PairInputState {
                    mode: PairInputMode::Edit(edit_key.clone()),
                    selected_item: KeyValuePair::Key,
                    key: edit_key,
                    value: edit_value,
                });
            }
            None => return,
        }
    }

    pub fn write_item(&mut self) {
        match self.current_screen {
            CurrentScreen::PairInput(ref mut input_state) => {
                let selected_section_values = match self.selected_section {
                    Section::RequestBody(_) => &mut self.section_values.request_body,
                    Section::RequestHeaders(_) => &mut self.section_values.request_headers,
                    _ => return,
                };

                match input_state.mode {
                    PairInputMode::Add => {
                        selected_section_values
                            .insert(input_state.key.clone(), input_state.value.clone());
                    }
                    PairInputMode::Edit(ref key) => {
                        if input_state.key != *key {
                            selected_section_values.remove(key);
                        }
                        selected_section_values
                            .insert(input_state.key.clone(), input_state.value.clone());
                    }
                };
            }
            _ => return,
        }
    }

    pub fn delete_item(&mut self) {
        let delete_index = match self.selected_section {
            Section::RequestBody(ref index) => index.clone(),
            Section::RequestHeaders(ref index) => index.clone(),
            _ => return,
        };

        match delete_index {
            Some(delete_index) => match self.selected_section {
                Section::RequestBody(_) => {
                    let delete_key = self
                        .section_values
                        .request_body
                        .keys()
                        .nth(delete_index as usize)
                        .unwrap()
                        .clone();
                    self.section_values.request_body.remove(&delete_key);
                }
                Section::RequestHeaders(_) => {
                    let delete_key = self
                        .section_values
                        .request_headers
                        .keys()
                        .nth(delete_index as usize)
                        .unwrap()
                        .clone();
                    self.section_values.request_headers.remove(&delete_key);
                }
                _ => return,
            },
            None => return,
        }
    }

    pub fn increment_section(&mut self, reverse: bool) {
        match self.selected_section {
            Section::RequestBody(_) => match reverse {
                false => {
                    self.selected_section = Section::RequestHeaders(None);
                }
                true => {
                    self.selected_section = Section::ResponseHeaders(None);
                }
            },
            Section::RequestHeaders(_) => match reverse {
                false => {
                    self.selected_section = Section::ResponseBody(None);
                }
                true => {
                    self.selected_section = Section::RequestBody(None);
                }
            },
            Section::ResponseBody(_) => match reverse {
                false => {
                    self.selected_section = Section::ResponseHeaders(None);
                }
                true => {
                    self.selected_section = Section::RequestHeaders(None);
                }
            },
            Section::ResponseHeaders(_) => match reverse {
                false => {
                    self.selected_section = Section::RequestBody(None);
                }
                true => {
                    self.selected_section = Section::ResponseBody(None);
                }
            },
        }
    }

    pub fn toggle_input_field(&mut self) {
        match self.current_screen {
            CurrentScreen::PairInput(ref mut input_state) => {
                match input_state.selected_item {
                    KeyValuePair::Key => input_state.selected_item = KeyValuePair::Value,
                    KeyValuePair::Value => input_state.selected_item = KeyValuePair::Key,
                };
            }
            _ => return,
        };
    }
}
