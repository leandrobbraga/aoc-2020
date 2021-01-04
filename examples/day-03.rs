use std::fs;

fn main() {
    let matrix: Vec<Vec<char>> = fs::read_to_string("./examples/input/day-03.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let slopes: Vec<(usize, usize)> = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

    let result: usize = slopes
        .iter()
        .map(|slope| solve(&matrix, slope.0, slope.1))
        .product();

    println!("result: {}", result)
}

fn solve(matrix: &[Vec<char>], v_step: usize, h_step: usize) -> usize {
    let n_rows: usize = matrix.len();
    let n_cols: usize = matrix[0].len();

    let mut n_trees: usize = 0;
    let mut h_pos: usize = 0;

    for v_pos in (v_step..n_rows).step_by(v_step) {
        h_pos += h_step;
        h_pos %= n_cols;

        if matrix[v_pos][h_pos] == '#' {
            n_trees += 1;
        }
    }

    println!("slope ({}, {}) - trees:{}", v_step, h_step, n_trees);

    n_trees
}
