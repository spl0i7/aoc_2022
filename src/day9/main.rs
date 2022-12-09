use std::collections::HashSet;
use std::error::Error;

const INPUT: &str = include_str!("input.txt");

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Coordinate {
    x: i8,
    y: i8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let instructions: Vec<Vec<&str>> = INPUT.split("\n")
        .map(|x| x.split(" ").collect())
        .collect();


    println!("{:?}", rope_sim::<2>(&instructions).unwrap());
    println!("{:?}", rope_sim::<10>(&instructions).unwrap());

    Ok(())
}

fn rope_sim<const N: usize>(instructions: &[Vec<&str>]) -> Result<usize, Box<dyn Error>> {
    let mut history = HashSet::<Coordinate>::new();

    let mut knots = [Coordinate { x: 0, y: 0 }; N];
    for i in instructions {
        let steps: usize = i[1].parse()?;
        for _ in 0..steps {
            match i[0] {
                "U" => knots[0].y += 1,
                "D" => knots[0].y -= 1,
                "L" => knots[0].x -= 1,
                "R" => knots[0].x += 1,
                _ => unreachable!()
            }

            for i in 1..knots.len() {
                let head = knots[i - 1];
                let tail = &mut knots[i];

                if head.x.abs_diff(tail.x) > 1 || head.y.abs_diff(tail.y) > 1 {
                    tail.x += (head.x - tail.x).signum();
                    tail.y += (head.y - tail.y).signum();
                }
            }

            history.insert(knots[N - 1]);
        }
    }

    Ok(history.len())
}