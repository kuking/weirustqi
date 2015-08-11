use base::game::*;
use base::game_result::*;

pub trait Scorer {
    fn score(game :&Game) -> GameResult;
}
