use std::fs;
mod game;
use game::*;

static RED_MAX_COUNT: u32 = 12;
static GREEN_MAX_COUNT: u32 = 13;
static BLUE_MAX_COUNT: u32 = 14;

fn main() {
    let file_path = "input.txt";
    //let file_path = "test_case.txt";
    let data = load_input(&file_path);
    let games: Vec<Game> = game::parse_input_to_games(data);
    let valid_games = get_valid_games(&games);

    let mut id_sum = 0;
    let mut power_sum: u32 = 0;
    for game in &games {
        let min_set_power = get_game_power_needed(&game);
        println!("Game ID: {}, Min Set Power: {}", game.id, min_set_power);
        power_sum += min_set_power;
    }
    for valid_game in valid_games {
        println!("Valid Game ID: {}", valid_game.id);
        id_sum += valid_game.id
    }
    println!("ID Sumation: {}, ", id_sum);
    println!("Min set power Sumation: {}", power_sum)
}

fn load_input(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Unable to read file");
}

fn get_valid_games(games: &Vec<Game>) -> Vec<&Game> {
    games
        .into_iter()
        .filter(|&g| is_valid_game(g))
        .collect::<Vec<&Game>>()
}

fn is_valid_game(game: &Game) -> bool {
    for set in &game.sets {
        if !is_valid_set(&set) {
            return false;
        }
    }

    return true;
}

fn is_valid_set(set: &Set) -> bool {
    is_die_count_within_limit(set)
}

fn is_die_count_within_limit(set: &Set) -> bool {
    return set.blue_die_count <= BLUE_MAX_COUNT
        && set.red_die_count <= RED_MAX_COUNT
        && set.green_die_count <= GREEN_MAX_COUNT;
}

fn get_game_power_needed(game: &Game) -> u32 {
    let mut power_needed = 0;
    for set in &game.sets {
        power_needed = if set.power > power_needed {
            set.power
        } else {
            power_needed
        }
    }
    return power_needed;
}
