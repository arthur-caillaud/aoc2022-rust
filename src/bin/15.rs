use advent_of_code::helpers::*;
use advent_of_code::solve;
use rayon::prelude::*;
use regex::Regex;
use std::ops::RangeInclusive;

fn main() {
    let input = &read_input(15);
    solve!(1, solve_part_1, input);
    solve!(2, solve_part_2, input);
}

fn solve_part_1(input: &str) -> Option<usize> {
    let map = Map::from(input);
    let solution = map.count_covered_row(2000000);

    Some(solution)
}

fn solve_part_2(input: &str) -> Option<isize> {
    let map = Map::from(input);
    let not_covered = map.find_not_covered(0..=4000000, 0..=4000000);
    let solution = not_covered.tuning_frequency();

    Some(solution)
}

struct Map {
    sensors: Vec<Sensor>,
    x_max: isize,
    x_min: isize,
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

        Self {
            sensors,
            x_max,
            x_min,
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
            .filter(|cell| self.is_covered(cell))
            .count()
    }

    /// Filters the input `Positions` to retrieve the ones that are not covered
    fn find_not_covered(
        &self,
        x_range: RangeInclusive<isize>,
        y_range: RangeInclusive<isize>,
    ) -> Position {
        x_range
            .into_par_iter()
            .flat_map(move |x| {
                y_range
                    .clone()
                    .filter_map(move |y| {
                        let position = Position { x, y };
                        if self.is_not_covered(&position) {
                            Some(position)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                    .into_par_iter()
            })
            .find_any(|_| true)
            .unwrap()
    }

    /// Returns whether the provided `Position` is covered by one of the `Map` sensors
    fn is_covered(&self, pos: &Position) -> bool {
        self.sensors.iter().any(|sensor| sensor.not_beacon(pos))
    }

    /// Returns whether the provided `Position` is not covered by any of the `Map` sensors
    fn is_not_covered(&self, pos: &Position) -> bool {
        self.sensors
            .iter()
            .all(|sensor| sensor.possible_beacon(pos))
    }

    /// Walks the line defined by `y` up to the boundaries of the `Map`
    fn walk_row(&self, y: isize) -> impl Iterator<Item = Position> + '_ {
        (self.x_min..=self.x_max).map(move |x| Position { x, y })
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
    /// Whether the provided `Position` cannot be a beacon
    fn not_beacon(&self, pos: &Position) -> bool {
        !self.is_detected(pos) && self.is_covered(pos)
    }

    /// Whether the provided `Position` can be a unknown beacon
    fn possible_beacon(&self, pos: &Position) -> bool {
        !self.is_detected(pos) && !self.is_covered(pos)
    }

    /// Returns whether the provided `Position` is covered
    fn is_covered(&self, pos: &Position) -> bool {
        Position::distance(&self.pos, pos) <= self.distance_to_closest
    }

    /// Returns whether the provided `Position` is detected as a beacon
    fn is_detected(&self, pos: &Position) -> bool {
        pos == &self.closest_beacon
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

    /// Computes the tuning frequency for the `Position`
    fn tuning_frequency(&self) -> isize {
        self.x * 4000000 + self.y
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
    fn test_solve_part_1() {
        let example = read_example(15);
        let map = Map::from(example.as_str());
        let solution = map.count_covered_row(10);

        assert_eq!(solution, 26);
    }

    #[test]
    fn test_solve_part_2() {
        let example = read_example(15);
        let map = Map::from(example.as_str());

        let not_covered = map.find_not_covered(0..=20, 0..=20);
        let tun_freq = not_covered.tuning_frequency();
        assert_eq!(tun_freq, 56000011);
    }
}
