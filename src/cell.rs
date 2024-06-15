use agb::display::object::Object;
use crate::{math};
use crate::math::get_cell_of_map_row;


pub struct VectorI2 {
    pub(crate) f: i16,
    pub(crate) s: i16,
}

pub struct VectorU2 {
    pub(crate) f: u16,
    pub(crate) s: u16,
}

/**
An Item is a struct that holds an object and a position.
*/
pub struct Player<'a> {
    pub(crate) object: Object<'a>,
    pub(crate) coordinates: VectorI2,
    pub(crate) direction: VectorI2,
    map: [u16; 10],
}

impl Player<'_> {

    pub(crate) fn new(object: Object,map: [u16; 10]) -> Player {
        let mut x = Player {
            object,
            coordinates: VectorI2 {
                f: 1,
                s: 1
            },
            direction: VectorI2 {
                f: 0,
                s: 0
            },
            // TODO: are you sure this is the right way to do this?
            map: map
        };

        x.update_pos();
        x.object.show();

        x
    }

    pub(crate) fn row(&self) -> i16 {
        self.coordinates.f
    }

    pub(crate) fn column(&self) -> i16 {
        self.coordinates.s
    }

    fn update_pos(&mut self) {
        self.object.set_x(math::j_to_y(self.column()) as u16);
        self.object.set_y(math::i_to_x(self.row())as u16);
    }

    fn set_row(&mut self, row: i16) {
        if(self.is_wall(row ,self.column())) {
            // reset direction to 0
            self.reset_direction();
            return;
        }
        self.direction.f = row-self.row();
        self.coordinates.f = row;
        self.update_pos()
    }

    fn set_column(&mut self, col: i16) {
        if(self.is_wall(self.row(),col)) {
            self.reset_direction();
            return;
        }
        self.direction.s = col-self.column();
        self.coordinates.s = col;
        self.update_pos();
    }

    fn is_wall(&mut self, row:i16,col: i16) -> bool {
        get_cell_of_map_row(&self.map[row as usize], col) == 1
    }

    pub(crate) fn move_left(&mut self) {
        if(self.is_moving()) {
            return;
        }
        self.set_column(self.column() - 1);
    }

    pub(crate) fn move_right(&mut self) {
        if(self.is_moving()) {
            return;
        }
        self.set_column(self.column() + 1);
    }

    pub(crate) fn move_up(&mut self) {
        if(self.is_moving()) {
            return;
        }
        self.set_row(self.row() - 1);
    }

    pub(crate) fn move_down(&mut self) {
        if(self.is_moving()) {
            return;
        }
        self.set_row(self.row() + 1);
    }

    pub(crate) fn apply_direction(&mut self) {
        if(self.direction.f == 0 && self.direction.s == 0) {
            return;
        }

        // move the player in the direction
        self.set_row(self.row() + (self.direction.f));
        self.set_column(self.column() + (self.direction.s));
    }

    /**
    Stops the player from moving, allowing the player to move in a different direction.
    */
    fn reset_direction(&mut self) {
        self.direction = VectorI2 {
            f: 0,
            s: 0
        }
    }

    fn is_moving(&self) -> bool {
        self.direction.f != 0 || self.direction.s != 0
    }
}