use base::board::*;
use base::game::*;
use base::game_result::*;
use base::color::*;
use base::moves::*;
use base::coord::*;

use std::collections::HashSet;


pub fn conservative_floodfill_scorer(game :&Game) -> GameResultRange {

    let mut wb = game.board().clone();

    let mut done = false;
    let mut curr_find = wb.find_first(Color::Empty);
    while !done {
        if let Some(curr) = curr_find {

            let ffr = flood_fill(&wb, curr);

            let blacks_l = ffr.blacks.len() as isize;
            let whites_l = ffr.whites.len() as isize;
            let is_white_territory = blacks_l == 0 && whites_l > 0;
            let is_black_territory = whites_l == 0 && blacks_l > 0;

            if is_black_territory {
                for coord in ffr.flooded {
                    wb.set_move(Move::Stone(coord, Color::BlackTerritory));
                }
            } else if is_white_territory {
                for coord in ffr.flooded {
                    wb.set_move(Move::Stone(coord, Color::WhiteTerritory));
                }
            } else {
                for coord in ffr.flooded {
                    wb.set_move(Move::Stone(coord, Color::Dame));
                }
            }

            curr_find = wb.find_next(Color::Empty, &curr);
        } else {
            done = true;
        }
    }

    println!("{}", game.pretty_print_with_board(&wb));

    calculate_result_range(&wb, game, 0, 0, 0)
}

/// similar to conservative_floodfill_scorer but if there are one or two stones in a group, assumes
/// they can be killed.

pub fn optimistic_floodfill_scorer(game :&Game) -> GameResultRange {

    let mut wb = game.board().clone();

    let mut extra_black_captured : u16 = 0;
    let mut extra_white_captured : u16 = 0;

    let mut done = false;
    let mut curr_find = wb.find_first(Color::Empty);
    while !done {
        if let Some(curr) = curr_find {

            let ffr = flood_fill(&wb, curr);

            let flooded_l = ffr.flooded.len() as isize;
            let blacks_l = ffr.blacks.len() as isize;
            let whites_l = ffr.whites.len() as isize;

            let is_white_territory = whites_l > 0 && blacks_l < whites_l && whites_l - blacks_l > 3;
            let is_black_territory = blacks_l > 0 && whites_l < blacks_l && blacks_l - whites_l > 3;

            if is_black_territory {
                for coord in ffr.whites {
                    extra_white_captured = extra_white_captured + 1;
                    wb.set_move(Move::Stone(coord, Color::BlackTerritory));
                }
                for coord in ffr.flooded {
                    wb.set_move(Move::Stone(coord, Color::BlackTerritory));
                }
            } else if is_white_territory {
                for coord in ffr.blacks {
                    extra_black_captured = extra_black_captured + 1;
                    wb.set_move(Move::Stone(coord, Color::WhiteTerritory));
                }
                for coord in ffr.flooded {
                    wb.set_move(Move::Stone(coord, Color::WhiteTerritory));
                }
            } else {
                for coord in ffr.flooded {
                    wb.set_move(Move::Stone(coord, Color::Dame));
                }
            }

            curr_find = wb.find_next(Color::Empty, &curr);
        } else {
            done = true;
        }
    }

    println!("{}", game.pretty_print_with_board(&wb));

    let extra_range = (extra_black_captured + extra_white_captured) * 3 / 2;
    calculate_result_range(&wb, game, extra_black_captured, extra_white_captured, extra_range)
}



fn calculate_result_range(board :&Board, game :&Game, extra_black_captured :u16, extra_white_captured :u16, extra_range :u16) -> GameResultRange {

    let counted_colors = count_colors(board);

    // calculate score, positive is white, negative is black.
    let count : f32 = game.komi()
                    + game.captured_count(Color::Black) as f32 // black captures is white points
                    - game.captured_count(Color::White) as f32
                    + extra_black_captured as f32
                    - extra_white_captured as f32
                    + counted_colors.white as f32
                    - counted_colors.black as f32
                    + counted_colors.white_territory as f32
                    - counted_colors.black_territory as f32;

    let winning_color = if count<0.0 { Color::Black } else { Color::White };
    let winning_score = count.abs();
    let range = counted_colors.dame + extra_range;

    GameResultRange::new(GameResult::Score(winning_color, winning_score), range)
}


pub struct FloodFillResult {
    pub flooded: Vec<Coord>,
    pub whites: Vec<Coord>,
    pub blacks: Vec<Coord>
}

pub fn flood_fill(board : &Board, coord : Coord) -> FloodFillResult {
    let mut flooded : HashSet<Coord> = HashSet::new();
    let mut whites : HashSet<Coord> = HashSet::new();
    let mut blacks : HashSet<Coord> = HashSet::new();

    let mut stack = vec!();
    if board.get(&coord) == Color::Empty {
        flooded.insert(coord);
        stack.push(coord);
    }
    while !stack.is_empty() {
        let curr = stack.pop().unwrap();
        let adjs = curr.adjacents(board.size());
        for adj in adjs {
            let adj_color = board.get(&adj);
            if adj_color == Color::Empty {
                if flooded.insert(adj) {
                    stack.push(adj);
                }
            } else if adj_color == Color::White {
                whites.insert(adj);
            } else if adj_color == Color::Black {
                blacks.insert(adj);
            }
        }
    }


    FloodFillResult {
        flooded: flooded.into_iter().collect(),
        whites: whites.into_iter().collect(),
        blacks: blacks.into_iter().collect()
    }
}

pub struct CountColorsResult {
    empty :u16,
    black :u16,
    white :u16,
    dame  :u16,
    black_territory :u16,
    white_territory :u16
}

pub fn count_colors(board :&Board) -> CountColorsResult {
    // the following is so we don't need to get-set in a hashmap for eat entry
    let mut empty : u16 = 0;
    let mut black : u16 = 0;
    let mut white : u16 = 0;
    let mut black_territory : u16 = 0;
    let mut white_territory : u16 = 0;
    let mut dame : u16 = 0;

    for color in board.data() {
        match color {
            &Color::Empty => empty = empty + 1,
            &Color::Black => black = black + 1,
            &Color::White => white = white + 1,
            &Color::BlackTerritory => black_territory = black_territory + 1,
            &Color::WhiteTerritory => white_territory = white_territory + 1,
            &Color::Dame => dame = dame + 1,
        }
    }

    CountColorsResult {
        empty: empty,
        black: black,
        white: white,
        dame: dame,
        black_territory: black_territory,
        white_territory: white_territory
    }
}
