fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }

            let number_str = line.replace("L", "-").replace("R", "");
            number_str.parse::<i32>().ok()
        })
        .collect()
}

fn wrap_position(position: i32) -> i32 {
    ((position % 100) + 100) % 100
}

pub fn part1(input: &str) -> String {
    let instructions = parse_input(input);
    let mut position = 50;
    let mut number_of_zeroes = 0;
    println!("{:?}", instructions);

    for instruction in instructions {
        position = wrap_position(position + instruction);
        println!("Current position: {}", position);
        if position == 0 {
            println!("Found a zero");
            number_of_zeroes += 1;
        }
    }
    return number_of_zeroes.to_string();
}

pub fn part2(input: &str) -> &str {
    todo!("Implement part 2");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
R1
L51
R50
L66
"#;

    #[test]
    fn test_parse_input() {
        let expected = vec![1, -51, 50, -66];
        let result = parse_input(EXAMPLE_INPUT);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1() {
        // Expect one occurence of 0
        let expected = "1";
        let result = part1(EXAMPLE_INPUT);

        assert_eq!(result, expected);
    }

    //     #[test]
    //     fn test_part2() {
    //         let input = r#""#;

    //         let expected = r#""#;

    //         let result = part2(input);
    //         assert_eq!(result, expected);
    //     }
}
