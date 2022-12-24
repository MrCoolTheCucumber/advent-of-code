use std::{cmp::Reverse, collections::BinaryHeap};

pub fn find_most() {
    let lines = common::get_lines!();

    let mut most_calories = 0;
    let mut current_calories = 0;

    for line in lines {
        let line = line.expect("Unable to read line");

        if line.is_empty() {
            if current_calories > most_calories {
                most_calories = current_calories;
            }

            current_calories = 0;
        } else {
            let calories = line.parse::<u64>().expect("Unable to parse calories");
            current_calories += calories;
        }
    }

    println!("Most calories: {most_calories}");
}

pub fn find_sum_of_top_three() {
    let lines = common::get_lines!();

    let mut heap = BinaryHeap::new();
    let mut current_calories = 0;

    for line in lines {
        let line = line.expect("Unable to read line");

        if line.is_empty() {
            heap.push(Reverse(current_calories));
            if heap.len() > 3 {
                heap.pop();
            }

            current_calories = 0;
        } else {
            let calories = line.parse::<u64>().expect("Unable to parse calories");
            current_calories += calories;
        }
    }

    let total_calories = heap.into_iter().map(|item| item.0).sum::<u64>();
    println!("Total calories: {total_calories}");
}
