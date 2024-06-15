use agb::display::object::Object;
use crate::{math};


pub struct VectorI2 {
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
}

impl Player<'_> {

    // constructor
    pub(crate) fn new(object: Object) -> Player {
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
        };

        x.update_pos();

        x
    }

    fn row(&self) -> u16 {
        self.coordinates.f
    }

    fn column(&self) -> u16 {
        self.coordinates.s
    }

    fn update_pos(&mut self) {
        self.object.set_x(math::i_to_x(self.row()));
        self.object.set_y(math::j_to_y(self.column()));
    }

    fn set_row(&mut self, row: u16) {
        self.coordinates.f = row;
        self.update_pos()
    }

    fn set_column(&mut self, col: u16) {
        self.coordinates.s = col;
        self.update_pos();
    }

    pub(crate) fn move_left(&mut self) {
        self.set_row(self.row() - 1);
    }

    pub(crate) fn move_right(&mut self) {
        self.set_row(self.row() + 1);
    }

    pub(crate) fn move_up(&mut self) {
        self.set_column(self.column() - 1);
    }

    pub(crate) fn move_down(&mut self) {
        self.set_column(self.column() + 1);
    }
}