extern crate rand;

use std::str::FromStr;
use std::fmt::{Formatter, Error, Display};
use std::hash::{Hash, Hasher};
use rand::Rng;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Coord {
    pub row : u8,
    pub col : u8
}

impl Coord {

    pub fn new(le_row :u8, le_col :u8) -> Self {
        Coord {
            row : le_row,
            col : le_col
        }
    }

    pub fn new_us(le_row :usize, le_col :usize) -> Self {
        Self::new(le_row as u8, le_col as u8)
    }

    pub fn random(board_size :usize) -> Self {
        let bs8 = board_size as u8;
        let mut rng = rand::thread_rng();
        Self::new(rng.gen::<u8>() % bs8, rng.gen::<u8>() % bs8)
    }

    pub fn all_possibles(board_size :usize) -> Vec<Self> {
        let mut coords = vec!();
        for x in 0..board_size {
            for y in 0..board_size {
                coords.push( Self::new_us(x,y) );
            }
        }
        coords
    }

    pub fn adjacents(&self, board_size :u8) -> Vec<Coord> {
        if self.col > board_size || self.row > board_size {
            return Vec::with_capacity(0);
        }
        let row_n = self.row as isize + 1;
        let row_s = self.row as isize - 1;
        let col_w = self.col as isize - 1;
        let col_e = self.col as isize + 1;
        let mut r : Vec<Coord> = Vec::with_capacity(4);
        if row_n < board_size as isize {
            r.push(Coord::new(row_n as u8, self.col));
        }
        if row_s >= 0 {
            r.push(Coord::new(row_s as u8, self.col));
        }
        if col_e < board_size as isize {
            r.push(Coord::new(self.row,  col_e as u8));
        }
        if col_w >= 0 {
            r.push(Coord::new(self.row,  col_w as u8));
        }
        r
    }

    pub fn from_sgf(s :&str, board_size :u8) -> Result<Self, ()> {
        let su = s.to_uppercase();
        if su.len() != 2 {
            return Err(())
        }
        let mut chars = su.chars();
        let a : u32 = chars.next().unwrap_or('\0') as u32;
        let b : u32 = chars.next().unwrap_or('\0') as u32;

        if a<65 || b<65 || a>250 || b>250 {
            return Err(())
        }

        let az : u8 = a as u8 - 65;
        let bz : u8 = b as u8 - 65;

        if  az >= board_size || bz >= board_size {
            return Err(())
        }
        Ok(Self::new(board_size-bz-1, az))
    }

}

impl FromStr for Coord {
    type Err = CoordParseError;

    fn from_str(s: &str) -> Result<Coord, Self::Err> {

        if s.len() < 2 {
            return Err(CoordParseError(()))
        }

        let mut chars = s.chars();

        // col
        let col : u8;
        if let Some(colc) = chars.next() {
            // the following might look awkard but it is X3 faster than string mangling.
            let colno = if colc>='a' && colc<='z' { colc as u32 - 32 } else { colc as u32 };
            if colno<'A' as u32 || colno>'Z' as u32 || colno == 'I' as u32 {
                return Err(CoordParseError(()))
            }
            if colno > 'I' as u32 {
                col = (colno - 'A' as u32 - 1) as u8;
            } else {
                col = (colno - 'A' as u32) as u8;
            }
        } else {
            return Err(CoordParseError(()))
        }

        // col
        let row : u8;
        match u8::from_str(&s[1..]) {
            Ok(rown) => row = rown - 1,
            Err(_) => return Err(CoordParseError(()))
        }

        Ok(Self::new(row,col))
    }

}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let colc = if self.col>8 { (self.col+1+65) as char } else { (self.col+65) as char };
        f.write_fmt(format_args!("{}{}", colc, self.row+1))
    }
}

impl Hash for Coord {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        state.write_u8(self.row);
        state.write_u8(self.col);
    }
}

#[derive(Debug)]
pub struct CoordParseError(());


// *********************************************************************************************
// Tests

#[cfg(test)]
mod tests {

    use std::str::FromStr;
    use std::collections::HashSet;
    use super::*;

    #[test]
    fn it_does_new() {
        let c1 = Coord::new(0,1);
        assert_eq!(0, c1.row);
        assert_eq!(1, c1.col);
    }

