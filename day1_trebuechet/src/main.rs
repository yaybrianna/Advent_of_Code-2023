use std::{fs, char::from_digit};
use regex::Regex;

fn main() {
    //let file_path = "input.txt";
    let file_path = "test_case.txt";
    let data = load_input(&file_path);
    let mut calibration_code: u32 = 0;
    for line in data.lines() {
        calibration_code += u32::from(get_calibration_number_from_line(line));
    }
    print!("Calibration Code: {}", calibration_code);
}

fn load_input(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Unable to read file");
}


fn get_calibration_number_from_line(line: &str) -> u32 {
    let mut number_as_string: String = String::from("");
    let mut number_vec: Vec<u32> = Vec::new();
    
    let mut answer_vec: Vec<u32> = vec![number_vec[0], number_vec[number_vec.len() - 1]];
    let line_answer: String = answer_vec.into_iter().map(|d|from_digit(d, 10).unwrap()).collect();
    println!("Line Number: {}", line_answer);
    return line_answer.parse::<u32>().unwrap();
}

fn get_all_possible_numbers_from_line(line: &str) -> Vec<u32>{
    let mut temp_string = String::from("");
    let mut mut_line = String::from(line);


}

static NUMBER_REGEX: &str = "zero|one|two|three|four|five|six|seven|eight|nine|[0-9]";
