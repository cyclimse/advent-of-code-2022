const SCORE_WIN: i32 = 6;
const SCORE_DRAW: i32 = 3;

const ROCK: char = 'A';
const PAPER: char = 'B';
const SCIS: char = 'C';

fn decrypt_shape(shape: char) -> char {
    match shape {
        'X' => ROCK,
        'Y' => PAPER,
        'Z' => SCIS,
        _ => panic!("unexpected char"),
    }
}

fn score_for_shape(shape: char) -> i32 {
    match shape {
        ROCK => 1,
        PAPER => 2,
        SCIS => 3,
        _ => panic!("unexpected char"),
    }
}

fn play_round(elf: char, me: char) -> i32 {
    let score = score_for_shape(me);
    match (elf, me) {
        (_, _) if elf == me => SCORE_DRAW + score,
        (ROCK, PAPER) => SCORE_WIN + score,
        (ROCK, SCIS) => score,
        (PAPER, ROCK) => score,
        (PAPER, SCIS) => SCORE_WIN + score,
        (SCIS, ROCK) => score + SCORE_WIN,
        (SCIS, PAPER) => score,
        _ => panic!("{}:{}", elf, me),
    }
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    input
        .split("\n")
        .map(|line| {
            let chrs: Vec<char> = line.chars().collect();
            let elf = chrs[0];
            let me = decrypt_shape(chrs[2]);
            play_round(elf, me)
        })
        .sum()
}

fn decrypt_shape_part2(elf: char, shape: char) -> char {
    match shape {
        'X' => match elf {
            ROCK => SCIS,
            PAPER => ROCK,
            SCIS => PAPER,
            _ => panic!("unexpected char"),
        },
        'Y' => elf,
        'Z' => match elf {
            ROCK => PAPER,
            PAPER => SCIS,
            SCIS => ROCK,
            _ => panic!("unexpected char"),
        },
        _ => panic!("unexpected char"),
    }
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> i32 {
    input
        .split("\n")
        .map(|line| {
            let chrs: Vec<char> = line.chars().collect();
            let elf = chrs[0];
            let me = decrypt_shape_part2(elf, chrs[2]);
            play_round(elf, me)
        })
        .sum()
}
