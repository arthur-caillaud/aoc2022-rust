use std::collections::HashMap;
use std::ops::Sub;

use advent_of_code::helpers::*;
use advent_of_code::solve;

fn main() {
    let input = &read_input(9);
    solve!(1, solve_part_1, input);
    solve!(2, solve_part_2, input);
}

fn solve_part_1(input: &str) -> Option<u32> {
    let mut grid = Grid::new(2);
    let moves = Move::from(&input);
    grid.exec_multiple(moves);
    let solution = grid.get_visited(1);

    Some(solution)
}

fn solve_part_2(input: &str) -> Option<u32> {
    let mut grid = Grid::new(10);
    let moves = Move::from(&input);
    grid.exec_multiple(moves);
    let solution = grid.get_visited(9);

    Some(solution)
}

struct Grid {
    nodes: Vec<Cell>,
    visited: Vec<HashMap<Cell, bool>>,
}
impl Grid {
    /// Build a new `Grid` with cells initialized at `(0,0)`
    fn new(nodes: u32) -> Self {
        let mut visited: Vec<HashMap<Cell, bool>> = (0..nodes).map(|_| HashMap::new()).collect();
        (0..nodes).for_each(|i| {
            visited[i as usize].insert(Cell(0, 0), true);
        });
        Self {
            nodes: (0..nodes).map(|_| Cell(0, 0)).collect(),
            visited,
        }
    }

    /// Retrieves the number of cells that have been visited by node `k`
    fn get_visited(&self, k: u32) -> u32 {
        self.visited[k as usize].keys().len() as u32
    }

    /// Executes multiples `Moves` on the `Grid`
    fn exec_multiple(&mut self, moves: Vec<Move>) {
        moves.iter().for_each(|mv| self.exec(mv))
    }

    /// Executes a `Move` on the `Grid`
    fn exec(&mut self, mv: &Move) {
        (0..mv.1).for_each(|_| self.move_once(&mv));
    }

    /// Executes a `Move` once into its `direction` by moving the
    /// `head` and reconciling the `tail`
    fn move_once(&mut self, mv: &Move) {
        self.move_node(0, &mv.0);
        (0..self.nodes.len()).for_each(|k| {
            self.reconcile_node(k);
            self.visited[k].insert(self.nodes[k], true);
        });
    }

    /// Reconciles the `tail` with the position of the `head`
    fn reconcile_node(&mut self, k: usize) {
        if k == 0 {
            return;
        }
        let distance = self.nodes[k - 1] - self.nodes[k];
        let moves = Direction::from_distance(distance);
        moves.iter().for_each(|dir| self.move_node(k, dir))
    }

    /// Moves the `tail` into `direction`
    fn move_node(&mut self, k: usize, dir: &Direction) {
        Self::move_cell(&mut self.nodes[k], dir)
    }

    /// Moves the provided `cell` into `direction`
    fn move_cell(cell: &mut Cell, dir: &Direction) {
        match dir {
            Direction::Left => cell.0 -= 1,
            Direction::Right => cell.0 += 1,
            Direction::Up => cell.1 += 1,
            Direction::Down => cell.1 += -1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cell(i32, i32);
impl Sub for Cell {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

struct Move(Direction, u32);
impl Move {
    /// Parses lines of an input `str` into a list of `Moves`
    fn from(s: &str) -> Vec<Self> {
        s.lines().map(Self::from_one).collect()
    }

    /// Parses a line into a `Move`
    fn from_one(s: &str) -> Self {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        Self(
            Direction::from(&tokens[0].chars().nth(0).unwrap()),
            tokens[1].parse::<u32>().unwrap(),
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    /// Parses a `char` into a `Direction`
    fn from(c: &char) -> Self {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unexpected char {}", c),
        }
    }

    /// Gets the `Directions` a cell should moved based on the `distance` vector
    fn from_distance(distance: Cell) -> Vec<Direction> {
        let Cell(dx, dy) = distance;
        let mut moves = vec![];
        match (dx.abs(), dy.abs()) {
            (2, _) => {
                moves.push(Direction::from_dx(dx).unwrap());
                if let Some(mv) = Direction::from_dy(dy) {
                    moves.push(mv);
                }
            }
            (_, 2) => {
                moves.push(Direction::from_dy(dy).unwrap());
                if let Some(mv) = Direction::from_dx(dx) {
                    moves.push(mv);
                }
            }
            _ => (),
        }

        moves
    }

    /// Gets the `Direction` a cell should moved to based on the `dx`
    fn from_dx(dx: i32) -> Option<Direction> {
        match dx.signum() {
            1 => Some(Direction::Right),
            -1 => Some(Direction::Left),
            _ => None,
        }
    }

    /// Gets the `Direction` a cell should moved to based on the `dy`
    fn from_dy(dy: i32) -> Option<Direction> {
        match dy.signum() {
            1 => Some(Direction::Up),
            -1 => Some(Direction::Down),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_1() {
        let input = read_example(9);
        let solution = solve_part_1(&input).unwrap();

        assert_eq!(solution, 13);
    }

    #[test]
    fn test_parse_move() {
        let input = read_example(9);
        let moves = Move::from(&input);

        assert_eq!(moves.len(), 8);
        assert_eq!(moves[1].0, Direction::Up);
        assert_eq!(moves[3].1, 1);
    }

    #[test]
    fn test_exec_move() {
        let mut grid = Grid::new(2);
        let mv = Move(Direction::Right, 4);
        grid.exec(&mv);

        assert_eq!(grid.nodes[0].0, 4);
        assert_eq!(grid.nodes[0].1, 0);
        assert_eq!(grid.nodes[1].0, 3);
        assert_eq!(grid.nodes[1].1, 0);
    }

    #[test]
    fn test_move_head() {
        let mut grid = Grid::new(2);
        grid.move_node(0, &Direction::Right);
        grid.move_node(0, &Direction::Right);
        grid.move_node(0, &Direction::Down);

        assert_eq!(grid.nodes[0].0, 2);
        assert_eq!(grid.nodes[0].1, -1);
    }

    #[test]
    fn test_reconcile_tail() {
        let mut grid = Grid::new(2);
        grid.move_node(0, &Direction::Left);
        grid.move_node(0, &Direction::Left);
        grid.move_node(0, &Direction::Down);
        grid.reconcile_node(1);

        assert_eq!(grid.nodes[1].0, -1);
        assert_eq!(grid.nodes[1].1, -1);
    }
}
