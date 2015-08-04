
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

    //#[test]
    fn test_simple_kill() {
        let mut g = Game::new(19, 5.5, 0);
        play_moves(&mut g, vec!(&"Black B1", &"White A1", &"Black A2") );

        assert_eq!(Color::Empty, g.board().get(Coord::from_str("A1").unwrap()));
        assert_eq!(1, g.dead_count(Color::White));
    }

}
