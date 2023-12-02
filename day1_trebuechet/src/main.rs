use regex::Regex;
use std::fs;

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

    let number_vec: Vec<u32> = get_all_possible_numbers_from_line(line);
    let answer_vec: Vec<String> = vec![number_vec[0].to_string(), number_vec[number_vec.len() - 1].to_string()];
    let line_answer = answer_vec.join("");

    println!("Line Number: {}", line_answer);
    return line_answer.parse::<u32>().unwrap();
}

fn get_all_possible_numbers_from_line(line: &str) -> Vec<u32> {
    let regex = Regex::new("(zero|one|two|three|four|five|six|seven|eight|nine|[0-9])").unwrap();
    let mut return_vec: Vec<u32> = vec![];    
    for (_, [match_str]) in regex.captures_iter(line).map(|m| m.extract()) {
        if match_str.parse::<u32>().is_ok() {
            return_vec.push(match_str.parse::<u32>().unwrap());
        }else {
            let number =  NUMBERS.iter().position(|&n| n.contains(match_str)).unwrap() as u32;
            return_vec.push(number);
        }
    }
    return return_vec;
}


static NUMBERS: &'static [&str] = &["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
