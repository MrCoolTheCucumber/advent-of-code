use std::collections::VecDeque;

use regex::Regex;

pub fn apply_rearrangement(is_cratemover_9001: bool) {
    let mut lines = common::get_lines!();
    let crate_re = Regex::new(r"(\s\s\s|\[[A-Z]\])\s?").unwrap();
    let move_re = Regex::new(r"move\s(\d+)\sfrom\s(\d+)\sto\s(\d+)").unwrap();

    let mut crates: Vec<VecDeque<char>> = (0..9).map(|_| VecDeque::new()).collect();

    // since we only have 1 input we need to pass, lets just hardcode stuff
    for _ in 0..8 {
        let line = lines.next().unwrap().expect("Unable to read line");
        crate_re
            .captures_iter(&line)
            .into_iter()
            .enumerate()
            .filter(|(_, capture)| !capture.get(1).unwrap().as_str().trim().is_empty())
            .for_each(|(idx, capture)| {
                let letter = capture.get(1).unwrap().as_str()[1..2]
                    .chars()
                    .next()
                    .unwrap();
                crates[idx].push_back(letter);
            });
    }

    let lines = lines.skip(2).map(|line| line.unwrap());
    for line in lines {
        let captures = move_re.captures(&line).unwrap();

        let amount: usize = captures.get(1).unwrap().as_str().parse().unwrap();
        let from: usize = captures.get(2).unwrap().as_str().parse().unwrap();
        let to: usize = captures.get(3).unwrap().as_str().parse().unwrap();

        match is_cratemover_9001 {
            true => {
                let mut order_retainer = VecDeque::new();
                for _ in 0..amount {
                    let c = crates[from - 1].pop_front().unwrap();
                    order_retainer.push_back(c);
                }

                for _ in 0..amount {
                    let c = order_retainer.pop_back().unwrap();
                    crates[to - 1].push_front(c);
                }
            }

            false => {
                for _ in 0..amount {
                    let c = crates[from - 1].pop_front().unwrap();
                    crates[to - 1].push_front(c);
                }
            }
        }
    }

    for c in &mut crates {
        print!("{}", c.pop_front().unwrap());
    }

    println!();
}
