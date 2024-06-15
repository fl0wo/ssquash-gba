use crate::SIZE_CELL;

pub fn j_to_y(y: i16) -> i16 {
    ((y as i32) * SIZE_CELL) as i16
}

pub fn i_to_x(x: i16) -> i16 {
    ((x as i32) * SIZE_CELL) as i16
}

pub fn x_to_i(x: i16) -> i16 {
    (x as i32 / SIZE_CELL) as i16
}

pub fn y_to_j(y: i16) -> i16 {
    (y as i32 / SIZE_CELL) as i16
}