    #[test]
    fn it_eq_with_diff_constructors() {
        let c1 = Coord::new(0,1);
        let c2 = Coord::new_us(0,1);
        assert_eq!(c1, c2);
    }

    #[test]
    fn it_from_str_happy() {
        match Coord::from_str(&"A2") {
            Ok(c)  => assert_eq!(Coord::new(1,0), c),
            Err(_) => panic!("Coord::form_str('A2') should be fine")
        }
    }

    #[test]
    fn it_does_with_coords_after_leter_i_and_two_digits() {
        match Coord::from_str(&"T19") {
            Ok(c)  => assert_eq!(Coord::new(18,18), c),
            Err(_) => panic!("Coord::from_str('T19') should be raw row:col 18:18 (base 0)")
        }
    }

    #[test]
    fn it_from_str_anycase() {
        assert_eq!(Coord::from_str(&"R18").unwrap(), Coord::from_str(&"r18").unwrap());
    }

    #[test]
    fn it_from_str_err_on_empty() {
        if let Ok(some) = Coord::from_str(&"") {
            panic!(format!("empty string should Err() but we got {:?}", some))
        }
    }

    #[test]
    fn it_from_str_to_str_eq() {
        for co in vec!("A1", "A2", "A3", "A20", "B3", "Z20", "G1", "H5", "Z99") {
            assert_eq!(format!("{}", Coord::from_str(&co).unwrap()), co);
        }
    }

    #[test]
    fn it_adjacents() {
        let adjs = Coord::from_str("D4").unwrap().adjacents(19);
        assert_eq!(4, adjs.len());
        assert!(adjs.contains(&Coord::from_str("D3").unwrap()));
        assert!(adjs.contains(&Coord::from_str("D5").unwrap()));
        assert!(adjs.contains(&Coord::from_str("C4").unwrap()));
        assert!(adjs.contains(&Coord::from_str("E4").unwrap()));
    }

    #[test]
    fn it_adjacents_in_bottom_left_corner() {
        let adjs = Coord::from_str("A1").unwrap().adjacents(19);
        assert_eq!(2, adjs.len());
        assert!(adjs.contains(&Coord::from_str("A2").unwrap()));
        assert!(adjs.contains(&Coord::from_str("B1").unwrap()));
    }

    #[test]
    fn it_adjacents_in_top_right_corner() {
        let adjs = Coord::from_str("T19").unwrap().adjacents(19);
        assert_eq!(2, adjs.len());
        assert!(adjs.contains(&Coord::from_str("S19").unwrap()));
        assert!(adjs.contains(&Coord::from_str("T18").unwrap()));
    }

    #[test]
    fn it_adjacents_right_on_a2() {
        let adjs = Coord::from_str("A2").unwrap().adjacents(19);
        assert_eq!(3, adjs.len());
        assert!(adjs.contains(&Coord::from_str("A3").unwrap()));
        assert!(adjs.contains(&Coord::from_str("A1").unwrap()));
        assert!(adjs.contains(&Coord::from_str("B2").unwrap()));
    }

    #[test]
    fn it_adjacents_right_on_q1() {
        let adjs = Coord::from_str("Q1").unwrap().adjacents(19);
        assert_eq!(3, adjs.len());
        assert!(adjs.contains(&Coord::from_str("Q2").unwrap()));
        assert!(adjs.contains(&Coord::from_str("P1").unwrap()));
        assert!(adjs.contains(&Coord::from_str("R1").unwrap()));
    }

    #[test]
    fn it_adjacents_right_on_t10() {
        let adjs = Coord::from_str("T10").unwrap().adjacents(19);
        assert_eq!(3, adjs.len());
        assert!(adjs.contains(&Coord::from_str("S10").unwrap()));
        assert!(adjs.contains(&Coord::from_str("T11").unwrap()));
        assert!(adjs.contains(&Coord::from_str("T9").unwrap()));
    }

    #[test]
    fn it_adjacents_right_on_k19() {
        let adjs = Coord::from_str("S19").unwrap().adjacents(19);
        assert_eq!(3, adjs.len());
        assert!(adjs.contains(&Coord::from_str("S18").unwrap()));
        assert!(adjs.contains(&Coord::from_str("T19").unwrap()));
        assert!(adjs.contains(&Coord::from_str("R19").unwrap()));
    }

