use crate::{
    jim::JimProperties,
    mapping::{Mapping, ToMapping},
    mode::Mode,
};

pub fn nmaps() -> Vec<Mapping> {
    vec![
        "i".to_mapping(i),
        "h".to_mapping(h),
        "j".to_mapping(j),
        "k".to_mapping(k),
        "l".to_mapping(l),
    ]
}

pub fn imaps() -> Vec<Mapping> {
    vec![]
}

fn i(jim: &mut JimProperties) {
    jim.mode = Mode::Insert
}

fn h(jim: &mut JimProperties) {
    jim.move_cursor_left(1)
}

fn j(jim: &mut JimProperties) {
    jim.move_cursor_down(1)
}

fn k(jim: &mut JimProperties) {
    jim.move_cursor_up(1)
}

fn l(jim: &mut JimProperties) {
    jim.move_cursor_right(1)
}

fn f(jim: &mut JimProperties) {
    // for i in jim.current_line().chars().skip(jim.cursor.xy_pos.x as usize).find() {
    //
    // }
}
