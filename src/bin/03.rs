use advent_of_code::helpers::*;
use advent_of_code::solve;

fn main() {
    let input = &read_input(3);
    solve!(1, solve_part_1, input);
}

pub fn solve_part_1(input: &str) -> Option<u64> {
    let bags = parse_input(input);
    let total = bags.iter().map(Bag::get_dup_priority).sum();

    Some(total)
}

/// Parses the input string into `Vec<(char, char)>`
fn parse_input(input: &str) -> Vec<Bag> {
    input
        .lines()
        .filter(|line| line.len() > 0)
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> Bag {
    let middle = line.chars().count() / 2;
    let mut chars = line.chars();
    let first_half: Vec<Item> = chars.by_ref().take(middle).map(Item).collect();
    let second_half: Vec<Item> = chars.map(Item).collect();
    Bag(Pocket(first_half), Pocket(second_half))
}

#[derive(Debug)]
struct Bag(Pocket, Pocket);

impl Bag {
    fn first_pocket(&self) -> &Pocket {
        &self.0
    }

    fn second_pocket(&self) -> &Pocket {
        &self.1
    }

    fn get_dup_priority(&self) -> u64 {
        self.find_duplicate().priority()
    }

    fn find_duplicate(&self) -> &Item {
        self.first_pocket()
            .items()
            .iter()
            .find(|item1| {
                self.second_pocket()
                    .items()
                    .iter()
                    .find(|item2| item1 == item2)
                    .is_some()
            })
            .unwrap()
    }
}

#[derive(Debug)]
struct Pocket(Vec<Item>);

impl Pocket {
    fn items(&self) -> &[Item] {
        &self.0
    }
}

#[derive(Debug, PartialEq)]
struct Item(char);

impl Item {
    fn value(&self) -> char {
        self.0
    }

    fn priority(&self) -> u64 {
        let position = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .position(|char| char == self.value())
            .unwrap() as u64;

        position + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "KxOsZpmD";
        let bag = parse_line(line);
        assert!(bag.first_pocket().items()[2].value() == 'O');
        assert!(bag.second_pocket().items()[0].value() == 'Z');
    }

    #[test]
    fn test_priority() {
        let item = Item('p');
        assert!(item.priority() == 16);
    }

    #[test]
    fn test_find_dup() {
        let input = read_input(3);
        let bags = parse_input(&input);

        assert!(bags[0].find_duplicate().value() == 'p');
        assert!(bags[1].find_duplicate().value() == 'L');
        assert!(bags[2].find_duplicate().value() == 'P');
    }

    #[test]
    fn test_solve_1() {
        let input = read_example(3);
        let answer = solve_part_1(&input).unwrap();

        assert!(answer == 157)
    }
}
