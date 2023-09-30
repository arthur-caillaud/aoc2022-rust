use advent_of_code::helpers::*;
use advent_of_code::solve;
use regex::Regex;

fn main() {
    let input = &read_input(7);
    solve!(1, solve_part_1, input);
}

fn solve_part_1(input: &str) -> Option<u64> {
    None
}

#[derive(PartialEq, Eq, Debug)]
enum FsCommand {
    LS(String),
    CD(String),
}

impl FsCommand {
    fn exec<'a>(&self, target: &'a mut FsFolder) -> Option<&'a FsFolder> {
        match self {
            FsCommand::CD(folder_name) => match target.find(&folder_name) {
                Some(FsEntry::Folder(folder)) => Some(folder),
                _ => None,
            },
            FsCommand::LS(output) => {
                target.append_ls_output(&output);
                None
            }
        }
    }

    fn parse(input: &str) -> Vec<Self> {
        let mut commands = vec![];

        let mut lines = input.lines().peekable();

        while let Some(line) = lines.peek().cloned() {
            if line.starts_with("$ cd ") {
                let folder_name = line
                    .split_whitespace()
                    .nth(2)
                    .unwrap_or_default()
                    .to_string();
                commands.push(FsCommand::CD(folder_name));
                // Consume the line we've just processed
                lines.next();
            } else if line.starts_with("$ ls") {
                // consume the "$ ls" line
                lines.next();

                let mut output = String::new();
                while lines.peek().is_some() && !lines.peek().unwrap().starts_with("$") {
                    output += lines.next().unwrap();
                }

                commands.push(FsCommand::LS(output));
            } else {
                // It's neither an "$ ls" nor a "$ cd" line, so we'll skip it.
                lines.next();
            }
        }

        commands
    }
}

#[derive(Debug)]
enum FsEntry {
    File(FsFile),
    Folder(FsFolder),
}

impl FsEntry {
    fn new(s: &str) -> Self {
        let tokens: Vec<_> = s.split_whitespace().collect();
        let name = tokens.get(1).unwrap().to_string();

        if let Ok(size) = tokens.get(0).unwrap().parse::<u32>() {
            Self::File(FsFile { name, size })
        } else {
            Self::Folder(FsFolder {
                name,
                children: vec![],
            })
        }
    }
}

#[derive(Debug)]
struct FsFile {
    name: String,
    size: u32,
}

#[derive(Debug)]
struct FsFolder {
    name: String,
    children: Vec<FsEntry>,
}

impl FsFolder {
    /// Computes the size of the folder by adding the size of all its inner items
    fn size(&self) -> u32 {
        self.children
            .iter()
            .map(|item| match item {
                FsEntry::File(file) => file.size,
                FsEntry::Folder(folder) => folder.size(),
            })
            .sum()
    }

    /// Appends the output of an `ls` command into the folder
    fn append_ls_output(&mut self, s: &str) {
        s.lines()
            .map(FsEntry::new)
            .for_each(|item| self.children.push(item))
    }

    /// May find an item in the children with the provided `name`
    fn find(&self, name: &str) -> Option<&FsEntry> {
        self.children.iter().find(|child| {
            let child_name = match child {
                FsEntry::File(file) => &file.name,
                FsEntry::Folder(folder) => &folder.name,
            };
            child_name == name
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_fs_command() {
        let s = read_example(7);
        let commands = FsCommand::parse(&s);
        assert_eq!(commands.len(), 10);

        match &commands[0] {
            FsCommand::CD(target) => assert_eq!(target, "/"),
            _ => (),
        };

        match &commands[5] {
            FsCommand::LS(target) => assert_eq!(target, "584 i"),
            _ => (),
        };
    }

    #[test]
    fn test_new_fs_entry() {
        match FsEntry::new("dir d") {
            FsEntry::Folder(folder) => assert_eq!(folder.name, "d"),
            _ => panic!("Expected a folder"),
        }

        match FsEntry::new("14848514 b.txt") {
            FsEntry::File(file) => {
                assert_eq!(file.size, 14848514);
                assert_eq!(file.name, "b.txt");
            }
            _ => panic!("Expected a file"),
        }
    }

    #[test]
    fn test_folder_size() {
        let folder = FsFolder {
            children: vec![],
            name: "/".to_string(),
        };
        assert_eq!(folder.size(), 0)
    }

    #[test]
    fn test_append_fs_output() {
        let mut folder = FsFolder {
            children: vec![],
            name: "/".to_string(),
        };
        folder.append_ls_output("dir a\n14848514 b.txt\n8504156 c.dat\ndir d");
        assert_eq!(folder.size(), 23352670)
    }
}
