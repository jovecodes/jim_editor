use nalgebra::Vector2;

use crate::file::JimFile;


#[derive(Debug, Default)]
pub struct Cursor {
    pub xy_pos: Vector2<usize>,
    pub index: usize,
}

impl Cursor {
    pub fn new(xy_pos: Vector2<usize>, index: usize) -> Self {
        Self { xy_pos, index }
    }

    pub fn write_to(&mut self, text_to_write: &str, file: &mut JimFile) {
        for char in text_to_write.chars() {
            self.write_char_to(char, file);
        }
    }

    pub fn write_char_to(&mut self, char: char, file: &mut JimFile) {
        file.contents.insert(self.index, char);

        if char == '\n' {
            self.move_down(&file);
            self.xy_pos = self.get_position(&file);
            self.move_full_left();
        } else {
            self.xy_pos.x += 1;
            self.index += 1;
        }
    }

    pub fn backspace(&mut self, file: &mut JimFile) {
        if self.index < 1 {
            return;
        }

        if file.contents.chars().nth(self.index - 1).unwrap() == '\n' {
            self.index -= 1;
            self.xy_pos.y -= 1;
            self.xy_pos.x = self.current_line_length(file).unwrap();
            self.xy_pos = self.get_position(file);
        } else {
            self.move_left(file);
        }
        file.contents.remove(self.index);
    }

    pub fn move_full_left(&mut self) {
        self.index -= self.xy_pos.x;
        self.xy_pos.x = 0;
    }

    pub fn move_right(&mut self, file: &JimFile) {
        if let Some(length) = self.current_line_length(file) {
            if self.xy_pos.x + 1 < length {
                self.index += 1;
                self.xy_pos.x += 1;
            }
        }
    }

    pub fn move_left(&mut self, file: &JimFile) {
        if self.xy_pos.x > 0 {
            let visable_pos = self.get_position(file);
            if visable_pos.x > 0 {
                self.xy_pos = visable_pos;
                self.index -= 1;
                self.xy_pos.x -= 1;
            }
        }
    }

    pub fn move_up(&mut self, file: &JimFile) {
        if self.xy_pos.y < 1 {
            return;
        }

        self.index -= self.get_position(file).x + 1;
        self.xy_pos.y -= 1;

        if let Some(length) = self.current_line_length(file) {
            self.index -= length - self.xy_pos.x.clamp(0, length);
        } else {
            eprintln!("Could not move UP")
        }
    }

    pub fn move_down(&mut self, file: &JimFile) {
        let lines: Vec<&str> = file.contents.lines().collect();
        if lines.len() - 1 <= self.xy_pos.y {
            return;
        }

        if let Some(length) = self.current_line_length(file) {
            self.index += length - self.get_position(file).x + 1;

            self.xy_pos.y += 1;
            self.index += self.get_position(file).x;
        } else {
            eprintln!("Could not move DOWN")
        }
    }

    pub fn force_move_right(&mut self, file: &JimFile) {
        if file.contents.chars().nth(self.index - 1).unwrap() == '\n' {
            return;
        }

        self.xy_pos.x += 1;
        self.index += 1;
    }

    pub fn get_position(&self, file: &JimFile) -> Vector2<usize> {
        let length = self.current_line_length(file).unwrap_or_default();
        Vector2::new(self.xy_pos.x.clamp(0, length), self.xy_pos.y)
    }

    fn current_line_length(&self, file: &JimFile) -> Option<usize> {
        Self::line_length(&file.contents, self.xy_pos.y)
    }

    fn line_length(text: &str, pos: usize) -> Option<usize> {
        let lines: Vec<&str> = text.lines().collect();
        lines.get(pos).map(|line| line.chars().count())
    }
}
