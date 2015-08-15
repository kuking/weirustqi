use base::*;
use base::moves::*;
use base::game::*;
use base::game_result::*;

use mcts::*;
use mcts::analytics::*;
use mcts::analytics::brain_keeper::*;

pub struct MrEd<'r> {
    game      :Game,
    cache     :game_state::GameStateCache,
    ministers :Vec<&'r minister::Minister>,
    keeper    :&'r BrainKeeper,
    scorer    :fn(&Game) -> GameResultRange
}

impl<'r> MrEd<'r> {

    pub fn new(game      :Game,
               ministers :Vec<&'r minister::Minister>,
               keeper    :&'r BrainKeeper,
               scorer    :fn(&Game) -> GameResultRange) -> MrEd<'r> {


        let game_state_cache = game_state::GameStateCache::new((&game).board().size());
        MrEd {
            game  :game,
            cache :game_state_cache,
            ministers :ministers,
            keeper :keeper,
            scorer :scorer
        }
    }

    pub fn game(&self) -> &game::Game { &self.game }
    pub fn game_as_mut(&mut self) -> &mut game::Game { &mut self.game }

    pub fn think(&mut self) {

        // if not done before, call all ministers for this function


        // decide which moves to evaluate
        // loop while time is avail
    }

    pub fn best_move(&self) -> moves::Move {
        Move::Pass(self.game.next_turn())
    }

}
