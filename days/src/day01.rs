use std::io::{BufRead};
use std::{fs::File, io::BufReader};

fn get_calorie_list(path: &str) -> Vec<i32> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut calories = Vec::new();
    let mut curr_sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim() == "" {
            calories.push(curr_sum);
            curr_sum = 0;
            continue
        }
        let num: i32 = line.parse().unwrap();
        curr_sum += num;
    }
    calories
}

pub fn part_a() {
    let cals = get_calorie_list("inputs/day01/input.txt");
    println!("Part A: Max calories is {}", cals.iter().max().unwrap());
}

pub fn part_b() {
    let mut cals = get_calorie_list("inputs/day01/input.txt");
    cals.sort_by(|a, b| b.cmp(a));
    let top_three = cals[0] + cals[1] + cals[2];
    println!("Part B: Sum of top three calories is {}", top_three);
}
