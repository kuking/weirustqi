extern crate rand;
use rand::Rng;

use base::*;
use base::coord::*;
use base::color::*;
use base::moves::*;
use base::board::*;
use base::game::*;
use base::game_result::*;

use mcts::*;
use mcts::analytics::*;
use mcts::analytics::brain_keeper::*;

pub struct MrEd<'r> {
    game      :Game,
    cache     :game_tree::GameTreeCache,
    generation :u64,
    keeper    :&'r BrainKeeper,
    scorer    :fn(&Game) -> GameResultRange,
    turn_best_move   :Move,
    turn_best_result :GameResultRange
}

impl<'r> MrEd<'r> {

    pub fn new(game      :Game,
               keeper    :&'r BrainKeeper,
               scorer    :fn(&Game) -> GameResultRange) -> MrEd<'r> {


        let game_tree_cache = game_tree::GameTreeCache::new((&game).board().size());
        let game_result = GameResultRange::new(GameResult::Draw, (game.board().size() as u16).pow(2));
        MrEd {
            game  :game,
            cache :game_tree_cache,
            generation :0,
            keeper :keeper,
            scorer :scorer,
            turn_best_move   :Move::Pass(Color::Black),
            turn_best_result :game_result
        }
    }

    pub fn game(&self) -> &game::Game { &self.game }
    pub fn game_as_mut(&mut self) -> &mut game::Game { &mut self.game }

    pub fn think(&mut self) {

        let my_color = self.game.next_turn();
        let all_coords = Coord::all_possibles(self.game.board().size() as usize);

        for coord in all_coords {
            let m = Move::Stone(coord, my_color);
            if Self::is_ok_move(self.game.board(), &m) {
                let mut game = self.game.clone();
                if game.play(m) {
                    let result = Self::super_fast_playout(game);
                    if result.better_than_for(&self.turn_best_result, my_color) {
                        self.turn_best_result = result;
                        self.turn_best_move = m;
                    }
                }
            }
        }

    }

    pub fn think_new(&mut self) {

        let my_color = self.game.next_turn();
        self.generation = self.generation + 1;

        let mut game_tree = self.cache.get_or_create_as_mut(&self.generation, &self.game);


    }


    pub fn new_turn(&mut self) {
        self.turn_best_move = Move::Pass(self.game.next_turn());
        self.turn_best_result = GameResultRange::new(GameResult::Draw, 10000);
    }

    fn is_ok_move(b : &Board, m :&Move) -> bool {
        if b.get(&m.coord()) != Color::Empty {
            false
        } else if b.is_eye(&m.coord(), &m.color()) {
            false
        } else {
            true
        }
    }

    pub fn best_move(&self) -> Move {
        self.turn_best_move
    }

    pub fn best_result(&self) -> GameResultRange {
        self.turn_best_result
    }

    pub fn suggested_move(&self) -> Move {
        // suggest passing if either it can't win, or previous is pass and it knows it will win.
        if self.game().next_turn() != self.turn_best_result.result.color() ||
           (self.game().next_turn() == self.turn_best_result.result.color() && self.turn_best_result.safe_win() && self.last_move_is_pass()) {
            Move::Pass(self.game.next_turn())
        } else {
            self.turn_best_move
        }
    }

    fn last_move_is_pass(&self) -> bool {
        let moves = self.game.moves();
        let n = moves.len();
        if n<2 {
            return false;
        }
        if let Some(last) = moves.get(n-1) {
            return last.is_pass()
        }
        false
    }

    // ------

    fn super_fast_playout(mut g :Game) -> GameResultRange {
        let mut rng = rand::thread_rng();
        let board_size = g.board().size() as usize;
        while !g.finished() && g.move_count() < 2*board_size*board_size {
            let mut count = 0;
            let turn_color = g.next_turn();
            let mut non_empty : Vec<usize> = vec!();
            // one move
            loop {
                count = count + 1;

                let coord;
                if count > board_size/3 {
                    if non_empty.is_empty() {
                        non_empty = g.board().data().iter().
                            enumerate().filter(|&(_,&v)| v == Color::Empty).map(|(n,_)|n).collect();
                    }
                    let random = rng.gen::<usize>() % non_empty.len();
                    coord = g.board().offset_to_coord(random);
                } else {
                    coord = Coord::random(board_size);
                }

                let m = Move::Stone(coord, turn_color);
                if Self::is_ok_move(g.board(), &m) && g.play(m) {
                    break;
                }

                if count > board_size/2 {
                    g.play(Move::Pass(turn_color));
                    break;
                }

            }
        }
        scorer::conservative_floodfill_scorer(&g)
    }

}
