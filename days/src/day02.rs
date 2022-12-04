use std::io::BufRead;
use std::{fs::File, io::BufReader};

#[derive(Debug, Clone, Copy)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

type Game = (Action, Action);

fn parse_action(action: char) -> Action {
    match action {
        'A' | 'X' => Action::Rock,
        'B' | 'Y' => Action::Paper,
        'C' | 'Z' => Action::Scissors,
        _ => panic!("bad input!"),
    }
}

fn decide_action(theirs: char, yours: char) -> Game {
    let theirs = parse_action(theirs);
    match (theirs, yours) {
        (_, 'Y') => (theirs, theirs),
        (Action::Rock, 'X') => (theirs, Action::Scissors),
        (Action::Rock, 'Z') => (theirs, Action::Paper),
        (Action::Paper, 'X') => (theirs, Action::Rock),
        (Action::Paper, 'Z') => (theirs, Action::Scissors),
        (Action::Scissors, 'X') => (theirs, Action::Paper),
        (Action::Scissors, 'Z') => (theirs, Action::Rock),
        _ => panic!("bad input!"),
    }
}

fn parse_games(path: &str, parse_fn: fn(char, char) -> Game) -> Vec<Game> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut games = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let line2 = line.trim().as_bytes();
        games.push(parse_fn(line2[0] as char, line2[2] as char));
    }
    games
}

fn action_score(action: Action) -> i32 {
    match action {
        Action::Rock => 1,
        Action::Paper => 2,
        Action::Scissors => 3,
    }
}

fn score_game(game: &Game) -> i32 {
    let loss = 0;
    let draw = 3;
    let win = 6;
    let choice = {
        let (_, yours) = game;
        action_score(*yours)
    };
    match game {
        (Action::Scissors, Action::Scissors) => choice + draw,
        (Action::Scissors, Action::Paper) => choice + loss,
        (Action::Scissors, Action::Rock) => choice + win,
        (Action::Paper, Action::Scissors) => choice + win,
        (Action::Paper, Action::Paper) => choice + draw,
        (Action::Paper, Action::Rock) => choice + loss,
        (Action::Rock, Action::Scissors) => choice + loss,
        (Action::Rock, Action::Paper) => choice + win,
        (Action::Rock, Action::Rock) => choice + draw,
    }
}

pub fn part_a() {
    let games = parse_games("inputs/day02/input.txt", |a, b| {
        (parse_action(a), parse_action(b))
    });
    let score: i32 = games.iter().map(score_game).sum();
    println!("Part A: Score is {}", score);
}

pub fn part_b() {
    let games = parse_games("inputs/day02/input.txt", decide_action);
    let score: i32 = games.iter().map(score_game).sum();
    println!("Part B: Score is {}", score);
}
