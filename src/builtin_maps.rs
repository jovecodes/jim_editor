use crossterm::event::KeyCode;

use crate::{
    jim::JimProperties,
    mapping::{Mapping, ToMapping},
    mode::Mode,
};

pub fn nmaps() -> Vec<Mapping> {
    vec![
        "i".to_mapping(i, false),
        "h".to_mapping(h, false),
        "j".to_mapping(j, false),
        "k".to_mapping(k, false),
        "l".to_mapping(l, false),
        "f".to_mapping(f, true),
        "t".to_mapping(t, true),
        "a".to_mapping(a, false),
    ]
}

pub fn imaps() -> Vec<Mapping> {
    vec![]
}

fn i(jim: &mut JimProperties) {
    jim.mode = Mode::Insert;
}

fn h(jim: &mut JimProperties) {
    jim.move_cursor_left(1);
}

fn j(jim: &mut JimProperties) {
    jim.move_cursor_down(1);
}

fn k(jim: &mut JimProperties) {
    jim.move_cursor_up(1);
}

fn l(jim: &mut JimProperties) {
    jim.move_cursor_right(1);
}

fn f(jim: &mut JimProperties) {
    if let Some(distance) = distance_to_next_button(jim) {
        jim.move_cursor_right(distance + 1);
    }
}

fn t(jim: &mut JimProperties) {
    if let Some(distance) = distance_to_next_button(jim) {
        jim.move_cursor_right(distance);
    }
}

fn a(jim: &mut JimProperties) {
    jim.force_move_right(1);
    jim.mode = Mode::Insert;
}

fn distance_to_next_button(jim: &mut JimProperties) -> Option<usize> {
    let chars: Vec<char> = jim.current_line().chars().skip(jim.cursor.xy_pos.x + 1).collect();
    let target = match jim.buttons_pressed.last().unwrap() {
        KeyCode::Char(char) => char,
        _ => todo!()
    };

    return find_char_position(&chars, *target);
}

fn find_char_position(chars: &[char], target: char) -> Option<usize> {
    chars.iter().position(|&c| c == target)
}
