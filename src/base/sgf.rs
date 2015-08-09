
use std::str::FromStr;

use base::gametree::*;
use base::game_result::*;
use base::color::*;
use base::coord::*;
use base::moves::*;
use base::rank::*;


    pub fn parse(s : String) -> Result<GameTree,()> {
        let chrs : Vec<char> = s.chars().collect();
        let mut gt = GameTree::new();

        match recursive_greedy_parser(&mut gt, &chrs, 0) {
            Ok(ii) => {
                if ii == chrs.len()  {
                    Ok(gt)
                } else {
                    Err(())
                }
            }
            Err(_) => Err(())
        }

    }

    fn recursive_greedy_parser(gt : &mut GameTree, chrs :&Vec<char>, i : usize) -> Result<usize,()> {

        let mut prev_cmd : String = String::new();
        let mut ii = skip_cr_lf_sp(chrs, i);
        if let Some(&ch) = chrs.get(ii) {
            if ch != '(' {
                return Err(())
            } else {
                ii = ii + 1
            }
        }

        ii = skip_cr_lf_sp(chrs, ii);

        loop {
            if let Some(&ch) = chrs.get(ii) {
                if ch == ')' {
                    ii = ii + 1;
                    break
                }
                if ch == ';' {
                    ii = ii + 1;
                    ii = skip_cr_lf_sp(chrs, ii);
                }
                if ch == ')' {
                    // we are done
                    return Ok(ii+1)
                }
            } else {
                break
            }

            match process_command_if_avail(gt, chrs, ii, prev_cmd) {
                Ok((p, cmd)) => {ii = p; prev_cmd = cmd},
                Err(_)     => return Err(())
            }

            ii = skip_cr_lf_sp(chrs, ii);
        }
        Ok(ii)
    }

    fn process_command_if_avail(gt : &mut GameTree, chrs :&Vec<char>, i :usize, prev_cmd : String) -> Result<(usize, String), ()> {

        let next_open_bracket = scan_with_limit(chrs,'[', i, chrs.len());
        let next_closing_bracket = scan_with_limit(chrs, ']', next_open_bracket, chrs.len());
        if next_open_bracket >= chrs.len() || next_closing_bracket >= chrs.len() {
            return Err(())
        }

        let cmd = if i==next_open_bracket { prev_cmd } else { up_string(chrs, i, next_open_bracket) };
        let params = sub_string(chrs, next_open_bracket + 1, next_closing_bracket);

        //println!("cmd={} and params={}", cmd, params);
        if cmd == "GM" { // Game Type
            if  params!="1" {
                return Err(())
            }
        } else if cmd == "FF" { // File Format
            if params!="4" {
                //FIXME: so far, only fileformat 4 is available.
                return Err(())
            }
        } else if cmd == "SZ" { // Board Size
            match usize::from_str(&params) {
                Ok(size) => gt.set_board_size(size),
                Err(_)   => return Err(())
            }
        } else if cmd == "PW" { // White name
            gt.set_white_name(params);
        } else if cmd == "PB" { // Black name
            gt.set_black_name(params);
        } else if cmd == "WR" { // White rank
            match Rank::from_str(&params) {
                    Ok(rank) => gt.set_white_rank(rank),
                    Err(_)   => return Err(())
            }
        } else if cmd == "BR" { // Black rank
            match Rank::from_str(&params) {
                    Ok(rank) => gt.set_black_rank(rank),
                    Err(_)   => return Err(())
            }
        } else if cmd == "DT" { // game date
            // skip
        } else if cmd == "PC" { // place where game was played
            // skip
        } else if cmd == "KM" {
            match f32::from_str(&params) {
                Ok(komi) => gt.set_komi(komi),
                Err(_)   => return Err(())
            }
        } else if cmd == "HA" { // handicap
            match u16::from_str(&params) {
                Ok(ha) => gt.set_handicap(ha),
                Err(_) => return Err(())
            }
        } else if cmd == "RE" {
            match GameResult::from_str(&params) {
                Ok(result) => gt.set_result(result),
                Err(_)     => return Err(())
            }
        } else if cmd == "RU" { // skip rules
        } else if cmd == "CA" { // skip charset
        } else if cmd == "ST" { // skip how variations should be shown
        } else if cmd == "AP" { // skip 'Application name'
        } else if cmd == "TM" { // skip 'Time Limit'
        } else if cmd == "OT" { // skip 'Overtime method'

        } else if cmd == "AB" { // adds black stones *not a move*
            // !!! this assumes it will be handled by 'handicap' setting in the game
            // this parser is not intended to be used as a generic parser for demostrations, etc.
            // gametree does not handle this sort of events, might consider fixed or improvement
            // in the future //FIXME -maybe-
        } else if cmd == "AW" { // adds white stones *not a move*
            // ^^ READ "AB" comment
        } else if cmd == "W" || cmd == "B" { // White moves, Black moves
            match get_move(&cmd, &params, gt.board_size()) {
                Ok(m)  => gt.push(GameNode::new_simple(m)),
                Err(_) => return Err(())
            }
        } else if cmd == "XX" {
        } else if cmd == "XX" {

        } else {
            println!("I dont know how to deal with: {}[{}]", cmd, params);
            return Err(())
        }

        Ok((next_closing_bracket + 1, cmd))
    }

    fn get_move(cmd :&String, params :&String, board_size :usize) -> Result<Move,()> {
        let color;
        if cmd == "B" {
            color = Color::Black
        } else if cmd == "W" {
            color = Color::White
        } else {
            return Err(())
        }
        if params=="" || (params=="tt" && board_size==19) {
            Ok(Move::Pass(color))
        } else {
            match Coord::from_sgf(&params, board_size as u8) {
                Ok(c)  => Ok(Move::Stone(c, color)),
                Err(_) => Err(())
            }
        }
    }

    fn scan_with_limit(chrs :&Vec<char>, scan_ch :char, i :usize, max :usize) -> usize {
        let mut ii = i;
        while ii < max {
            if let Some(&ch) = chrs.get(ii) {
                if scan_ch == ch {
                    return ii
                }
            }
           ii = ii + 1;
        }
        ii
    }

    fn sub_string(chrs :&Vec<char>, start :usize, end :usize) -> String {
        let mut s = String::with_capacity(end-start);
        let mut i = start;
        while i < end {
            match chrs.get(i) {
                Some(&ch) => {
                    if ch != '\\' {
                        s.push(ch);
                        i = i + 1;
                    } else {
                        i = skip_cr_lf_sp(chrs, i+1);
                    }
                },
                None => return s
            }
        }
        s
    }

    fn up_string(chrs :&Vec<char>, start :usize, end :usize) -> String {
        sub_string(chrs, start, end).to_uppercase()
    }


    fn skip_cr_lf_sp(chrs: &Vec<char>, i : usize) -> usize {
        let mut ii = i;
        loop {
            if let Some(&ch) = chrs.get(ii) {
                if ch == '\n' || ch == '\r' || ch == ' ' {
                    ii = ii + 1
                } else {
                    return ii
                }
            } else {
                return ii
            }
        }
    }

    pub fn write() -> String {
        String::new()
    }



