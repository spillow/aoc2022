use itertools::Itertools;
use std::collections::HashSet;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn mk_hash(s: &str) -> HashSet<char> {
    s.chars().collect()
}

fn get_priority(c: char) -> i32 {
    match c {
        'a'..='z' => c as i32 - 'a' as i32 + 1,
        'A'..='Z' => c as i32 - 'A' as i32 + 1 + 26,
        _ => panic!("bad input!"),
    }
}

pub fn part_a() {
    let file = File::open("inputs/day03/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let comp_size = line.len() / 2;
        let (fst, snd) = (mk_hash(&line[..comp_size]), mk_hash(&line[comp_size..]));
        let dup = *fst.intersection(&snd).last().unwrap();
        total += get_priority(dup);
    }

    println!("Part A: sum of priorities is {}", total);
}

pub fn part_b() {
    let file = File::open("inputs/day03/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;
    for lines in &reader.lines().chunks(3) {
        let set = lines
            .map(|line| mk_hash(&line.unwrap()))
            .reduce(|acc, h| acc.intersection(&h).cloned().collect())
            .unwrap();
        let dup = *set.iter().last().unwrap();
        total += get_priority(dup);
    }

    println!("Part B 2: sum of group priorities is {}", total);
}
