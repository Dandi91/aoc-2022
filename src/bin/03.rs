use std::collections::HashSet;

fn get_priority(c: &char) -> i32 {
    if c.is_ascii_lowercase() {
        return *c as i32 - 97 + 1;
    } else if c.is_ascii_uppercase() {
        return *c as i32 - 65 + 27;
    }
    panic!("Char is not a valid ASCII letter");
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut result = 0;

    for line in input.lines() {
        let middle = line.len() / 2;
        let (left_part, right_part) = line.split_at(middle);
        for c in left_part.chars() {
            if right_part.contains(c) {
                result += get_priority(&c);
                break;
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut result = 0;

    let mut line_num = 0;
    let mut current_set = HashSet::new();
    for line in input.lines() {
        let line_set = HashSet::from_iter(line.chars());
        if line_num == 0 {
            current_set = line_set;
        } else {
            current_set = current_set.intersection(&line_set).copied().collect();
        }

        line_num += 1;
        if line_num > 2 {
            result += current_set.iter().map(get_priority).sum::<i32>();
            line_num = 0;
            current_set.clear();
        }
    }

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
