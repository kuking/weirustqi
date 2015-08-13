
#[cfg(test)]
mod scorer_regression_test {

    use zip::read::ZipArchive;
    use std::fs::File;
    use std::io::Read;

    use base::gametree::*;
    use base::game::*;
    use base::*;

    //#[test]
    fn it_scores_all_games_in_regression_tests() {

        let mut total = 0;
        let mut conservative_ok = 0;
        let mut optimistic_ok = 0;


        //let mut zipf = File::open("sgfs-db/kgs-newest/dl.u-go.net/gamerecords/KGS-2015_01-19-1212-.zip").unwrap();
        let mut zipf = File::open("sgfs-db/kgs-newest/dl.u-go.net/gamerecords/KGS-2005-19-13941-.zip").unwrap();
        let mut zip = ZipArchive::new(zipf).unwrap();

        for i in 0..zip.len() {
            let mut file = zip.by_index(i).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content);

            if content.len() != 0 {
                if let Ok(gt) = sgf::parse(content) {
                    if gt.moves().len() < gt.board_size()*gt.board_size()/2 && !gt.result().is_time() {

                        println!("Filename: {}", file.name());

                        let mut game = game::Game::new_for_gametree(&gt);
                        for m in gt.moves() {
                            if !game.play(m.themove()) { println!("move failed: {:?}", m.themove())}
                        }
                        let cons_r = scorer::conservative_floodfill_scorer(&game);

                        println!("Conservative est: {} real:{} is good estimation? {}", cons_r, gt.result(), cons_r.includes(gt.result()));
                        let opt_r = scorer::optimistic_floodfill_scorer(&game);
                        println!("Optimistic est: {} real:{} is good estimation? {}", opt_r, gt.result(), opt_r.includes(gt.result()));

                        if cons_r.includes(gt.result()) {
                            conservative_ok = conservative_ok + 1
                        }
                        if opt_r.includes(gt.result()) {
                            optimistic_ok = optimistic_ok + 1;
                        }
                        total = total + 1;
                    }

                }
            }

        }

        println!("Conservative {}/{:2}%- Optimistic {}/{:2} - out of {}",
                conservative_ok, conservative_ok as f32 * 100.0 / total as f32,
                optimistic_ok, optimistic_ok as f32 * 100.0 / total as f32,
                total);

        assert!(false);

    }


}
