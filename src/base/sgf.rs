
use std::str::FromStr;

use base::gametree::*;
use base::moves::*;


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
            } else {
                break
            }

            match process_command_if_avail(gt, chrs, ii) {
                Ok(p) => ii = p,
                Err(_) => return Err(())
            }

            ii = skip_cr_lf_sp(chrs, ii);
        }
        Ok(ii)
    }

    fn process_command_if_avail(gt : &mut GameTree, chrs :&Vec<char>, i :usize) -> Result<usize, ()> {

        let next_open_bracket = scan_with_limit(chrs,'[', i, chrs.len());
        let next_closing_bracket = scan_with_limit(chrs, ']', next_open_bracket, chrs.len());
        if next_open_bracket >= chrs.len() || next_closing_bracket >= chrs.len() {
            return Err(())
        }
        let cmd = up_string(chrs, i, next_open_bracket);
        let params = sub_string(chrs, next_open_bracket + 1, next_closing_bracket);

        println!("cmd={} and params={}", cmd, params);
        if cmd == "GM" { // Game Type
            if  params!="1" {
                return Err(())
            }
        } else if cmd == "FF" { // File Format
            if params!="4" {
                println!("I only know how to deal with FileFormat 4.. hopefully we can increase this");
                return Err(())
            }
        } else if cmd == "SZ" { // Board Size
            match usize::from_str(&params) {
                Ok(size) => gt.set_board_size(size),
                Err(_)   => return Err(())
            }
        } else if cmd == "PW" {
            gt.set_white_name(params);
        } else if cmd == "PB" {
            gt.set_black_name(params);
        } else if cmd == "WR" {




        } else {
            println!("I dont know how to deal with: {}[{}]", cmd, params);
            return Err(())
        }



        Ok(next_closing_bracket + 1)
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

    use super::*;

    //#[test]
    fn no_moves_just_headers() {
        let gt = parse( "(;GM[1]\nFF[4]\nSZ[19]\nPW[gaopo]\nWR[6d]\nPB[Dom]\nBR[6d]\nDT[2015-05-31]
PC[The KGS Go Server at http://www.gokgs.com/]\nKM[6.50]\nRE[B+13.50]\nRU[Japanese]CA[UTF-8]\nST[2]
AP[CGoban:3]\nTM[0]\nOT[3x10 byo-yomi])".to_string()).unwrap();
    }

    #[test]
    fn it_handles_back_slashes() {
        let gt = parse("(;GM[1];FF[4];SZ[19];PW[ed\\uar\\do])".to_string()).unwrap();
        assert_eq!("eduardo", gt.white_name());
    }

}
