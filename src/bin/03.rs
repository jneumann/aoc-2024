advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total = 0;

    for r in re.captures_iter(input) {
        let a: u32 = r[1].parse().unwrap();
        let b: u32 = r[2].parse().unwrap();

        total += a * b;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let fn_regex = Regex::new(r"(do|don't|mul)\((\d+)?,?(\d+)?\)").unwrap();

    let mut total: u32 = 0;
    let mut active: bool = true;

    for r in fn_regex.captures_iter(input) {
        match &r[1] {
            "do" => {
                active = true;
            },
            "don't" => {
                active = false;
            },
            "mul" => {
                if active {
                    total += r[2].parse::<u32>().unwrap() * r[3].parse::<u32>().unwrap();
                }
            },
            _ => {
                panic!("Unexpected command: {:?}", r[1].to_string());
            }
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
