use advent_of_code_2025::read_today_data_file;

fn parse_input(file: &String) -> Vec<Vec<i64>> {
    file.split('\n')
        .map(|r| {
            r.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect::<Vec<i64>>()
        })
        .collect()
}

fn part_one(file: &String) -> i64 {
    let parsed_input = parse_input(file);
    let mut sum = 0i64;
    for row in parsed_input {
        let mut left = 0;
        let mut right = 0;
        for (index, &i) in row.iter().enumerate() {
            if i > left && index != row.len() - 1 {
                left = i;
                right = 0;
            } else if i > right {
                right = i;
            }
        }
        sum += left * 10 + right;
    }
    sum
}

fn part_two(file: &String) -> i64 {
    let parsed_input = parse_input(file);
    let mut sum = 0i64;
    for row in parsed_input {
        let mut batteries: Vec<i64> = vec![0; 12];
        let mut point = 0;
        for (index, &i) in row.iter().enumerate() {
            for pos in 1..=12 {
                if (pos > point || i > batteries[pos - 1]) && index <= row.len() - (13 - pos) {
                    batteries[pos - 1] = i;
                    point = pos;
                    break;
                }
            }
        }
        sum += batteries.iter().fold(0, |acc, &e| 10 * acc + e);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part_one_as_given() {
        let result = part_one(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 357);
    }

    #[test]
    fn test_part_two_as_given() {
        let result = part_two(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 3121910778619);
    }
}

fn main() {
    let file = read_today_data_file(String::from("03"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file);
    println!("Part two result: {part_two_result}");
}
