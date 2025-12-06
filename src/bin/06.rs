use advent_of_code_2025::read_today_data_file;

fn parse_input(file: &String) -> Vec<Vec<&str>> {
    file.lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn part_one(file: &String) -> i64 {
    let parsed_input = parse_input(file);
    let mut total = 0;
    for i in 0..parsed_input[0].len() {
        let mut value = parsed_input[0][i].parse::<i64>().unwrap();
        for j in 1..parsed_input.len() - 1 {
            let this_value = parsed_input[j][i].parse::<i64>().unwrap();
            if parsed_input[parsed_input.len() - 1][i] == "*" {
                value *= this_value;
            } else {
                value += this_value;
            }
        }
        total += value;
    }

    total
}

fn parse_part_two(file: &String) -> Vec<Vec<char>> {
    file.lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<char>>>()
}

fn part_two(file: &String) -> i64 {
    let rows = parse_part_two(file);
    let n = rows.len() - 1;
    let width = rows[0].len();
    let mut total = 0;
    let mut nums = Vec::new();
    for i in (0..width).rev() {
        let mut this_num = 0;
        for y in 0..n {
            if rows[y][i] != ' ' {
                this_num = 10 * this_num + rows[y][i].to_digit(10).unwrap();
            }
        }
        nums.push(this_num as i64);
        if rows[n][i] != ' ' {
            if rows[n][i] == '+' {
                total += nums.iter().fold(0, |acc, e| acc + e);
            } else if rows[n][i] == '*' {
                total += nums.iter().fold(1, |acc, e| acc * e.max(&1));
            }
            nums = Vec::new();
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part_one_as_given() {
        let result = part_one(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_part_two_as_given() {
        let result = part_two(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 3263827);
    }
}

fn main() {
    let file = read_today_data_file(String::from("06"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file);
    println!("Part two result: {part_two_result}");
}
