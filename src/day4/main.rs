use std::ops::Range;
use std::cmp::{max, min};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let ranges: Vec<(Range<i32>, Range<i32>)> = INPUT.split("\n")
        .into_iter()
        .map(|x| x.split(",").collect::<Vec<&str>>())
        .map(|x| (to_range(x[0]), to_range(x[1]))).collect();

    println!("{:?}", part1(&ranges));
    println!("{:?}", part2(&ranges));
}

fn part1(ranges: &Vec<(Range<i32>, Range<i32>)>) -> usize {
    ranges
        .iter()
        .filter(|(r1, r2)| fully_overlapping(r1, r2))
        .count()
}

fn part2(ranges: &Vec<(Range<i32>, Range<i32>)>) -> usize {
    ranges
        .iter()
        .filter(|(r1, r2)| overlapping(r1, r2))
        .count()
}


fn fully_overlapping(r1: &Range<i32>, r2: &Range<i32>) -> bool {
    (r1.start <= r2.start && r1.end >= r2.end) || (r2.start <= r1.start && r2.end >= r1.end)
}

fn overlapping(r1: &Range<i32>, r2: &Range<i32>) -> bool {
    max(r1.start, r2.start) <= min(r1.end, r2.end)
}

fn to_range(s: &str) -> Range<i32> {
    let n: Vec<i32> = s.split("-")
        .filter_map(|x| x.parse().ok())
        .collect();

    n[0]..n[1]
}