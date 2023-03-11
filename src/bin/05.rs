use std::iter::zip;
use once_cell::sync::Lazy;
use regex::Regex;

static PARSE_MOVE: Lazy<Regex> = Lazy::new(|| Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap());

pub fn parse_crate_line(line: &str) -> Vec<char> {
    line.chars().skip(1).step_by(4).collect()
}

fn solve(input: &str, reverse: bool) -> Option<String> {
    let mut rows = vec![];
    let mut line_iter = input.lines();

    loop {
        let line = line_iter.next().unwrap();
        if line.starts_with(" 1") { break; }
        rows.push(parse_crate_line(line));
    }

    line_iter.next();
    let mut stacks: Vec<Vec<char>> = rows[0].iter().map(|_| vec![]).collect();

    for row in rows.iter().rev() {
        for (char, stack) in zip(row, stacks.iter_mut()) {
            if char.is_alphabetic() {
                stack.push(*char);
            }
        }
    }

    for line in line_iter {
        let groups = PARSE_MOVE.captures(line).unwrap();
        let boxes_count = groups[1].parse::<usize>().unwrap();

        let src = stacks.get_mut(groups[2].parse::<usize>().unwrap() - 1)?;

        let src_left_boxes = src.len() - boxes_count;
        let mut slice = src.drain(src_left_boxes..).collect::<Vec<char>>();
        if reverse {
            slice.reverse();
        }

        let dst = stacks.get_mut(groups[3].parse::<usize>().unwrap() - 1)?;
        dst.extend(slice);
    }

    Some(stacks.iter().map(|stack| stack.last().unwrap()).collect())
}

pub fn part_one(input: &str) -> Option<String> {
    solve(input, true)
}

pub fn part_two(input: &str) -> Option<String> {
    solve(input, false)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
