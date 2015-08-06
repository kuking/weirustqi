
use std::collections::LinkedList;

use base::moves::*;

#[derive(Clone, Debug)]
pub struct GameTree {
    black_name  :String,
    white_name  :String,
    board_size  :usize,
    handicap    :u16,
    komi        :f32,
    moves       :LinkedList<GameNode>
}

#[derive(Clone, Debug)]
pub struct GameNode {
    amove       :Move,
    comment     :String,
    variants    :LinkedList<GameNode>
}


impl GameTree {

    pub fn parse(s : String) -> Self {
        Self::new()
    }

    pub fn write() -> String {
        String::new()
    }

    pub fn new() -> GameTree {
        GameTree {
            black_name: String::new(),
            white_name: String::new(),
            board_size: 0,
            handicap: 0,
            komi: 0.0,
            moves :LinkedList::new()
        }
    }

    pub fn set_black_name(&mut self, black_name :String) { self.black_name = black_name }
    pub fn black_name(&self) -> String { self.black_name.clone() }

    pub fn set_white_name(&mut self, white_name :String) { self.white_name = white_name }
    pub fn white_name(&self) -> String { self.white_name.clone() }

    pub fn set_board_size(&mut self, board_size :usize) { self.board_size = board_size }
    pub fn board_size(&self) -> usize { self.board_size }

    pub fn set_komi(&mut self, komi :f32) { self.komi = komi }
    pub fn komi(&self) -> f32 { self.komi }

    pub fn set_handicap(&mut self, handicap :u16) { self.handicap = handicap }
    pub fn handicap(&self) -> u16 { self.handicap }


}

// *********************************************************************************************
// Tests

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_can_be_created_and_retains_basic_properties() {
        let mut gt = GameTree::new();
        gt.set_black_name("black".to_string());
        gt.set_white_name("white".to_string());
        gt.set_board_size(19);
        gt.set_komi(5.5);
        gt.set_handicap(1);

        assert_eq!("black".to_string(), gt.black_name());
        assert_eq!("white".to_string(), gt.white_name());
        assert_eq!(19, gt.board_size());
        assert_eq!(5.5, gt.komi());
        assert_eq!(1, gt.handicap());
    }


}
