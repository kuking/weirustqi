pub mod board;
pub mod game;

use std::cmp::Eq;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Color {
    Empty = 0,
    Black = 1,
    White = 2
}

impl Color {
    pub fn from_u8(u :u8) -> Color {
        match u {
            0 => Color::Empty,
            1 => Color::Black,
            2 => Color::White,
            n => panic!(format!("{} is not a Color!", &n))
        }
    }
    #[inline]
    pub fn f(color :&Color) -> u8 {
        *color as u8
    }
}

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
    pub fn from_str(st : &str) -> Self {
        Self::new(0,0)
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

pub enum Move {
    Pass,
    Stone { color :Color, coord : Coord }
}
