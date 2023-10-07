use advent_of_code::helpers::*;
use advent_of_code::solve;
use regex::Regex;

fn main() {
    let input = &read_input(11);
    solve!(1, solve_part_1, input);
    solve!(2, solve_part_2, input);
}

fn solve_part_1(input: &str) -> Option<usize> {
    let mut monkeys = Monkeys::parse(input, true);
    monkeys.run_n_round(20);
    let solution = monkeys.get_monkey_business();

    Some(solution)
}

fn solve_part_2(input: &str) -> Option<usize> {
    let mut monkeys = Monkeys::parse(input, false);
    monkeys.run_n_round(10000);
    let solution = monkeys.get_monkey_business();

    Some(solution)
}

#[derive(Debug)]
struct Monkeys(Vec<Monkey>);
impl Monkeys {
    /// Parses multiple `Monkeys` from a block of input text
    fn parse(text: &str, with_relief: bool) -> Self {
        Self(
            text.split("\n\n")
                .map(|txt| Monkey::parse(txt, with_relief))
                .collect(),
        )
    }

    /// Retrieves the current level of monkey business
    fn get_monkey_business(&self) -> usize {
        let most_active = self.get_n_most_active(2);

        most_active[0] * most_active[1]
    }

    /// Retrieves the values of `inspected` for `n` most active `Monkeys`
    fn get_n_most_active(&self, n: usize) -> Vec<usize> {
        let mut inspected: Vec<usize> = self.0.iter().map(|monkey| monkey.inspected).collect();
        inspected.sort_by(|a, b| b.cmp(a));

        inspected[0..n as usize].to_vec()
    }

    /// Runs `n` full rounds
    fn run_n_round(&mut self, n: usize) {
        (0..n).for_each(|_| self.run_round());
    }

    /// Runs a full round
    fn run_round(&mut self) {
        (0..self.0.len()).for_each(|k| self.run_monkey_round(k));
    }

    /// Runs a round for `Monkey` k
    fn run_monkey_round(&mut self, k: usize) {
        let common_multiple: usize = self.0.iter().map(|m| m.divisible_test).product();
        let items: Vec<(usize, usize)> = self.0.get_mut(k).unwrap().inspect_all(common_multiple);

        for (k, item) in &items {
            if let Some(target_monkey) = self.0.get_mut(*k as usize) {
                target_monkey.items.push(*item);
            }
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    divisible_test: usize,
    target_monkey_test_true: usize,
    target_monkey_test_false: usize,
    inspected: usize,
    with_relief: bool,
}
impl Monkey {
    /// `Monkey` inspects all of its items and returns a list of `(target_money, item)`
    fn inspect_all(&mut self, modulo: usize) -> Vec<(usize, usize)> {
        (0..self.items.len())
            .map(|_| self.inspect(modulo))
            .collect()
    }

    /// `Monkey` inspects the first item in its list and returns `(target_monkey, item)`
    fn inspect(&mut self, modulo: usize) -> (usize, usize) {
        let item = self.items.remove(0);
        let new = if self.with_relief {
            self.worry(item) / 3
        } else {
            self.worry(item) % modulo
        };
        self.inspected += 1;

        if new % self.divisible_test == 0 {
            (self.target_monkey_test_true, new)
        } else {
            (self.target_monkey_test_false, new)
        }
    }

    /// Takes an `item` as input and update worry level
    fn worry(&mut self, item: usize) -> usize {
        match self.operation {
            Operation::Add(x) => item + x,
            Operation::AddOld => item + item,
            Operation::Mult(x) => item * x,
            Operation::MultOld => item * item,
        }
    }

    /// Parses a `Monkey` from a block of input text
    fn parse(txt: &str, with_relief: bool) -> Self {
        let mut monkey = Monkey::new(with_relief);
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
    fn new(with_relief: bool) -> Self {
        Self {
            items: vec![],
            operation: Operation::AddOld,
            divisible_test: 0,
            target_monkey_test_true: 0,
            target_monkey_test_false: 0,
            inspected: 0,
            with_relief,
        }
    }

    /// Parses a line defining the starting items
    fn parse_items(&mut self, line: &str) {
        let reg = Regex::new("Starting items: (.+)").unwrap();
        let caps = reg.captures(line).unwrap();
        let items_txt = caps.get(1).unwrap().as_str().split(", ");
        self.items = items_txt
            .map(|token| token.parse::<usize>().unwrap())
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
        let value = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        self.divisible_test = value;
    }

    /// Parses a line defining how to throw objects after test
    fn parse_throw_monkey(&mut self, line: &str) {
        let reg = Regex::new("If (.+): throw to monkey (\\d+)$").unwrap();
        let caps = reg.captures(line).unwrap();
        let monkey = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();

        match caps.get(1).unwrap().as_str() {
            "true" => self.target_monkey_test_true = monkey,
            "false" => self.target_monkey_test_false = monkey,
            x => panic!("Incorrect match {}", x),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Add(usize),
    AddOld,
    Mult(usize),
    MultOld,
}
impl Operation {
    /// Parses a string defining an `Operation`
    fn from(s: &str) -> Self {
        let mut tokens = s.split_whitespace();
        match (tokens.next(), tokens.next()) {
            (Some("*"), Some("old")) => Operation::MultOld,
            (Some("+"), Some("old")) => Operation::AddOld,
            (Some("*"), Some(x)) => Operation::Mult(x.parse::<usize>().unwrap()),
            (Some("+"), Some(x)) => Operation::Add(x.parse::<usize>().unwrap()),
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
        let monkeys = Monkeys::parse(&example, true);

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
        let mut monkeys = Monkeys::parse(&example, true);
        let monkey = monkeys.0.get_mut(0).unwrap();

        let (target1, item1) = monkey.inspect(1);
        assert_eq!(target1, 3);
        assert_eq!(item1, 500);

        let (target2, item2) = monkey.inspect(1);
        assert_eq!(target2, 3);
        assert_eq!(item2, 620);
    }

    #[test]
    fn test_run_one_round() {
        let example = read_example(11);
        let mut monkeys = Monkeys::parse(&example, true);
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

    #[test]
    fn test_solve_part_2() {
        let example = read_example(11);
        let solution = solve_part_2(&example).unwrap();

        assert_eq!(solution, 2713310158)
    }
}
