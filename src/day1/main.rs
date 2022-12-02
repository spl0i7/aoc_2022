use std::io::{BufRead, BufReader};
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::open("src/day1/input.txt")?;
    let mut reader = BufReader::new(file);

    let mut elf_calories = calories(&mut reader)?;
    elf_calories.sort();
    elf_calories.reverse();

    println!("part 1 = {}", elf_calories[0]);
    println!("part 2 = {}", elf_calories.iter().take(3).sum::<i32>());

    Ok(())
}

fn calories(reader: &mut BufReader<File>) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut line = String::new();
    let mut result = Vec::new();
    let mut current_elf = 0;
    while let Ok(n) = reader.read_line(&mut line) {
        if n == 0 {
            break;
        }
        match line.trim().len() {
            0 => {
                result.push(current_elf);
                current_elf = 0;
            }
            _ => {
                current_elf += line.trim().parse::<i32>()?;
            }
        }
        line.clear();
    }


    Ok(result)
}
