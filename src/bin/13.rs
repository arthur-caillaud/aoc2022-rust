use std::cmp::Ordering;

use advent_of_code::helpers::*;
use advent_of_code::solve;
use regex::Regex;

fn main() {
    let input = &read_input(13);
    solve!(1, solve_part_1, input);
}

fn solve_part_1(input: &str) -> Option<usize> {
    let pairs = PacketPair::parse_multiple(input);
    let ordered_pairs_idx =
        pairs
            .iter()
            .enumerate()
            .filter_map(|(k, pair)| match pair.correct_order() {
                true => Some(k + 1),
                false => None,
            });

    Some(ordered_pairs_idx.sum())
}

#[derive(Debug)]
struct PacketPair(Packet, Packet);
impl PacketPair {
    /// Parses the whole input into a list of `PacketPairs`
    fn parse_multiple(input: &str) -> Vec<Self> {
        input.split("\n\n").map(Self::parse).collect()
    }

    /// Parses a `PacketPair`
    fn parse(input: &str) -> Self {
        let mut packets = input.lines().map(Item::parse);
        Self(packets.next().unwrap(), packets.next().unwrap())
    }

    fn correct_order(&self) -> bool {
        println!("Using correct order on {:?}", self);
        self.0 < self.1
    }
}

#[derive(Clone, Debug)]
enum Item {
    Int(usize),
    List(Vec<Item>),
}
type Packet = Vec<Item>;
impl Item {
    /// Compares 2 `Packets`
    fn compare(lhs: &Packet, rhs: &Packet) -> Ordering {
        for k in 0.. {
            match (lhs.get(k), rhs.get(k)) {
                (None, None) => return Ordering::Equal,
                (None, _) => return Ordering::Less,
                (_, None) => return Ordering::Greater,
                (Some(lhs_item), Some(rhs_item)) => {
                    if lhs_item != rhs_item {
                        return lhs_item.partial_cmp(rhs_item).unwrap();
                    }
                }
            }
        }
        unreachable!()
    }

    /// Parses an input line into a `Packet`
    fn parse(input: &str) -> Packet {
        let regex = Regex::new(r"(\[)|(\])|(\d+)").unwrap();
        let mut tokens = regex
            .captures_iter(input)
            .map(|c| c.get(0).unwrap().as_str());

        tokens.next(); // Here we consume the first "[" that leads to error otherwise

        Self::parse_multiple(&mut tokens)
    }

    /// Parses a token iterator into a list of `Items`
    fn parse_multiple<'a, I>(tokens: &mut I) -> Packet
    where
        I: Iterator<Item = &'a str>,
    {
        let mut items = vec![];

        while let Some(token) = tokens.next() {
            match token {
                "[" => items.push(Self::List(Self::parse_multiple(tokens))),
                "]" => break,
                int => {
                    let item = Item::Int(int.parse::<usize>().unwrap());
                    items.push(item);
                }
            }
        }

        items
    }
}

impl PartialOrd for Item {
    /// - If both values are integers, the lower integer should come first.
    ///     If the left integer is lower than the right integer, the inputs are in the right order.
    ///     If the left integer is higher than the right integer, the inputs are not in the right order.
    ///     Otherwise, the inputs are the same integer; continue checking the next part of the input.
    /// - If both values are lists, compare the first value of each list, then the second value, and so on.
    ///     If the left list runs out of items first, the inputs are in the right order.
    ///     If the right list runs out of items first, the inputs are not in the right order.
    ///     If the lists are the same length and no comparison makes a decision about the order, continue checking the next part of the input.
    /// - If exactly one value is an integer, convert the integer to a list which contains that integer as its only value, then retry the comparison.
    ///     For example, if comparing [0,0,0] and 2, convert the right value to [2] (a list containing 2); the result is then found by instead comparing [0,0,0] and [2].
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Item::Int(lhs), Item::Int(rhs)) => lhs.partial_cmp(rhs),
            (Item::List(lhs), Item::List(rhs)) => Some(Item::compare(lhs, rhs)),
            (Item::Int(_), Item::List(_)) => Item::List(vec![self.clone()]).partial_cmp(other),
            (Item::List(_), Item::Int(_)) => self.partial_cmp(&Item::List(vec![other.clone()])),
        }
    }
}
impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Item::Int(lhs), Item::Int(rhs)) => lhs == rhs,
            (Item::List(lhs), Item::List(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_packets() {
        let example = read_example(13);
        let pairs = PacketPair::parse_multiple(&example);
        assert_eq!(pairs.len(), 8);

        let packet_4 = &pairs[1].1;
        assert_eq!(packet_4[0], Item::List(vec![Item::Int(1)]));
        assert_eq!(packet_4[1], Item::Int(4));
    }

    #[test]
    fn test_compare_item() {
        let example = read_example(13);
        let pairs = PacketPair::parse_multiple(&example);

        let pair_1 = &pairs[0];
        assert!(pair_1.0 < pair_1.1);

        let pair_2 = &pairs[1];
        assert!(pair_2.0 < pair_2.1);

        let pair_3 = &pairs[2];
        assert!(pair_3.0 > pair_3.1);
    }

    #[test]
    fn test_solve_1() {
        let input = read_example(13);
        let solution = solve_part_1(&input).unwrap();

        assert_eq!(solution, 13)
    }
}
