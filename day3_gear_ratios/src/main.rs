use std::fs;

fn main() {
    //let file_path = "input.txt";
    let file_path = "test_case.txt";
    let data = load_input(&file_path);
    let vec_2d_data = convert_input_to_2d_vect(data);
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

fn get_all_neighboring_numbers(positions: &Vec<Position>, input_vec_2d: &Vec<Vec<char>>) {
    let mut neighboring_number_positions: Vec<Position> = vec![];
    let row_max_pos = (input_vec_2d.len() as u32) - 1;
    let col_max_pos = (input_vec_2d[0].len() as u32) - 1;
    for pos in positions {
        let is_top_edge = pos.row == 0;
        let is_left_edge = pos.col == 0;
        let is_bottom_edge = pos.row == row_max_pos;
        let is_right_edge = pos.col == col_max_pos;

        
    }
}

struct Position {
    row: u32,
    col: u32,
}
