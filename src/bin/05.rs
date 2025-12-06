use advent_of_code_2025::read_today_data_file;
use regex::Regex;

fn parse_input(file: &String) -> (Vec<(i64, i64)>, Vec<i64>) {
    let mut ranges = Vec::new();
    let mut ingredients = Vec::new();
    let mut end_of_range = false;
    let range_regex = Regex::new(r"^([0-9]+)\-([0-9]+)$").unwrap();
    for line in file.lines() {
        if line.is_empty() {
            end_of_range = true;
            continue;
        }
        if end_of_range {
            ingredients.push(line.parse::<i64>().unwrap());
        } else {
            let c = range_regex.captures(line).unwrap();
            let a = &c[1].parse::<i64>().unwrap();
            let b = &c[2].parse::<i64>().unwrap();
            ranges.push((*a, *b));
        }
    }
    return (ranges, ingredients);
}

fn part_one(file: &String) -> i64 {
    let (ranges, ingredients) = parse_input(file);
    let mut count = 0;
    for num in ingredients {
        for (start, end) in &ranges {
            if num >= *start && num <= *end {
                count += 1;
                break;
            }
        }
    }
    count
}

fn part_two(file: &String) -> i64 {
    let (mut ranges, _ingredients) = parse_input(file);

    // Combine ranges
    loop {
        let mut has_changed = false;
        let mut new_range = Vec::new();
        'a: for &(start, end) in &ranges {
            for (i, &(ostart, oend)) in new_range.iter().enumerate() {
                if start <= oend && end >= ostart {
                    new_range[i] = (start.min(ostart), end.max(oend));
                    has_changed = true;
                    continue 'a;
                }
            }
            new_range.push((start, end));
        }
        ranges = new_range;
        if !has_changed {
            break;
        }
    }

    // Add combined ranges
    let mut count = 0;
    for (start, end) in ranges {
        count += end - start + 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part_one_as_given() {
        let result = part_one(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_two_as_given() {
        let result = part_two(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 14);
    }
}

fn main() {
    let file = read_today_data_file(String::from("05"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file);
    println!("Part two result: {part_two_result}");
}