// *********************************************************************************************
// Tests

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use super::*;
    use base::moves::*;
    use base::color::*;
    use base::rank::*;
    use base::game_result::*;

    #[test]
    fn no_moves_just_headers() {
        let gt = parse( "(;GM[1]\nFF[4]\nSZ[19]\nPW[gaopo]\nWR[2d]\nPB[Dom]\nBR[6d]\nDT[2015-05-31]
PC[The KGS Go Server at http://www.gokgs.com/]\nKM[6.50]\nRE[B+13.50]\nRU[Japanese]CA[UTF-8]\nST[2]
AP[CGoban:3]\nTM[0]\nOT[3x10 byo-yomi] HA[4])".to_string()).unwrap();
        assert_eq!(19, gt.board_size());
        assert_eq!("gaopo", gt.white_name());
        assert_eq!(Rank::Dan(2, true), *gt.white_rank());
        assert_eq!("Dom", gt.black_name());
        assert_eq!(Rank::Dan(6, true), *gt.black_rank());
        assert_eq!(6.5, gt.komi());
        assert_eq!(4, gt.handicap());
    }

    #[test]
    fn it_handles_back_slashes() {
        let gt = parse("(;GM[1];FF[4];SZ[19];PW[ed\\uar\\do])".to_string()).unwrap();
        assert_eq!("eduardo", gt.white_name());
    }

    #[test]
    fn happy_path() {
        let gt = parse("(;GM[1] FF[4] SZ[19] PW[somerville] WR[7d] PB[yz221] BR[5d] DT[2011-02-01]
                      PC[The KGS Go Server at http://www.gokgs.com/] KM[0.50] RE[B+Resign] RU[AGA]
                      OT[100x10 byo-yomi] CA[UTF-8] ST[2] AP[CGoban:3] TM[0] HA[2] AB[pd] [dp]
                      ;W[qp];B[dd];W[fq];B[op];W[mp];B[on];W[qn];B[qq];W[rq];B[pq];W[ro];B[ip]
                      ;W[mn];B[ol];W[fo];B[dn];W[in];B[gp];W[fp];B[hn];W[ho];B[io];W[hm];B[gn]
                      ;W[go];B[jn];W[im];B[lo];W[mo];B[jm];W[jl];B[lm];W[ln];B[km];W[oo];B[po]
                      ;W[no];B[pn];W[pp];B[qm];W[qo];B[gm];W[jo];B[gl];W[hk];B[hl];W[il];B[ik]
                      ;W[jk];B[ij];W[mm];B[jj];W[ll];B[ko];W[kn];B[jp];W[kq];B[lj];W[kl];B[em]
                      ;W[ch];B[cf];W[ck];B[bl];W[ek];B[bk];W[fh];B[ff];W[bj];B[gh];W[cl];B[bm]
                      ;W[gi];B[hh];W[fg];B[gf];W[gk];B[dq];W[er];B[dr];W[rm];B[qf];W[kc];B[mc]
                      ;W[hc];B[gb];W[hb];B[lb];W[fd];B[fc];W[ec];B[eb];W[ed];B[dc];W[he];B[gd]
                      ;W[gc];B[fb];W[ge];B[fe];W[hd];B[kb];W[kf];B[jc];W[jd];B[kd];W[ic];B[lc]
                      ;W[ke];B[je];W[hf];B[gg];W[ie];B[id];W[ee];B[ef];W[jd];B[fi];W[ei];B[id]
                      ;W[de];B[ce];W[jd];B[fj];W[dg];B[df];W[jf];B[ej];W[dj];B[di];W[ci];B[eh]
                      ;W[eg];B[gj];W[dh];B[ei];W[hj];B[hi];W[fk];B[gi];W[cm];B[cn];W[gd];B[ql]
                      ;W[qi];B[oi];W[pg];B[qg];W[ph];B[rl];W[ng];B[lg];W[mf];B[lf];W[le];B[me]
                      ;W[ne];B[md];W[od];B[oc];W[pe];B[qd];W[nk];B[ni];W[pj];B[ok];W[oj];B[nj]
                      ;W[mk];B[pi];W[qj];B[qh];W[mj];B[mi];W[li];B[kh];W[kj];B[ki];W[pf];B[qe]
                      ;W[lk];B[mh];W[cd];B[cc];W[bg];B[el];W[bd];B[bc];W[af];B[ig];W[bf];B[ah]
                      ;W[be];B[ad];W[jg];B[jh];W[mg];B[of];W[oh];B[oe];W[ii];B[ji];W[ai];B[bh]
                      ;W[cg];B[nd];W[if];B[nf];W[sl];B[sk];W[sm];B[sn];W[rn];B[rk];W[rh];B[ri]
                      ;W[rj];B[sj];W[si];B[pk];W[om];B[nm];W[nn];B[pm];W[nl];B[qk];W[ds];B[cs]
                      ;W[es];B[br];W[jb];B[kc];W[ga];B[db];W[fa];B[bb];W[eo];B[do];W[en];B[dm])"
                      .to_string()).unwrap();

        assert_eq!("yz221", gt.black_name());
        assert_eq!("somerville", gt.white_name());
        assert_eq!(Rank::Dan(5, true), *gt.black_rank());
        assert_eq!(Rank::Dan(7, true), *gt.white_rank());
        assert_eq!(GameResult::Resign(Color::Black), *gt.result());
        assert_eq!(240, gt.moves().len());
        assert_eq!(Move::from_str("Black N11").unwrap(), gt.moves()[173].themove());
        assert_eq!(Move::from_str("White H9").unwrap(), gt.moves()[44].themove());
        assert_eq!(Move::from_str("White R4").unwrap(), gt.moves()[0].themove());
        assert_eq!(Move::from_str("Black D7").unwrap(), gt.moves()[239].themove());
    }

}
