use advent_of_code::helpers::*;
use advent_of_code::solve;

fn main() {
    let input = &read_input(10);
    solve!(1, solve_part_1, input);
}

fn solve_part_1(input: &str) -> Option<i32> {
    let commands = Command::from(input);
    let mut cpu = CPU::new();
    cpu.exec(commands);
    let solution = cpu.get_strength();

    Some(solution)
}

#[derive(Debug)]
struct CPU {
    x: Vec<i32>,
}
impl CPU {
    /// Builds a new CPU with `1` set in register `x`
    fn new() -> Self {
        Self { x: vec![1] }
    }

    /// Retrieves total strength by computing strength
    fn get_strength(&self) -> i32 {
        let strengths = [20, 60, 100, 140, 180, 220]
            .map(|k| self.get_strength_at(k))
            .to_vec();
        strengths.iter().sum()
    }

    /// Retrieves the signal strength during cycle `n`
    fn get_strength_at(&self, n: usize) -> i32 {
        self.get_x_at(n) * (n as i32)
    }

    /// Retrieves the value of register `x` during cycle `n`
    fn get_x_at(&self, n: usize) -> &i32 {
        &self.x.get(n - 1).unwrap()
    }

    /// Executes the provided list of `Commands`
    fn exec(&mut self, commands: Vec<Command>) {
        commands.iter().for_each(|cmd| self.exec_one(cmd));
    }

    /// Executes the provided `Command`
    fn exec_one(&mut self, command: &Command) {
        match command {
            Command::Noop => self.exec_noop(),
            Command::Addx(dx) => self.exec_addx(&dx),
        }
    }

    /// Executes a `noop` command
    fn exec_noop(&mut self) {
        let curr_x = self.x.last().unwrap();
        self.x.push(curr_x.clone())
    }

    /// Executes an `addx` command
    fn exec_addx(&mut self, dx: &i32) {
        self.exec_noop();
        let curr_x = self.x.last().unwrap();
        self.x.push(curr_x + dx);
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Addx(i32),
    Noop,
}
impl Command {
    fn from(s: &str) -> Vec<Self> {
        s.lines().map(Command::from_line).collect()
    }

    fn from_line(s: &str) -> Self {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        match &tokens[..] {
            ["noop"] => Command::Noop,
            ["addx", x] => Command::Addx(x.parse::<i32>().unwrap()),
            _ => panic!("Unknown instruction {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instructions() {
        let input = read_example(10);
        let commands = Command::from(&input);

        assert_eq!(commands[0], Command::Addx(15));
        assert_eq!(commands[9], Command::Noop);
    }

    #[test]
    fn test_signal_strength() {
        let input = read_example(10);
        let commands = Command::from(&input);
        let mut cpu = CPU::new();
        cpu.exec(commands);

        assert_eq!(cpu.get_strength_at(220), 3960);
    }

    #[test]
    fn test_exec() {
        let input = read_example(10);
        let commands = Command::from(&input);
        let mut cpu = CPU::new();
        cpu.exec(commands);

        assert_eq!(cpu.get_x_at(220), &18);
    }

    #[test]
    fn test_solve_1() {
        let input = read_example(10);
        let solution = solve_part_1(&input).unwrap();

        assert_eq!(solution, 13140)
    }
}
