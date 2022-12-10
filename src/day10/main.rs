use std::error::Error;

const INPUT: &str = include_str!("input.txt");
const INSPECTION: [usize; 6] = [20, 60, 100, 140, 180, 220];
const WIDTH: usize = 40;

#[derive(Default)]
struct CPU {
    cycle: usize,
    signal: i64,
    part1: usize,
    intermediate: Vec<char>,
    part2: Vec<Vec<char>>,
}

impl CPU {
    fn new() -> Self {
        CPU { signal: 1, ..Default::default() }
    }
    fn increase_cycle(&mut self) {
        let idx = (self.cycle % WIDTH) as i64;
        if idx == 0 {
            self.part2.push(self.intermediate.clone());
            self.intermediate.clear();
        }
        match (-1..2).any(|x| idx == self.signal - x) {
            true => self.intermediate.push('#'),
            false => self.intermediate.push('.')
        };

        self.cycle += 1;
        if INSPECTION.contains(&self.cycle) { self.part1 += self.cycle * self.signal as usize }
    }
    fn run(&mut self, instructions: &Vec<Vec<&str>>) -> Result<(), Box<dyn Error>> {
        for i in instructions {
            match i[0] {
                "noop" => {
                    self.increase_cycle();
                }
                "addx" => {
                    self.increase_cycle();
                    self.increase_cycle();
                    self.signal += i[1].parse::<i64>()?;
                }
                _ => unreachable!(),
            }
        }
        self.part2.push(self.intermediate.clone());

        Ok(())
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let instructions: Vec<Vec<&str>> = INPUT.split("\n")
        .map(|x| x.split(" ").collect()).collect();

    let mut cpu = CPU::new();
    cpu.run(&instructions)?;

    println!("{:?}", cpu.part1);
    for i in cpu.part2 {
        println!("{}", i.iter().collect::<String>());
    }

    Ok(())
}
