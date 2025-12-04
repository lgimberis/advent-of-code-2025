use advent_of_code_2025::read_today_data_file;

fn parse_input(file: &String) -> Vec<Vec<char>> {
    file.split('\n')
        .filter(|row| !row.is_empty())
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>()
}

fn get_candidates(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut candidates = Vec::new();
    if x > 0 {
        candidates.push((x - 1, y));
        if y > 0 {
            candidates.push((x - 1, y - 1));
        }
        if y < height - 1 {
            candidates.push((x - 1, y + 1));
        }
    }
    if y > 0 {
        candidates.push((x, y - 1));
    }
    if y < height - 1 {
        candidates.push((x, y + 1));
    }
    if x < width - 1 {
        candidates.push((x + 1, y));
        if y > 0 {
            candidates.push((x + 1, y - 1));
        }
        if y < height - 1 {
            candidates.push((x + 1, y + 1));
        }
    }
    candidates
}
fn part_one(file: &String) -> i64 {
    let parsed_input = parse_input(file);
    let mut count = 0i64;
    for (y, row) in parsed_input.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c != '@' {
                continue;
            }
            let mut this_roll_adjacent = 0;
            for (ox, oy) in get_candidates(x, y, row.len(), parsed_input.len()) {
                if parsed_input[oy][ox] == '@' {
                    this_roll_adjacent += 1;
                }
            }
            if this_roll_adjacent < 4 {
                count += 1;
            }
        }
    }
    count
}

fn part_two(file: &String) -> i64 {
    let mut parsed_input = parse_input(file);
    let mut overall = 0i64;
    let mut count = 1i64;
    while count != 0i64 {
        count = 0;
        let mut input_copy = parsed_input.clone();
        for (y, row) in parsed_input.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                if c != '@' {
                    continue;
                }
                let mut this_roll_adjacent = 0;
                for (ox, oy) in get_candidates(x, y, row.len(), parsed_input.len()) {
                    if parsed_input[oy][ox] == '@' {
                        this_roll_adjacent += 1;
                    }
                }
                if this_roll_adjacent < 4 {
                    count += 1;
                    input_copy[y][x] = '.';
                }
            }
        }
        parsed_input = input_copy;
        overall += count;
    }
    overall
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part_one_as_given() {
        let result = part_one(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part_two_as_given() {
        let result = part_two(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 43);
    }
}

fn main() {
    let file = read_today_data_file(String::from("04"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file);
    println!("Part two result: {part_two_result}");
}
