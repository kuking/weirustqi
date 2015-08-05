use std::fmt::{Display, Formatter, Error};
use std::str::FromStr;

use std::collections::HashSet;


use base::color::*;
use base::moves::*;
use base::coord::*;
use base::board::*;

#[derive(Debug)]
pub struct Game {
    board    : Board,
    komi     : f32,
    handicap : u16,
    captured_black : u16,
    captured_white : u16,
    next_turn : Color,
    finished : bool,
    super_ko : HashSet<u64>,
    moves : Vec<Move>
}

impl Game {

    pub fn new(board_size :usize, komi :f32, handicap :usize) -> Self {
        let mut le_board = Board::new(board_size);
        if handicap>0 {
            Self::set_handicap_stones(&mut le_board, handicap)
        }
        Game {
            board : le_board,
            komi : komi,
            handicap : handicap as u16,
            captured_black : 0,
            captured_white : 0,
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

    pub fn captured_count(&self, color : Color) -> u16 {
        match color {
            Color::White => self.captured_white,
            Color::Black => self.captured_black,
            Color::Empty => panic!("No dead count for empty intersection")
        }
    }

    // Mostly game logic below

    pub fn play(&mut self, m :Move) -> bool {

        if self.finished {
            return false
        }

        if let Move::Pass(c) = m {
            if c != self.next_turn {
                return false
            }
            self.state_update_for_move(&m);
            return true;
        }

        if let Move::Stone(coord, color) = m {

            if color != self.next_turn {
                return false
            }

            if self.board.get(&coord) != Color::Empty {
                return false
            }

            let opposite_color = self.next_turn.opposite();
            let mut as_kill = false;
            for adj in self.board.adjacents_by_color(&coord, &opposite_color) {
                //println!("is_given_coord_last_liberty_for_adj_chain {}, {}, {} = {}", coord, adj, opposite_color, self.board.is_given_coord_last_liberty_for_adj_chain(coord, adj, opposite_color));
                if self.board.is_given_coord_last_liberty_for_adj_chain(coord, adj, opposite_color) {
                    let captured = self.board.remove_chain(adj, opposite_color);
                    self.account_captured(captured);
                    as_kill = true;
                }
            }
            if as_kill {
                self.state_update_for_move(&m);
                return true
            }

            if self.board.adjacents_by_color(&coord, &Color::Empty).len()>0 { //FIXME: this can be more efficient
                self.state_update_for_move(&m);
                return true
            }

        }

        false
    }

    fn state_update_for_move(&mut self, m :&Move) {
        // it is a given the move is valid
        self.next_turn = if self.next_turn == Color::Black { Color::White } else { Color::Black };
        // two passes in a row, game is finished
        if let Move::Pass(_) = *m {
            if self.moves.len() > 0 {
                if let Some(&Move::Pass(_)) = self.moves.last() {
                    self.finished = true;
                }
            }
        }
        // record the move
        self.moves.push(*m);
        self.board.set_move(*m);
        self.super_ko.insert(self.board.zobrist());
    }

    fn account_captured(&mut self, captured :usize) {
        match self.next_turn.opposite() {
            Color::White => self.captured_white = self.captured_white + captured as u16,
            Color::Black => self.captured_black = self.captured_black + captured as u16,
            Color::Empty => panic!("It should never be the turn for 'empty'")
        }
    }

    pub fn pretty_print(&self) -> String {
        // not the best method, or elegant one, but does the job and it is not meant to be efficient
        let mut st = String::new();
        st = st + &"\n";
        st = st + &format!("    White (O) has captured {} stones\n", self.captured_count(Color::Black)); //because white captures black stones
        st = st + &format!("    Black (X) has captured {} stones\n", self.captured_count(Color::White));
        st = st + &"\n";
        // header row list
        st = st + &"    ";
        for i in (0..self.board.size()) {
            let rowc = if i>8 { (i+1+65) as char } else { (i+65) as char };
            st = st + &format!("{} ", rowc);
        }
        if self.move_count()>0 {
            st = st + &format!("      Last move: {}", self.moves.last().unwrap());
        }
        st = st + &"\n";
        // rows
        let markers : Vec<Coord> = Self::markers_coords(self.board.size());
        for r in (0..self.board.size()) {
            let row = self.board.size() - r;
            st = st + &format!("{:3} ", row);

            for c in (0..self.board.size()) {
                let coord = Coord::new(row-1,c);
                let color = self.board.get(&coord);
                let color_ch = match color {
                    Color::Empty => if markers.contains(&coord) {'+'} else {'.'},
                    Color::Black => 'X',
                    Color::White => 'O'
                };

                let highlight = self.move_count()>0
                             && self.moves.last().unwrap().is_stone()
                             && self.moves.last().unwrap().coord() == coord;

                if highlight {
                    st = st + &format!("{}({})", 8 as char, color_ch);
                } else {
                    st = st + &format!("{} ", color_ch);
                }
            }
            st = st + &format!("{:2}\n", row);

        }
        // bottom row list
        st = st + &"    ";
        for i in (0..self.board.size()) {
            let rowc = if i>8 { (i+1+65) as char } else { (i+65) as char };
            st = st + &format!("{} ", rowc);
        }
        st + &"\n\n"
    }


    // mostly private

    fn handicap_coords_for(board_size :u8) -> Vec<Coord> {
        // no fast, but this doesn't need to be too fast.
        match board_size {
            19 => vec!( Coord::from_str(&"D4").unwrap(),  Coord::from_str(&"Q16").unwrap(),
                        Coord::from_str(&"D16").unwrap(), Coord::from_str(&"Q4").unwrap(),
                        Coord::from_str(&"D10").unwrap(), Coord::from_str(&"Q10").unwrap(),
                        Coord::from_str(&"K4").unwrap(),  Coord::from_str(&"K16").unwrap(),
                        Coord::from_str(&"K10").unwrap() ),
            _ => vec!()
        }
    }

    fn markers_coords(board_size :u8) -> Vec<Coord> {
        // no fast, but this doesn't need to be fast, used only by pretty_print
        // in the future, when more handicap is added, this will need to be limit to the first 9 in 19x19, etc.
        Self::handicap_coords_for(board_size)
    }

    fn set_handicap_stones(board : &mut Board, handicap :usize) {
        let coords = Self::handicap_coords_for(board.size());
        assert!(coords.len()>0, format!("I dont know how to process handicaps for boards of size {}", board.size()));
        for i in 0..handicap {
            board.set_move(Move::Stone(coords[i], Color::Black))
        }
    }

}

impl Display for Game {
    fn fmt(&self, fmt : &mut Formatter) -> Result<(), Error> {
        fmt.write_fmt(format_args!("({}x{}#{} k={:2} h={} cb={} cw={} #{:x})",
            self.board().size(), self.board().size(),
            self.move_count(),
            self.komi,
            self.handicap,
            self.captured_black, self.captured_white,
            self.board.zobrist()))
    }
}


// *********************************************************************************************
// Tests

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use super::*;
    use base::color::*;
    use base::coord::*;
    use base::moves::*;

    fn assert_color(g :&Game, c :Color, pos :&str) {
        assert_eq!(c, g.board().get(&Coord::from_str(pos).unwrap()));
    }

    #[test]
    fn it_builds_a_simple_new_game() {
        let g = Game::new(19, 5.5, 0);
        assert_eq!(19, g.board().size());
        assert_eq!(5.5, g.komi());
        assert_eq!(0, g.handicap());
        assert_eq!(0, g.captured_count(Color::Black));
        assert_eq!(0, g.captured_count(Color::White));
        assert_eq!(Color::Black, g.next_turn());
        assert_eq!(0, g.move_count());
        assert!(g.moves().is_empty());
        assert!(!g.finished());
    }

    #[test]
    fn handicap_it_sets_white_as_first_player_when_handicaped() {
        let g = Game::new(19, 0.0, 1);
        assert_eq!(Color::White, g.next_turn());
    }

    #[test]
    fn handicap_it_sets_stones_in_place() {
        let g = Game::new(19, 0.0, 9);
        assert_color(&g, Color::Black, "D4");
        assert_color(&g, Color::Black, "Q16");
        assert_color(&g, Color::Black, "D16");
        assert_color(&g, Color::Black, "Q4");
        assert_color(&g, Color::Black, "D10");
        assert_color(&g, Color::Black, "Q10");
        assert_color(&g, Color::Black, "K4");
        assert_color(&g, Color::Black, "K16");
        assert_color(&g, Color::Black, "K10");
    }

    #[test]
    fn handicap_it_sets_stones_in_place_2() {
        let g = Game::new(19, 0.0, 3);
        assert_color(&g, Color::Black, "D4");
        assert_color(&g, Color::Black, "Q16");
        assert_color(&g, Color::Black, "D16");
        assert_color(&g, Color::Empty, "Q4");
    }

    #[test]
    fn simplest_finished_game() {
        let mut g = Game::new(19, 5.5, 0);
        assert!(g.play(Move::from_str("Black D4").unwrap()));
        assert!(g.play(Move::from_str("White Q16").unwrap()));
        assert!(g.play(Move::from_str("Black Pass").unwrap()));
        assert!(!g.finished());
        assert!(g.play(Move::from_str("White Pass").unwrap()));
        assert!(g.finished());
    }

    #[test]
    fn passing_is_valid_move_but_not_after_game_is_finished() {
        let mut g = Game::new(19, 5.5, 0);
        let black_pass = Move::from_str("Black Pass").unwrap();
        let white_pass = Move::from_str("White Pass").unwrap();
        // black pass
        assert!(!g.finished());
        assert!(!g.play(white_pass));
        assert!(g.play(black_pass));
        // white pass
        assert!(!g.finished());
        assert!(!g.play(black_pass)); // black move is not valid anymore
        assert!(g.play(white_pass));
        // and the game is finish
        assert!(g.finished());
        assert!(!g.play(white_pass));
        assert!(!g.play(black_pass));
    }

    #[test]
    fn playing_twice_on_same_place_is_not_valid_move() {
        let mut g = Game::new(19, 5.5, 0);
        let black_move = Move::from_str("Black D4").unwrap();
        let black_move2 = Move::from_str("Black D5").unwrap();
        let white_move = Move::from_str("White Q16").unwrap();
        let white_move2 = Move::from_str("White Q15").unwrap();
        assert!(g.play(black_move));
        assert!(g.play(white_move));
        assert!(!g.play(black_move));
        assert!(g.play(black_move2));
        assert!(!g.play(white_move));
        assert!(g.play(white_move2));
    }

}
