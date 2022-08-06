use agb::input::Tri;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Orientation {
    Left,
    Right,
    Up,
    Down,
}

impl Orientation {
    pub fn update(&self, horizontal: Tri, vertical: Tri) -> Self {
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
            // simply take the new direction, or conserve it if zero on both axis
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

    pub fn horizontal(&self) -> Tri {
        match self {
            Self::Left => Tri::Negative,
            Self::Right => Tri::Positive,
            _ => Tri::Zero,
        }
    }

    pub fn vertical(&self) -> Tri {
        match self {
            Self::Up => Tri::Negative,
            Self::Down => Tri::Positive,
            _ => Tri::Zero,
        }
    }
}
