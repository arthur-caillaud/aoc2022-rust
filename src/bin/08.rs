use advent_of_code::helpers::*;
use advent_of_code::solve;

fn main() {
    let input = &read_input(8);
    solve!(1, solve_part_1, input);
    solve!(2, solve_part_2, input);
}

fn solve_part_1(input: &str) -> Option<u32> {
    let forest = Forest::new(input);
    let visible_trees = forest.get_visible_trees();
    Some(visible_trees.len() as u32)
}

fn solve_part_2(input: &str) -> Option<u32> {
    let forest = Forest::new(input);
    forest
        .map(|pos| forest.get_scenic_score(pos))
        .iter()
        .max()
        .copied()
}

#[derive(Debug)]
struct Forest(Vec<Vec<Tree>>);
type Tree = u32;
impl Forest {
    /// Build a new `Forest` by parsing the provided `String`
    fn new(s: &str) -> Self {
        let trees = s
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        Forest(trees)
    }

    /// Get the list of all visible trees
    fn get_visible_trees(&self) -> Vec<&u32> {
        self.filter(|pos| self.tree_is_visible(pos))
    }

    /// Whether the tree at `pos` is visible
    fn tree_is_visible(&self, pos: (u32, u32)) -> bool {
        Direction::VALUES
            .iter()
            .any(|direction| self.tree_is_visible_from(pos, direction))
    }

    /// Whether the tree at `pos` is visible from `direction`
    fn tree_is_visible_from(&self, pos: (u32, u32), direction: &Direction) -> bool {
        let hedge = self.get_hedge(pos, direction);
        let is_not_visible = hedge.iter().any(|&tree| tree >= self.size(pos));

        !is_not_visible
    }

    /// Retrieves the size of the tree at `pos`
    fn size(&self, (i, j): (u32, u32)) -> &u32 {
        &self.0[i as usize][j as usize]
    }

    /// Retrieves the hedge made by trees extending from `pos` in the provided `direction`
    fn get_hedge(&self, pos: (u32, u32), direction: &Direction) -> Vec<&u32> {
        self.filter(|(i, j)| match direction {
            Direction::Bottom => i as u32 > pos.0 && j as u32 == pos.1,
            Direction::Left => i as u32 == pos.0 && (j as u32) < pos.1,
            Direction::Right => i as u32 == pos.0 && j as u32 > pos.1,
            Direction::Top => (i as u32) < pos.0 && j as u32 == pos.1,
        })
    }

    /// Gets the scenic score for tree at `pos`
    fn get_scenic_score(&self, pos: (u32, u32)) -> u32 {
        Direction::VALUES
            .map(|direction| self.get_viewing_distance_from(pos, &direction))
            .into_iter()
            .reduce(|acc, dist| acc * dist)
            .unwrap()
    }

    /// Gets the viewing distance from `pos` in `direction`
    fn get_viewing_distance_from(&self, pos: (u32, u32), direction: &Direction) -> u32 {
        let hedge = self.get_directed_hedge(pos, direction);
        let mut distance = 0;
        for tree in hedge {
            distance += 1;
            if tree >= self.size(pos) {
                break;
            }
        }
        distance as u32
    }

    /// Useful when wanting to walk the hedge from the center outwards
    fn get_directed_hedge(&self, pos: (u32, u32), direction: &Direction) -> Vec<&u32> {
        let mut hedge = self.get_hedge(pos, direction);
        if direction == &Direction::Top || direction == &Direction::Left {
            hedge.reverse();
        }
        hedge
    }

    /// Retrieves all the trees matching for the provided `predicate`
    fn filter<F>(&self, predicate: F) -> Vec<&u32>
    where
        F: Fn((u32, u32)) -> bool,
    {
        let mut trees = vec![];

        self.0.iter().enumerate().for_each(|(i, hedge)| {
            hedge.iter().enumerate().for_each(|(j, tree)| {
                if predicate((i as u32, j as u32)) {
                    trees.push(tree)
                }
            })
        });

        trees
    }

    /// Retrieves all the trees matching for the provided `predicate`
    fn map<F>(&self, predicate: F) -> Vec<u32>
    where
        F: Fn((u32, u32)) -> u32,
    {
        let mut values = vec![];

        self.0.iter().enumerate().for_each(|(i, hedge)| {
            hedge
                .iter()
                .enumerate()
                .for_each(|(j, _)| values.push(predicate((i as u32, j as u32))))
        });

        values
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}
impl Direction {
    const VALUES: [Self; 4] = [Self::Top, Self::Bottom, Self::Left, Self::Right];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_1() {
        let input = read_example(8);
        let solution = solve_part_1(&input).unwrap();
        assert_eq!(solution, 21);
    }

    #[test]
    fn test_solve_part_2() {
        let input = read_example(8);
        let solution = solve_part_2(&input).unwrap();
        assert_eq!(solution, 8);
    }

    #[test]
    fn test_viewing_distance() {
        let input = read_example(8);
        let forest = Forest::new(&input);

        assert_eq!(forest.get_viewing_distance_from((1, 2), &Direction::Top), 1);
    }

    #[test]
    fn test_scenic_score() {
        let input = read_example(8);
        let forest = Forest::new(&input);

        assert_eq!(forest.get_scenic_score((0, 0)), 0);
        assert_eq!(forest.get_scenic_score((1, 2)), 4);
        assert_eq!(forest.get_scenic_score((3, 2)), 8);
    }

    #[test]
    fn test_new_forest() {
        let input = read_example(8);
        let forest = Forest::new(&input);

        assert_eq!(forest.size((0, 2)), &3);
        assert_eq!(forest.size((1, 4)), &2);
        assert_eq!(forest.size((2, 0)), &6);
    }

    #[test]
    fn test_tree_is_visible() {
        let input = read_example(8);
        let forest = Forest::new(&input);

        assert_eq!(forest.tree_is_visible((0, 0)), true);
        assert_eq!(forest.tree_is_visible((1, 1)), true);
        assert_eq!(forest.tree_is_visible((1, 3)), false);
        assert_eq!(forest.tree_is_visible((2, 2)), false);
    }

    #[test]
    fn test_get_hedge() {
        let input = read_example(8);
        let forest = Forest::new(&input);

        let hedge_1 = forest.get_hedge((1, 1), &Direction::Top);
        assert_eq!(hedge_1.len(), 1);
        assert_eq!(hedge_1[0], &0);

        let hedge_2 = forest.get_hedge((3, 3), &Direction::Left);
        assert_eq!(hedge_2.len(), 3);
        assert_eq!(hedge_2[1], &3);

        let hedge_3 = forest.get_hedge((4, 0), &Direction::Bottom);
        assert_eq!(hedge_3.len(), 0);
    }
}
