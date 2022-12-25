use std::cmp;

fn main() {
    find_visible_tree_count();
    find_highest_scenic_score();
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

fn find_highest_scenic_score() {
    let lines = common::get_lines!();
    let trees: Vec<Vec<i32>> = lines
        .into_iter()
        .map(|line| line.unwrap())
        .map(|line| {
            line.chars()
                .map(|char| char.to_string().parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let height = trees.len();
    let width = trees[0].len();

    let mut scores: Vec<Vec<i32>> = (0..height)
        .map(|_| (0..width).map(|_| 0).collect())
        .collect();

    for h in 0..height {
        for w in 0..width {
            // calculate scnenic score for tree [h, w]
            let tree_height = trees[h][w];

            // search up
            let mut up_score = 0;
            'up: for y in (0..h).rev() {
                let height = trees[y][w];
                up_score += 1;

                if height >= tree_height {
                    break 'up;
                }
            }

            // down
            let mut down_store = 0;
            #[allow(clippy::needless_range_loop)]
            'down: for y in (h + 1)..height {
                let height = trees[y][w];
                down_store += 1;

                if height >= tree_height {
                    break 'down;
                }
            }

            // left
            let mut left_score = 0;
            'left: for x in (0..w).rev() {
                let height = trees[h][x];
                left_score += 1;

                if height >= tree_height {
                    break 'left;
                }
            }

            // right
            let mut right_score = 0;
            'right: for x in (w + 1)..width {
                let height = trees[h][x];
                right_score += 1;

                if height >= tree_height {
                    break 'right;
                }
            }

            scores[h][w] = up_score * down_store * left_score * right_score;
        }
    }

    let max_score = *scores.iter().flatten().max().unwrap();
    println!("Highest scenic score: {}", max_score);
}
