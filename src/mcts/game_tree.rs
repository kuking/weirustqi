use std::collections::HashMap;

use base::*;
use base::color::*;
use base::coord::*;

pub struct GameTreeNode {
    game     :game::Game,
    stats    :HashMap<Coord, MoveStat>,
    last_used_gen  :u64
}

pub struct GameTreeCache {
    entries :HashMap<u64, GameTreeNode>
}

#[derive(Debug)]
pub struct MoveStat {
    votes   :u32,
    black_wins :u32,
    white_wins :u32,
}

impl GameTreeNode {

    pub fn new(game : game::Game, generation :u64) -> GameTreeNode {
        GameTreeNode {
            game    : game.clone(),
            stats   : HashMap::with_capacity(game.board().size() as usize), //FIXME: tune
            last_used_gen : generation
        }
    }

    pub fn next_to_explore(&self, count :usize) -> Vec<Coord> {
        let mut res : Vec<Coord> = vec!();
        let turn = self.game.next_turn();
        for (coord, stat) in &self.stats {
            let score = stat.votes() + stat.wins_for(turn);
            let played = stat.played();



        }
        res
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

impl MoveStat {

    pub fn wins_for(&self, color :Color) -> u32 {
        if color == Color::White {
            self.white_wins
        } else {
            self.black_wins
        }
    }

    pub fn played(&self) -> u32 {
        self.white_wins + self.black_wins
    }

    pub fn votes(&self) -> u32 {
        self.votes
    }


}
