
use base::moves::*;
use base::game_result::*;
use base::rank::*;

#[derive(Clone, Debug)]
pub struct GameTree {
    black_name  :String,
    white_name  :String,
    black_rank  :Rank,
    white_rank  :Rank,
    board_size  :usize,
    handicap    :u16,
    komi        :f32,
    result      :GameResult,
    moves       :Vec<GameNode>
}

#[derive(Clone, Debug)]
pub struct GameNode {
    themove       :Move,
    comment     :String,
    variants    :Vec<GameNode>
}


impl GameTree {

    pub fn new() -> GameTree {
        GameTree {
            black_name: String::new(),
            white_name: String::new(),
            black_rank: Rank::Unknown,
            white_rank: Rank::Unknown,
            board_size: 0,
            handicap: 0,
            komi: 0.0,
            result: GameResult::Unknown,
            moves :Vec::new()
        }
    }

    pub fn set_black_name(&mut self, black_name :String) { self.black_name = black_name }
    pub fn black_name(&self) -> &String { &self.black_name }

    pub fn set_white_name(&mut self, white_name :String) { self.white_name = white_name }
    pub fn white_name(&self) -> &String { &self.white_name }

    pub fn set_white_rank(&mut self, white_rank :Rank) { self.white_rank = white_rank }
    pub fn white_rank(&self) -> &Rank { &self.white_rank }

    pub fn set_black_rank(&mut self, black_rank :Rank) { self.black_rank = black_rank }
    pub fn black_rank(&self) -> &Rank { &self.black_rank }

    pub fn set_board_size(&mut self, board_size :usize) { self.board_size = board_size }
    pub fn board_size(&self) -> usize { self.board_size }

    pub fn set_komi(&mut self, komi :f32) { self.komi = komi }
    pub fn komi(&self) -> f32 { self.komi }

    pub fn set_handicap(&mut self, handicap :u16) { self.handicap = handicap }
    pub fn handicap(&self) -> u16 { self.handicap }

    pub fn set_result(&mut self, result :GameResult) { self.result = result }
    pub fn result(&self) -> &GameResult { &self.result }

    pub fn moves(&self) -> &Vec<GameNode> { &self.moves }
    pub fn moves_as_mut<'r>(&'r mut self) -> &'r mut Vec<GameNode> { &mut self.moves }

    pub fn push(&mut self, gn :GameNode) {
        self.moves.push(gn);
    }

}

impl GameNode {

    pub fn new_simple(m :Move) -> GameNode {
        GameNode { themove: m, comment: String::new(), variants: Vec::with_capacity(0) }
    }

    pub fn new(m :Move, s :&str) -> GameNode {
        GameNode { themove: m, comment: s.to_string(), variants: Vec::with_capacity(0) }
    }

    pub fn push(& mut self, gn :GameNode) {
        self.variants.push(gn)
    }

    pub fn themove(&self) -> Move { self.themove }
    pub fn comment(&self) -> &String { &self.comment }
    pub fn variants(&self) -> &Vec<GameNode> { &self.variants }
    pub fn has_variants(&self) -> bool { !&self.variants.is_empty() }
}

// *********************************************************************************************
// Tests

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use super::*;
    use base::moves::*;
    use base::color::*;
    use base::rank::*;
    use base::game_result::*;

    fn amove(s :&str) -> Move {
        Move::from_str(s).unwrap()
    }

    #[test]
    fn it_can_be_created_and_retains_basic_properties() {
        let mut gt = GameTree::new();
        gt.set_black_name("black".to_string());
        gt.set_white_name("white".to_string());
        gt.set_black_rank(Rank::from_str("5p").unwrap());
        gt.set_white_rank(Rank::from_str("4d").unwrap());
        gt.set_board_size(19);
        gt.set_komi(5.5);
        gt.set_handicap(1);
        gt.set_result(GameResult::from_str("B+4.5").unwrap());

        assert_eq!(&"black".to_string(), gt.black_name());
        assert_eq!(&"white".to_string(), gt.white_name());
        assert_eq!(Rank::Pro(5, true), *gt.black_rank());
        assert_eq!(Rank::Dan(4, true), *gt.white_rank());
        assert_eq!(19, gt.board_size());
        assert_eq!(5.5, gt.komi());
        assert_eq!(1, gt.handicap());
        assert_eq!(GameResult::Score(Color::Black, 4.5), *gt.result());
    }

    #[test]
    fn it_can_push_moves_into_it_simple() {
        let mut gt = GameTree::new();
        gt.push(GameNode::new_simple(amove("Black A1")));

        assert_eq!(1, gt.moves.len());
        let gn = gt.moves().first().unwrap();
        assert_eq!(amove("Black A1"), gn.themove());
        assert_eq!(&String::new(), gn.comment());
        assert_eq!(0, gn.variants().len());
    }

    #[test]
    fn it_can_push_moves_into_it_with_comments() {
        let mut gt = GameTree::new();
        gt.push(GameNode::new(amove("Black A1"), "a good move"));

        assert_eq!(1, gt.moves.len());
        let gn = gt.moves().first().unwrap();
        assert_eq!(amove("Black A1"), gn.themove());
        assert_eq!(&"a good move", gn.comment());
        assert_eq!(0, gn.variants().len());
    }

    #[test]
    fn it_can_push_moves_into_it_complex_with_variant() {
        // B-A1 -> W-A2 -> B-A3 -> W-A4
        //             \-> B-B3 -> W-B4

        let mut gt = GameTree::new();
        gt.push( GameNode::new_simple(amove("Black A1")) );
        gt.push( GameNode::new_simple(amove("White A2")) );
        gt.push( GameNode::new_simple(amove("Black A3")) );
        gt.push( GameNode::new_simple(amove("White A4")) );
        {
            //let mut sub_node = gt.moves_as_mut().front_mut().unwrap();
            let mut sub_node = gt.moves_as_mut().iter_mut().nth(1).unwrap();
            sub_node.push( GameNode::new_simple(amove("Black B3")) );
            sub_node.push( GameNode::new_simple(amove("White B4")) );
        }

        // normal path
        {
            let mut iter = gt.moves().iter();
            assert_eq!(amove("Black A1"), iter.next().unwrap().themove() );
            assert_eq!(amove("White A2"), iter.next().unwrap().themove() );
            assert_eq!(amove("Black A3"), iter.next().unwrap().themove() );
            assert_eq!(amove("White A4"), iter.next().unwrap().themove() );
            assert!( iter.next().is_none() );
        }

        // variants checking, only on the second should have a variant
        {
            let mut iter = gt.moves().iter();
            assert!(!iter.next().unwrap().has_variants() );
            assert!( iter.next().unwrap().has_variants() );
            assert!(!iter.next().unwrap().has_variants() );
            assert!(!iter.next().unwrap().has_variants() );
        }

        // now lets check the variants
        {
            let mut iter = gt.moves().iter();
            let gn = iter.nth(1).unwrap();
            let mut gn_iter = gn.variants().iter();
            assert_eq!(amove("Black B3"),  gn_iter.next().unwrap().themove() );
            assert_eq!(amove("White B4"),  gn_iter.next().unwrap().themove() );
            assert!( gn_iter.next().is_none() );
        }
    }



}
