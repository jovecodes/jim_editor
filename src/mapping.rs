use crossterm::event::KeyCode;

use crate::jim::JimProperties;

pub struct Mapping {
    buttons: Vec<KeyCode>,
    wait_for_next_press: bool,
    already_pressed: bool,
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
    pub fn new(
        buttons: Vec<KeyCode>,
        on_pressed: fn(&mut JimProperties),
        wait_for_next_press: bool,
    ) -> Self {
        Self {
            buttons,
            on_pressed,
            wait_for_next_press,
            already_pressed: false,
        }
    }

    pub fn try_use(&mut self, properties: &mut JimProperties) {
        if properties.cant_press_maps && !self.already_pressed {
            return;
        }
        if properties.use_mapping(&self.buttons) || self.already_pressed {
            if !self.wait_for_next_press || self.already_pressed {
                self.already_pressed = false;
                properties.cant_press_maps = false;
                (self.on_pressed)(properties);
            } else if self.wait_for_next_press {
                self.already_pressed = true;
                properties.cant_press_maps = true;
            }
            properties.buttons_pressed.clear();
        }
    }
}

pub trait ToMapping {
    fn to_mapping(&self, on_pressed: fn(&mut JimProperties), wait_for_next_press: bool) -> Mapping;
}

impl ToMapping for str {
    fn to_mapping(&self, on_pressed: fn(&mut JimProperties), wait_for_next_press: bool) -> Mapping {
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
            } else if char == '<' {
                special_mode = true;
            } else {
                buttons.push(KeyCode::Char(char));
            }
        }

        Mapping::new(buttons, on_pressed, wait_for_next_press)
    }
}
