// Provided you haven't disabled it, agb does provide an allocator, so it is possible
// to use both the `core` and the `alloc` built in crates.
#![no_std]
// `agb` defines its own `main` function, so you must declare your game's main function
// using the #[agb::entry] proc macro. Failing to do so will cause failure in linking
// which won't be a particularly clear error message.
#![no_main]
// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

extern crate alloc;

use alloc::vec::Vec;
use agb::{include_aseprite, println};

use agb::{
    display::{
        HEIGHT,
        object::{Graphics, OamManaged, Object, Size, Sprite, Tag},
        palette16::Palette16,
        tiled::RegularBackgroundSize, WIDTH,
    },
    display::tiled::{TiledMap, TileFormat, TileSet, TileSetting},
    input::Button,
};
use crate::cell::{Player, VectorI2};

mod cell;
mod math;


// Import the sprites in to this static. This holds the sprite
// and palette data in a way that is manageable by agb.
static GRAPHICS: &Graphics = include_aseprite!("gfx/sprites.aseprite");
static WALL: &Tag = GRAPHICS.tags().get("Paddle End");
static BALL: &Tag = GRAPHICS.tags().get("Ball");

// design a wall map matrix of 1 and 0
// 1 = wall
// 0 = no wall

static WALL_ID: u8 = 0;
static BALL_ID: u8 = 1;
static PLAYER_ID: u8 = 2;

static SIZE_CELL: i32 = 16;

static WALL_MAP: [[u8; 15]; 10] = [
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
];

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    // Get the object manager
    let object = gba.display.object.get_managed();
    let vblank = agb::interrupt::VBlank::get();
    let mut input = agb::input::ButtonController::new();

    // Place this at some point on the screen, (50, 50) for example
    let mut player_sprite = object
        .object_sprite(BALL.sprite(0));

    let mut player = Player::new(player_sprite);

    player.object.show();

    object.commit();

    let walls = gen_walls(&object);

    let mut x_velocity = 0;
    let mut y_velocity = 0;


    loop {
        input.update();

        // if w a s d (up, left, down, right) is pressed, move the player
        if input.is_just_pressed(Button::LEFT) {
            player.move_left();
        }
        if input.is_just_pressed(Button::RIGHT) {
            player.move_right();
        }
        if input.is_just_pressed(Button::UP) {
            player.move_up();
        }
        if input.is_just_pressed(Button::DOWN) {
            player.move_down();
        }

        // Wait for vblank, then commit the objects to the screen
        agb::display::busy_wait_for_vblank();
        object.commit();
    }
}

fn gen_walls<'a>(object: &'a OamManaged<'a>) -> Vec<Object<'a>>{

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
