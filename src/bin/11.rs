use advent_of_code::helpers::*;
use advent_of_code::solve;
use regex::Regex;

fn main() {
    let input = &read_input(11);
    solve!(1, solve_part_1, input);
}

fn solve_part_1(input: &str) -> Option<u32> {
    let mut monkeys = Monkeys::parse(input);
    monkeys.run_n_round(20);
    let solution = monkeys.get_monkey_business();

    Some(solution)
}

#[derive(Debug)]
struct Monkeys(Vec<Monkey>);
impl Monkeys {
    /// Parses multiple `Monkeys` from a block of input text
    fn parse(txt: &str) -> Self {
        Self(txt.split("\n\n").map(Monkey::parse).collect())
    }

    /// Retrieves the current level of monkey business
    fn get_monkey_business(&self) -> u32 {
        let most_active = self.get_n_most_active(2);

        most_active[0] * most_active[1]
    }

    /// Retrieves the values of `inspected` for `n` most active `Monkeys`
    fn get_n_most_active(&self, n: u32) -> Vec<u32> {
        let mut inspected: Vec<u32> = self.0.iter().map(|monkey| monkey.inspected).collect();
        inspected.sort_by(|a, b| b.cmp(a));

        inspected[0..n as usize].to_vec()
    }

    /// Runs `n` full rounds
    fn run_n_round(&mut self, n: u32) {
        (0..n).for_each(|_| self.run_round());
    }

    /// Runs a full round
    fn run_round(&mut self) {
        (0..self.0.len()).for_each(|k| self.run_monkey_round(k));
    }

    /// Runs a round for `Monkey` k
    fn run_monkey_round(&mut self, k: usize) {
        let items = self.0.get_mut(k).unwrap().inspect_all();

        for (k, item) in &items {
            if let Some(target_monkey) = self.0.get_mut(*k as usize) {
                target_monkey.items.push(*item);
            }
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    divisible_test: u32,
    target_monkey_test_true: u32,
    target_monkey_test_false: u32,
    inspected: u32,
}
impl Monkey {
    /// `Monkey` inspects all of its items and returns a list of `(target_money, item)`
    fn inspect_all(&mut self) -> Vec<(u32, u32)> {
        (0..self.items.len()).map(|_| self.inspect()).collect()
    }

    /// `Monkey` inspects the first item in its list and returns `(target_monkey, item)`
    fn inspect(&mut self) -> (u32, u32) {
        let item = self.items.remove(0);
        let worried = self.worry(item);
        let bored = (worried as f32 / 3_f32).floor() as u32;
        self.inspected += 1;

        if bored % self.divisible_test == 0 {
            (self.target_monkey_test_true, bored)
        } else {
            (self.target_monkey_test_false, bored)
        }
    }

    /// Takes an `item` as input and update worry level
    fn worry(&mut self, item: u32) -> u32 {
        match self.operation {
            Operation::Add(x) => item + x,
            Operation::AddOld => item + item,
            Operation::Mult(x) => item * x,
            Operation::MultOld => item * item,
        }
    }

    /// Parses a `Monkey` from a block of input text
    fn parse(txt: &str) -> Self {
        let mut monkey = Monkey::new();
        let mut lines = txt.lines();
        lines.next();
        monkey.parse_items(lines.next().unwrap());
        monkey.parse_operation(lines.next().unwrap());
        monkey.parse_divisible_test(lines.next().unwrap());
        monkey.parse_throw_monkey(lines.next().unwrap());
        monkey.parse_throw_monkey(lines.next().unwrap());

        monkey
    }

    /// Builds a blank `Monkey`. Should not be used.
    fn new() -> Self {
        Self {
            items: vec![],
            operation: Operation::AddOld,
            divisible_test: 0,
            target_monkey_test_true: 0,
            target_monkey_test_false: 0,
            inspected: 0,
        }
    }

    /// Parses a line defining the starting items
    fn parse_items(&mut self, line: &str) {
        let reg = Regex::new("Starting items: (.+)").unwrap();
        let caps = reg.captures(line).unwrap();
        let items_txt = caps.get(1).unwrap().as_str().split(", ");
        self.items = items_txt
            .map(|token| token.parse::<u32>().unwrap())
            .collect();
    }

    /// Parses a line defining the `Operation`
    fn parse_operation(&mut self, line: &str) {
        let reg = Regex::new("Operation: new = old (.+)").unwrap();
        let caps = reg.captures(line).unwrap();
        let operation_txt = caps.get(1).unwrap().as_str();
        self.operation = Operation::from(operation_txt);
    }

    /// Parses a line defining the divisible test
    fn parse_divisible_test(&mut self, line: &str) {
        let reg = Regex::new("Test: divisible by (\\d+)$").unwrap();
        let caps = reg.captures(line).unwrap();
        let value = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        self.divisible_test = value;
    }

    /// Parses a line defining how to throw objects after test
    fn parse_throw_monkey(&mut self, line: &str) {
        let reg = Regex::new("If (.+): throw to monkey (\\d+)$").unwrap();
        let caps = reg.captures(line).unwrap();
        let monkey = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();

        match caps.get(1).unwrap().as_str() {
            "true" => self.target_monkey_test_true = monkey,
            "false" => self.target_monkey_test_false = monkey,
            x => panic!("Incorrect match {}", x),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Add(u32),
    AddOld,
    Mult(u32),
    MultOld,
}
impl Operation {
    /// Parses a string defining an `Operation`
    fn from(s: &str) -> Self {
        let mut tokens = s.split_whitespace();
        match (tokens.next(), tokens.next()) {
            (Some("*"), Some("old")) => Operation::MultOld,
            (Some("+"), Some("old")) => Operation::AddOld,
            (Some("*"), Some(x)) => Operation::Mult(x.parse::<u32>().unwrap()),
            (Some("+"), Some(x)) => Operation::Add(x.parse::<u32>().unwrap()),
            _ => panic!("Unknown operation {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_monkeys() {
        let example = read_example(11);
        let monkeys = Monkeys::parse(&example);

        assert_eq!(monkeys.0.len(), 4);
        assert_eq!(monkeys.0[0].items.len(), 2);
        assert_eq!(monkeys.0[1].operation, Operation::Add(6));
        assert_eq!(monkeys.0[2].divisible_test, 13);
        assert_eq!(monkeys.0[3].target_monkey_test_true, 0);
        assert_eq!(monkeys.0[3].target_monkey_test_false, 1);
    }

    #[test]
    fn test_worry() {
        let example = read_example(11);
        let mut monkeys = Monkeys::parse(&example);
        let monkey = monkeys.0.get_mut(0).unwrap();

        let (target1, item1) = monkey.inspect();
        assert_eq!(target1, 3);
        assert_eq!(item1, 500);

        let (target2, item2) = monkey.inspect();
        assert_eq!(target2, 3);
        assert_eq!(item2, 620);
    }

    #[test]
    fn test_run_one_round() {
        let example = read_example(11);
        let mut monkeys = Monkeys::parse(&example);
        monkeys.run_round();

        assert_eq!(monkeys.0[0].items.len(), 4);
        assert_eq!(monkeys.0[0].items[0], 20);
        assert_eq!(monkeys.0[1].items.len(), 6);
        assert_eq!(monkeys.0[1].items[0], 2080);
        assert_eq!(monkeys.0[2].items.len(), 0);
        assert_eq!(monkeys.0[3].items.len(), 0);
    }

    #[test]
    fn test_solve_part_1() {
        let example = read_example(11);
        let solution = solve_part_1(&example).unwrap();

        assert_eq!(solution, 10605)
    }
}
