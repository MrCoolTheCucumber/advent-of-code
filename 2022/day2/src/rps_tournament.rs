#[derive(Clone, Copy)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Hand {
    pub fn from_opponent(val: &char) -> Self {
        match val {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            _ => unreachable!("Invalid input from opponent column: {val}"),
        }
    }

    pub fn from_suggested(val: &char) -> Self {
        match val {
            'X' => Self::Rock,
            'Y' => Self::Paper,
            'Z' => Self::Scissors,
            _ => unreachable!("Invalid input from opponent column: {val}"),
        }
    }

    pub fn from_required(val: &char, opponent: Self) -> Self {
        match val {
            'X' => match opponent {
                Hand::Rock => Self::Scissors,
                Hand::Paper => Self::Rock,
                Hand::Scissors => Self::Paper,
            },
            'Y' => opponent,
            'Z' => match opponent {
                Hand::Rock => Self::Paper,
                Hand::Paper => Self::Scissors,
                Hand::Scissors => Self::Rock,
            },
            _ => unreachable!("Invalid input from opponent column: {val}"),
        }
    }

    pub fn calculate_score(suggested: Self, opponent: Self) -> u64 {
        let round_score = match suggested {
            Hand::Rock => match opponent {
                Hand::Rock => 3,
                Hand::Paper => 0,
                Hand::Scissors => 6,
            },
            Hand::Paper => match opponent {
                Hand::Rock => 6,
                Hand::Paper => 3,
                Hand::Scissors => 0,
            },
            Hand::Scissors => match opponent {
                Hand::Rock => 0,
                Hand::Paper => 6,
                Hand::Scissors => 3,
            },
        };

        let hand_score = suggested as u64;

        round_score + hand_score
    }
}

pub fn calculate_score() {
    let lines = common::get_lines!();

    let score = lines
        .into_iter()
        .map(|line| {
            let line = line.expect("Unable to parse line");
            let chars = line.chars().collect::<Vec<_>>();

            let opponent = Hand::from_opponent(&chars[0]);
            let suggested = Hand::from_suggested(&chars[2]);

            Hand::calculate_score(suggested, opponent)
        })
        .sum::<u64>();

    println!("Score: {score}");
}

pub fn calculate_score_part_two() {
    let lines = common::get_lines!();

    let score = lines
        .into_iter()
        .map(|line| {
            let line = line.expect("Unable to parse line");
            let chars = line.chars().collect::<Vec<_>>();

            let opponent = Hand::from_opponent(&chars[0]);
            let suggested = Hand::from_required(&chars[2], opponent);

            Hand::calculate_score(suggested, opponent)
        })
        .sum::<u64>();

    println!("Score (part 2): {score}");
}
