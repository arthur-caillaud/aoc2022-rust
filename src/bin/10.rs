use advent_of_code::helpers::*;
use advent_of_code::solve;

fn main() {
    let input = &read_input(10);
    solve!(1, solve_part_1, input);
    solve!(2, solve_part_2, input);
}

fn solve_part_1(input: &str) -> Option<i32> {
    let commands = Command::from(input);
    let mut cpu = CPU::new();
    cpu.exec(commands);
    let solution = cpu.get_strength();

    Some(solution)
}

fn solve_part_2(input: &str) -> Option<String> {
    let commands = Command::from(input);
    let mut cpu = CPU::new();
    cpu.exec(commands);
    let screen = Screen::from(cpu);
    let solution = screen.print();

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

struct Screen(Vec<Vec<Pixel>>);
impl Screen {
    /// Builds a `Screen` from the values
    fn from(cpu: CPU) -> Self {
        let mut screen: Self = Screen(vec![]);
        cpu.x.iter().enumerate().for_each(|(cycle, &x)| {
            let position = Pixel::get_position(cycle + 1);
            let pixel = Pixel::from(cycle + 1, x);
            if let Some(row) = screen.0.get_mut(position.0) {
                row.push(pixel);
            } else {
                screen.0.push(vec![pixel]);
            }
        });

        screen
    }

    /// Prints the `Screen` by outputing a `String` containing all printed `Pixels`
    fn print(&self) -> String {
        let mut image = String::from("");
        self.0.iter().for_each(|row| {
            row.iter().for_each(|pixel| {
                image.push(pixel.print());
            });
            image.push('\n');
        });

        image
    }
}

enum Pixel {
    Lit,
    Dark,
}
impl Pixel {
    /// Builds a new pixel from the CPU `cycle` value and `x` register value
    fn from(cycle: usize, x: i32) -> Self {
        let column = Pixel::get_position(cycle).1 as i32;
        let distance = column - x;
        match distance.abs() {
            0 | 1 => Self::Lit,
            _ => Self::Dark,
        }
    }

    /// Prints the `Pixel` by converting it into a `char`
    fn print(&self) -> char {
        match self {
            Pixel::Dark => '.',
            Pixel::Lit => '#',
        }
    }

    /// Retrieves the position a `Pixel` from the CPU `cycle`
    fn get_position(cycle: usize) -> (usize, usize) {
        let column = (cycle - 1) % 40;
        let row = (cycle - 1) / 40;

        (row, column)
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

    #[test]
    fn test_print_pixel() {
        let pixel1 = Pixel::from(8, 11);
        assert_eq!(pixel1.print(), '.');

        let pixel2 = Pixel::from(13, 12);
        assert_eq!(pixel2.print(), '#');
    }

    #[test]
    fn test_solve_2() {
        let input = read_example(10);
        let solution = solve_part_2(&input).unwrap();

        println!("{}", solution)
    }
}
