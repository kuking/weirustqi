
#[cfg(test)]
mod game_test {

    use std::str::FromStr;

    use base::game::*;
    use base::coord::*;
    use base::moves::*;
    use base::color::*;

    fn play_moves(game :&mut Game, moves : Vec<&str>) {
        for m in moves {
            assert!( game.play( Move::from_str(m).unwrap() ));
        }
    }

    fn try_play_invalid_move(g :&mut Game, mov : &str) {
        let mo = Move::from_str(mov).unwrap();
        assert!( !g.play(mo));
    }

    fn assert_color(g :&Game, c :Color, pos :&str) {
        assert_eq!(c, g.board().get(&Coord::from_str(pos).unwrap()));
    }

    #[test]
    fn simplest_kill() {
        let mut g = Game::new(19, 5.5, 0);
        play_moves(&mut g, vec!(&"Black B1", &"White A1", &"Black A2") );

        assert_eq!(Color::Empty, g.board().get(&Coord::from_str("A1").unwrap()));
        assert_eq!(1, g.captured_count(Color::White));
    }

    #[test]
    fn four_kills() {
        /*
           A B C D E F G H J K L M N O P Q R S T
        19 . . . . . . . . . . . . . . . . . . . 19
        18 . . . . . . . . . . . . . . . . . . . 18
        17 . . . . . . . . . . . . . . . . . . . 17
        16 . . . + . . . . . + . . . . . + . . . 16
        15 . . . . . . . . . . . . . . . . . . . 15
        14 . . . . . . . . . . . . . . . . . . . 14
        13 . . . . . . . . . . . . . . . . . . . 13
        12 . . . . . . . . . . . . . . . . . . . 12
        11 . . . X . . . . O O . . . . . . . . . 11  X = black
        10 . . . X . . . O * * O . . . . + . . . 10  * = dead black (4)
         9 . . . X . . . O * * O . . . . . . . .  9  O = white
         8 . . . X . . . . O O . . . . . . . . .  8
         7 . . . . . . . . . . . . . . . . . . .  7
         6 . . . . . . . . . . . . . . . . . . .  6
         5 . . . . . . . . . . . . . . . . . . .  5
         4 . . . + . . . . . + . . . . . + . . .  4
         3 . . . . . . . . . . . . . . . . . . .  3
         2 . . . . . . . . . . . . . . . . . . .  2
         1 . . . . . . . . . . . . . . . . . . .  1
           A B C D E F G H J K L M N O P Q R S T
        */
        let mut g = Game::new(19, 5.5, 0);
        play_moves(&mut g, vec!("black k10", "white l10", "black k9",  "white l9",  "black j10",
                    "white k11", "black j9", "white j11", "black d10", "white h10", "black d9",
                    "white h9",  "black d8", "white j8",  "black d11", "white k8"));

        assert_color(&g, Color::Empty, "k10");
        assert_color(&g, Color::Empty, "j9");
        assert_color(&g, Color::Empty, "j10");
        assert_color(&g, Color::Empty, "k9");
        assert_eq!(0, g.captured_count(Color::White));
        assert_eq!(4, g.captured_count(Color::Black));
    }

    #[test]
    fn simple_kill_then_double_kill() {
        let mut g = Game::new(19, 5.5, 0);
        play_moves(&mut g, vec!("black k10", "white l10", "black k9",  "white l9", "black j10",
                    "white k11", "black j9", "white j11", "black d10", "white h10", "black d9",
                    "white h9",  "black d8",  "white j8", "black d11", "white k8", // up to here eats 4
                    "black k10", "white j10", "black j9", "white k9")); // then eats 2 different groups

        assert_color(&g, Color::Empty, "k10");
        assert_color(&g, Color::Empty, "j9");
        assert_color(&g, Color::White, "j10");
        assert_color(&g, Color::White, "k9");
        assert_eq!(0, g.captured_count(Color::White));
        assert_eq!(6, g.captured_count(Color::Black));
    }

    #[test]
    fn basic_ko() {
        /*
        *  5 . . . . . . . . .
        *  4 . . . + . . . . .
        *  3 X X . . . . . . .
        *  2 . X . . . . . . .
        *  1(X)O O O . . . . .
        *    A B C D E F G H J
        */
        let mut g = Game::new(19, 5.5, 0);

        play_moves(&mut g, vec!("black a1", "white b1", "black a3", "white c1", "black b2", "white a2")); // eat, valid.
        assert_eq!((1,0), g.captured());

        try_play_invalid_move(&mut g, "black a1");
        play_moves(&mut g, vec!("black b3"));

        // two variants
        let mut g2 = g.clone();

        // variant: white finishes the ko
        play_moves(&mut g2, vec!("white a1"));
        assert_eq!((1,0), g.captured());

        // variant: white plays somewhere else, black can play in the eye (ex ko) again
        play_moves(&mut g, vec!("white d1", "black a1")); // now black can eat again, because w-d1
        try_play_invalid_move(&mut g, "white a2");// and white can´t eat at A2 because its a KO
        assert_eq!((1,1), g.captured());
    }

    #[test]
    fn test_suicide() {
        let mut g = Game::new(19, 5.5, 0);
        play_moves(&mut g, vec!("black a2", "white f10", "black b1"));
        try_play_invalid_move(&mut g, "white a1")
    }

}
