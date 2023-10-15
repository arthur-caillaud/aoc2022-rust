use advent_of_code::helpers::*;
use advent_of_code::solve;
use regex::Regex;

fn main() {
    let input = &read_input(15);
    solve!(1, solve_part_1, input);
    solve!(2, solve_part_2, input);
}

fn solve_part_1(input: &str) -> Option<usize> {
    let map = Map::from(input);
    Some(map.count_covered_row(2000000))
}

fn solve_part_2(input: &str) -> Option<u64> {
    None
}

struct Map {
    sensors: Vec<Sensor>,
    x_max: isize,
    x_min: isize,
    y_max: isize,
    y_min: isize,
}
impl From<Vec<Sensor>> for Map {
    fn from(sensors: Vec<Sensor>) -> Self {
        let x_min = sensors
            .iter()
            .map(|sensor| sensor.pos.x - sensor.distance_to_closest)
            .min()
            .unwrap();
        let x_max = sensors
            .iter()
            .map(|sensor| sensor.pos.x + sensor.distance_to_closest)
            .max()
            .unwrap();
        let y_min = sensors
            .iter()
            .map(|sensor| sensor.pos.y - sensor.distance_to_closest)
            .min()
            .unwrap();
        let y_max = sensors
            .iter()
            .map(|sensor| sensor.pos.y + sensor.distance_to_closest)
            .max()
            .unwrap();

        Self {
            sensors,
            x_max,
            x_min,
            y_max,
            y_min,
        }
    }
}
impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let positions = Position::parse_multiple(value);
        let sensors: Vec<Sensor> = positions.iter().map(Sensor::from).collect();
        Map::from(sensors)
    }
}

impl Map {
    /// Counts the number of cells covered on row `y`
    fn count_covered_row(&self, y: isize) -> usize {
        self.walk_row(y)
            .iter()
            .filter(|cell| self.is_covered(cell))
            .count()
    }

    /// Counts the number of cells covered on column `x`
    fn count_covered_col(&self, x: isize) -> usize {
        self.walk_col(x)
            .iter()
            .filter(|cell| self.is_covered(cell))
            .count()
    }

    /// Returns whether the provided `Position` is covered by one of the `Map` sensors
    fn is_covered(&self, pos: &Position) -> bool {
        self.sensors.iter().any(|sensor| sensor.is_covered(pos))
    }

    /// Walks the line defined by `y` up to the boundaries of the `Map`
    fn walk_row(&self, y: isize) -> Vec<Position> {
        (self.x_min..=self.x_max)
            .map(|x| Position { x, y })
            .collect()
    }

    /// Walks the col defined by `x` up to the boundaries of the `Map`
    fn walk_col(&self, x: isize) -> Vec<Position> {
        (self.y_min..=self.y_max)
            .map(|y| Position { x, y })
            .collect()
    }
}

struct Sensor {
    closest_beacon: Position,
    distance_to_closest: isize,
    pos: Position,
}
impl From<&(Position, Position)> for Sensor {
    fn from((sensor, beacon): &(Position, Position)) -> Self {
        let distance_to_closest = sensor.distance(beacon);

        Self {
            closest_beacon: *beacon,
            distance_to_closest,
            pos: *sensor,
        }
    }
}
impl Sensor {
    /// Returns whether the provided `Position` is covered
    fn is_covered(&self, pos: &Position) -> bool {
        pos != &self.closest_beacon && self.pos.distance(pos) <= self.distance_to_closest
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Position {
    x: isize,
    y: isize,
}
impl Position {
    fn parse_multiple(input: &str) -> Vec<(Position, Position)> {
        input.lines().map(Self::parse).collect()
    }

    fn parse(input: &str) -> (Position, Position) {
        let regex: Regex =
            Regex::new(r"Sensor at x=(.+), y=(.+): closest beacon is at x=(.+), y=(.+)").unwrap();
        let caps = regex.captures(input).unwrap();
        let sensor_x = caps.get(1).unwrap().as_str().parse::<isize>().unwrap();
        let sensor_y = caps.get(2).unwrap().as_str().parse::<isize>().unwrap();
        let beacon_x = caps.get(3).unwrap().as_str().parse::<isize>().unwrap();
        let beacon_y = caps.get(4).unwrap().as_str().parse::<isize>().unwrap();

        (
            Position {
                x: sensor_x,
                y: sensor_y,
            },
            Position {
                x: beacon_x,
                y: beacon_y,
            },
        )
    }

    /// Computes the distance between two `Positions` on the `Map`
    fn distance(&self, beacon: &Position) -> isize {
        (self.x.abs_diff(beacon.x) + self.y.abs_diff(beacon.y)) as isize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let example = read_example(15);
        let positions = Position::parse_multiple(&example);

        assert_eq!(positions.len(), 14);
        assert_eq!(positions[3].0.x, 12);
        assert_eq!(positions[5].1.y, 16);
    }

    #[test]
    fn test_distance() {
        let sensor = Position { x: 8, y: 7 };
        let beacon = Position { x: 2, y: 10 };

        assert_eq!(sensor.distance(&beacon), 9);
    }

    #[test]
    fn test_count_covered_row() {
        let example = read_example(15);
        let map = Map::from(example.as_str());
        let solution = map.count_covered_row(10);

        assert_eq!(solution, 26);
    }
}