    #[test]
    fn it_adjacents_border_case() {
        assert_eq!(0, Coord::from_str("A1").unwrap().adjacents(1).len());
    }

    #[test]
    fn it_adjacents_outside_board() {
        assert_eq!(0, Coord::from_str("T19").unwrap().adjacents(9).len());
    }

    #[test]
    fn it_hashes_coords() {
        let mut h : HashSet<Coord> = HashSet::new();
        assert_eq!(0, h.len());
        h.insert(Coord::new(1,2));
        assert_eq!(1, h.len());
        // same coord, len is the same
        h.insert(Coord::new(1,2));
        assert_eq!(1, h.len());
        // but another coord will make the set to have two elems
        h.insert(Coord::new(3,3));
        assert_eq!(2, h.len());
        // and another ..
        h.insert(Coord::new(4,5));
        assert_eq!(3, h.len());
    }

    #[test]
    fn it_does_from_sgf() {
        assert_eq!(Coord::from_str("A19").unwrap(), Coord::from_sgf("aa", 19).unwrap());
        assert_eq!(Coord::from_str("T19").unwrap(), Coord::from_sgf("sa", 19).unwrap());
        assert_eq!(Coord::from_str("T1").unwrap(),  Coord::from_sgf("ss", 19).unwrap());

        assert_eq!(Coord::from_str("A9").unwrap(), Coord::from_sgf("aa", 9).unwrap());
        assert_eq!(Coord::from_str("A5").unwrap(), Coord::from_sgf("aa", 5).unwrap());

        assert_eq!(Coord::from_str("D15").unwrap(), Coord::from_sgf("de", 19).unwrap());
        assert_eq!(Coord::from_str("O10").unwrap(), Coord::from_sgf("nj", 19).unwrap());
        assert_eq!(Coord::from_str("T19").unwrap(), Coord::from_sgf("sa", 19).unwrap());
        assert_eq!(Coord::from_str("A1").unwrap(),  Coord::from_sgf("as", 19).unwrap());
        assert_eq!(Coord::from_str("T1").unwrap(),  Coord::from_sgf("ss", 19).unwrap());
        assert_eq!(Coord::from_str("A19").unwrap(), Coord::from_sgf("aa", 19).unwrap());
        assert_eq!(Coord::from_str("J16").unwrap(), Coord::from_sgf("id", 19).unwrap());
        assert_eq!(Coord::from_str("R13").unwrap(), Coord::from_sgf("qg", 19).unwrap());
        assert_eq!(Coord::from_str("H5").unwrap(),  Coord::from_sgf("ho", 19).unwrap());
        assert_eq!(Coord::from_str("K10").unwrap(), Coord::from_sgf("jj", 19).unwrap());
        assert_eq!(Coord::from_str("K9").unwrap(),  Coord::from_sgf("jk", 19).unwrap());

        assert_eq!(Coord::from_str("C7").unwrap(),  Coord::from_sgf("cc", 9).unwrap());
        assert_eq!(Coord::from_str("E5").unwrap(),  Coord::from_sgf("ee", 9).unwrap());
        assert_eq!(Coord::from_str("G3").unwrap(),  Coord::from_sgf("gg", 9).unwrap());
        assert_eq!(Coord::from_str("J9").unwrap(),  Coord::from_sgf("ia", 9).unwrap());
        assert_eq!(Coord::from_str("A9").unwrap(),  Coord::from_sgf("aa", 9).unwrap());
        assert_eq!(Coord::from_str("A1").unwrap(),  Coord::from_sgf("ai", 9).unwrap());
        assert_eq!(Coord::from_str("J1").unwrap(),  Coord::from_sgf("ii", 9).unwrap());
    }

    #[test]
    fn it_handles_invalid_from_sgf() {
        assert!(Coord::from_sgf("sa", 5).is_err());
        assert!(Coord::from_sgf("", 9).is_err());
    }

    // benchs
    use test::Bencher;

    #[bench]
    fn bench_from_str(b: &mut Bencher) {
        b.iter(|| Coord::from_str(&"R19").unwrap() )
    }

    #[bench]
    fn bench_adjacents(b: &mut Bencher) {
        b.iter(|| Coord::new(5,0).adjacents(19) )
    }

}
