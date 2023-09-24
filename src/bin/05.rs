use regex::Regex;
use std::str::FromStr;

use advent_of_code::helpers::*;
use advent_of_code::solve;

fn main() {
    let input = &read_input(5);
    solve!(1, solve_part_1, &input)
}

fn solve_part_1(input: &str) -> Option<String> {
    let mut split = input.split("\n\n");
    let stack_input = split.next().unwrap();
    let steps_input = split.next().unwrap();

    let mut stacks = Stacks::from_str(stack_input);
    let steps = steps_input.parse::<Steps>().unwrap();

    stacks.m_apply(steps.0);
    Some(stacks.get_message())
}

struct Steps(Vec<Step>);

impl Steps {
    fn nth(&self, i: u64) -> &Step {
        &self.0[i as usize]
    }
}

impl FromStr for Steps {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let steps: Vec<Step> = s.lines().map(|s| s.parse::<Step>().unwrap()).collect();

        Ok(Steps(steps))
    }
}

#[derive(Clone)]
struct Step {
    count: u64,
    from: u64,
    to: u64,
}

impl FromStr for Step {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        let caps = regex.captures(s).unwrap();

        let count = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
        let from = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
        let to = caps.get(3).unwrap().as_str().parse::<u64>().unwrap();

        Ok(Self { count, from, to })
    }
}

#[derive(Clone)]
struct Stacks(Vec<Stack>);

impl Stacks {
    fn get_message(&self) -> String {
        let chars: Vec<char> = self
            .0
            .iter()
            .map(|stack| stack.get_top_product().0)
            .collect();

        chars.into_iter().collect::<String>()
    }

    fn m_apply(&mut self, steps: Vec<Step>) {
        steps.iter().for_each(|step| self.apply(step))
    }

    fn apply(&mut self, step: &Step) {
        (0..step.count).for_each(|_| self.move_one(step.from, step.to));
    }

    fn move_one(&mut self, from: u64, to: u64) {
        if let Some(product) = self.0[from as usize - 1].0.pop() {
            self.0[to as usize - 1].0.push(product)
        }
    }

    fn from_str(s: &str) -> Self {
        let first_line = s.lines().rev().nth(1).unwrap();
        let mut stacks = Stacks::init(first_line);

        s.lines()
            .rev()
            .enumerate()
            .filter_map(|(i, line)| if i == 0 { None } else { Some(line) })
            .map(|line| parse_line(line))
            .for_each(|tokens| stacks.parse_push(tokens));

        stacks
    }

    fn init(s: &str) -> Stacks {
        let n = (s.len() + 1) / 4;
        Stacks(vec![Stack::new(); n])
    }

    fn parse_push(&mut self, tokens: Vec<String>) {
        tokens
            .iter()
            .enumerate()
            .for_each(|(k, token)| match token.parse::<Product>() {
                Ok(product) => self.0[k].0.push(product),
                _ => (),
            })
    }
}

fn parse_line(s: &str) -> Vec<String> {
    let clean_line = s
        .chars()
        .enumerate()
        .filter_map(|(i, c)| if i % 4 == 3 { None } else { Some(c) })
        .collect::<String>();

    clean_line
        .chars()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| chunk.iter().collect())
        .collect()
}

#[derive(Clone)]
struct Stack(Vec<Product>);

impl Stack {
    fn new() -> Stack {
        Stack(vec![])
    }

    fn get_top_product(&self) -> &Product {
        self.0.last().unwrap()
    }
}

#[derive(Clone)]
struct Product(char);

impl FromStr for Product {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let char = s.trim().chars().nth(1);

        match char {
            Some(letter) => Ok(Product(letter)),
            None => Err(format!("Invalid product {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_1() {
        let input = read_example(5);
        let res = solve_part_1(&input).unwrap();

        assert_eq!(res, "CMZ")
    }

    #[test]
    fn test_parse_steps() {
        let input = read_example(5);
        let step_lines = input.split("\n\n").nth(1).unwrap();
        let steps = step_lines.parse::<Steps>().unwrap();

        assert_eq!(steps.nth(0).from, 2);
        assert_eq!(steps.nth(1).count, 3);
        assert_eq!(steps.nth(2).to, 1);
    }

    #[test]
    fn test_parse_box() {
        let s = "[D]";
        let product = s.parse::<Product>().unwrap();

        assert_eq!(product.0, 'D')
    }

    #[test]
    fn test_parse_move() {
        let s = "move 13 from 1 to 3";
        let step = s.parse::<Step>().unwrap();

        assert_eq!(step.count, 13);
        assert_eq!(step.from, 1);
        assert_eq!(step.to, 3);
    }

    #[test]
    fn test_parse_line() {
        let s = "[N] [C]    ";
        let chunks = parse_line(&s);

        assert_eq!(chunks[0], "[N]");
        assert_eq!(chunks[1], "[C]");
        assert_eq!(chunks[2], "   ");
    }

    #[test]
    fn test_parse_stacks() {
        let input = read_example(5);
        let stacks_input = input.split("\n\n").nth(0).unwrap();
        let stacks = Stacks::from_str(stacks_input);

        assert_eq!(stacks.0[0].0[1].0, 'N');
        assert_eq!(stacks.0[1].0[2].0, 'D');
        assert_eq!(stacks.0[2].0[0].0, 'P');
    }

    #[test]
    fn test_apply_step() {
        let input = read_example(5);
        let stacks_input = input.split("\n\n").nth(0).unwrap();
        let mut stacks = Stacks::from_str(stacks_input);
        let step = Step {
            count: 1,
            from: 2,
            to: 1,
        };

        stacks.apply(&step);
        assert_eq!(stacks.0[0].0[2].0, 'D');
        assert_eq!(stacks.0[1].0.len(), 2);
    }

    #[test]
    fn test_stacks_get_message() {
        let input = read_example(5);
        let stacks_input = input.split("\n\n").nth(0).unwrap();
        let stacks = Stacks::from_str(stacks_input);
        let message = stacks.get_message();

        assert_eq!(message, "NDP")
    }
}
