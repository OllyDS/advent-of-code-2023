use itertools::Itertools;

use crate::utils::convert_file_to_vec;

pub fn day_one_calc() {
    let mut contents: Vec<String> = convert_file_to_vec("./src/files/day_01.txt".to_string());

    let filter_numerals: Vec<String> = contents
        .iter_mut()
        .map(|el: &mut String| el.chars().filter(|char: &char| char.is_numeric()).collect())
        .collect();

    let collect_nums: Vec<i32> = filter_numerals
        .into_iter()
        .map(|s: String| {
            if s.len() == 1 {
                return format!("{}{}", s, s).parse::<i32>().expect("To parse the duplicated string as a number");
            }
            
            let mut chars: std::str::Chars<'_> = s.chars();
            let first: char = chars.nth(0).expect("it to capture the first number in the vec");
            let last: char = chars.last().expect("it to capture the last number in the vec");
            return format!("{}{}", first, last).parse::<i32>().expect("To merge and parse the chars to a number");
        }).collect_vec();

    let calculate_total: i32 = collect_nums.iter().sum();
    println!("{:?}", calculate_total);
}
