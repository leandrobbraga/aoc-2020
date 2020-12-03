use std::fs;

fn main() {
    let map: Vec<String> = fs::read_to_string("./examples/input/day-03.txt")
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();

    let matrix: Vec<Vec<char>> = map.iter().map(|line| line.chars().collect()).collect();
    let slopes = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

    let mut result = 1;

    for slope in slopes {
        result = result * solve(&matrix, slope.0, slope.1);
    }

    println!("{}", result)
}

fn solve(matrix: &Vec<Vec<char>>, mut vertical_step: usize, mut horizontal_step: usize) -> usize {
    let n_rows: usize = matrix.len();
    let n_columns: usize = matrix[0].len();
    let mut n_trees: usize = 0;
    let mut j: usize = 0;

    for i in (vertical_step..n_rows).step_by(vertical_step) {
        j = j + horizontal_step;
        j = j % n_columns;

        if matrix[i][j] == '#' {
            n_trees = n_trees + 1;
        }
    }
    println!(
        "vertical:{} horizontal:{} - n_trees:{}",
        vertical_step, horizontal_step, n_trees
    );
    n_trees
}
