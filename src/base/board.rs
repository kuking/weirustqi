use std::fmt::{Display, Formatter, Error};
use std::hash::{Hash, Hasher};

use base::color::*;
use base::moves::*;
use base::coord::*;

include!("zobrist-const.rs");

#[derive(Clone, Eq, Debug)]
pub struct Board  {
    size :u8,
    data :Vec<Color>,
    zobrist :u64
}

impl Board {
    pub fn new(board_size :usize) -> Self {
        debug_assert!(board_size<=BOARD_MAX_SIDE, format!("I'm sorry boards up to {} are possible; is 19 not enough?", BOARD_MAX_SIDE));
        debug_assert!(board_size>4, "Do you really want a board smaller than 5x5?");
        Board {
            size : board_size as u8,
            data : (0..board_size*board_size).map(|_| Color::Empty).collect(),
            zobrist : LE_ZOBRIST_BOARD[board_size-1]
        }
    }

    pub fn set_move(&mut self, m :Move) {
        if let Move::Stone(coord, color) = m {
            let o = self.data_offset(coord);
            //zobrist
            let curr = self.data[o];
            if curr != Color::Empty {
                self.zobrist = self.zobrist ^ LE_ZOBRISTS[curr as usize][o];
            }
            self.zobrist = self.zobrist ^ LE_ZOBRISTS[color as usize][o];
            // position
            self.data[o] = color;
        }
    }

    pub fn set_moves(&mut self, ms :Vec<Move>) {
        for m in ms {
            self.set_move(m);
        }
    }


    pub fn get(&self, coord :Coord) -> Color {
        self.data[self.data_offset(coord)]

    }

    #[inline]
    fn data_offset(&self, coord : Coord) -> usize {
        debug_assert!(coord.row < self.size || coord.row < self.size);
        coord.row as usize * self.size as usize + coord.col as usize
    }

    #[inline]
    pub fn size(&self) -> u8 {
        self.size
    }

    #[inline]
    pub fn zobrist(&self) -> u64 {
        self.zobrist
    }
}


impl Display for Board {
    fn fmt(&self, fmt : &mut Formatter) -> Result<(), Error> {
        fmt.write_str(&format!("I'm a board size {}!\n", &self.size))
    }
}

impl Hash for Board {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        state.write_u64(self.zobrist);
    }
}

impl PartialEq for Board {
    fn eq(&self, other :&Board) -> bool {
        self.size() == other.size() && self.zobrist() == other.zobrist()
    }
    fn ne(&self, other :&Board) -> bool {
        !self.eq(other)
    }
}

