use once_cell::sync::Lazy;
use regex::Regex;

static PARSE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap());

fn parse_line(line: &str) -> Vec<i32> {
    let groups = PARSE.captures(line).unwrap();
    groups.iter().skip(1).map(|x| x.unwrap().as_str().parse().unwrap()).collect()
}

fn count_lines(input: &str, flip: bool) -> i32 {
    input.lines().map(|line| {
        let mut nums = parse_line(line);
        if flip { nums.swap(2, 3); }
        nums
    }).filter(|nums|
        (nums[0] <= nums[2] && nums[1] >= nums[3]) || (nums[2] <= nums[0] && nums[3] >= nums[1])
    ).count() as i32
}

pub fn part_one(input: &str) -> Option<i32> {
    let result = count_lines(input, false);
    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let result = count_lines(input, true);
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
