use crate::SIZE_CELL;

pub fn j_to_y(y: i16) -> i16 {
    // TODO: understand why we need this -1
    (((y-1) as i32) * SIZE_CELL) as i16
}

pub fn i_to_x(x: i16) -> i16 {
    (((x) as i32) * SIZE_CELL) as i16
}

pub fn x_to_i(x: i16) -> i16 {
    (x as i32 / SIZE_CELL) as i16
}

pub fn y_to_j(y: i16) -> i16 {
    (y as i32 / SIZE_CELL) as i16
}

pub fn get_cell_of_map_row(row: &u16, j: i16) -> u16 {
    (((*row >> ((15 - j) as u16) & 1) as i16)) as u16
}