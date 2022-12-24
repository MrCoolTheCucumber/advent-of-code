use std::collections::HashSet;

trait HasPriority {
    fn priority(&self) -> u32;
}

impl HasPriority for &char {
    fn priority(&self) -> u32 {
        let as_num = (**self) as u32;
        match as_num {
            65..=90 => as_num - 65 + 27,
            97..=122 => as_num - 97 + 1,
            _ => unreachable!("Unexpected input"),
        }
    }
}

pub fn find_sum_of_priorities() {
    let lines = common::get_lines!();
    let mut sum = 0;

    'lines: for line in lines {
        let line = line.expect("Unable to parse line");

        let chars = line.chars().collect::<Vec<_>>();
        let mut compartments = chars.chunks_exact(chars.len() / 2);

        let first_compartment = compartments.next().expect("No first compartment?");
        let mut letters = first_compartment.iter().copied().collect::<HashSet<_>>();

        let second_compartment = compartments.next().expect("No second compartment?");
        for item in second_compartment {
            if letters.remove(item) {
                sum += item.priority();
                continue 'lines;
            }
        }

        panic!("Duplicate item in line not found");
    }

    println!("Sum of priorities: {sum}");
}

pub fn find_sum_part_two() {
    let lines = common::get_lines!()
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .expect("Unable to read lines");
    let groups = lines.chunks_exact(3);

    let mut sum = 0;

    for group in groups {
        let ruck1: HashSet<char> = group[0].chars().collect();
        let ruck2: HashSet<char> = group[1].chars().collect();

        let badge = group[2]
            .chars()
            .into_iter()
            .find(|item| ruck1.contains(item) && ruck2.contains(item))
            .expect("Unable to find badge");

        sum += (&badge).priority()
    }

    println!("Sum of badge priorities: {sum}");
}
