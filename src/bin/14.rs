use std::collections::HashSet;

use advent_of_code::helpers::*;
use advent_of_code::solve;

fn main() {
    let input = &read_input(14);
    solve!(1, solve_part_1, input);
    solve!(2, solve_part_2, input);
}

fn solve_part_1(input: &str) -> Option<usize> {
    let mut map = Map::init(&input);

    while map.insert_sand() {}

    Some(map.sands.len())
}

fn solve_part_2(input: &str) -> Option<usize> {
    None
}

#[derive(Debug)]
struct Map {
    obstacles: HashSet<Position>,
    sands: HashSet<Position>,
    source: Position,
}

impl Map {
    /// Inserts a new sand from the source. Returns `true` if the sand was correctly inserted.
    /// Returns `false` if it fell indefinitely.
    fn insert_sand(&mut self) -> bool {
        if let Some(sand) = self.fall_from(&self.source) {
            self.sands.insert(sand);
            self.obstacles.insert(sand);
            true
        } else {
            false
        }
    }

    /// Falls from `Position`. Returns `None` if it falls indefinitely. Returns the `Position`
    /// where it should fall to otherwise.
    fn fall_from(&self, pos: &Position) -> Option<Position> {
        if let Some(first_obstacle) = self.find_obstacle_under(pos) {
            if let Some(_) = self.find_obstacle_left(first_obstacle) {
                if let Some(_) = self.find_obstacle_right(first_obstacle) {
                    Some((first_obstacle.0, first_obstacle.1 - 1))
                } else {
                    self.fall_from(&(first_obstacle.0 + 1, first_obstacle.1))
                }
            } else {
                self.fall_from(&(first_obstacle.0 - 1, first_obstacle.1))
            }
        } else {
            None
        }
    }

    /// May find a rock left to the provided `Position`
    fn find_obstacle_left(&self, coord: &Position) -> Option<&Position> {
        let right_pos = &(coord.0 - 1, coord.1);
        self.obstacles
            .get(right_pos)
            .or_else(|| self.sands.get(right_pos))
    }

    /// May find a rock right to the provided `Position`
    fn find_obstacle_right(&self, coord: &Position) -> Option<&Position> {
        let left_pos = &(coord.0 + 1, coord.1);
        self.obstacles
            .get(left_pos)
            .or_else(|| self.sands.get(left_pos))
    }

    /// May find a rock under the provided `Position`
    fn find_obstacle_under(&self, coord: &Position) -> Option<&Position> {
        self.obstacles
            .iter()
            .filter(|rock| coord.0 == rock.0 && coord.1 < rock.1)
            .min_by(|pos1, pos2| pos1.1.cmp(&pos2.1))
    }

    /// Initializes the `Map` from the `str` input
    fn init(input: &str) -> Self {
        let mut map = Self {
            obstacles: HashSet::new(),
            source: (500, 0),
            sands: HashSet::new(),
        };
        input.lines().map(Self::parse_line).for_each(|coords| {
            (0..coords.len() - 1).for_each(|k| {
                let from = coords.get(k).unwrap();
                let to = coords.get(k + 1).unwrap();
                map.insert_rocks(*from, *to);
            })
        });

        map
    }

    /// Insert rocks in the map between the provided `Positions`
    fn insert_rocks(&mut self, (x1, y1): Position, (x2, y2): Position) {
        let x_range = if x1 <= x2 { x1..=x2 } else { x2..=x1 };
        let y_range = if y1 <= y2 { y1..=y2 } else { y2..=y1 };
        x_range.for_each(|x| {
            self.obstacles.insert((x, y1));
        });
        y_range.for_each(|y| {
            self.obstacles.insert((x1, y));
        });
    }

    /// Parses a `Position` boundary defined by "X0,Y0 -> X1,Y1"
    fn parse_line(input: &str) -> Vec<Position> {
        input.split(" -> ").map(Map::parse_rock).collect()
    }

    /// Parses a `Position` defined by "X,Y" as `str`
    fn parse_rock(input: &str) -> Position {
        let coords: Vec<usize> = input
            .split(',')
            .map(|token| token.parse::<usize>().unwrap())
            .collect();

        (*coords.get(0).unwrap(), *coords.get(1).unwrap())
    }
}

type Position = (usize, usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = read_example(14);
        let map = Map::init(&input);

        assert_eq!(map.obstacles.len(), 20);

        let rock1 = map.obstacles.get(&(498, 5));
        assert_ne!(rock1, None);
        let rock2 = map.obstacles.get(&(500, 9));
        assert_ne!(rock2, None);
        let no_rock = map.obstacles.get(&(500, 2));
        assert_eq!(no_rock, None);
    }

    #[test]
    fn test_rock_under() {
        let input = read_example(14);
        let map = Map::init(&input);

        let rock1 = map.find_obstacle_under(&map.source).unwrap();
        assert_eq!(rock1, &(500, 9));
        let rock2 = map.find_obstacle_under(&(498, 0)).unwrap();
        assert_eq!(rock2, &(498, 4));
        let no_rock = map.find_obstacle_under(&(493, 0));
        assert_eq!(no_rock, None);
    }

    #[test]
    fn test_insert_sand() {
        let input = read_example(14);
        let mut map = Map::init(&input);

        map.insert_sand(); // Insert 1st sand grain
        assert_ne!(map.sands.get(&(500, 8)), None);

        map.insert_sand(); // Insert 2nd sand grain
        assert_ne!(map.sands.get(&(499, 8)), None);

        assert_eq!(map.insert_sand(), true);
        assert_eq!(map.insert_sand(), true);
        assert_eq!(map.insert_sand(), true);
        assert_eq!(map.sands.contains(&(500, 7)), true);
        assert_eq!(map.sands.len(), 5);
    }

    #[test]
    fn test_solve_part_1() {
        let input = read_example(14);
        let solution = solve_part_1(&input).unwrap();

        assert_eq!(solution, 24)
    }
}
