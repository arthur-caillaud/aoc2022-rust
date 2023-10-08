use advent_of_code::helpers::*;
use advent_of_code::solve;

use petgraph::algo::dijkstra;
use petgraph::graph::DiGraph;
use petgraph::stable_graph::NodeIndex;
use petgraph::IntoWeightedEdge;

fn main() {
    let input = &read_input(12);
    solve!(1, solve_part_1, input);
    solve!(2, solve_part_2, input);
}

fn solve_part_2(input: &str) -> Option<usize> {
    let mountains_bag = MountainsBag::parse(&input);
    let mountains = Mountains::from(&mountains_bag);
    let starts = mountains_bag.find_lowest();
    let end = mountains_bag.find_end().unwrap();

    let path = starts
        .iter()
        .map(|start| mountains.path_length(start.clone(), end))
        .min();

    path
}

fn solve_part_1(input: &str) -> Option<usize> {
    let mountains_bag = MountainsBag::parse(&input);
    let mountains = Mountains::from(&mountains_bag);
    let start = mountains_bag.find_start().unwrap();
    let end = mountains_bag.find_end().unwrap();
    let path = mountains.path_length(start.clone(), end.clone());

    Some(path)
}

#[derive(Debug)]
struct Mountains {
    graph: DiGraph<(), Path>,
}
impl Mountains {
    fn path_length(&self, from: Position, to: Position) -> usize {
        let res = dijkstra(&self.graph, from.into(), Some(to.into()), |_| 1 as usize);

        res.get(&to.into()).unwrap_or(&usize::MAX).clone()
    }

    fn from(mountains: &MountainsBag) -> Self {
        let paths = mountains.get_all_paths();
        let graph = DiGraph::<(), Path>::from_edges(paths);

        Self { graph }
    }
}

#[derive(Debug)]
struct MountainsBag(Vec<Vec<Mountain>>);
impl MountainsBag {
    fn parse(s: &str) -> Self {
        Self(
            s.lines()
                .map(|line| line.chars().map(Mountain::parse).collect())
                .collect(),
        )
    }

    fn get(&self, pos: Position) -> &Mountain {
        &self.0[pos.0][pos.1]
    }

    fn find_lowest(&self) -> Vec<Position> {
        let mut positions = vec![];

        for i in 0..self.0.len() {
            for j in 0..self.0[0].len() {
                if self.get(Position(i, j)).height == 1 {
                    positions.push(Position(i, j))
                }
            }
        }

        positions
    }

    fn find_start(&self) -> Option<Position> {
        for i in 0..self.0.len() {
            for j in 0..self.0[0].len() {
                if self.get(Position(i, j)).start {
                    return Some(Position(i, j));
                }
            }
        }
        None
    }

    fn find_end(&self) -> Option<Position> {
        for i in 0..self.0.len() {
            for j in 0..self.0[0].len() {
                if self.get(Position(i, j)).end {
                    return Some(Position(i, j));
                }
            }
        }
        None
    }

    fn get_all_paths(&self) -> Vec<Path> {
        let mut paths = vec![];
        let height = self.0.len();
        let width = self.0[0].len();
        for i in 0..height {
            for j in 0..width {
                let mut neighbors: Vec<Position> = vec![];
                if i > 0 {
                    neighbors.push(Position(i - 1, j)) // Up neighbor
                }
                if i < height - 1 {
                    neighbors.push(Position(i + 1, j)) // Down neighbor
                }
                if j > 0 {
                    neighbors.push(Position(i, j - 1)) // Left neighbor
                }
                if j < width - 1 {
                    neighbors.push(Position(i, j + 1)) // Right neighbor
                }
                neighbors.iter().for_each(|neighbor| {
                    if self.is_possible_path(&Position(i, j), &neighbor) {
                        paths.push(Path(Position(i, j), neighbor.clone()));
                    }
                });
            }
        }

        paths
    }

    fn is_possible_path(&self, from: &Position, to: &Position) -> bool {
        let mountain_from = &self.0[from.0][from.1];
        let mountain_to = &self.0[to.0][to.1];

        mountain_from.height() >= mountain_to.height() - 1
    }
}

#[derive(Debug, Clone, Copy)]
struct Path(Position, Position);
impl IntoWeightedEdge<Path> for Path {
    type NodeId = NodeIndex;
    fn into_weighted_edge(self) -> (Self::NodeId, Self::NodeId, Self) {
        (self.0.into(), self.1.into(), self)
    }
}

#[derive(Debug, Clone, Copy)]
struct Position(usize, usize);
impl Into<NodeIndex> for Position {
    fn into(self) -> NodeIndex {
        let Self(x, y) = self;

        NodeIndex::new(((x + y) * (x + y + 1)) / 2 + y)
    }
}

#[derive(Debug)]
struct Mountain {
    height: usize,
    start: bool,
    end: bool,
}
impl Mountain {
    fn height(&self) -> usize {
        self.height
    }

    fn parse(c: char) -> Self {
        let alphabet = "abcdefghijklmnopqrstuvwxyz".chars();
        match alphabet.enumerate().find(|(_, char)| c == *char) {
            Some((elevation, _)) => Self {
                height: elevation + 1,
                start: false,
                end: false,
            },
            None => {
                if c == 'S' {
                    Self {
                        height: 1,
                        start: true,
                        end: false,
                    }
                } else {
                    // This is "E"
                    Self {
                        height: 26,
                        start: false,
                        end: true,
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mountains() {
        let example = read_example(12);
        let mountains = MountainsBag::parse(&example);

        assert_eq!(mountains.get(Position(0, 0)).height(), 1);
        assert_eq!(mountains.get(Position(4, 2)).height(), 4);
    }

    #[test]
    fn test_get_all_paths() {
        let example = read_example(12);
        let mountains_bag = MountainsBag::parse(&example);

        assert_eq!(mountains_bag.get_all_paths().len(), 111)
    }

    #[test]
    fn test_mountains_graph() {
        let example = read_example(12);
        let mountains_bag = MountainsBag::parse(&example);
        let mountains = Mountains::from(&mountains_bag);

        assert_eq!(mountains.graph.edge_count(), 111)
    }

    #[test]
    fn test_path() {
        let example = read_example(12);
        let mountains_bag = MountainsBag::parse(&example);
        let mountains = Mountains::from(&mountains_bag);

        let path_length_1 = mountains.path_length(Position(0, 0), Position(1, 0));
        assert_eq!(path_length_1, 1);

        let path_length_3 = mountains.path_length(Position(0, 0), Position(3, 2));
        assert_eq!(path_length_3, 5);
    }

    #[test]
    fn test_solve_part_1() {
        let example = read_example(12);
        let solution = solve_part_1(&example).unwrap();

        assert_eq!(solution, 31);
    }

    #[test]
    fn test_solve_part_2() {
        let example = read_example(12);
        let solution = solve_part_2(&example).unwrap();

        assert_eq!(solution, 29);
    }
}
