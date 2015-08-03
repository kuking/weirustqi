use base::color::Color;
use base::coord::Coord;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Pass,
    Stone { color :Color, coord : Coord }
}
