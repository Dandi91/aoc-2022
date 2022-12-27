use std::collections::HashMap;
use std::str::FromStr;

use once_cell::sync::Lazy;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum OpponentPlay {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for OpponentPlay {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => Err(())
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum YourPlay {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for YourPlay {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Rock),
            "Y" => Ok(Self::Paper),
            "Z" => Ok(Self::Scissors),
            _ => Err(())
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(())
        }
    }
}

static SCORES: Lazy<HashMap<(OpponentPlay, YourPlay), i32>> = Lazy::new(|| HashMap::from([
    ((OpponentPlay::Rock, YourPlay::Rock), 1 + 3),
    ((OpponentPlay::Rock, YourPlay::Paper), 2 + 6),
    ((OpponentPlay::Rock, YourPlay::Scissors), 3 + 0),
    ((OpponentPlay::Paper, YourPlay::Rock), 1 + 0),
    ((OpponentPlay::Paper, YourPlay::Paper), 2 + 3),
    ((OpponentPlay::Paper, YourPlay::Scissors), 3 + 6),
    ((OpponentPlay::Scissors, YourPlay::Rock), 1 + 6),
    ((OpponentPlay::Scissors, YourPlay::Paper), 2 + 0),
    ((OpponentPlay::Scissors, YourPlay::Scissors), 3 + 3),
]));

static DECISIONS: Lazy<HashMap<(OpponentPlay, Outcome), YourPlay>> = Lazy::new(|| HashMap::from([
    ((OpponentPlay::Rock, Outcome::Lose), YourPlay::Scissors),
    ((OpponentPlay::Rock, Outcome::Draw), YourPlay::Rock),
    ((OpponentPlay::Rock, Outcome::Win), YourPlay::Paper),
    ((OpponentPlay::Paper, Outcome::Lose), YourPlay::Rock),
    ((OpponentPlay::Paper, Outcome::Draw), YourPlay::Paper),
    ((OpponentPlay::Paper, Outcome::Win), YourPlay::Scissors),
    ((OpponentPlay::Scissors, Outcome::Lose), YourPlay::Paper),
    ((OpponentPlay::Scissors, Outcome::Draw), YourPlay::Scissors),
    ((OpponentPlay::Scissors, Outcome::Win), YourPlay::Rock),
]));

pub fn part_one(input: &str) -> Option<i32> {
    let mut score = 0;

    for line in input.lines() {
        let x: Vec<&str> = line.split(' ').collect();
        let opponent: OpponentPlay = x[0].parse().unwrap();
        let your: YourPlay = x[1].parse().unwrap();
        score += SCORES[&(opponent, your)];
    }

    Some(score)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut score = 0;

    for line in input.lines() {
        let x: Vec<&str> = line.split(' ').collect();
        let opponent: OpponentPlay = x[0].parse().unwrap();
        let outcome: Outcome = x[1].parse().unwrap();

        let decision = DECISIONS[&(opponent, outcome)];
        score += SCORES[&(opponent, decision)];
    }

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
