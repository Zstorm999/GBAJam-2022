#![no_std]
#![no_main]
// This is required to allow writing tes&ts
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

mod player;
mod utils;

use agb::{
    display::{
        tiled::{RegularBackgroundSize, TileFormat, TileSet, TileSetting},
        Priority,
    },
    fixnum::Vector2D,
    input::ButtonController,
    mgba,
};

use player::Player;
use utils::Orientation;

agb::include_gfx!("gfx/tile_sheet.toml");

#[agb::entry]
fn agb_main(mut gba: agb::Gba) -> ! {
    main(gba)
}

fn main(mut gba: agb::Gba) -> ! {
    // initialize all the GBA hardware
    let (tiled, mut vram) = gba.display.video.tiled0();
    let objects_controller = gba.display.object.get();
    let v_blank = agb::interrupt::VBlank::get();
    let mut input = ButtonController::new();

    let mut _debug = mgba::Mgba::new().unwrap();

    let mut parallax = tiled.background(Priority::P3, RegularBackgroundSize::Background32x32);
    let mut background = tiled.background(Priority::P2, RegularBackgroundSize::Background32x32);
    // foreground
    // ui_layer

    let tileset = TileSet::new(tile_sheet::grass.tiles, TileFormat::FourBpp);
    vram.set_background_palettes(tile_sheet::grass.palettes);

    for y in 0..20u16 {
        for x in 0..30u16 {
            parallax.set_tile(
                &mut vram,
                (x, y).into(),
                &tileset,
                TileSetting::new(0, false, false, 0),
            );

            background.set_tile(
                &mut vram,
                (x, y).into(),
                &tileset,
                TileSetting::new(1, false, false, 0),
            );
        }
    }

    parallax.commit(&mut vram);
    background.commit(&mut vram);

    parallax.show();
    background.show();

    let mut player = Player::new(Orientation::Down, Vector2D::new(0, 0), &objects_controller);
    objects_controller.commit();

    v_blank.wait_for_vblank();

    loop {
        // parse controls
        input.update();

        // update entities
        player.update(&input, &objects_controller);

        // update backgrounds

        // show entities and backgrounds
        objects_controller.commit();

        v_blank.wait_for_vblank();
    }
}
