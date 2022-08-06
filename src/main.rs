#![no_std]
#![no_main]
// This is required to allow writing tes&ts
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

mod utils;

use agb::{
    display::{
        object::{Graphics, Tag},
        tiled::{RegularBackgroundSize, TileFormat, TileSet, TileSetting},
        Priority,
    },
    input::{Button, ButtonController, Tri},
    mgba,
};
//use utils::abs;

#[agb::entry]
fn agb_main(mut gba: agb::Gba) -> ! {
    main(gba)
}

agb::include_gfx!("gfx/tile_sheet.toml");
const GRAPHICS: &Graphics = agb::include_aseprite!("gfx/square_character.aseprite");
const PLAYER_LEFT: &Tag = GRAPHICS.tags().get("Left");
const PLAYER_RIGHT: &Tag = GRAPHICS.tags().get("Right");
const PLAYER_UP: &Tag = GRAPHICS.tags().get("Up");
const PLAYER_DOWN: &Tag = GRAPHICS.tags().get("Down");

#[derive(Debug, Clone, Copy, PartialEq)]
enum Orientation {
    Left,
    Right,
    Up,
    Down,
}

impl Orientation {
    fn update(&self, horizontal: Tri, vertical: Tri) -> Self {
        if horizontal != Tri::Zero && vertical != Tri::Zero {
            // going diagonal

            if horizontal != self.horizontal() && vertical != self.vertical() {
                // no compatible component, vertical is default
                match vertical {
                    Tri::Negative => return Orientation::Up,
                    Tri::Positive => return Orientation::Down,
                    Tri::Zero => unreachable!(),
                }
            } else {
                // compatible on at least one component, we keep direction
                return *self;
            }
        } else {
            // going in a straight line
            // simply take the direction, or conserve it if zero on both axis
            return match horizontal {
                Tri::Negative => Orientation::Left,
                Tri::Positive => Orientation::Right,
                Tri::Zero => match vertical {
                    Tri::Negative => Orientation::Up,
                    Tri::Positive => Orientation::Down,
                    Tri::Zero => *self,
                },
            };
        }
    }

    fn horizontal(&self) -> Tri {
        match self {
            Self::Left => Tri::Negative,
            Self::Right => Tri::Positive,
            _ => Tri::Zero,
        }
    }

    fn vertical(&self) -> Tri {
        match self {
            Self::Up => Tri::Negative,
            Self::Down => Tri::Positive,
            _ => Tri::Zero,
        }
    }
}

fn main(mut gba: agb::Gba) -> ! {
    let (tiled, mut vram) = gba.display.video.tiled0();
    let objects_controller = gba.display.object.get();

    let mut bg = tiled.background(Priority::P1, RegularBackgroundSize::Background32x32);
    let mut bg2 = tiled.background(Priority::P0, RegularBackgroundSize::Background32x32);

    let tileset = TileSet::new(tile_sheet::grass.tiles, TileFormat::FourBpp);
    vram.set_background_palettes(tile_sheet::grass.palettes);

    for y in 0..20u16 {
        for x in 0..30u16 {
            bg.set_tile(
                &mut vram,
                (x, y).into(),
                &tileset,
                TileSetting::new(0, false, false, 0),
            );

            bg2.set_tile(
                &mut vram,
                (x, y).into(),
                &tileset,
                TileSetting::new(1, false, false, 0),
            );
        }
    }

    bg2.commit(&mut vram);
    bg.commit(&mut vram);

    bg2.show();
    bg.show();

    let v_blank = agb::interrupt::VBlank::get();
    v_blank.wait_for_vblank();

    let mut player = objects_controller.object_sprite(PLAYER_DOWN.animation_sprite(0));
    player.set_x(0);
    player.set_y(0);
    player.set_priority(Priority::P0);

    objects_controller.commit();

    let mut x_pos: u16 = 0;
    let mut x_velocity: i16 = 0;
    let mut y_pos: u16 = 0;
    let mut y_velocity: i16 = 0;

    let mut input = ButtonController::new();
    let mut debug = mgba::Mgba::new().unwrap();

    debug.set_level(mgba::DebugLevel::Debug);

    debug
        .print(
            format_args!("Before entering loop"),
            mgba::DebugLevel::Debug,
        )
        .unwrap();

    let mut last_orientation = Orientation::Down;

    loop {
        // parse controls

        input.update();
        x_pos = match input.x_tri() {
            Tri::Positive => x_pos + 1,
            Tri::Negative => {
                if x_pos == 0 {
                    0
                } else {
                    x_pos - 1
                }
            }
            Tri::Zero => x_pos,
        };

        y_pos = match input.y_tri() {
            Tri::Positive => y_pos + 1,
            Tri::Negative => {
                if y_pos == 0 {
                    0
                } else {
                    y_pos - 1
                }
            }
            Tri::Zero => y_pos,
        };

        let new_orientation = last_orientation.update(input.x_tri(), input.y_tri());

        if new_orientation != last_orientation {
            // we need to change player sprite !
            let tag = match new_orientation {
                Orientation::Left => PLAYER_LEFT,
                Orientation::Right => PLAYER_RIGHT,
                Orientation::Up => PLAYER_UP,
                Orientation::Down => PLAYER_DOWN,
            };

            let sp_borrow = objects_controller.sprite(tag.animation_sprite(0));
            player.set_sprite(sp_borrow);

            last_orientation = new_orientation;
        }

        player.set_x(x_pos);
        player.set_y(y_pos);

        objects_controller.commit();

        v_blank.wait_for_vblank();
    }
}
