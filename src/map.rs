use agb::display::object::{OamManaged, Object};
use alloc::vec::Vec;
use crate::{math, WALL, WALL_MAP};

pub fn gen_walls<'a>(object: &'a OamManaged<'a>) -> Vec<Object<'a>>{

    // based on the WALL_MAP matrix, generate the walls
    let mut walls: Vec<Object> = Vec::new();
    for (y, row) in WALL_MAP.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == 1 {
                let mut wall = object.object_sprite(WALL.sprite(0));
                wall
                    .set_x(math::i_to_x(x as u16))
                    .set_y(math::j_to_y(y as u16))
                    .show();
                walls.push(wall);
            }
        }
    }

    return walls;
}
