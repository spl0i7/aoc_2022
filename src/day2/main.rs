use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum RoundResult {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl Move {
    fn play_move(&self, other: Move) -> RoundResult {
        if *self == other {
            return RoundResult::Draw;
        }
        if *self == other.win_move() {
            return RoundResult::Lose;
        }
        RoundResult::Win
    }
    fn win_move(&self) -> Move {
        match self {
            Move::Paper => Move::Rock,
            Move::Rock => Move::Scissor,
            Move::Scissor => Move::Paper,
        }
    }
    fn lose_move(&self) -> Move {
        match self {
            Move::Paper => Move::Scissor,
            Move::Rock => Move::Paper,
            Move::Scissor => Move::Rock,
        }
    }
}

impl FromStr for Move {
    type Err = std::io::Error;
    fn from_str(input: &str) -> Result<Move, Self::Err> {
        match input {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissor),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
    }
}


impl FromStr for RoundResult {
    type Err = std::io::Error;
    fn from_str(input: &str) -> Result<RoundResult, Self::Err> {
        match input {
            "X" => Ok(RoundResult::Lose),
            "Y" => Ok(RoundResult::Draw),
            "Z" => Ok(RoundResult::Win),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::open("src/day2/input.txt")?;
    let mut reader = BufReader::new(file);

    println!("{}", part1(&mut reader)?);
    reader.seek(SeekFrom::Start(0))?;
    println!("{}", part2(&mut reader)?);

    Ok(())
}

fn part1(reader: &mut BufReader<File>) -> Result<i32, Box<dyn Error>> {
    let mut line = String::new();
    let mut scores = 0;
    while let Ok(n) = reader.read_line(&mut line) {
        if n == 0 {
            break;
        }

        let current_line: Vec<&str> = line.trim().split(" ").collect();

        let opponent_move = Move::from_str(current_line[0])?;
        let my_move = Move::from_str(current_line[1])?;

        scores += my_move.play_move(opponent_move) as i32 + my_move as i32;

        line.clear();
    }

    Ok(scores)
}

fn part2(reader: &mut BufReader<File>) -> Result<i32, Box<dyn Error>> {
    let mut line = String::new();
    let mut scores = 0;
    while let Ok(n) = reader.read_line(&mut line) {
        if n == 0 {
            break;
        }

        let current_line: Vec<&str> = line.trim().split(" ").collect();

        let opponent_move = Move::from_str(current_line[0])?;

        let my_move = match RoundResult::from_str(current_line[1])? {
            RoundResult::Lose => opponent_move.win_move(),
            RoundResult::Draw => opponent_move,
            RoundResult::Win => opponent_move.lose_move(),
        };

        scores += my_move.play_move(opponent_move) as i32 + my_move as i32;

        line.clear();
    }

    Ok(scores)
}