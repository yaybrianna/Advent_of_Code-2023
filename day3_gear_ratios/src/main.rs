use regex::Regex;
use std::fs;

fn main() {
    let file_path = "input.txt";
    //let file_path = "test_case.txt";
    let data = load_input(&file_path);
    let vec_2d_data = convert_input_to_2d_vect(data);
    let sym_positions = get_all_symbol_positions(&vec_2d_data);
    let parts = get_all_parts(&vec_2d_data);
    let valid_parts: Vec<Part> = parts
        .into_iter()
        .filter(|p| is_valid_part(p, &sym_positions))
        .collect();

    let mut part_num_sum = 0;

    for valid_part in valid_parts {
        part_num_sum += valid_part.value;
    }
    println!("Part Sum: {}", part_num_sum);
}

fn load_input(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Unable to read file");
}

fn convert_input_to_2d_vect(input: String) -> Vec<Vec<char>> {
    let mut vec_rows: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        let mut vec_col: Vec<char> = vec![];
        for chr in line.chars() {
            vec_col.push(chr);
        }
        vec_rows.push(vec_col);
    }
    return vec_rows;
}

fn get_all_symbol_positions(input_vec_2d: &Vec<Vec<char>>) -> Vec<Position> {
    let mut row_count = 0;
    let mut sym_positions: Vec<Position> = vec![];
    for row in input_vec_2d {
        let mut col_count = 0;
        for col in row {
            if !col.is_alphanumeric() && col != &'.' {
                sym_positions.push(Position {
                    row: row_count,
                    col: col_count,
                })
            }
            col_count += 1;
        }
        row_count += 1;
    }
    return sym_positions;
}

fn get_all_parts(input_vec_2d: &Vec<Vec<char>>) -> Vec<Part> {
    let regex: Regex = Regex::new(r"[0-9]+").unwrap();
    let mut row_count = 0;
    let mut parts: Vec<Part> = vec![];

    for row in input_vec_2d {
        let row_str: String = row.into_iter().collect();
        let caps = regex.find_iter(&row_str);
        for cap in caps {
            let part: Part = Part {
                value: cap.as_str().parse::<u32>().unwrap(),
                size: (cap.end() - cap.start()) as u32,
                position: Position {
                    row: row_count,
                    col: cap.start() as u32,
                },
            };
            parts.push(part);
        }
        row_count += 1;
    }

    return parts;
}

fn is_valid_part(part: &Part, sym_positions: &Vec<Position>) -> bool {
    let row_start = if part.position.row == 0 {
        0
    } else {
        part.position.row - 1
    };
    let col_start = if part.position.col == 0 {
        0
    } else {
        part.position.col - 1
    };
    let row_end = part.position.row + 1;
    let col_end = part.position.col + part.size + 1;
    for row_pos in row_start..(row_end + 1) {
        for col_pos in col_start..(col_end) {
            if sym_positions
                .into_iter()
                .any(|p| p.row == row_pos && p.col == col_pos)
            {
                return true;
            }
        }
    }
    return false;
}

#[derive(Debug)]
struct Position {
    row: u32,
    col: u32,
}
#[derive(Debug)]
struct Part {
    value: u32,
    size: u32,
    position: Position,
}
