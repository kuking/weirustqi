
#[cfg(test)]
mod game_test {

    use std::str::FromStr;

    use base::game::*;
    use base::coord::*;
    use base::moves::*;
    use base::color::*;

    //#[test]
    fn test_simple_kill() {
        let mut g = Game::new(19, 5.5, 0);
        assert!( g.play( & Move::Stone { coord: Coord::from_str(&"B1").unwrap(), color: Color::Black } ));
    }



}
