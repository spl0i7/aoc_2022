use std::cmp::max;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut grid: Vec<Vec<char>> = Vec::new();

    INPUT.split("\n")
        .map(|x| x.chars())
        .map(|x| Vec::from_iter(x))
        .for_each(|x| grid.push(x));

    println!("{}", part1(&grid));
    println!("{}", part2(&grid));
}

fn part1(grid: &Vec<Vec<char>>) -> usize {
    let mut counter = 0;
    let height = grid.len();
    let width = grid.len();

    for i in 0..height {
        for j in 0..width {
            if i == height - 1 || i == 0 || j == 0 || j == width - 1 {
                counter += 1;
                continue;
            }

            if [(0..j).all(|k| grid[i][j] > grid[i][k]),
                (j + 1..width).all(|k| grid[i][j] > grid[i][k]),
                (0..i).all(|k| grid[i][j] > grid[k][j]),
                (i + 1..height).all(|k| grid[i][j] > grid[k][j])].iter().any(|x| *x) { counter += 1 }
        }
    }

    counter
}

fn part2(grid: &Vec<Vec<char>>) -> usize {
    let mut score = 0;
    let height = grid.len();
    let width = grid.len();

    for i in 1..height - 1 {
        for j in 1..width - 1 {
            let mut current_score = 1;

            let mut current_itr = 0;
            for k in (0..j).rev() {
                current_itr += 1;
                if grid[i][j] <= grid[i][k] {
                    break;
                }
            }
            current_score *= current_itr;

            let mut current_itr = 0;
            for k in j + 1..width {
                current_itr += 1;
                if grid[i][j] <= grid[i][k] {
                    break;
                }
            }
            current_score *= current_itr;

            let mut current_itr = 0;
            for k in (0..i).rev() {
                current_itr += 1;
                if grid[i][j] <= grid[k][j] {
                    break;
                }
            }
            current_score *= current_itr;

            let mut current_itr = 0;
            for k in i + 1..height {
                current_itr += 1;
                if grid[i][j] <= grid[k][j] {
                    break;
                }
            }
            current_score *= current_itr;

            score = max(score, current_score);
        }
    }

    score
}