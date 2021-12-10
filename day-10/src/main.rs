
use std::fs;

fn read_input(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("File path must be valid")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn score_invalid_character(character: char) -> usize {
    match character {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!()
    }
}

fn score_missing_ending(character: char) -> usize {
    match character {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!()
    }
}

fn matching_brace(character: char) -> char {
    match character {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!()
    }
}

fn opening_character(character: char) -> bool {
    match character  {
        '(' | '{' | '[' | '<' => true,
        _ => false
    }
}

enum LineError {
    Corrupted(usize),
    Incomplete(usize)
}

fn score_program_line(line: &String) -> LineError {
    let mut character_stack: Vec<char> = Vec::new();

    for character in line.chars() {
        if opening_character(character) {
            character_stack.push(character);
        } else {
            let last_in = character_stack.pop();

            if last_in.is_none() || matching_brace(last_in.unwrap()) != character {
                return LineError::Corrupted(score_invalid_character(character));
            }
        }
    }

    let incomplete_score = character_stack
        .iter()
        .rev()
        .map(|&character| score_missing_ending(character))
        .fold(0, |acc, n| acc * 5 + n);

    LineError::Incomplete(incomplete_score)
}

fn part_one(program: &Vec<String>) -> usize {
    program
        .iter()
        .map(|line| match score_program_line(line) { LineError::Corrupted(x) => x, _ => 0 } )
        .sum()
}

fn part_two(program: &Vec<String>) -> usize {
    let mut incomplete_lines: Vec<usize> = program
        .iter()
        .map(|line| score_program_line(line) )
        .filter_map(|error| match error { LineError::Incomplete(x) => Some(x), _ => None })
        .collect();

    incomplete_lines.sort();

    incomplete_lines[incomplete_lines.len() / 2]
}

fn main() {
    let input = read_input("input");
    println!("Day 10 Part 1: {}", part_one(&input));
    println!("Day 10 Part 2: {}", part_two(&input));
}

#[test]
fn test_part_one() {
    let input = read_input("test");
    assert_eq!(part_one(&input), 26397);
}

#[test]
fn test_part_two() {
    let input = read_input("test");
    assert_eq!(part_two(&input), 288957);
}
