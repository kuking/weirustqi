
use base::*;

pub enum Vote {
    Avoid(coord::Coord),
    Vote(coord::Coord, i8)
}
