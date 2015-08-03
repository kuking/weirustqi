use std::str::FromStr;
use std::fmt::{Formatter, Error, Display};

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

    pub fn all_possibles(board_size :usize) -> Vec<Self> {
        let mut coords = vec!();
        for x in 0..board_size {
            for y in 0..board_size {
                coords.push( Self::new_us(x,y) );
            }
        }
        coords
    }
}

impl FromStr for Coord {
    type Err = CoordParseError;

    fn from_str(s: &str) -> Result<Coord, Self::Err> {

        if s.len() < 2 {
            return Err(CoordParseError(()))
        }

        let mut chars = s.chars();

        // row
        let row : u8;
        if let Some(rowc) = chars.next() {
            // the following might look awkard but it is X3 faster than string mangling.
            let rowno = if rowc>='a' && rowc<='z' { rowc as u32 - 32 } else { rowc as u32 };
            if rowno<'A' as u32 || rowno>'Z' as u32 || rowno == 'I' as u32 {
                return Err(CoordParseError(()))
            }
            if rowno > 'I' as u32 {
                row = (rowno - 'A' as u32 - 1) as u8;
            } else {
                row = (rowno - 'A' as u32) as u8;
            }
        } else {
            return Err(CoordParseError(()))
        }

        // col
        let col : u8;
        match u8::from_str(&s[1..]) {
            Ok(coln) => col = coln - 1,
            Err(_) => return Err(CoordParseError(()))
        }

        Ok(Self::new(row,col))
    }

}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let rowc = if self.row>8 { (self.row+1+65) as char } else { (self.row+65) as char };
        f.write_fmt(format_args!("{}{}", rowc, self.col+1))
    }
}

#[derive(Debug)]
pub struct CoordParseError(());


// *********************************************************************************************
// Tests

#[cfg(test)]
mod test {

    use std::str::FromStr;
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
            Ok(c)  => assert_eq!(Coord::new(0,1), c),
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

    // benchs
    use test::Bencher;

    #[bench]
    fn bench_from_str(b: &mut Bencher) {
        b.iter(|| Coord::from_str(&"R19").unwrap() );
    }

}
