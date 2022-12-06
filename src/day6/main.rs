use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let v: Vec<char> = INPUT.chars().collect();

    println!("{:?}", part1(&v).unwrap_or_default());
    println!("{:?}", part2(&v).unwrap_or_default());
}

fn part1(haystack: &[char]) -> Option<usize> {
    find_preamble(4, haystack)
}

fn part2(haystack: &[char]) -> Option<usize> {
    find_preamble(14, haystack)
}

fn find_preamble(n: usize, haystack: &[char]) -> Option<usize> {
    match haystack.windows(n).position(|x| HashSet::<&char>::from_iter(x.iter()).len() == n) {
        None => None,
        Some(v) => Some(v + n)
    }
}
