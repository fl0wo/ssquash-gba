use agb::display::object::{OamManaged, Object};
use alloc::vec::Vec;
use crate::{math, WALL, WALL_MAP};
use crate::math::get_cell_of_map_row;

pub fn gen_walls<'a>(object: &'a OamManaged<'a>) -> Vec<Object<'a>>{

    // based on the WALL_MAP matrix, generate the walls
    let mut walls: Vec<Object> = Vec::new();
    for (i, row) in WALL_MAP.iter().enumerate() {
        // for each bit in row (each cell)

        // for the 16 bits in the row
        for j in 0..16 {
            let cell = get_cell_of_map_row(row, j); // this gets the i-th bit of the row

            if cell == 1 {
                let mut wall = object.object_sprite(WALL.sprite(0));
                wall
                    .set_x(math::j_to_y(j) as u16)
                    .set_y(math::i_to_x(i as i16) as u16)
                    .show();
                walls.push(wall);
            }
        }
    }

    return walls;
}
