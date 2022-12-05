use sscanf::sscanf;

const INPUT: &str = include_str!("input.txt");

// [M]                     [N] [Z]
// [F]             [R] [Z] [C] [C]
// [C]     [V]     [L] [N] [G] [V]
// [W]     [L]     [T] [H] [V] [F] [H]
// [T]     [T] [W] [F] [B] [P] [J] [L]
// [D] [L] [H] [J] [C] [G] [S] [R] [M]
// [L] [B] [C] [P] [S] [D] [M] [Q] [P]
// [B] [N] [J] [S] [Z] [W] [F] [W] [R]
//  1   2   3   4   5   6   7   8   9

fn main() {
    let stacks = vec![
        vec!['B', 'L', 'D', 'T', 'W', 'C', 'F', 'M'],
        vec!['N', 'B', 'L'],
        vec!['J', 'C', 'H', 'T', 'L', 'V'],
        vec!['S', 'P', 'J', 'W'],
        vec!['Z', 'S', 'C', 'F', 'T', 'L', 'R'],
        vec!['W', 'D', 'G', 'B', 'H', 'N', 'Z'],
        vec!['F', 'M', 'S', 'P', 'V', 'G', 'C', 'N'],
        vec!['W', 'Q', 'R', 'J', 'F', 'V', 'C', 'Z'],
        vec!['R', 'P', 'M', 'L', 'H'],
    ];
    let instructions: Vec<(usize, usize, usize)> = INPUT
        .split("\n")
        .into_iter()
        .filter_map(|x| {
            sscanf!(x, "move {} from {} to {}", usize, usize, usize).ok()
        }).collect();

    println!("{}", part1(stacks.clone(), &instructions));
    println!("{}", part2(stacks.clone(), &instructions));
}


fn part1(mut stacks: Vec<Vec<char>>, instructions: &Vec<(usize, usize, usize)>) -> String {
    instructions.iter()
        .for_each(|(arg, src, dst)| {
            let n = stacks[src - 1].len() - arg;
            let collected: Vec<char> = stacks[src - 1].drain(n..).rev().collect();
            stacks[dst - 1].extend(&collected);
        });

    stacks.iter().filter_map(|x| x.last()).collect()
}

fn part2(mut stacks: Vec<Vec<char>>, instructions: &Vec<(usize, usize, usize)>) -> String {
    instructions.iter()
        .for_each(|(arg, src, dst)| {
            let n = stacks[src - 1].len() - arg;
            let collected: Vec<char> = stacks[src - 1].drain(n..).collect();
            stacks[dst - 1].extend(&collected);
        });

    stacks.iter().filter_map(|x| x.last()).collect()
}