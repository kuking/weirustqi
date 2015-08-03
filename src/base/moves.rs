use std::str::FromStr;
use std::fmt::{Formatter, Error, Display};

use base::color::Color;
use base::coord::Coord;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Pass (Color ),
    Stone ( Coord, Color )
}

impl FromStr for Move {
    type Err = MoveParseError;
    fn from_str(s :&str) -> Result<Move, Self::Err> {

        if (s.len()<6) {
            return Err(MoveParseError(()))
        }

        let color;
        match Color::from_str(&s[0..5]) {
            Ok(c)  => color = c,
            Err(_) => return Err(MoveParseError(()))
        }

        if color == Color::Empty {
            return Err(MoveParseError(()))
        }

        if &s[6..].to_uppercase() == "PASS" {
            return Ok(Move::Pass(color))
        }

        let coord;
        match Coord::from_str(&s[6..]) {
            Ok(c)  => coord = c,
            Err(_) => return Err(MoveParseError(()))
        }

        Ok(Move::Stone(coord, color))
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Move::Pass(color) => f.write_fmt(format_args!("{} Pass", &color)),
            Move::Stone(coord, color) => f.write_fmt(format_args!("{} {}", color, coord))
        }
    }
}


#[derive(Debug)]
pub struct MoveParseError(());


// *********************************************************************************************
// Tests

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use super::*;
    use base::color::*;
    use base::coord::*;

    #[test]
    fn it_from_str_happy() {
        assert_eq!(Move::Pass(Color::White), Move::from_str(&"White Pass").unwrap());
        assert_eq!(Move::Pass(Color::Black), Move::from_str(&"Black Pass").unwrap());
        assert_eq!(Move::Stone(Coord::from_str(&"A1").unwrap(), Color::White), Move::from_str(&"White A1").unwrap() );
        assert_eq!(Move::Stone(Coord::from_str(&"G7").unwrap(), Color::Black), Move::from_str(&"Black G7").unwrap() );
    }

    #[test]
    fn it_from_str_happy_anycase() {
        assert_eq!(Move::Pass(Color::Black), Move::from_str(&"BlAck pASS").unwrap());
        assert_eq!(Move::Stone(Coord::from_str(&"A1").unwrap(), Color::White), Move::from_str(&"WhitE a1").unwrap() );
    }

    #[test]
    fn it_from_str_unhappy() {
        assert!(Move::from_str(&"meh").is_err());
        assert!(Move::from_str(&"White").is_err());
        assert!(Move::from_str(&"White Meh").is_err());
        assert!(Move::from_str(&"meh A1").is_err());
    }

    #[test]
    fn if_from_str_empty_not_valid_move() {
       assert!(Move::from_str(&"Empty A1").is_err());
       assert!(Move::from_str(&"empty pass").is_err());
   }

    #[test]
    fn it_from_str_to_str_eq() {
        for co in vec!("Black A1", "White T5", "White G7") {
            assert_eq!(format!("{}", Move::from_str(&co).unwrap()), co);
        }
    }

}
