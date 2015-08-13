use std::str::FromStr;
use std::fmt::{Formatter, Error, Display};
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Color {
    Empty = 0,
    Black = 1,
    White = 2,
    Dame = 4,
    BlackTerritory = 5,
    WhiteTerritory = 6
}

impl Color {
    pub fn from_u8(u :u8) -> Color {
        match u {
            0 => Color::Empty,
            1 => Color::Black,
            2 => Color::White,
            4 => Color::Dame,
            5 => Color::BlackTerritory,
            6 => Color::WhiteTerritory,
            n => panic!(format!("{} is not a Color!", &n))
        }
    }
    #[inline]
    pub fn f(color :&Color) -> u8 {
        *color as u8
    }
    pub fn opposite(&self) -> Color {
        match *self {
            Color::Empty => Color::Dame,
            Color::Black => Color::White,
            Color::White => Color::Black,
            Color::Dame  => Color::Empty,
            Color::BlackTerritory => Color::WhiteTerritory,
            Color::WhiteTerritory => Color::BlackTerritory
        }
    }

    pub fn as_char(&self) -> char {
        match *self {
            Color::Empty => '?',
            Color::Black => 'B',
            Color::White => 'W',
            Color::Dame  => '.',
            Color::BlackTerritory => 'X',
            Color::WhiteTerritory => 'O'
        }
    }
}

impl FromStr for Color {
    type Err = ColorParseError;

    fn from_str(s: &str) -> Result<Color, Self::Err> {
        match s.to_uppercase().as_ref() {
            "EMPTY" => Ok(Color::Empty),
            "BLACK" => Ok(Color::Black),
            "WHITE" => Ok(Color::White),
            "DAME"  => Ok(Color::Dame),
            "BLACK_TERRITORY" => Ok(Color::BlackTerritory),
            "WHITE_TERRITORY" => Ok(Color::WhiteTerritory),
            _ => Err(ColorParseError(()))
        }
    }

}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Color::Empty => f.write_str("Empty"),
            Color::Black => f.write_str("Black"),
            Color::White => f.write_str("White"),
            Color::Dame  => f.write_str("Dame"),
            Color::BlackTerritory => f.write_str("Black_Territory"),
            Color::WhiteTerritory => f.write_str("White_Territory")
        }
    }
}

impl Hash for Color {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        state.write_u8(*self as u8);
    }
}

#[derive(Debug)]
pub struct ColorParseError(());

// *********************************************************************************************
// Tests

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use super::*;

    #[test]
    fn it_from_str_happy() {
        assert_eq!(Color::Empty, Color::from_str(&"Empty").unwrap());
        assert_eq!(Color::Black, Color::from_str(&"Black").unwrap());
        assert_eq!(Color::White, Color::from_str(&"White").unwrap());
        assert_eq!(Color::Dame, Color::from_str(&"Dame").unwrap());
        assert_eq!(Color::BlackTerritory, Color::from_str(&"Black_Territory").unwrap());
        assert_eq!(Color::WhiteTerritory, Color::from_str(&"White_Territory").unwrap());


    }

    #[test]
    fn it_from_str_happy_anycase() {
        assert_eq!(Color::Empty, Color::from_str(&"EMPTY").unwrap());
        assert_eq!(Color::Black, Color::from_str(&"BlAcK").unwrap());
        assert_eq!(Color::White, Color::from_str(&"white").unwrap());
        assert_eq!(Color::WhiteTerritory, Color::from_str(&"white_TERRITORY").unwrap());
    }

    #[test]
    fn it_from_str_unhappy() {
        assert!(Color::from_str(&"meh").is_err());
    }


    #[test]
    fn it_from_str_to_str_eq() {
        for co in vec!("Black", "White", "Empty") {
            assert_eq!(format!("{}", Color::from_str(&co).unwrap()), co);
        }
    }

    #[test]
    fn it_as_char() {
        assert_eq!('B', Color::Black.as_char());
        assert_eq!('W', Color::White.as_char());
        assert_eq!('?', Color::Empty.as_char());
        assert_eq!('.', Color::Dame.as_char());
        assert_eq!('X', Color::BlackTerritory.as_char());
        assert_eq!('O', Color::WhiteTerritory.as_char());
    }

}
