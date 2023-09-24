use advent_of_code::helpers::*;
use advent_of_code::solve;

fn main() {
    let input = &read_input(3);
    solve!(1, solve_part_1, input);
    solve!(2, solve_part_2, input);
}

fn solve_part_1(input: &str) -> Option<u64> {
    let bags = parse_input(input);
    let total = bags.iter().map(Bag::get_dup_priority).sum();

    Some(total)
}

fn solve_part_2(input: &str) -> Option<u64> {
    let bags = parse_input(input);
    let groups = Group::from(bags);
    let triplicates = groups.iter().map(Group::find_triplicate);
    let sum = triplicates.map(|item| item.priority()).sum();

    Some(sum)
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

#[derive(Clone)]
struct Group(Bag, Bag, Bag);

impl Group {
    /// Builds `Groups` of `Bags` by slicing input `Bags` into chunks of 3 `Bags`
    fn from(bags: Vec<Bag>) -> Vec<Group> {
        bags.chunks(3)
            .map(|chunk| Group(chunk[0].clone(), chunk[1].clone(), chunk[2].clone()))
            .collect()
    }

    /// Returns the first `Item` in the list of triplicates. Useful when assuming
    /// there is only 1 triplicate.
    fn find_triplicate(&self) -> Item {
        self.find_triplicates()[0]
    }

    /// Finds all triplicate `Items` between the three bags
    fn find_triplicates(&self) -> Vec<Item> {
        let (bag1, bag2) = self.merge_bags();
        let duplicates1 = bag1.find_duplicates();
        let duplicates2 = bag2.find_duplicates();

        duplicates1
            .iter()
            .map(|&item| *item)
            .filter(|&item1| {
                duplicates2
                    .iter()
                    .any(|&item2| item1.value() == item2.value())
            })
            .collect()
    }

    fn merge_bags(&self) -> (Bag, Bag) {
        let bag1_pocket1 = self.0.merge_pockets();
        let bag1_pocket2 = self.1.merge_pockets();
        let bag2_pocket1 = self.1.merge_pockets();
        let bag2_pocket2 = self.2.merge_pockets();

        let bag1 = Bag(bag1_pocket1, bag1_pocket2);
        let bag2 = Bag(bag2_pocket1, bag2_pocket2);

        return (bag1, bag2);
    }
}

#[derive(Clone)]
struct Bag(Pocket, Pocket);

impl Bag {
    fn first_pocket(&self) -> &Pocket {
        &self.0
    }

    fn second_pocket(&self) -> &Pocket {
        &self.1
    }

    /// Returns the `Pocket` made by merging the two `Pockets`
    fn merge_pockets(&self) -> Pocket {
        let mut merged_items = self.0.items().to_vec();
        merged_items.extend(self.1.items().to_vec());
        Pocket(merged_items)
    }

    fn get_dup_priority(&self) -> u64 {
        self.find_duplicate().priority()
    }

    /// Returns the first item in the list of duplicates. Useful when assuming
    /// there is only 1 duplicate
    fn find_duplicate(&self) -> &Item {
        self.find_duplicates()[0]
    }

    /// Finds all duplicate items between the two pockets
    fn find_duplicates(&self) -> Vec<&Item> {
        self.first_pocket()
            .items()
            .iter()
            .filter(|item1| {
                self.second_pocket()
                    .items()
                    .iter()
                    .find(|item2| item1 == item2)
                    .is_some()
            })
            .collect()
    }
}

#[derive(Clone)]
struct Pocket(Vec<Item>);

impl Pocket {
    fn items(&self) -> &[Item] {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
        assert_eq!(bag.first_pocket().items()[2].value(), 'O');
        assert_eq!(bag.second_pocket().items()[0].value(), 'Z');
    }

    #[test]
    fn test_priority() {
        let item = Item('p');
        assert!(item.priority() == 16);
    }

    #[test]
    fn test_find_dup() {
        let input = read_example(3);
        let bags = parse_input(&input);

        assert_eq!(bags[0].find_duplicate().value(), 'p');
        assert_eq!(bags[1].find_duplicate().value(), 'L');
        assert_eq!(bags[2].find_duplicate().value(), 'P');
    }

    #[test]
    fn test_solve_1() {
        let input = read_example(3);
        let answer = solve_part_1(&input).unwrap();

        assert_eq!(answer, 157)
    }

    #[test]
    fn test_find_triplicate() {
        let input = read_example(3);
        let bags = parse_input(&input);
        let group1 = Group(bags[0].clone(), bags[1].clone(), bags[2].clone());
        let group2 = Group(bags[3].clone(), bags[4].clone(), bags[5].clone());
        assert_eq!(group1.find_triplicate().value(), 'r');
        assert_eq!(group2.find_triplicate().value(), 'Z');
    }

    #[test]
    fn test_solve_2() {
        let input = read_example(3);
        let answer = solve_part_2(&input).unwrap();

        assert_eq!(answer, 70)
    }
}
