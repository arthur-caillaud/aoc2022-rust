use std::fs;

/// Returns the `String` from file "inputs/{day}.txt"
pub fn read_input(day: u64) -> String {
    let input_path = get_input_path(day);
    let s = fs::read_to_string(input_path);
    s.unwrap()
}

/// Returns the `String` from file "examples/{day}.txt"
pub fn read_example(day: u64) -> String {
    let input_path = get_examples_path(day);
    let s = fs::read_to_string(input_path);
    s.unwrap()
}

/// Gets the path for a file in "examples" folder
fn get_examples_path(day: u64) -> String {
    get_path("examples", day)
}

/// Gets the path for a file in "inputs" folder
fn get_input_path(day: u64) -> String {
    get_path("inputs", day)
}

/// Gets the path for a file in one of our txt folders
fn get_path(folder: &str, day: u64) -> String {
    let day_padded = format!("{day:02}");
    format!("src/{folder}/{day_padded}.txt")
}
