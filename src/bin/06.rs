use std::collections::HashSet;

fn solve(input: &&str, seq_len: usize) -> Option<i32> {
    for idx in 0..input.len() {
        let slice = &input[idx..idx + seq_len].chars().collect::<HashSet<char>>();
        if slice.len() == seq_len {
            return Some((idx + seq_len) as i32);
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<i32> {
    solve(&input, 4)
}

pub fn part_two(input: &str) -> Option<i32> {
    solve(&input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
