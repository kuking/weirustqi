
use std::collections::HashSet;
use std::collections::HashMap;

use base::Color;
use super::Move;
use super::Coord;

use base::board::Board;

pub struct Game {
    board    : Board,
    komi     : f32,
    handicap : u16,
    black_dead : u16,
    white_dead : u16,
    next_turn : Color,
    finished : bool,
    super_ko : HashSet<u64>,
    moves : Vec<Move>
}

impl Game {

    pub fn new(board_size :usize, komi :f32, handicap :u16) -> Self {
        let mut le_board = Board::new(board_size);

        Game {
            board : le_board,
            komi : komi,
            handicap : handicap,
            black_dead : 0,
            white_dead : 0,
            next_turn : if handicap>0 {Color::White} else {Color::Black},
            finished : false,
            super_ko : HashSet::new(),
            moves : Vec::with_capacity((board_size*board_size)) //FIXME: do better estimation
            // japanese rules usually are 3/2 i.e. 250 moves in 19x19, but with chinese rules
            // it tends to be around 300.
        }
    }

    pub fn board(&self) -> &Board { &self.board }
    pub fn komi(&self) -> f32 { self.komi }
    pub fn handicap(&self) -> u16 { self.handicap }
    pub fn next_turn(&self) -> Color { self.next_turn }
    pub fn moves(&self) -> &Vec<Move> { &self.moves }
    pub fn move_count(&self) -> usize { self.moves.len() }
    pub fn finished(&self) -> bool { self.finished }

    pub fn dead_count(&self, color : Color) -> u16 {
        match color {
            Color::White => self.white_dead,
            Color::Black => self.black_dead,
            Color::Empty => panic!("No dead count for empty intersection")
        }
    }


    pub fn handicap_coords_for(board_size :usize) -> Vec<Coord> {
        match board_size {
            19 => vec!( Coord::from_str(&"D4"), Coord::from_str(&"Q16"), Coord::from_str(&"D16"),
                        Coord::from_str(&"Q4"), Coord::from_str(&"D10"), Coord::from_str(&"Q10"),
                        Coord::from_str(&"K4"), Coord::from_str(&"K16"), Coord::from_str(&"K10") ),
            _ => vec!()
        }
    }


}

// *********************************************************************************************
#[cfg(test)]
mod test {

    use super::*;
    use base::*;

    #[test]
    fn it_builds_a_simple_new_game() {
        let g = Game::new(19, 5.5, 0);
        assert_eq!(19, g.board().size());
        assert_eq!(5.5, g.komi());
        assert_eq!(0, g.handicap());
        assert_eq!(0, g.dead_count(Color::Black));
        assert_eq!(0, g.dead_count(Color::White));
        assert_eq!(Color::Black, g.next_turn());
        assert_eq!(0, g.move_count());
        assert!(g.moves().is_empty());
        assert!(!g.finished());
    }

    #[test]
    fn handicap__it_sets_white_as_first_player_when_handicaped() {
        let g = Game::new(19, 0.0, 1);
        assert_eq!(Color::White, g.next_turn());
    }

    #[test]
    fn handicap__it_sets_stones_in_place_for_handicap() {
        let g = Game::new(19, 0.0, 1);
        let b = g.board();
    }


}
