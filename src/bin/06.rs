use std::collections::HashMap;

use advent_of_code::helpers::*;
use advent_of_code::solve;

fn main() {
    let input = &read_input(6);
    solve!(1, solve_part_1, input);
}

pub fn solve_part_1(input: &str) -> Option<u64> {
    let mut solution: Option<u64> = None;
    for k in 0..(input.len() - 4) {
        let slice = &input[k..k + 4];
        if is_all_different_chars(slice) {
            solution = Some((k + 4) as u64);
            break;
        }
    }

    solution
}

fn is_all_different_chars(s: &str) -> bool {
    let mut chars: HashMap<char, usize> = HashMap::new();

    for c in s.chars() {
        *chars.entry(c).or_insert(0) += 1;
    }

    !chars.values().any(|&n| n > 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        let s1 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let sol1 = solve_part_1(s1).unwrap();
        assert_eq!(sol1, 5);

        let s2 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let sol2 = solve_part_1(s2).unwrap();
        assert_eq!(sol2, 11);

        let s3 = "aaaaaaaaaaa";
        let sol3 = solve_part_1(s3);
        assert_eq!(sol3, None);
    }

    #[test]
    fn test_is_all_different_chars() {
        let s1 = "abcd";
        assert_eq!(is_all_different_chars(s1), true);

        let s2 = "tufitezo";
        assert_eq!(is_all_different_chars(s2), false);
    }
}
