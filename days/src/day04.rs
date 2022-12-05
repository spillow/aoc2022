use itertools::Itertools;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

type Range = (i32, i32);

fn covers(a: Range, b: Range) -> bool {
    b.0 >= a.0 && b.1 <= a.1
}

fn overlap(a: Range, b: Range) -> bool {
    !(a.1 < b.0 || a.0 > b.1)
}

fn parse_range(s: &str) -> Range {
    let (a, b) = s.split('-').next_tuple().unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}

fn count_instances(path: &str, check_fn: fn(Range, Range) -> bool) -> i32 {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut cnt = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let (fst, snd) = line.split(',').map(parse_range).next_tuple().unwrap();
        if check_fn(fst, snd) {
            cnt += 1;
        }
    }
    cnt
}

pub fn part_a() {
    println!(
        "Part A: # of covers {}",
        count_instances("inputs/day04/input.txt", |a, b| covers(a, b) || covers(b, a))
    );
}

pub fn part_b() {
    println!(
        "Part B: # of covers {}",
        count_instances("inputs/day04/input.txt", overlap)
    );
}
