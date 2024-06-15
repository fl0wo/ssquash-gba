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
use agb::input::ButtonController;
use crate::cell::{Player, VectorI2};

mod cell;
mod math;
mod input;
mod map;


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

static WALL_MAP: [u16; 10] = [
    32767, // 111111111111111 in binary or can save the number as 0xFFFF
    16385, // 100000000000001 in binary
    16385, // ...
    16385,
    16385,
    16385,
    16385,
    16385,
    16385,
    32765, // 111111111111101 in binary
];

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    // Get the object manager
    let object = gba.display.object.get_managed();
    let vblank = agb::interrupt::VBlank::get();
    let mut input = ButtonController::new();

    // Place this at some point on the screen, (50, 50) for example
    let player_sprite = object
        .object_sprite(BALL.sprite(0));

    let mut player = Player::new(player_sprite, WALL_MAP);

    object.commit();

    let walls = map::gen_walls(&object);

    loop {
        // if w a s d (up, left, down, right) is pressed, move the player
        input::check_movement(&mut input, &mut player);

        // Wait for vblank, then commit the objects to the screen
        agb::display::busy_wait_for_vblank();
        object.commit();
    }
}
