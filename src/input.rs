use agb::input::{Button, ButtonController};
use crate::cell::Player;

pub fn check_movement(input: &mut ButtonController, player: &mut Player) {
    input.update();
    if input.is_pressed(Button::LEFT) {
        player.move_left();
    }
    if input.is_pressed(Button::RIGHT) {
        player.move_right();
    }
    if input.is_pressed(Button::UP) {
        player.move_up();
    }
    if input.is_pressed(Button::DOWN) {
        player.move_down();
    }
}
