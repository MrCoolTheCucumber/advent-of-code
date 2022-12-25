use std::cmp;

fn main() {
    find_visible_tree_count();
}

fn find_visible_tree_count() {
    let lines = common::get_lines!();
    let mut trees: Vec<Vec<(i32, bool)>> = lines
        .into_iter()
        .map(|line| line.unwrap())
        .map(|line| {
            line.chars()
                .map(|char| (char.to_string().parse::<i32>().unwrap(), false))
                .collect::<Vec<_>>()
        })
        .collect();

    let mut total_visible = 0;

    let mut fold_closure = |tallest_seen: i32, tree: &mut (i32, bool)| {
        if !tree.1 && tree.0 > tallest_seen {
            total_visible += 1;
            tree.1 = true;
        }

        cmp::max(tallest_seen, tree.0)
    };

    // search row_wise first
    for tree_row in &mut trees {
        // left to right
        tree_row.iter_mut().fold(-1, &mut fold_closure);

        // right to left
        tree_row.iter_mut().rev().fold(-1, &mut fold_closure);
    }

    // search collumn wise
    // convert tree to columnwise
    let height = trees.len();
    let width = trees[0].len();

    let mut rotated_trees: Vec<Vec<(i32, bool)>> = Vec::new();

    for w in 0..width {
        let new_row = (0..height).map(|h| trees[h][w]).collect::<Vec<_>>();
        rotated_trees.push(new_row);
    }

    let mut trees = rotated_trees;

    for tree_row in &mut trees {
        // left to right
        tree_row.iter_mut().fold(-1, &mut fold_closure);

        // right to left
        tree_row.iter_mut().rev().fold(-1, &mut fold_closure);
    }

    println!("Total visible: {}", total_visible);
}
