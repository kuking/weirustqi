use base::*;
use mcts::*;
use mcts::analytics::*;

pub struct MrEd<'r> {
    game      :game::Game,
    cache     :game_state::GameStateCache,
    ministers :Vec<&'r minister::Minister>,
    timer     :&'r time_keeper::TimeKeeper,
    scorer    :&'r scorer::Scorer
}

impl<'r> MrEd<'r> {

    pub fn new(game      :game::Game,
               ministers :Vec<&'r minister::Minister>,
               timer     :&'r time_keeper::TimeKeeper,
               scorer    :&'r scorer::Scorer) -> MrEd<'r> {


        let game_state_cache = game_state::GameStateCache::new((&game).board().size());
        MrEd {
            game  :game,
            cache :game_state_cache,
            ministers :ministers,
            timer :timer,
            scorer :scorer
        }
       }

    pub fn think(&mut self) {

        // if not done before, call all ministers for this function


        // decide which moves to evaluate
        // loop while time is avail
    }

}