use std::collections::HashMap;

use base::*;

pub struct GameTreeNode {
    zobrist  :u64,
    game     :game::Game,
    stats    :HashMap<coord::Coord, MoveStat>,
    last_used_gen  :u64
}

pub struct GameTreeCache {
    entries :HashMap<u64, GameTreeNode>
}

pub struct MoveStat {
    pub votes   :u32,
    pub black_wins :u32,
    pub white_wins :u32,
}

impl GameTreeNode {

    pub fn new(game : game::Game, generation :u64) -> GameTreeNode {
        GameTreeNode {
            zobrist : game.board().zobrist(),
            game    : game.clone(),
            stats   : HashMap::with_capacity(game.board().size() as usize), //FIXME: tune
            last_used_gen : generation
        }
    }

}

impl GameTreeCache {

    pub fn new(board_size :u8) -> GameTreeCache {
        let est_cache_size = (board_size as usize).pow(5); //FIXME: TUNE pls
        // values that will be generated are: 9=59049, 13=371293, 19=2476099
        GameTreeCache {
            entries :HashMap::with_capacity(est_cache_size)
        }
    }

    pub fn purge_older_than(&mut self, generation :u64) {
        let to_remove : Vec<_> = self.entries.iter()
                .filter(|&(_ ,v)| v.last_used_gen < generation)
                .map(|(k, _ )| k.clone() )
                .collect();
        for remove in to_remove { self.entries.remove(&remove); }
    }

    fn get_as_mut(&mut self, generation :&u64, game : &game::Game) -> Option<&mut GameTreeNode> {
        self.entries.get_mut( &game.board().zobrist() )
    }

    fn insert(&mut self, generation :u64, game : game::Game) -> Option<GameTreeNode> {
        self.entries.insert(game.board().zobrist(), GameTreeNode::new(game, generation) )
    }

    pub fn get_or_create_as_mut(&mut self, generation : &u64, game : &game::Game) -> &mut GameTreeNode {
        // got a bit crazy with keepting mutability, etc... /TODO: should be more performant
        if !self.entries.contains_key(&game.board().zobrist()) {
            self.insert(generation.clone(), game.clone());
        }

        if let Some(entry) = self.get_as_mut(generation, game) {
            entry.last_used_gen = *generation;
            entry
        } else {
            panic!("Failed to retrieve content");
        }
    }


}
