use std::str::FromStr;

use std::fmt::{Formatter, Error, Display};
use std::hash::{Hash, Hasher};

use base::color::*;

#[derive(Copy, Clone, PartialEq, Debug)] // cant use Eq as f32 does not eq
pub enum GameResult {
    Score(Color, f32),
    Resign(Color),
    Forfeit(Color),
    Time(Color),
    Draw,
    Void,
    Unknown
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct GameResultRange {
    pub result : GameResult,
    pub range  : u16
}

impl GameResult {

    pub fn is_score(&self) -> bool {
        match self {
            &GameResult::Score(_,_) => true,
            _ => false
        }
    }

    pub fn is_time(&self) -> bool {
        match self {
            &GameResult::Time(_) => true,
            _ => false
        }
    }

    pub fn is_resign(&self) -> bool {
        match self {
            &GameResult::Resign(_) => true,
            _ => false
        }
    }
}


impl FromStr for GameResult {

    type Err = GameResultParseError;

    fn from_str(s: &str) -> Result<GameResult, Self::Err> {

        let su = s.to_uppercase();

        if su.is_empty() || su == "?" {
            return Ok(GameResult::Unknown)
        } else if su == "VOID" {
            return Ok(GameResult::Void)
        } else if su == "DRAW" || su == "0" {
            return Ok(GameResult::Draw)
        } else if su == "B+T" || su == "B+TIME" {
            return Ok(GameResult::Time(Color::Black))
        } else if su == "W+T" || su == "W+TIME" {
            return Ok(GameResult::Time(Color::White))
        } else if su == "B+R" || su == "B+RESIGN" {
            return Ok(GameResult::Resign(Color::Black))
        } else if su == "W+R" || su == "W+RESIGN" {
            return Ok(GameResult::Resign(Color::White))
        } else if su == "B+F" || su == "B+FORFEIT" {
            return Ok(GameResult::Forfeit(Color::Black))
        }  else if su == "W+F" || su == "W+FORFEIT" {
            return Ok(GameResult::Forfeit(Color::White))
        } else {
            let mut sp = su.split("+");

            let color;
            if let Some(colst) = sp.next() {
                match colst {
                    "B" => color = Color::Black,
                    "W" => color = Color::White,
                    _ => return Err(GameResultParseError(()))
                }
            } else {
                return Err(GameResultParseError(()))
            }

            let score;
            if let Some(numst) = sp.next() {
                match f32::from_str(numst) {
                    Ok(n)  => score = n,
                    Err(_) => return Err(GameResultParseError(()))
                }
            } else {
                return Err(GameResultParseError(()))
            }

            return Ok(GameResult::Score(color, score))
        }

        return Err(GameResultParseError(()))
    }

}

impl Display for GameResult {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &GameResult::Score(color, score)  => f.write_fmt(format_args!("{}+{}", color.as_char(), score)),
            &GameResult::Resign(color)        => f.write_fmt(format_args!("{}+R", color.as_char())),
            &GameResult::Forfeit(color)       => f.write_fmt(format_args!("{}+F", color.as_char())),
            &GameResult::Time(color)          => f.write_fmt(format_args!("{}+T", color.as_char())),
            &GameResult::Draw                 => f.write_str("Draw"),
            &GameResult::Void                 => f.write_str("Void"),
            &GameResult::Unknown              => f.write_str("?")
        }
    }
}

impl Hash for GameResult {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        state.write( format!("{}", self).as_bytes() )
    }
}

#[derive(Debug)]
pub struct GameResultParseError(());


impl GameResultRange {
    pub fn new(result :GameResult, range :u16) -> Self {
        GameResultRange {
            result: result,
            range: range
        }
    }

    pub fn includes(&self, other_result :&GameResult) -> bool {
        let myself : f32;
        match self.result {
            GameResult::Score(color, score) => { if color==Color::Black { myself = -score } else { myself = score} },
            GameResult::Resign(_)  => myself = 0.0,
            GameResult::Forfeit(_) => myself = 0.0,
            GameResult::Draw       => myself = 0.0,
            GameResult::Time(_)    => myself = 0.0,
            GameResult::Void       => return false,
            GameResult::Unknown    => return false
        }
        let other : f32;
        match other_result {
            &GameResult::Score(color, score) => { if color==Color::Black { other = -score } else { other = score} },
            &GameResult::Resign(_)  => other = 0.0,
            &GameResult::Forfeit(_) => other = 0.0,
            &GameResult::Draw       => other = 0.0,
            &GameResult::Time(_)    => other = 0.0,
            &GameResult::Void       => return false,
            &GameResult::Unknown    => return false
        }
        return (myself-other).abs() <= self.range as f32;
    }

}

