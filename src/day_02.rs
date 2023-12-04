use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::convert_file_to_vec;

pub fn day_two_part_one_calc() {
    let contents: Vec<String> = convert_file_to_vec("./src/files/day_02.txt".to_string());

    let max_values: HashMap<&str, i32> = HashMap::from([
        ("blue", 14),
        ("green", 13),
        ("red", 12)
    ]);

    let mut valid_games: HashMap<i32, bool> = HashMap::new();
    
    contents.iter().for_each(|row: &String| {
        let initial_split: Vec<&str> = row.split(":").collect_vec();
        let game_number: i32 = initial_split[0].split(" ").collect_vec()[1].parse::<i32>().expect("To collect the game number");
        let colors: String = initial_split[1].split(|string| string == ';' || string == ',').collect::<String>();

        let mut temp_num: i32 = 0;
        for color in colors.trim().split(" ") {
            if color.parse::<i32>().is_ok() {
                // convert our character to an int
                let c_num: i32 = color.parse::<i32>().expect("Our char to parse to an int successfully");
                // store the int for comparison in our next loop
                temp_num = c_num;
                // store our game number in our hashmap 
                valid_games.insert(game_number, true);
            } else {
                // check to see if our temp num exceeds the number of available cubes for that color
                if temp_num > *max_values.get(color).unwrap() {
                    // if it does, delete the game number from our hashmap
                    valid_games.remove(&game_number);
                    // break the current loop as this game should now be over
                    break
                }
            }
        }
    });

    let total: i32 = valid_games.keys().sum();
    print!("day 2.1 answer: {:?}\n", total);
}

pub fn day_two_part_two_calc() {
    let contents: Vec<String> = convert_file_to_vec("./src/files/day_02.txt".to_string());

    let mut powers: Vec<i32> = Vec::new();
    
    contents.iter().for_each(|row: &String| {
        let mut min_values: HashMap<&str, i32> = HashMap::new();
        let initial_split: Vec<&str> = row.split(":").collect_vec();
        let colors: String = initial_split[1].split(|string| string == ';' || string == ',').collect::<String>();

        let mut temp_num: i32 = 0;
        for color in colors.trim().split(" ") {
            if color.parse::<i32>().is_ok() {
                // convert & store the int for comparison in our next loop
                temp_num = color.parse::<i32>().expect("Our char to parse to an int successfully");
            } else {
                // check to see if the key already exists
                if min_values.contains_key(color) {
                    // if so, is the temp number higher than the one we have stored?
                    if temp_num > *min_values.get(&color).unwrap() {
                        // if so, replace it
                        min_values.insert(color, temp_num);
                    }
                    // if not, move on to the next color
                    continue;
                }
                // if it doesn't exist, add a new record for the color
                min_values.insert(color, temp_num);
            }
        }

        // multiply all the numbers collected in the current game. 'fold' is the same as a 'reduce' fn
        let game_power: i32 = min_values.values().into_iter().fold(1,|a, b| a * b);
        // store the number in an array
        powers.push(game_power);
    });

    let total: i32 = powers.iter().sum();
    print!("day 2.2 answer: {:?}\n", total);
}
