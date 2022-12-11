use std::collections::HashSet;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn run(path: &str, marker_len: usize) -> usize {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let s = reader.lines().last().unwrap().unwrap();
    let mut cnt = 0;
    for (i, _) in s.chars().enumerate() {
        let cur_chunk = &s[i..i + marker_len];
        let hs: HashSet<char> = cur_chunk.chars().collect();
        if hs.len() == marker_len {
            break;
        }
        cnt += 1;
    }

    cnt + marker_len
}

pub fn part_a() {
    println!("Part A: {}", run("inputs/day06/input.txt", 4));
}

pub fn part_b() {
    println!("Part B: {}", run("inputs/day06/input.txt", 14));
}
