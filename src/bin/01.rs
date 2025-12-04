use advent_of_code_2025::read_today_data_file;

fn parse_input(file: &String) -> Vec<i64> {
    file.lines()
        .map(|line| {
            line.replace("L", "-")
                .replace("R", "")
                .parse::<i64>()
                .unwrap()
        })
        .collect::<Vec<i64>>()
}

fn part_one(file: &String) -> i64 {
    let parsed_input = parse_input(file);
    let mut position = 50;
    let mut zeroes = 0;
    for x in parsed_input {
        position += x;
        while position < 0 {
            position += 100;
        }
        while position > 99 {
            position -= 100;
        }
        if position == 0 {
            zeroes += 1;
        }
    }
    zeroes
}

fn part_two(file: &String) -> i64 {
    let parsed_input = parse_input(file);
    let mut position = 50;
    let mut zeroes = 0;
    for x in parsed_input {
        while position + x < 0 {
            if position != 0 {
                zeroes += 1;
            }
            position += 100;
        }
        position += x;
        if position == 0 {
            zeroes += 1;
        } else {
            while position > 99 {
                position -= 100;
                zeroes += 1;
            }
        }
    }
    zeroes
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    const EDGE_CASE: &str = "L50
R1
L1
R1
L1
R1
L1
R1
L1"; // 0, 1, 0, 1, 0, 1, 0, 1, 0
    #[test]
    fn test_part_one_as_given() {
        let result = part_one(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_two_as_given() {
        let result = part_two(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part_two_edge_case() {
        let result = part_two(&String::from(EDGE_CASE));
        assert_eq!(result, 5);
    }
}

fn main() {
    let file = read_today_data_file(String::from("01"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file);
    println!("Part two result: {part_two_result}");
}
