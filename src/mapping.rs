use crossterm::event::KeyCode;

use crate::jim::JimProperties;

pub struct Mapping {
    buttons: Vec<KeyCode>,
    on_pressed: fn(&mut JimProperties),
}

impl std::fmt::Debug for Mapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IMap")
            .field("buttons", &self.buttons)
            .finish()
    }
}

impl Mapping {
    pub fn new(buttons: Vec<KeyCode>, on_pressed: fn(&mut JimProperties)) -> Self {
        Self {
            buttons,
            on_pressed,
        }
    }

    pub fn try_use(&self, properties: &mut JimProperties) {
        if properties.use_mapping(&self.buttons) {
            (self.on_pressed)(properties);
        }
    }
}

pub trait ToMapping {
    fn to_mapping(&self, on_pressed: fn(&mut JimProperties)) -> Mapping;
}

impl ToMapping for str {
    fn to_mapping(&self, on_pressed: fn(&mut JimProperties)) -> Mapping {
        let mut buttons = vec![];

        let mut special_mode = false;
        let mut special_chars = vec![];

        for char in self.chars() {
            if special_mode {
                if char == '>' {
                    special_mode = false;
                    special_chars.clear();
                }

                special_chars.push(char);
                if special_chars == vec!['l', 'C', 'r'] {
                    buttons.push(KeyCode::Modifier(
                        crossterm::event::ModifierKeyCode::LeftControl,
                    ));
                } else if special_chars == vec!['r', 'C', 'r'] {
                    buttons.push(KeyCode::Modifier(
                        crossterm::event::ModifierKeyCode::RightControl,
                    ));
                } else if special_chars == vec!['E', 's', 'c'] {
                    buttons.push(KeyCode::Esc);
                }
            } else {
                if char == '<' {
                    special_mode = true;
                } else {
                    buttons.push(KeyCode::Char(char));
                }
            }
        }

        Mapping::new(buttons, on_pressed)
    }
}
