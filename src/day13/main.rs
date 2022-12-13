use std::cmp::Ordering;
use crate::Packet::{List, Num};
use serde::{Deserialize};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Deserialize, Eq, PartialEq, Clone, PartialOrd)]
#[serde(untagged)]
enum Packet {
    Num(i32),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, r: &Packet) -> Ordering {
        match (self, r) {
            (Num(a), Num(b)) => a.cmp(b),
            (List(a), List(b)) => a.cmp(b),
            (Num(a), List(b)) => vec![Num(*a)].cmp(b),
            (List(a), Num(b)) => a.cmp(&vec![Num(*b)]),
        }
    }
}

fn main() {
    let lists: Vec<Packet> = INPUT
        .split("\n")
        .filter_map(|x| serde_json::from_str::<Packet>(x).ok())
        .collect();

    println!("{:?}", part1(&lists));
    println!("{:?}", part2(&lists));
}

#[elapsed_time::elapsed]
fn part1(v: &[Packet]) -> usize {
    v.chunks(2)
        .enumerate()
        .map(|(x, y)| if y[0] <= y[1] { x + 1 } else { 0 })
        .sum()
}

#[elapsed_time::elapsed]
fn part2(v: &[Packet]) -> usize {
    let delim1 = &List(vec![Num(2)]);
    let delim2 = &List(vec![Num(6)]);

    let mut packets = Vec::from_iter(v.iter());
    packets.push(delim1);
    packets.push(delim2);

    packets.sort();
    let p1 = packets.iter().position(|x| *x == delim1).unwrap();
    (p1 + 1) * (packets.iter().position(|x| *x == delim2).unwrap() + 1)
}

