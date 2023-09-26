use std::collections::HashMap;

use advent_of_code::helpers::*;
use advent_of_code::solve;

fn main() {
    let input = &read_input(6);
    solve!(1, solve_part_1, input);
    solve!(1, solve_part_2, input);
}

fn solve_part_1(input: &str) -> Option<u64> {
    solve(input, 4)
}

fn solve_part_2(input: &str) -> Option<u64> {
    solve(input, 14)
}

fn solve(input: &str, size: usize) -> Option<u64> {
    let mut solution: Option<u64> = None;
    for k in 0..(input.len() - size) {
        let slice = &input[k..k + size];
        if is_all_different_chars(slice) {
            solution = Some((k + size) as u64);
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
    }

    #[test]
    fn test_solve_part_two() {
        let s1 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let sol1 = solve_part_2(s1).unwrap();
        assert_eq!(sol1, 19);

        let s2 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let sol2 = solve_part_2(s2).unwrap();
        assert_eq!(sol2, 23);
    }

    #[test]
    fn test_is_all_different_chars() {
        let s1 = "abcd";
        assert_eq!(is_all_different_chars(s1), true);

        let s2 = "tufitezo";
        assert_eq!(is_all_different_chars(s2), false);
    }
}
