use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn part1() -> u64 {
    INPUT.
        split("\n")
        .map(|x| x.split_at(x.len() / 2))
        .filter_map(|x| {
            let mut h1: HashSet<char> = HashSet::from_iter(x.0.chars().into_iter());
            let h2: HashSet<char> = HashSet::from_iter(x.1.chars().into_iter());
            h1.retain(|item| h2.contains(item));
            h1.into_iter().next()
        })
        .map(|x| char_score(x))
        .sum::<u64>()
}

fn part2() -> u64 {
    let backpacks: Vec<&str> = INPUT.
        split("\n").
        collect();


    backpacks.chunks(3)
        .map(|m| (m[0], m[1], m[2]))
        .filter_map(|(x, y, z)| {
            let sets: Vec<HashSet<char>> = vec![
                HashSet::from_iter(y.chars().into_iter()),
                HashSet::from_iter(z.chars().into_iter()),
            ];
            let mut result: HashSet<char> = HashSet::from_iter(x.chars().into_iter());
            result.retain(|item| { sets.iter().all(|set| set.contains(item)) });
            result.into_iter().next()
        })
        .map(|x| char_score(x))
        .sum::<u64>()
}

fn char_score(x: char) -> u64 {
    if x >= 'a' && x <= 'z' { return x as u64 - 'a' as u64 + 1; }
    if x >= 'A' && x <= 'Z' { return x as u64 - 'A' as u64 + 27; }
    0
}