use std::fs;

fn main() {
    let file_path = "input.txt";
    let data = load_input(&file_path);
    let mut calibration_code: u32 = 0;
    for line in data.lines(){
        calibration_code += u32::from(get_number_from_line(line));
    }
    print!("Calibration Code: {}", calibration_code);
    
}

fn load_input(file_path: &str) -> String{
    return fs::read_to_string(file_path).expect("Unable to read file");
}

fn get_number_from_line(line: &str) -> u8 {
    let mut number_string  = String::from("");
    for char in line.chars() {
        if char.is_digit(10) {
            number_string.push(char);
            break;
        }
    }
    for char in line.chars().rev() {
        if char.is_digit(10) {
            number_string.push(char);
            break;
        }
    }
    println!("Line Number: {}", number_string);
    return number_string.parse::<u8>().unwrap();
}