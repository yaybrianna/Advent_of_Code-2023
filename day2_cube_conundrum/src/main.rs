use std::fs;
fn main() {
    let file_path = "input.txt";
    //let file_path = "test_case.txt";
    let data = load_input(&file_path);
}

fn load_input(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Unable to read file");
}