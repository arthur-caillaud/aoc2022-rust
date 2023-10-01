use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Lines;

use advent_of_code::helpers::*;
use advent_of_code::solve;

fn main() {
    let input = &read_input(7);
    solve!(1, solve_part_1, input);
}

fn solve_part_1(input: &str) -> Option<u32> {
    let commands = FsCommand::parse(input);
    let mut fs = Fs::new();
    fs.exec_multiple(commands);
    let sum = fs.root.find_small_dirs().iter().map(|dir| dir.size()).sum();

    Some(sum)
}

#[derive(Debug)]
struct Fs {
    current_path: Vec<String>,
    root: Dir,
}

impl Fs {
    /// Makes a new `Fs` with an empty root dir "/"
    fn new() -> Self {
        Self {
            current_path: vec![],
            root: Dir::new(String::from("/")),
        }
    }

    /// Executes multiple `FsCommands` on the filesystem
    fn exec_multiple(&mut self, cmds: Vec<FsCommand>) {
        cmds.iter().for_each(|cmd| self.exec(cmd))
    }

    /// Executes the provided `FsCommand` on the filesystem
    fn exec(&mut self, cmd: &FsCommand) {
        match cmd {
            FsCommand::CD(path) if path == ".." => {
                self.current_path.pop();
            }
            FsCommand::CD(path) if path == "/" => {
                self.current_path.clear();
            }
            FsCommand::CD(path) => {
                self.current_path.push(path.clone());
            }
            FsCommand::LS(ref output) => {
                if let Some(current_dir) = self.get_current() {
                    current_dir.append_ls_output(output);
                } else {
                    self.root.append_ls_output(&output);
                }
            }
        }
    }

    /// Retrieves the directory targeted by `current_path`
    fn get_current(&mut self) -> Option<&mut Dir> {
        let mut current: &mut Dir = &mut self.root;

        for segment in &self.current_path {
            current = if let Some(Node::Dir(dir)) = current.children.get_mut(segment) {
                dir
            } else {
                return None;
            };
        }

        Some(current)
    }
}

#[derive(PartialEq, Eq, Debug)]
enum FsCommand {
    LS(String),
    CD(String),
}

impl FsCommand {
    /// Parses a list of `FsCommand` from an `input` string
    fn parse(input: &str) -> Vec<Self> {
        let mut commands = vec![];

        let mut lines = input.lines().peekable();

        while let Some(line) = lines.peek().cloned() {
            if line.starts_with("$ cd ") {
                let folder_name = FsCommand::parse_cd(line);
                commands.push(FsCommand::CD(folder_name));
                lines.next();
            } else if line.starts_with("$ ls") {
                lines.next();
                let output = FsCommand::parse_ls(lines.clone());
                commands.push(FsCommand::LS(output));
            } else {
                lines.next();
            }
        }

        commands
    }

    /// Parses a `FsCommand::CD` from an input `line`
    fn parse_cd(line: &str) -> String {
        line.split_whitespace()
            .nth(2)
            .unwrap_or_default()
            .to_string()
    }

    /// Parses an `FsCommand:LS` from multiple `lines`
    fn parse_ls(mut lines: Peekable<Lines>) -> String {
        let mut commands = vec![];
        while lines.peek().is_some() && !lines.peek().unwrap().starts_with("$") {
            commands.push(lines.next().unwrap())
        }

        commands.join("\n")
    }
}

#[derive(Debug, Clone)]
enum Node {
    File(File),
    Dir(Dir),
}

impl Node {
    fn new(s: &str) -> Self {
        let tokens: Vec<_> = s.split_whitespace().collect();
        let name = tokens.get(1).unwrap().to_string();

        if let Ok(size) = tokens.get(0).unwrap().parse::<u32>() {
            Self::File(File { name, size })
        } else {
            Self::Dir(Dir::new(name))
        }
    }
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: u32,
}

#[derive(Debug, Clone)]
struct Dir {
    name: String,
    children: HashMap<String, Node>,
}

impl Dir {
    /// Makes a new `Dir` from its `name`
    fn new(name: String) -> Self {
        Self {
            name,
            children: HashMap::new(),
        }
    }

    /// Whether the `Dir` is smaller than 100kB
    fn is_small(&self) -> bool {
        self.size() <= 100000
    }

    /// Computes the size of the folder by adding the size of all its inner items
    fn size(&self) -> u32 {
        self.children
            .iter()
            .map(|(_, item)| match item {
                Node::File(file) => file.size,
                Node::Dir(folder) => folder.size(),
            })
            .sum()
    }

    /// Appends the output of an `ls` command into the folder
    fn append_ls_output(&mut self, s: &str) {
        s.lines().map(Node::new).for_each(|item| {
            match &item {
                Node::Dir(dir) => self.children.insert(String::from(dir.name.clone()), item),
                Node::File(file) => self.children.insert(String::from(file.name.clone()), item),
            };
        })
    }

    /// Return the list of directories (children and self included) smaller than 100kB
    fn find_small_dirs(&self) -> Vec<&Dir> {
        let mut dirs = vec![];

        if self.is_small() {
            dirs.push(self);
        }

        for (_, child) in &self.children {
            if let Node::Dir(dir) = child {
                dirs.extend(dir.find_small_dirs())
            }
        }

        dirs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        let input = read_example(7);
        let solution = solve_part_1(&input).unwrap();
        assert_eq!(solution, 95437)
    }

    #[test]
    fn test_get_current() {
        let mut fs = Fs::new();
        fs.exec(&FsCommand::CD(String::from("/")));
        let current_dir = fs.get_current().unwrap();
        assert_eq!(current_dir.name, "/");
    }

    #[test]
    fn test_build_fs() {
        let input = read_example(7);
        let commands = FsCommand::parse(&input);
        let mut fs = Fs::new();
        fs.exec_multiple(commands);

        assert_eq!(fs.root.children.len(), 4);
    }

    #[test]
    fn test_new_fs_command() {
        let s = read_example(7);
        let commands = FsCommand::parse(&s);
        assert_eq!(commands.len(), 10);

        match &commands[0] {
            FsCommand::CD(target) => assert_eq!(target, "/"),
            _ => panic!("Command 0 should be a `cd`"),
        };

        match &commands[5] {
            FsCommand::LS(target) => assert_eq!(target, "584 i"),
            _ => panic!("Command 5 should be an `ls`"),
        };
    }

    #[test]
    fn test_new_fs_entry() {
        match Node::new("dir d") {
            Node::Dir(folder) => assert_eq!(folder.name, "d"),
            _ => panic!("Expected a folder"),
        }

        match Node::new("14848514 b.txt") {
            Node::File(file) => {
                assert_eq!(file.size, 14848514);
            }
            _ => panic!("Expected a file"),
        }
    }

    #[test]
    fn test_folder_size() {
        let folder = Dir::new(String::from("/"));
        assert_eq!(folder.size(), 0)
    }

    #[test]
    fn test_append_fs_output() {
        let mut folder = Dir::new(String::from("/"));
        folder.append_ls_output("dir a\n14848514 b.txt\n8504156 c.dat\ndir d");
        assert_eq!(folder.size(), 23352670)
    }
}
