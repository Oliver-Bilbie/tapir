pub fn evaluate_new_index(current: Option<u8>, length: u8, reverse: bool) -> u8 {
    match reverse {
        false => match current {
            Some(current_index) => {
                if current_index < length - 1 {
                    current_index + 1
                } else {
                    length - 1
                }
            }
            None => 0,
        },
        true => match current {
            Some(current_index) => {
                if current_index > 0 {
                    current_index - 1
                } else {
                    0
                }
            }
            None => length - 1,
        },
    }
}
