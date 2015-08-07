use std::str::FromStr;

use std::fmt::{Formatter, Error, Display};
use std::hash::{Hash, Hasher};

use regex::Regex;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Rank {
    Pro(u8, bool),
    Dan(u8, bool),
    Kyu(u8, bool),
    Unknown
}


impl FromStr for Rank {

    type Err = RankParseError;

    fn from_str(s: &str) -> Result<Rank, Self::Err> {

        if s.is_empty() || s == "?" {
            return Ok(Rank::Unknown)
        }

        let re = Regex::new(r"(?i)^(\d{1,2}) *(PRO|P|DAN|D|KYU|K) *([\\?\\*])?$").unwrap();
        if let Some(cap) = re.captures(s) {

            let number;
            match cap.at(1) {
                Some(n) => number = u8::from_str(n).unwrap_or(255),
                None => return Err(RankParseError(()))
            }
            let categ;
            match cap.at(2) {
                Some(c) => categ = c,
                None => return Err(RankParseError(()))
            }
            let qualifier;
            match cap.at(3) {
                Some(q) => qualifier = q!="?",
                None => qualifier = true
            }

            let categ_char = categ.chars().next().unwrap_or(' ');
            if categ_char == 'p' || categ_char == 'P' {
                if number >= 1 && number <=10 {
                    return Ok(Rank::Pro(number, qualifier))
                }
            }
            if categ_char == 'd' || categ_char == 'D' {
                if number >=1 && number <=8 {
                    return Ok(Rank::Dan(number, qualifier))
                }
            }
            if categ_char == 'k' || categ_char == 'K' {
                if number >=1 && number <=30 {
                    return Ok(Rank::Kyu(number, qualifier))
                }
            }

        }
        return Err(RankParseError(()))
    }

}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Rank::Pro(n, b) => { f.write_fmt(format_args!("{}p", n)).and( if !b {f.write_str("?")} else { Ok(()) }) },
            &Rank::Dan(n, b) => { f.write_fmt(format_args!("{}d", n)).and( if !b {f.write_str("?")} else { Ok(()) }) },
            &Rank::Kyu(n, b) => { f.write_fmt(format_args!("{}k", n)).and( if !b {f.write_str("?")} else { Ok(()) }) },
            &Rank::Unknown => f.write_str("?")
        }
    }
}

impl Hash for Rank {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        match self {
            &Rank::Pro(n, b) => { state.write_u32('p' as u32); state.write_u8(n); state.write_u8(b as u8) },
            &Rank::Dan(n, b) => { state.write_u32('p' as u32); state.write_u8(n); state.write_u8(b as u8) },
            &Rank::Kyu(n, b) => { state.write_u32('p' as u32); state.write_u8(n); state.write_u8(b as u8) },
            &Rank::Unknown => state.write_u64(0xdeadbeef)
        }
    }
}

#[derive(Debug)]
pub struct RankParseError(());

// *********************************************************************************************
// Tests

#[cfg(test)]
mod tests {

    use std::str::FromStr;
    use super::*;

    #[test]
    fn from_str_unknown() {
        assert_eq!(Rank::Unknown, Rank::from_str("").unwrap());
        assert_eq!(Rank::Unknown, Rank::from_str("?").unwrap());
    }

    #[test]
    fn from_str_invalid() {
        assert!( Rank::from_str("this is not a ranking string").is_err());
        assert!( Rank::from_str("p").is_err());
        assert!( Rank::from_str("pro").is_err());
        assert!( Rank::from_str("k").is_err());
        assert!( Rank::from_str("kyu").is_err());
        assert!( Rank::from_str("d").is_err());
        assert!( Rank::from_str("dan").is_err());
        assert!( Rank::from_str("200 pro").is_err());
        assert!( Rank::from_str("100 dan?").is_err());
        assert!( Rank::from_str("*").is_err());
    }

    #[test]
    fn from_str_valid() {
        assert_eq!(Rank::Dan(5, true), Rank::from_str("5D").unwrap());

        assert_eq!(Rank::Dan(5, true), Rank::from_str("5d").unwrap());
        assert_eq!(Rank::Dan(5, true), Rank::from_str("5d*").unwrap());

        assert_eq!(Rank::Dan(5, false), Rank::from_str("5d?").unwrap());
        assert_eq!(Rank::Dan(5, false), Rank::from_str("5 dan ?").unwrap());

        assert_eq!(Rank::Pro(3, true), Rank::from_str("3p").unwrap());
        assert_eq!(Rank::Pro(3, true), Rank::from_str("3p*").unwrap());
        assert_eq!(Rank::Pro(3, true), Rank::from_str("3P").unwrap());
        assert_eq!(Rank::Pro(3, true), Rank::from_str("3pro").unwrap());
        assert_eq!(Rank::Pro(3, true), Rank::from_str("3PRO").unwrap());
        assert_eq!(Rank::Pro(3, true), Rank::from_str("3PRO*").unwrap());
        assert_eq!(Rank::Pro(3, true), Rank::from_str("3 p").unwrap());
        assert_eq!(Rank::Pro(3, true), Rank::from_str("3 P").unwrap());

        assert_eq!(Rank::Kyu(10, true), Rank::from_str("10 k").unwrap());
        assert_eq!(Rank::Kyu(10, true), Rank::from_str("10k").unwrap());
        assert_eq!(Rank::Kyu(10, true), Rank::from_str("10kyu").unwrap());
        assert_eq!(Rank::Kyu(10, true), Rank::from_str("10Kyu").unwrap());
        assert_eq!(Rank::Kyu(10, true), Rank::from_str("10 KyU*").unwrap());
        assert_eq!(Rank::Kyu(10, false), Rank::from_str("10 KyU ?").unwrap());
    }

    #[test]
    fn from_str_overflow() {
        assert!(Rank::from_str("11p").is_err());
        assert!(Rank::from_str("9p").is_ok());
        assert!(Rank::from_str("0p").is_err());
        assert!(Rank::from_str("-1p").is_err());

        assert!(Rank::from_str("9d").is_err());
        assert!(Rank::from_str("8d").is_ok());
        assert!(Rank::from_str("0d").is_err());
        assert!(Rank::from_str("-1d").is_err());

        assert!(Rank::from_str("31k").is_err());
        assert!(Rank::from_str("30k").is_ok());
        assert!(Rank::from_str("10k").is_ok());
        assert!(Rank::from_str("1k").is_ok());
        assert!(Rank::from_str("0k").is_err());
        assert!(Rank::from_str("-1k").is_err());
    }

    #[test]
    fn it_from_str_to_str_eq() {
        for co in vec!("9p", "9p?", "25k?", "20k", "8d", "8d?") {
            assert_eq!(format!("{}", Rank::from_str(&co).unwrap()), co);
        }
    }

    #[test]
    fn it_eq() {
        assert_eq!(Rank::Pro(5,true), Rank::Pro(5, true));
        assert!(Rank::Pro(5,true) != Rank::Pro(5, false));
        assert!(Rank::Dan(5,true) != Rank::Pro(5, false));
        assert!(Rank::Kyu(5,true) != Rank::Pro(5, true));
    }

}
