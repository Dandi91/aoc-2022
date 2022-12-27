use std::collections::VecDeque;

pub fn part_one(input: &str) -> Option<i32> {
    let mut max_elf = 0;
    let mut current_elf = 0;
    for line in input.lines() {
        match line {
            "" => {
                if current_elf > max_elf {
                    max_elf = current_elf;
                }
                current_elf = 0;
            }
            _ => {
                let num: i32 = line.parse().unwrap();
                current_elf += num;
            }
        }
    }

    Some(max_elf)
}

const TOP_ELVES_NUM: usize = 3;

pub fn part_two(input: &str) -> Option<i32> {
    let mut top_elves = VecDeque::with_capacity(TOP_ELVES_NUM);
    let mut current_elf = 0;

    for line in input.lines() {
        match line {
            "" => {
                add_to_deque(&mut top_elves, current_elf);
                current_elf = 0;
            }
            _ => {
                let num: i32 = line.parse().unwrap();
                current_elf += num;
            }
        }
    }

    add_to_deque(&mut top_elves, current_elf);
    Some(top_elves.iter().sum())
}

fn add_to_deque(top_elves: &mut VecDeque<i32>, current_elf: i32) {
    let idx = top_elves.partition_point(|&x| x < current_elf);
    if top_elves.len() < TOP_ELVES_NUM {
        top_elves.insert(idx, current_elf);
    } else if idx > 0 {
        top_elves.pop_front();
        top_elves.insert(idx - 1, current_elf);
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
