use std::cmp::Ordering;

use advent_of_code::helpers::*;
use advent_of_code::solve;

fn main() {
    let input = &read_input(2);
    solve!(1, solve_part_1, input);
    solve!(2, solve_part_2, input);
}

/// Solves the first part of the problem
fn solve_part_1(input: &str) -> Option<u64> {
    solve(input, make_round_1)
}

/// Solves the second part of the problem
fn solve_part_2(input: &str) -> Option<u64> {
    solve(input, make_round_2)
}

/// Solves the problem on the provided `input` with the chosen strategy method
/// for making `Rounds`
fn solve(input: &str, make_round: fn((char, char)) -> Round) -> Option<u64> {
    let chars = parse_input(input);
    let rounds: Vec<Round> = chars.into_iter().map(make_round).collect();
    let score = rounds.iter().map(Round::score).sum();

    Some(score)
}

/// Parses the input string into `Vec<(char, char)>`
fn parse_input(input: &str) -> Vec<(char, char)> {
    input
        .split("\n")
        .filter(|block| block.len() > 0)
        .map(parse_block)
        .collect()
}

/// Parses a block of the input string into a char tuple
fn parse_block(block: &str) -> (char, char) {
    let chars: Vec<char> = block
        .split_whitespace()
        .filter_map(|s| s.chars().next())
        .collect();

    match chars.as_slice() {
        &[a, b] => (a, b),
        _ => panic!("Malformed chars: {:?}", chars),
    }
}

/// Parses a `char` tuple into a `Round` (1st part)
fn make_round_1(chars: (char, char)) -> Round {
    let first_shape = parse_first_shape(chars.0);
    let second_shape = match chars.1 {
        'X' => Shape::Rock,
        'Y' => Shape::Paper,
        'Z' => Shape::Scissors,
        c => panic!("Invalid second shape: {}", c),
    };

    Round(first_shape, second_shape)
}

/// Parses a `char` tuple into a `Round` (2nd part)
fn make_round_2(chars: (char, char)) -> Round {
    let first_shape = parse_first_shape(chars.0);
    let second_shape = match chars.1 {
        'X' => first_shape.get_weaker(),
        'Y' => first_shape,
        'Z' => first_shape.get_stronger(),
        c => panic!("Invalid second shape: {}", c),
    };

    Round(first_shape, second_shape)
}

fn parse_first_shape(c: char) -> Shape {
    match c {
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        'C' => Shape::Scissors,
        c => panic!("Invalid first shape: {}", c),
    }
}

/// Describes the `Shapes` that can be played in the game
#[derive(PartialEq, Eq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

/// Defines the score for each `Shape`
impl Shape {
    fn score(&self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn get_stronger(&self) -> Self {
        match *self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn get_weaker(&self) -> Shape {
        match *self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }
}

/// Provides ordering to the `Shape` used for describing
impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if *self == Shape::Paper && *other == Shape::Scissors {
            Some(Ordering::Less)
        } else if *self == Shape::Rock && *other == Shape::Paper {
            Some(Ordering::Less)
        } else if *self == Shape::Scissors && *other == Shape::Rock {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

/// Describes a round of a Rock Paper Scissors game made of the
/// enemy `Shape` on left side and player `Shape` on right
struct Round(Shape, Shape);

/// Provides the `Round` struct with a constructor and a scoring function
impl Round {
    fn score(&self) -> u64 {
        let round_score = self.round_score();
        let shape_score = self.1.score();

        round_score + shape_score
    }

    fn round_score(&self) -> u64 {
        let is_victory = self.1 > self.0;
        let is_defeat = self.0 > self.1;
        if is_victory {
            6
        } else if is_defeat {
            0
        } else {
            3
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_block() {
        let input = read_example(2);
        let chars = parse_input(&input);
        assert!(chars[0].0 == 'A');
        assert!(chars[0].1 == 'Y');
        assert!(chars[1].0 == 'B');
        assert!(chars[1].1 == 'X');
        assert!(chars[2].0 == 'C');
        assert!(chars[2].1 == 'Z');
    }

    #[test]
    fn test_make_round_1() {
        let input = ('A', 'Z');
        let round = make_round_1(input);
        assert!(round.0 == Shape::Rock);
        assert!(round.1 == Shape::Scissors);
    }

    #[test]
    fn test_make_round_2() {
        let input = ('A', 'Y');
        let round = make_round_2(input);
        assert!(round.0 == Shape::Rock);
        assert!(round.1 == Shape::Rock);
    }

    #[test]
    fn test_shape_ord() {
        assert!(Shape::Rock > Shape::Scissors);
        assert!(Shape::Paper < Shape::Scissors);
    }

    #[test]
    fn test_round_score() {
        let round1: Round = Round(Shape::Rock, Shape::Paper);
        let round2: Round = Round(Shape::Paper, Shape::Rock);
        let round3: Round = Round(Shape::Scissors, Shape::Scissors);
        assert!(round1.score() == 8);
        assert!(round2.score() == 1);
        assert!(round3.score() == 6);
    }
}
