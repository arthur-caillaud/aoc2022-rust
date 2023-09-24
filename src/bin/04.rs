use std::str::FromStr;

use advent_of_code::helpers::*;
use advent_of_code::solve;

fn main() {
    let input = &read_input(4);
    solve!(1, solve_part_1, input);
    solve!(2, solve_part_2, input);
}

fn solve_part_1(input: &str) -> Option<u64> {
    solve(input, Pair::is_fully_overlapping)
}

fn solve_part_2(input: &str) -> Option<u64> {
    solve(input, Pair::is_overlapping)
}

fn solve(input: &str, overlap: fn(&Pair) -> bool) -> Option<u64> {
    let pairs = Pair::from_strs(input);
    let overlaps = pairs.iter().filter(|pair| overlap(pair));
    let length = overlaps.count() as u64;

    Some(length)
}

/// Describes a `Pair` of `Assignments`
struct Pair(Assignment, Assignment);

impl Pair {
    /// Whether the two `Assignments` in the `Pair` are overlapping
    fn is_overlapping(&self) -> bool {
        let low_one = self.0 .0;
        let low_two = self.1 .0;
        let up_one = self.0 .1;
        let up_two = self.1 .1;

        (low_one <= up_two) && (low_two <= up_one)
    }

    /// Whether one of the two `Assignments` is fully overlapping the other
    fn is_fully_overlapping(&self) -> bool {
        let low_one = self.0 .0;
        let low_two = self.1 .0;
        let up_one = self.0 .1;
        let up_two = self.1 .1;

        let one_in_two = low_one <= low_two && up_one >= up_two;
        let two_in_one = low_two <= low_one && up_two >= up_one;

        one_in_two || two_in_one
    }

    fn from_strs(input: &str) -> Vec<Self> {
        input
            .lines()
            .map(|input| input.parse::<Pair>().unwrap())
            .collect()
    }
}

/// Implements parsing `String` into `Pair`
impl FromStr for Pair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        let assignment_1 = parts[0].parse::<Assignment>().unwrap();
        let assignment_2 = parts[1].parse::<Assignment>().unwrap();

        Ok(Self(assignment_1, assignment_2))
    }
}

struct Assignment(u64, u64);

/// Implements parsing `String` into `Assignment`
impl FromStr for Assignment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        let start = parts[0].parse::<u64>().unwrap();
        let end = parts[1].parse::<u64>().unwrap();

        Ok(Self(start, end))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_1() {
        let input = read_example(4);
        let result = solve_part_1(&input).unwrap();

        assert_eq!(result, 2);
    }

    #[test]
    fn test_solve_part_2() {
        let input = read_example(4);
        let result = solve_part_2(&input).unwrap();

        assert_eq!(result, 4);
    }

    #[test]
    fn test_pair_is_overlapping() {
        let pair1 = Pair(Assignment(2, 8), Assignment(3, 7));
        let pair2 = Pair(Assignment(5, 7), Assignment(7, 9));

        assert_eq!(pair1.is_overlapping(), true);
        assert_eq!(pair2.is_overlapping(), true);
    }

    #[test]
    fn test_pair_is_not_overlapping() {
        let pair1 = Pair(Assignment(2, 4), Assignment(6, 8));
        let pair2 = Pair(Assignment(2, 3), Assignment(4, 5));

        assert_eq!(pair1.is_overlapping(), false);
        assert_eq!(pair2.is_overlapping(), false);
    }

    #[test]
    fn test_pair_is_fully_overlapping() {
        let pair = Pair(Assignment(2, 8), Assignment(3, 7));

        assert_eq!(pair.is_fully_overlapping(), true);
    }

    #[test]
    fn test_pair_is_not_fully_overlapping() {
        let pair = Pair(Assignment(5, 7), Assignment(7, 9));

        assert_eq!(pair.is_fully_overlapping(), false);
    }

    #[test]
    fn test_pair_from_multiple() {
        let input = read_example(4);
        let sections = Pair::from_strs(&input);

        assert_eq!(sections[0].0 .0, 2);
        assert_eq!(sections[1].1 .0, 4);
        assert_eq!(sections[2].1 .1, 9);
    }

    #[test]
    fn test_parse_pair() {
        let s = "1-10,4-8";
        let Pair(assignment1, assignment2) = s.parse::<Pair>().unwrap();

        assert_eq!(assignment1.0, 1);
        assert_eq!(assignment1.1, 10);
        assert_eq!(assignment2.0, 4);
        assert_eq!(assignment2.1, 8);
    }

    #[test]
    fn test_parse_assignment() {
        let s = "2-12";
        let assignment = s.parse::<Assignment>().unwrap();

        assert_eq!(assignment.0, 2);
        assert_eq!(assignment.1, 12);
    }
}
