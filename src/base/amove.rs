use base::color::Color;
use base::coord::Coord;

pub enum Move {
    Pass,
    Stone { color :Color, coord : Coord }
}
