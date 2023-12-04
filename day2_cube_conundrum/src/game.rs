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
    let game = Game {
        id: game_string.split(" ").map(|s| s).collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .unwrap(),
        sets: parse_sets_from_set_strings(set_strings),
    };
    return game;
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
    let mut set: Set = Set {
        green_die_count: green_string.split(" ").collect::<Vec<&str>>()[0]
            .parse::<u32>()
            .unwrap_or(0),
        red_die_count: red_string.split(" ").collect::<Vec<&str>>()[0]
            .parse::<u32>()
            .unwrap_or(0),
        blue_die_count: blue_string.split(" ").collect::<Vec<&str>>()[0]
            .parse::<u32>()
            .unwrap_or(0),
        power: 0,
    };
    set.power = parse_set_power(&set);
    return set;
}

fn parse_set_power(set: &Set) -> u32{
    let  blue_count = if set.blue_die_count == 0 {
        1
    }else{
        set.blue_die_count
    };
    let red_count = if set.red_die_count == 0 {
        1
    }else{
        set.red_die_count
    };
    let  green_count = if set.green_die_count == 0 {
        1
    }else{
        set.green_die_count
    };
    


    return blue_count * red_count * green_count;
}

pub struct Game {
    pub id: u32,
    pub sets: Vec<Set>,
}

pub struct Set {
    pub green_die_count: u32,
    pub red_die_count: u32,
    pub blue_die_count: u32,
    pub power: u32,
}
