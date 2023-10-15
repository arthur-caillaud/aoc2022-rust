use std::env::current_dir;
use std::fs::read_to_string as read_file;
use std::path::PathBuf;

/// Returns the `String` from file "inputs/{day}.txt"
pub fn read_input(day: u64) -> String {
    let input_path: PathBuf = get_input_path(day);
    let s = read_file(input_path);
    s.unwrap()
}

/// Returns the `String` from file "examples/{day}.txt"
pub fn read_example(day: u64) -> String {
    let input_path = get_examples_path(day);
    let s = read_file(input_path);
    s.unwrap()
}

/// Gets the path for a file in "examples" folder
fn get_examples_path(day: u64) -> PathBuf {
    get_path("examples", day)
}

/// Gets the path for a file in "inputs" folder
fn get_input_path(day: u64) -> PathBuf {
    get_path("inputs", day)
}

/// Gets the path for a file in one of our txt folders
fn get_path(folder: &str, day: u64) -> PathBuf {
    let cwd = current_dir().unwrap();
    cwd.join("src").join(folder).join(format!("{day:02}.txt"))
}
