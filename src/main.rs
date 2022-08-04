#![no_std]
#![no_main]
// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

mod utils;

use agb::{
    display::{
        tiled::{RegularBackgroundSize, TileFormat, TileSet, TileSetting},
        Priority,
    },
    rng, syscall,
};
use utils::abs;

#[agb::entry]
fn agb_main(mut gba: agb::Gba) -> ! {
    main(gba)
}

agb::include_gfx!("gfx/tile_sheet.toml");

fn main(mut gba: agb::Gba) -> ! {
    let (tiled, mut vram) = gba.display.video.tiled0();

    let mut bg = tiled.background(Priority::P0, RegularBackgroundSize::Background32x32);

    let tileset = TileSet::new(tile_sheet::grass.tiles, TileFormat::FourBpp);
    vram.set_background_palettes(tile_sheet::grass.palettes);

    for y in 0..20u16 {
        for x in 0..30u16 {
            let nb = abs(rng::gen() % 2) as u16;

            bg.set_tile(
                &mut vram,
                (x, y).into(),
                &tileset,
                TileSetting::new(nb, false, false, 0),
            )
        }
    }

    bg.commit(&mut vram);
    bg.show();

    loop {
        syscall::halt();
    }
}
