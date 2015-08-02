use std::fmt::{Display, Formatter, Error};

use super::Color;
use super::Move;
use super::Coord;

include!("zobrist-const.rs");

#[derive(Clone)]
pub struct Board  {
    size :u8,
    data :Vec<Color>,
    zobrist :u64
}

impl Board {
    pub fn new(board_size :usize) -> Self {
        if board_size>BOARD_MAX_SIDE {
            panic!(format!("I'm sorry boards up to {} are possible; is 19 not enough?", BOARD_MAX_SIDE));
        }
        if board_size<5 {
            panic!("A board less than 5x5?");
        }
        Board {
            size : board_size as u8,
            data : (0..board_size*board_size).map(|_| Color::Empty).collect(),
            zobrist : LE_ZOBRIST_BOARD[board_size-1]
        }
    }

    pub fn set_move(&mut self, m :Move) {
        if let Move::Stone {color, coord} = m {
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
        //assert!(coord.row > self.size || coord.row > self.size);
        coord.row as usize * self.size as usize + coord.col as usize
    }
    pub fn size(&self) -> u8 {
        self.size
    }
}


impl Display for Board {
    fn fmt(&self, fmt : &mut Formatter) -> Result<(), Error> {
        fmt.write_str(&format!("I'm a board size {}!\n", &self.size));
        return Ok(());
    }
}



#[cfg(test)]
mod test {

    use super::*;
    use super::super::*;

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
        let lecoord = Coord::new(2,3);
        let mut board = Board::new(32);

        assert_eq!(Color::Empty, board.get(lecoord));

        board.set_move(Move::Stone{color: Color::White, coord : lecoord});
        assert_eq!(Color::White, board.get(lecoord));

        board.set_move(Move::Stone{color: Color::Black, coord : lecoord});
        assert_eq!(Color::Black, board.get(lecoord));

        board.set_move(Move::Stone{color: Color::Empty, coord : lecoord});
        assert_eq!(Color::Empty, board.get(lecoord));
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
                res.push ( Move::Stone {color: lecolor, coord: lecoord} );
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
