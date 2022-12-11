use std::collections::VecDeque;
use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");

#[derive(Default, Clone)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Vec<String>,
    div: usize,
    true_idx: usize,
    false_idx: usize,
    count: usize,
}

impl Monkey {
    fn operate(&self, n: usize) -> Result<usize, Box<dyn Error>> {
        let op2 = match self.operation[2].as_str() {
            "old" => n,
            x => x.parse()?,
        };
        Ok(match self.operation[1].as_str() {
            "+" => n + op2,
            "*" => n * op2,
            _ => unreachable!()
        })
    }
}

impl FromStr for Monkey {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<&str> = s.split("\n").collect();

        Ok(Monkey {
            items: splits[1]
                .replace("Starting items: ", "")
                .trim().split(", ").filter_map(|x| x.parse::<usize>().ok()).collect(),
            operation: splits[2]
                .replace("Operation: new = ", "")
                .trim()
                .split(" ").map(|x| x.to_string()).collect(),
            div: splits[3].replace("Test: divisible by ", "")
                .trim().parse()?,
            true_idx: splits[4].replace("If true: throw to monkey ", "")
                .trim().parse()?,
            false_idx: splits[5].replace("If false: throw to monkey ", "")
                .trim().parse()?,
            count: 0,
        })
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let monkeys: Vec<Monkey> = INPUT.split("\n\n").filter_map(|x| Monkey::from_str(x).ok()).collect();

    println!("{}", part1(monkeys.clone())?);
    println!("{}", part2(monkeys.clone())?);

    Ok(())
}


fn part1(monkeys: Vec<Monkey>) -> Result<usize, Box<dyn Error>> {
    monkey_business::<_, 20>(monkeys, |x| x / 3)
}

fn part2(monkeys: Vec<Monkey>) -> Result<usize, Box<dyn Error>> {
    let modulus = monkeys.iter().map(|x| x.div).product::<usize>();
    monkey_business::<_, 10000>(monkeys, |x| x % modulus)
}

fn monkey_business<F: Fn(usize) -> usize, const N: usize>(mut monkeys: Vec<Monkey>, transform: F) -> Result<usize, Box<dyn Error>> {
    for _ in 0..N {
        for i in 0..monkeys.len() {
            let true_idx = monkeys[i].true_idx;
            let false_idx = monkeys[i].false_idx;

            while let Some(top) = monkeys[i].items.pop_front() {
                let transformed = transform(monkeys[i].operate(top)?);
                match transformed % monkeys[i].div == 0 {
                    true => monkeys[true_idx].items.push_back(transformed),
                    false => monkeys[false_idx].items.push_back(transformed),
                }
                monkeys[i].count += 1;
            }
        }
    }
    let mut v: Vec<usize> = monkeys.into_iter().map(|x| x.count).collect();
    v.sort_by(|a, b| b.cmp(a));
    Ok(v[0] * v[1])
}