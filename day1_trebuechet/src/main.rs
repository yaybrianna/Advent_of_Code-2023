use regex::Regex;
use std::fs;

fn main() {
    let file_path = "input.txt";
    //let file_path = "test_case.txt";
    let data = load_input(&file_path);
    let mut calibration_code: u32 = 0;
    let mut count: u32 = 0;
    for line in data.lines() {
        count += 1;
        let number = get_calibration_number_from_line(line);
        println!("Line {}: {}", count, number);
        calibration_code += number;
    }
    println!("Calibration Code: {}", calibration_code);
}

fn load_input(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Unable to read file");
}

fn get_calibration_number_from_line(line: &str) -> u32 {
    let number_vec: Vec<u32> = get_all_possible_numbers_from_line(line);
    let line_answer: String = vec![
        number_vec[0].to_string(),
        number_vec[number_vec.len() - 1].to_string(),
    ]
    .join("")
    .to_string();
    return line_answer.parse::<u32>().unwrap();
}

fn get_all_possible_numbers_from_line(line: &str) -> Vec<u32> {
    let regex = Regex::new(r"(zero|one|two|three|four|five|six|seven|eight|nine|[0-9])").unwrap();
    let mut return_vec: Vec<u32> = vec![];
    let mut matches = vec![];
    for (index, _char) in line.chars().enumerate() {
        let tmp_line = line.split_at(index).1;
        matches.push(regex.find(tmp_line));
    }
    for match_str in matches.iter().filter(|m| m.is_some()).map(|m| m.unwrap().as_str()) {
        if match_str.parse::<u32>().is_ok() {
            return_vec.push(match_str.parse::<u32>().unwrap());
        } else {
            let number = NUMBERS.iter().position(|&n| n.contains(match_str)).unwrap() as u32;
            return_vec.push(number);
        }
    }

    return return_vec;
}

static NUMBERS: &'static [&str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
