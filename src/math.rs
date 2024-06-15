use crate::SIZE_CELL;

pub fn j_to_y(y: u16) -> u16 {
    ((y as i32) * SIZE_CELL) as u16
}

pub fn i_to_x(x: u16) -> u16 {
    ((x as i32) * SIZE_CELL) as u16
}

pub fn x_to_i(x: u16) -> u16 {
    (x as i32 / SIZE_CELL) as u16
}

pub fn y_to_j(y: u16) -> u16 {
    (y as i32 / SIZE_CELL) as u16
}
