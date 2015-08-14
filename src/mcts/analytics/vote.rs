
use base::*;

/// Votes should be in a way that are usefull for both White and Black. i.e. if it is a very good
/// move for Black, but this board configuration is now analysed for White-to-play, the vote
/// should highlight the fact that a position is to be taken care by white too, i.e. by connecting.
pub enum Vote {
    Avoid(coord::Coord),
    Vote(coord::Coord, i8)
}
