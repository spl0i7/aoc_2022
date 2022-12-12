use std::cmp::{min};
use std::collections::{HashMap, HashSet, VecDeque};

const INPUT: &str = include_str!("input.txt");

const LOWEST_ELEVATION: u8 = 'a' as u8;
const HIGHEST_ELEVATION: u8 = 'z' as u8;
const START: u8 = 'S' as u8;
const END: u8 = 'E' as u8;

fn main() {
    let mut grid = INPUT
        .split("\n")
        .map(|x| x.chars().map(|x| x as u8).collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    let mut potential_starts = Vec::new();
    let mut part1_start = (0, 0);
    let mut end = (0, 0);
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == START {
                grid[i][j] = LOWEST_ELEVATION;
                part1_start = (i, j);
                potential_starts.push(part1_start);
            }

            if grid[i][j] == LOWEST_ELEVATION {
                potential_starts.push((i, j));
            }

            if grid[i][j] == END {
                grid[i][j] = HIGHEST_ELEVATION;
                end = (i, j);
            }
        }
    }


    println!("part 1 = {}", bfs(&grid, vec![part1_start], end));
    println!("part 2 = {}", bfs(&grid, potential_starts, end));
}

fn bfs(grid: &[Vec<u8>], starts: Vec<(usize, usize)>, end: (usize, usize)) -> i32 {
    let mut result = i32::MAX;
    let neighbours: Vec<(i64, i64)> = vec![(0, 1), (1, 0), (-1, 0), (0, -1)];

    for start in starts {
        let mut queue = VecDeque::new();
        queue.push_back(((start), 0));

        let mut visited = HashSet::new();
        let mut path_length: HashMap<(usize, usize), i32> = HashMap::new();
        path_length.insert(start, 0);

        while let Some(next) = queue.pop_front() {
            let top = next.0;

            if visited.contains(&top) { continue; }

            if top == end { result = min(result, *path_length.get(&top).unwrap()); }

            let path_score = path_length.get(&top).unwrap() + 1;

            for n in &neighbours {
                let p_k = (top.0 as i64 + n.0, top.1 as i64 + n.1);
                if p_k.0 >= 0 && p_k.1 >= 0 && p_k.1 < grid[top.0].len() as i64 && p_k.0 < grid.len() as i64 {
                    let key = (p_k.0 as usize, p_k.1 as usize);
                    if grid[top.0][top.1] + 1 >= grid[key.0][key.1] && !visited.contains(&key) {
                        path_length.insert(key, path_score);
                        queue.push_back((key, path_score as usize));
                    }
                }
            }

            visited.insert(top);
        }
    }

    result
}
