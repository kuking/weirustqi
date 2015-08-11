use base::*;
use mcts::analytics::*;

pub trait Minister {
    fn analyse(game :&game::Game) -> Vec<vote::Vote>;
}
