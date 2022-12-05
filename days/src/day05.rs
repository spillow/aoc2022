use itertools::Itertools;
use std::collections::HashMap;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

#[derive(Debug)]
struct Action {
    num: i32,
    src: i32,
    dst: i32,
}

type Stacks = HashMap<i32, Vec<char>>;

fn stack(s: &str) -> Vec<char> {
    s.chars().collect()
}

fn build_stacks() -> Stacks {
    let mut m = HashMap::new();
    m.insert(1, stack("RNFVLJSM"));
    m.insert(2, stack("PNDZFJWH"));
    m.insert(3, stack("WRCDG"));
    m.insert(4, stack("NBS"));
    m.insert(5, stack("MZWPCBFN"));
    m.insert(6, stack("PRMW"));
    m.insert(7, stack("RTNGLSW"));
    m.insert(8, stack("QTHFNBV"));
    m.insert(9, stack("LMHZNF"));
    m
}

fn parse_actions(path: &str) -> Vec<Action> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut actions = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let (num, src, dst) = line
            .split(' ')
            .filter(|v| *v != "")
            .map(|v| v.parse::<i32>().unwrap())
            .next_tuple()
            .unwrap();
        actions.push(Action { num, src, dst });
    }
    actions
}

fn execute_a(actions: &Vec<Action>, stacks: &mut Stacks) {
    for action in actions {
        for _ in 0..action.num {
            let src = stacks.get_mut(&action.src).unwrap();
            let val = src.pop().unwrap();
            let dst = stacks.get_mut(&action.dst).unwrap();
            dst.push(val);
        }
    }
}

fn execute_b(actions: &Vec<Action>, stacks: &mut Stacks) {
    let mut v = Vec::new();
    for action in actions {
        let src = stacks.get_mut(&action.src).unwrap();
        for _ in 0..action.num {
            let val = src.pop().unwrap();
            v.push(val);
        }
        let dst = stacks.get_mut(&action.dst).unwrap();
        for _ in 0..action.num {
            let val = v.pop().unwrap();
            dst.push(val);
        }
    }
}

fn get_result(stacks: Stacks) -> String {
    let mut v: Vec<(i32, Vec<char>)> = stacks.into_iter().collect();
    v.sort_by_key(|(k, _)| *k);
    v.iter().map(|(_, v)| *v.last().unwrap()).collect()
}

pub fn part_a() {
    let actions = parse_actions("inputs/day05/input.txt");
    let mut stacks = build_stacks();
    execute_a(&actions, &mut stacks);
    println!("Part A: {}", get_result(stacks));
}

pub fn part_b() {
    let actions = parse_actions("inputs/day05/input.txt");
    let mut stacks = build_stacks();
    execute_b(&actions, &mut stacks);
    println!("Part B: {}", get_result(stacks));
}
