use std::collections::HashSet;

use advent_of_code_2024::read_today_data_file;

fn parse_input(file: &String) -> Vec<(&str, &str)> {
    file.strip_suffix("\r\n")
        .or(file.strip_suffix("\n"))
        .unwrap_or(file)
        .split(",")
        .map(|x| {
            let mut r = x.split("-");
            return (r.next().unwrap(), r.next().unwrap());
        })
        .collect::<Vec<(&str, &str)>>()
}

fn value_of_invalid_ids(start: &str, end: &str, depth: u32) -> i64 {
    let starting_digits = start.len();
    let ending_digits = end.len();
    let mut sets: HashSet<i64> = HashSet::new();
    for l in starting_digits..=ending_digits {
        let custom_min = l == starting_digits;
        let min_value = if custom_min {
            start.parse::<i64>().unwrap()
        } else {
            10i64.pow(l as u32 - 1)
        };
        let custom_max = l == ending_digits;
        let max_value = if custom_max {
            end.parse::<i64>().unwrap()
        } else {
            10i64.pow(l as u32) - 1
        };
        for d in 2..=std::cmp::min(depth, ending_digits as u32) {
            if l as u32 % d != 0 {
                continue;
            }
            //
            // Iterate all possible doubles with this number of digits
            // Notably, this requires the leading digit to be 1
            let part_size = 10i64.pow(l as u32 / d);
            let scan_bottom = 10i64.pow(l as u32 / d - 1);
            let scan_top = 10i64.pow(l as u32 / d);
            for i in scan_bottom..scan_top {
                let mut value = 0;
                for _dd in 1..=d {
                    value = value * part_size + i;
                }
                if value >= min_value && value <= max_value {
                    sets.insert(value);
                }
            }
        }
    }
    sets.iter().sum()
}

fn sum_of_invalid_ids(ids: Vec<(&str, &str)>, depth: u32) -> i64 {
    ids.into_iter()
        .fold(0i64, |acc, e| acc + value_of_invalid_ids(e.0, e.1, depth))
}

fn part_one(file: &String) -> i64 {
    let parsed_input = parse_input(file);
    sum_of_invalid_ids(parsed_input, 2)
}

fn part_two(file: &String) -> i64 {
    let parsed_input = parse_input(file);
    sum_of_invalid_ids(parsed_input, 999999)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part_one_as_given() {
        let result = part_one(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_part_two_as_given() {
        let result = part_two(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 4174379265);
    }
}

fn main() {
    let file = read_today_data_file(String::from("02"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file);
    println!("Part two result: {part_two_result}");
}
