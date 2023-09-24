use advent_of_code::helpers::*;
use advent_of_code::solve;

fn main() {
    let input = &read_input(4);
    solve!(1, solve_part_1, input);
}

pub fn solve_part_1(input: &str) -> Option<u64> {
    let pairs = Pair::from_multiple(input);
    let overlaps = pairs.iter().filter(|pair| pair.is_overlapping());
    let length = overlaps.count() as u64;

    Some(length)
}

struct Pair(Assignment, Assignment);

impl Pair {
    fn is_overlapping(&self) -> bool {
        let low_one = self.0 .0;
        let low_two = self.1 .0;
        let up_one = self.0 .1;
        let up_two = self.1 .1;

        let one_in_two = low_one <= low_two && up_one >= up_two;
        let two_in_one = low_two <= low_one && up_two >= up_one;

        one_in_two || two_in_one
    }

    fn from_multiple(input: &str) -> Vec<Self> {
        input.lines().map(Self::from).collect()
    }

    fn from(line: &str) -> Self {
        let parts: Vec<&str> = line.split(',').collect();

        Pair(Assignment::from(parts[0]), Assignment::from(parts[1]))
    }
}

struct Assignment(u64, u64);

impl Assignment {
    fn from(assignment: &str) -> Self {
        let parts: Vec<&str> = assignment.split('-').collect();
        let start = parts[0]
            .parse::<u64>()
            .expect(&format!("Start parse failed: {}", parts[0]));
        let end = parts[1]
            .parse::<u64>()
            .expect(&format!("End parse failed: {}", parts[1]));

        Self(start, end)
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
    fn test_pair_is_overlapping() {
        let pair = Pair(Assignment(2, 8), Assignment(3, 7));

        assert_eq!(pair.is_overlapping(), true);
    }

    #[test]
    fn test_pair_is_not_overlapping() {
        let pair = Pair(Assignment(5, 7), Assignment(7, 9));

        assert_eq!(pair.is_overlapping(), false);
    }

    #[test]
    fn test_pair_from_multiple() {
        let input = read_example(4);
        let sections = Pair::from_multiple(&input);

        assert_eq!(sections[0].0 .0, 2);
        assert_eq!(sections[1].1 .0, 4);
        assert_eq!(sections[2].1 .1, 9);
    }

    #[test]
    fn test_pair_from() {
        let line = "1-10,4-8";
        let Pair(section1, section2) = Pair::from(line);

        assert_eq!(section1.0, 1);
        assert_eq!(section1.1, 10);
        assert_eq!(section2.0, 4);
        assert_eq!(section2.1, 8);
    }

    #[test]
    fn test_section_from_one() {
        let assignment = "2-12";
        let section = Assignment::from(assignment);

        assert_eq!(section.0, 2);
        assert_eq!(section.1, 12);
    }
}
