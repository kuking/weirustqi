extern crate weirustqi;

/*
use weirustqi::base::board::*;
use weirustqi::base::coord::*;
use weirustqi::base::moves::*;
use weirustqi::base::game::*;
use std::str::FromStr;


fn play_moves(g :&mut Game, moves : Vec<&str>) {
    for m in moves {
        let mo = Move::from_str(m).unwrap();
        println!(">I'm about to play {}", &mo);
        assert!( g.play( mo ));
        print!("{}{}\n\n", g.pretty_print(), g);
    }
}

fn try_play_invalid_move(g :&mut Game, mov : &str) {
    let mo = Move::from_str(mov).unwrap();
    println!(">I'm about to try to play in {} but it should fail", &mo);
    assert!( !g.play(mo));
    println!("... indeed it failed, so nothing changed");
}



fn check_correct_positions_in_board() {
    let mut g = Game::new(19, 5.5, 0);
    play_moves(&mut g, vec!("black a1", "white a2", "black a3", "white a4", "black a5", "white a6"));
    play_moves(&mut g, vec!("black b1", "white c1", "black d1", "white e1", "black f1", "white g1"));
    play_moves(&mut g, vec!("black d4", "white e5", "black f6"));
}

fn check_eat_four() {
    let mut g = Game::new(19, 5.5, 0);
    play_moves(&mut g, vec!("black k10", "white l10", "black k9",  "white l9", "black j10",
                "white k11", "black j9", "white j11", "black d10", "white h10", "black d9",
                "white h9",  "black d8",  "white j8", "black d11", "white k8", // up to here eats 4
                "black k10", "white j10", "black j9", "white k9")); // then eats 2 different groups
}

fn check_ko() {
    let mut g = Game::new(19, 5.5, 0);
    play_moves(&mut g, vec!("black a1", "white b1", "black a3", "white c1", "black b2", "white a2")); // eat, valid.
    try_play_invalid_move(&mut g, "black a1"); // invalid, ko
    play_moves(&mut g, vec!("black b3"));

    let mut g2 = g.clone();
    println!(" two variants now... we verify white can play and finish the ko");
    play_moves(&mut g2, vec!("white a1"));

    println!(" rolling back... we verify how black can play");
    play_moves(&mut g, vec!("white d1", "black a1")); // now black can eat again
    try_play_invalid_move(&mut g, "white a2"); // and white can´t eat at A2 because its a KO
}

fn check_suicide() {
    let mut g = Game::new(19, 5.5, 0);
    play_moves(&mut g, vec!("black a2", "white f10", "black b1"));
    try_play_invalid_move(&mut g, "white a1")
}


fn play_random_game() {
    let board_size = 19;
    let mut g = Game::new(board_size, 5.5, 0);

    while !g.finished() && g.move_count() < board_size*board_size {
        let mut count = 0;
        let turn_color = g.next_turn();
        loop {
            let m = Move::Stone( Coord::random(board_size), turn_color);
            if g.play(m) {
                break;
            }
            count = count + 1;
            if count > board_size*2 {
                g.play(Move::Pass(turn_color));
                break;
            }
        }
        println!("{}", g.pretty_print());
    }
}
*/


/*
use std::fs::File;
use std::io::Read;
use weirustqi::base::*;

fn calculate_result() {

    let mut f = File::open("sgfs-db/examples/2015-04-01-19.sgf").unwrap();
    //let mut f = File::open("sgfs-db/examples/100.sgf").unwrap();

    let mut s = String::new();
    f.read_to_string(&mut s);
    println!("parsing now... {}", &s);
    let gt;
    match sgf::parse(s) {
        Ok(v)  => gt = v,
        Err(e) => panic!("I couldn't parse the SGF file: {:?}",e)
    }
    let mut game = game::Game::new_for_gametree(&gt);
    for m in gt.moves() {
        assert!(game.play(m.themove()));
        //println!("{}", game.pretty_print());
    }

    let cons_r = scorer::conservative_floodfill_scorer(&game);
    println!("Conservative est: {} real:{} is good estimation? {}", cons_r, gt.result(), cons_r.includes(gt.result()));
    let opt_r = scorer::optimistic_floodfill_scorer(&game);
    println!("Optimistic est: {} real:{} is good estimation? {}", opt_r, gt.result(), opt_r.includes(gt.result()));
}
*/


use weirustqi::mcts::mred::*;
use weirustqi::mcts::analytics::*;
use weirustqi::mcts::analytics::brain_keeper::*;
use weirustqi::base::game::*;
use weirustqi::base::*;

use std::io::Write;
use std::io::stdout;

fn mred_play() {

    let bk = DefaultBrainKeeper::new(10000, 10000, 1);
    let mut mred = MrEd::new(Game::new(19, 4.5, 0), &bk, scorer::optimistic_floodfill_scorer);

    while !mred.game().finished() {

        mred.new_turn();

        println!("MrEd is thinking ...");
        for _ in 1..10 {
            mred.think();
            print!("[{}: {}] ", mred.best_move(), mred.best_result());
            stdout().flush();
        }
        println!("");

        let best_move = mred.best_move();
        let best_result = mred.best_result();
        let suggested_move = mred.suggested_move();
        mred.game_as_mut().play( suggested_move );

        println!("MrEd moves: {} (best result is {}) - Suggested move: {}\n{}", &best_move, &best_result, &suggested_move, mred.game().pretty_print());
    }
    let (board, result) = scorer::optimistic_floodfill_scorer_with_board(mred.game());
    println!("Final board is: \n{}\nFinal result estimated is {}", mred.game().pretty_print_with_board(&board), result);
}

pub fn main() {
    if cfg!(debug_assertions) {
        println!("debug build")
    } else {
        println!("release build")
    }

    println!("Hello wei-rust-qi");

    //check_correct_positions_in_board();
    //check_eat_four();
    //check_ko();
    //play_random_game();
    //check_suicide();
    //calculate_result();

    mred_play();


}
