pub fn parse_input_to_games(file: String) -> Vec<Game> {
    let mut games: Vec<Game> = vec![];
    for line in file.lines() {
        games.push(parse_game_from_line(line))
    }
    return games;
}

pub fn parse_game_from_line(line: &str) -> Game {
    let game_vec: Vec<&str> = line.split(": ").map(|s| s).collect();
    let game_string = game_vec[0];
    let set_strings: Vec<&str> = game_vec[1].split("; ").map(|s| s).collect();
    let sets_vec = parse_sets_from_set_strings(set_strings);
    let game = Game {
        id: game_string.split(" ").map(|s| s).collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .unwrap(),
        minimum_set_power: get_minimum_set_power(&sets_vec),
        sets: sets_vec
    };
    return game;
}

pub fn get_minimum_set_power(sets: &Vec<Set>) -> u32{
    let mut maximum_blue: u32 = 0;
    let mut maximum_red: u32 = 0;
    let mut maximum_green: u32 = 0;

    for set in sets {
        maximum_blue = if set.blue_die_count > maximum_blue {
            set.blue_die_count
        }else{
            maximum_blue
        };
        maximum_red = if set.red_die_count > maximum_red {
            set.red_die_count
        }else{
            maximum_red
        };
        maximum_green = if set.green_die_count > maximum_green {
            set.green_die_count
        }else{
             maximum_green
        };
    }

    return maximum_blue * maximum_green * maximum_red
}

pub fn parse_sets_from_set_strings(set_strings: Vec<&str>) -> Vec<Set> {
    let mut sets: Vec<Set> = vec![];
    for set in set_strings {
        sets.push(parse_set_from_set_string(set))
    }
    return sets;
}

pub fn parse_set_from_set_string(set_string: &str) -> Set {
    let color_strings: Vec<&str> = set_string.split(", ").map(|s| s).collect();
    let blue_string = color_strings
        .iter()
        .find(|s| s.contains("blue"))
        .unwrap_or(&"");
    let red_string = color_strings
        .iter()
        .find(|s| s.contains("red"))
        .unwrap_or(&"");
    let green_string = color_strings
        .iter()
        .find(|s| s.contains("green"))
        .unwrap_or(&"");
    let set: Set = Set {
        green_die_count: green_string.split(" ").collect::<Vec<&str>>()[0]
            .parse::<u32>()
            .unwrap_or(0),
        red_die_count: red_string.split(" ").collect::<Vec<&str>>()[0]
            .parse::<u32>()
            .unwrap_or(0),
        blue_die_count: blue_string.split(" ").collect::<Vec<&str>>()[0]
            .parse::<u32>()
            .unwrap_or(0),
    };
    return set;
}


pub struct Game {
    pub id: u32,
    pub sets: Vec<Set>,
    pub minimum_set_power: u32
}

pub struct Set {
    pub green_die_count: u32,
    pub red_die_count: u32,
    pub blue_die_count: u32,
}
