use crate::utils::Orientation;
use agb::{
    display::{
        object::{Graphics, Object, ObjectController, Tag},
        Priority,
    },
    fixnum::Vector2D,
    input::{ButtonController, Tri},
};

pub struct Player<'a> {
    orientation: Orientation,
    position: Vector2D<u16>,
    sprite: Object<'a>,
}

const GRAPHICS: &Graphics = agb::include_aseprite!("gfx/square_character.aseprite");
const PLAYER_LEFT: &Tag = GRAPHICS.tags().get("Left");
const PLAYER_RIGHT: &Tag = GRAPHICS.tags().get("Right");
const PLAYER_UP: &Tag = GRAPHICS.tags().get("Up");
const PLAYER_DOWN: &Tag = GRAPHICS.tags().get("Down");

impl<'a> Player<'a> {
    pub fn oriented_tag(orientation: Orientation) -> &'static Tag {
        match orientation {
            Orientation::Left => PLAYER_LEFT,
            Orientation::Right => PLAYER_RIGHT,
            Orientation::Up => PLAYER_UP,
            Orientation::Down => PLAYER_DOWN,
        }
    }

    pub fn new(
        orientation: Orientation,
        position: Vector2D<u16>,
        controller: &'a ObjectController,
    ) -> Self {
        let mut sprite =
            controller.object_sprite(Player::oriented_tag(orientation).animation_sprite(0));
        sprite.set_priority(Priority::P2);

        Player {
            orientation,
            position,
            sprite,
        }
    }

    pub fn update(&mut self, input: &ButtonController, controller: &'a ObjectController) {
        self.position.x = match input.x_tri() {
            Tri::Positive => self.position.x + 1,
            Tri::Negative => {
                if self.position.x == 0 {
                    0
                } else {
                    self.position.x - 1
                }
            }
            Tri::Zero => self.position.x,
        };

        self.position.y = match input.y_tri() {
            Tri::Positive => self.position.y + 1,
            Tri::Negative => {
                if self.position.y == 0 {
                    0
                } else {
                    self.position.y - 1
                }
            }
            Tri::Zero => self.position.y,
        };

        let new_orientation = self.orientation.update(input.x_tri(), input.y_tri());

        if new_orientation != self.orientation {
            // we need to change player sprite !
            let tag = Player::oriented_tag(new_orientation);
            self.sprite
                .set_sprite(controller.sprite(tag.animation_sprite(0)));

            self.orientation = new_orientation;
        }

        self.sprite.set_x(self.position.x);
        self.sprite.set_y(self.position.y);
    }
}
