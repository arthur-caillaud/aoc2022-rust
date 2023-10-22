use std::usize;

use advent_of_code::helpers::*;
use advent_of_code::solve;

fn main() {
    let input = &read_input(18);
    solve!(1, solve_part_1, input);
    solve!(2, solve_part_2, input);
}

fn solve_part_1(input: &str) -> Option<usize> {
    let droplet = LavaDroplet::from(input);
    Some(droplet.surface())
}

fn solve_part_2(input: &str) -> Option<usize> {
    None
}

struct LavaDroplet {
    pixels: Vec<LavaPixel>,
}
impl From<&str> for LavaDroplet {
    /// Builds a `LavaDroplet` from a `str` list of `LavaPixels`
    fn from(value: &str) -> Self {
        let pixels = value.lines().map(LavaPixel::from).collect();
        Self { pixels }
    }
}
impl LavaDroplet {
    /// Returns the surface of the `LavaDroplet`
    fn surface(&self) -> usize {
        self.pixels
            .iter()
            .map(|pixel| 6 - self.count_adjacents(pixel))
            .sum()
    }

    /// Return number of `LavaPixels` that are adjacent to provided `pixel`
    fn count_adjacents(&self, pixel: &LavaPixel) -> usize {
        self.pixels
            .iter()
            .filter(|lhs| lhs.is_adjacent(pixel))
            .count()
    }
}

struct LavaPixel {
    x: usize,
    y: usize,
    z: usize,
}
impl From<&str> for LavaPixel {
    /// Builds a `LavaPixel` from the input `str` "x,y,z"
    fn from(value: &str) -> Self {
        let coords: Vec<usize> = value
            .split(',')
            .map(|token| token.parse::<usize>().unwrap())
            .collect();

        Self {
            x: *coords.get(0).unwrap(),
            y: *coords.get(1).unwrap(),
            z: *coords.get(2).unwrap(),
        }
    }
}
impl LavaPixel {
    /// Whether two `LavaPixels` are adjacents
    fn is_adjacent(&self, rhs: &Self) -> bool {
        let Self {
            x: x0,
            y: y0,
            z: z0,
        } = self;
        let Self {
            x: x1,
            y: y1,
            z: z1,
        } = rhs;

        let distance = x0.abs_diff(*x1) + y0.abs_diff(*y1) + z0.abs_diff(*z1);

        distance == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lava() {
        let example = read_example(18);
        let droplet = LavaDroplet::from(example.as_str());

        assert_eq!(droplet.pixels.len(), 13);
    }

    #[test]
    fn test_are_adjacent() {
        let example = read_example(18);
        let droplet = LavaDroplet::from(example.as_str());

        let pixel_0 = droplet.pixels.get(0).unwrap();
        let pixel_1 = droplet.pixels.get(1).unwrap();
        let pixel_2 = droplet.pixels.get(2).unwrap();

        assert_eq!(pixel_0.is_adjacent(pixel_1), true);
        assert_eq!(pixel_0.is_adjacent(pixel_2), true);
        assert_eq!(pixel_1.is_adjacent(pixel_2), false);
    }

    #[test]
    fn test_surface() {
        let example = read_example(18);
        let droplet = LavaDroplet::from(example.as_str());

        assert_eq!(droplet.surface(), 64);
    }
}
