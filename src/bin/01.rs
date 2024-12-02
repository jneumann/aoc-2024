advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    let mut total = 0;

    for line in input.lines() {
        let iter = line.split_whitespace().collect::<Vec<&str>>();

        left.push(iter[0].parse::<u32>().unwrap());
        right.push(iter[1].parse::<u32>().unwrap());
    }

    left.sort();
    right.sort();

    for (i, _) in left.iter().enumerate() {
        if left[i] > right[i] && left[i] != right[i] {
            total += left[i] - right[i];
        } else {
            total += right[i] - left[i];
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    let mut total = 0;

    for line in input.lines() {
        let iter = line.split_whitespace().collect::<Vec<&str>>();

        left.push(iter[0].parse::<u32>().unwrap());
        right.push(iter[1].parse::<u32>().unwrap());
    }

    for (_, l) in left.iter().enumerate() {
        let qty = &right.clone().into_iter().filter(|r| r == l).count();

        total += l * *qty as u32;
    }

    dbg!(total);
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