impl Display for GameResultRange {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_fmt(format_args!("{}(±{})", self.result, self.range))
    }
}

impl Hash for GameResultRange {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        state.write( format!("{}", self).as_bytes() )
    }
}

// *********************************************************************************************
// Tests

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use super::*;
    use base::color::*;

    #[test]
    fn from_str_invalid() {
        assert!( GameResult::from_str("this is not a game result  string").is_err());
        assert!( GameResult::from_str("B").is_err());
        assert!( GameResult::from_str("B-123123").is_err());
        assert!( GameResult::from_str("B+W").is_err());
        assert!( GameResult::from_str("B+").is_err());
        assert!( GameResult::from_str("???").is_err());
    }

    #[test]
    fn from_str_valid() {

        assert_eq!(GameResult::Score(Color::White, 4.5), GameResult::from_str("W+4.5").unwrap());
        assert_eq!(GameResult::Score(Color::Black, 0.5), GameResult::from_str("B+0.5").unwrap());

        assert_eq!(GameResult::Resign(Color::White), GameResult::from_str("W+R").unwrap());
        assert_eq!(GameResult::Resign(Color::White), GameResult::from_str("W+Resign").unwrap());
        assert_eq!(GameResult::Resign(Color::Black), GameResult::from_str("B+R").unwrap());
        assert_eq!(GameResult::Resign(Color::Black), GameResult::from_str("B+Resign").unwrap());

        assert_eq!(GameResult::Forfeit(Color::White), GameResult::from_str("W+F").unwrap());
        assert_eq!(GameResult::Forfeit(Color::White), GameResult::from_str("W+Forfeit").unwrap());
        assert_eq!(GameResult::Forfeit(Color::Black), GameResult::from_str("B+F").unwrap());
        assert_eq!(GameResult::Forfeit(Color::Black), GameResult::from_str("B+Forfeit").unwrap());

        assert_eq!(GameResult::Time(Color::White), GameResult::from_str("W+T").unwrap());
        assert_eq!(GameResult::Time(Color::White), GameResult::from_str("W+Time").unwrap());
        assert_eq!(GameResult::Time(Color::Black), GameResult::from_str("B+T").unwrap());
        assert_eq!(GameResult::Time(Color::Black), GameResult::from_str("B+Time").unwrap());

        assert_eq!(GameResult::Draw, GameResult::from_str("0").unwrap());
        assert_eq!(GameResult::Draw, GameResult::from_str("Draw").unwrap());

        assert_eq!(GameResult::Void, GameResult::from_str("Void").unwrap());

        assert_eq!(GameResult::Unknown, GameResult::from_str("?").unwrap());
    }

    #[test]
    fn it_from_str_to_str_eq() {
        for co in vec!("B+10", "B+4.5", "W+3", "W+R", "Void", "?", "B+F", "W+T") {
            assert_eq!(format!("{}", GameResult::from_str(&co).unwrap()), co);
        }
    }

    #[test]
    fn it_eq() {
        assert_eq!(GameResult::Score(Color::White, 3.5), GameResult::Score(Color::White, 3.5));
        assert_eq!(GameResult::Resign(Color::Black), GameResult::Resign(Color::Black));
        assert_eq!(GameResult::Forfeit(Color::Black), GameResult::Forfeit(Color::Black));
        assert_eq!(GameResult::Time(Color::Black), GameResult::Time(Color::Black));
        assert_eq!(GameResult::Draw, GameResult::Draw);
        assert_eq!(GameResult::Void, GameResult::Void);
        assert_eq!(GameResult::Unknown, GameResult::Unknown);
    }

    #[test]
    fn it_game_result_range_eq() {
        assert_eq!( GameResultRange::new(GameResult::Score(Color::Black, 5.0), 10), GameResultRange::new(GameResult::Score(Color::Black, 5.0), 10));
        assert!( GameResultRange::new(GameResult::Score(Color::Black, 5.0), 10) != GameResultRange::new(GameResult::Score(Color::Black, 5.0), 5));
        assert!( GameResultRange::new(GameResult::Score(Color::Black, 5.0), 10) != GameResultRange::new(GameResult::Score(Color::Black, 6.0), 10));
        assert!( GameResultRange::new(GameResult::Draw, 10) != GameResultRange::new(GameResult::Score(Color::Black, 5.0), 10));
    }

    #[test]
    fn it_game_result_range_for_score() {
        let score_b1 = GameResult::Score(Color::Black, 0.5);
        let score_b2 = GameResult::Score(Color::Black, 8.5);
        let score_w1 = GameResult::Score(Color::White, 3.0);

        // simplex
        assert!( GameResultRange::new(score_b1, 10).includes(&score_b2) );
        assert!( ! GameResultRange::new(score_b1, 5).includes(&score_b2) );
        assert!( GameResultRange::new(score_b1, 5).includes(&score_w1) ); // B+0.5(+/-5) includes W+3
        assert!( GameResultRange::new(score_w1, 20).includes(&score_b1) );
        assert!( GameResultRange::new(score_w1, 4).includes(&score_b1) );
        assert!( ! GameResultRange::new(score_w1, 4).includes(&score_b2) );

        // but a B+5 +/-7 vs a W+5 should be false, as there are 10 in distance
        assert!( ! GameResultRange::new(GameResult::Score(Color::Black, 5.0), 7)
                        .includes(&GameResult::Score(Color::White, 5.0)) );

        // Resign, Forfeit, Time, Draw assumes "Score=0"
        // resign
        assert!( GameResultRange::new(score_b1, 1).includes(&GameResult::Resign(Color::Black)) );
        assert!( GameResultRange::new(score_b1, 1).includes(&GameResult::Resign(Color::White)) );
        assert!( ! GameResultRange::new(score_b2, 1).includes(&GameResult::Resign(Color::Black)) );
        assert!( ! GameResultRange::new(score_b2, 1).includes(&GameResult::Resign(Color::White)) );
        assert!( GameResultRange::new(score_b2, 10).includes(&GameResult::Resign(Color::White)) );

        // Forfeit
        assert!( GameResultRange::new(score_b1, 1).includes(&GameResult::Forfeit(Color::Black)) );
        assert!( GameResultRange::new(score_b1, 1).includes(&GameResult::Forfeit(Color::White)) );
        assert!( ! GameResultRange::new(score_b2, 1).includes(&GameResult::Forfeit(Color::Black)) );
        assert!( ! GameResultRange::new(score_b2, 1).includes(&GameResult::Forfeit(Color::White)) );
        assert!( GameResultRange::new(score_b2, 10).includes(&GameResult::Forfeit(Color::White)) );

        // Time
        assert!( GameResultRange::new(score_b1, 1).includes(&GameResult::Time(Color::Black)) );
        assert!( GameResultRange::new(score_b1, 1).includes(&GameResult::Time(Color::White)) );
        assert!( ! GameResultRange::new(score_b2, 1).includes(&GameResult::Time(Color::Black)) );
        assert!( ! GameResultRange::new(score_b2, 1).includes(&GameResult::Time(Color::White)) );
        assert!( GameResultRange::new(score_b2, 10).includes(&GameResult::Time(Color::White)) );

        // Draw
        assert!( GameResultRange::new(score_b1, 1).includes(&GameResult::Draw) );
        assert!( ! GameResultRange::new(score_b2, 1).includes(&GameResult::Draw) );
        assert!( GameResultRange::new(score_b2, 10).includes(&GameResult::Draw) );

        // Void and Unknown are always not included in anything, just non-sense
        assert!( ! GameResultRange::new(score_b1, 10).includes(&GameResult::Void ));
        assert!( ! GameResultRange::new(score_b1, 10).includes(&GameResult::Unknown ));

        // border-cases
        assert!( ! GameResultRange::new(score_b1, 0).includes(&GameResult::Resign(Color::Black)) );
        assert!( GameResultRange::new(score_b1, 0).includes(&score_b1) );
    }

}
