use advent_of_code::helpers::*;
use advent_of_code::solve;

fn main() {
    let input = &read_input(1);
    solve!(1, solve, input);
}

/// Solves the problem on the `input` string
fn solve(input: &str) -> Option<u64> {
    let foods = parse_input(input);
    let elves = make_elves(foods);
    Some(find_biggest(elves))
}

/// Makes all our little elves
fn make_elves(foods: Vec<Vec<u64>>) -> Vec<Elf> {
    foods
        .into_iter()
        .map(|elf_food| Elf { foods: elf_food })
        .collect()
}

/// Parses the input string into `Vec<Vec<u64>>`
fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input.split("\n\n").map(parse_block).collect()
}

/// Parses a block of the input string into `Vec<u64>`
fn parse_block(block: &str) -> Vec<u64> {
    block
        .split("\n")
        .filter(|token| token.len() > 0)
        .map(|token| token.parse::<u64>().unwrap())
        .collect()
}

/// Describes the `Elf` with its respective calories inside `foods`
struct Elf {
    foods: Vec<u64>,
}

/// Provides the `Elf` with `total` for computing its total calories
impl Elf {
    pub fn total(&self) -> u64 {
        let mut total = 0;
        for food in &self.foods {
            total += food
        }
        total
    }
}

/// Find the `Elf` with biggest total calories and return the total
fn find_biggest(elves: Vec<Elf>) -> u64 {
    let mut max = 0;
    elves.iter().for_each(|elf| {
        let total = elf.total();
        if total >= max {
            max = total
        }
    });

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elf_total() {
        let elf = Elf {
            foods: vec![1000, 1500, 3000],
        };
        assert!(elf.total() == 5500)
    }

    #[test]
    fn test_find_biggest() {
        let elf1 = Elf { foods: vec![1, 1] };
        let elf2 = Elf { foods: vec![5, 5] };
        let elves = vec![elf1, elf2];
        let biggest = find_biggest(elves);
        assert!(biggest == 10)
    }

    #[test]
    fn test_parse_input() {
        let input = &read_example(1);
        let tokens = parse_input(input);
        assert!(tokens[0][0] == 1000);
        assert!(tokens[0][1] == 2000);
        assert!(tokens[1][0] == 3000);
    }
}