// *********************************************************************************************
// Tests

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use super::*;
    use super::BOARD_MAX_SIDE;

    use base::color::*;
    use base::moves::*;
    use base::coord::*;

    #[test]
    fn it_creates_boards() {
        let board = Board::new(19);
        assert_eq!(19, board.size());
    }

    #[test]
    #[should_panic]
    fn it_wont_create_boards_bigger_than_32() {
        Board::new(33);
    }

    #[test]
    fn it_maintains_state() {
        let coord = Coord::new(2,3);
        let mut board = Board::new(32);

        assert_eq!(Color::Empty, board.get(coord));

        board.set_move(Move::Stone(coord, Color::White));
        assert_eq!(Color::White, board.get(coord));

        board.set_move(Move::Stone(coord, Color::Black));
        assert_eq!(Color::Black, board.get(coord));

        board.set_move(Move::Stone(coord, Color::Empty));
        assert_eq!(Color::Empty, board.get(coord));
    }

    fn given_board_with_two_moves() -> Board {
        let mut board = Board::new(19);
        board.set_move(Move::Stone(Coord::new(2,3), Color::White));
        board.set_move(Move::Stone(Coord::new(3,2), Color::Black));
        board
    }

    #[test]
    fn it_maintains_state_complex() {
        let mut board = given_board_with_two_moves();
        board.set_move(Move::Stone(Coord::new(2,3), Color::White));
        board.set_move(Move::Stone(Coord::new(3,2), Color::Black));
        assert_eq!(board.get(Coord::new(2,3)), Color::White);
        assert_eq!(board.get(Coord::new(3,2)), Color::Black);
    }

    #[test]
    fn it_handles_board_limits() {
        let mut board = Board::new(BOARD_MAX_SIDE);
        for coord in Coord::all_possibles(BOARD_MAX_SIDE) {
            board.set_move( Move::Stone(coord, Color::Black));
        }
        let max_coord = Coord::new_us(BOARD_MAX_SIDE-1, BOARD_MAX_SIDE-1);
        let min_coord = Coord::new_us(0,0);
        board.set_move( Move::Stone(max_coord, Color::White) );
        board.set_move( Move::Stone(min_coord, Color::White) );
        // it tests all positions to be sure there is not 'sliding' into neighbour intersection
        for coord in Coord::all_possibles(BOARD_MAX_SIDE) {
            let color = board.get(coord);
            if coord == max_coord || coord == min_coord {
                assert_eq!(Color::White, color);
            } else {
                assert_eq!(Color::Black, color);
            }
        }
    }

    #[test]
    fn it_eq_zobrist_for_two_empty_boards() {
        let b1 = Board::new(19);
        let b2 = Board::new(19);
        assert_eq!(b1.zobrist(), b2.zobrist());
        assert!(b1.zobrist() != 0);
    }

    #[test]
    fn it_wont_zero_zobrist_for_empty_board() {
        assert!(Board::new(5).zobrist() != 0);
        assert!(Board::new(9).zobrist() != 0);
        assert!(Board::new(11).zobrist() != 0);
        assert!(Board::new(19).zobrist() != 0);
        assert!(Board::new(32).zobrist() != 0);
    }

    #[test]
    fn it_wont_eq_zobrist_for_empty_boards_of_diff_sizes() {
        assert!(Board::new(9).zobrist() != Board::new(11).zobrist());
        assert!(Board::new(11).zobrist() != Board::new(19).zobrist());
        assert!(Board::new(19).zobrist() != Board::new(32).zobrist());
    }

    #[test]
    fn it_hashes_boards() {
        let mut h : HashSet<Board> = HashSet::new();
        assert_eq!(0, h.len());
        // one board
        h.insert(Board::new(9));
        assert_eq!(1, h.len());
        // same board, len is the same
        h.insert(Board::new(9));
        assert_eq!(1, h.len());
        // but another sized board will two elements in the set
        h.insert(Board::new(11));
        assert_eq!(2, h.len());
        // one of 19 - so three
        h.insert(Board::new(19));
        assert_eq!(3, h.len());
        // another of same size (19), but modified modified should be different
        h.insert(given_board_with_two_moves());
        assert_eq!(4, h.len());
        // but again, should not add any extra entry
        h.insert(given_board_with_two_moves());
        assert_eq!(4, h.len());
        // border_case
        h.insert(Board::new(BOARD_MAX_SIDE));
        assert_eq!(5, h.len());
    }

    #[test]
    fn it_equals() {
        assert!(Board::new(14) == Board::new(14));
        assert!(Board::new(14) != Board::new(15));
    }

    #[test]
    fn it_equals_modified_boards() {
        assert_eq!(given_board_with_two_moves(), given_board_with_two_moves());
    }

    //
    // benchs for vec board
    //
    use test::Bencher;

    fn all_moves(board_size :usize) -> Vec<Move> {
        let mut res : Vec<Move> = vec!();
        for rown in 0..board_size {
            for coln in 0..board_size {
                let lecolor = if (rown*coln) % 2 == 0 { Color::White } else { Color::Black };
                let lecoord = Coord::new_us(rown, coln);
                res.push ( Move::Stone (lecoord, lecolor) );
            }
        }
        res
    }

    #[bench]
    fn create_19x19_board(b: &mut Bencher) {
        b.iter(|| Board::new(19) );
    }

    #[bench]
    fn clone_19x19_board(b: &mut Bencher) {
        let board = Board::new(19);
        b.iter(|| { (0..1).all(|_|board.clone().size() == 19) });
    }

    #[bench]
    fn create_9x9_board(b: &mut Bencher) {
        b.iter(|| Board::new(9) );
    }

    #[bench]
    fn set_19x19_board(b: &mut Bencher) {
        let mut board = Board::new(19);
        b.iter(|| board.set_moves( all_moves(19) ));
    }

}
