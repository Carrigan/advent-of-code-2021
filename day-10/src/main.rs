
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

fn score_program_line(line: &String) -> usize {
    let mut character_stack: Vec<char> = Vec::new();

    for character in line.chars() {
        if opening_character(character) {
            character_stack.push(character);
        } else {
            let last_in = character_stack.pop();

            if last_in.is_none() || matching_brace(last_in.unwrap()) != character {
                return score_invalid_character(character);
            }
        }
    }

    0
}

fn part_one(program: &Vec<String>) -> usize {
    program
        .iter()
        .map(|line| score_program_line(line))
        .sum()
}

fn main() {
    let input = read_input("input");
    println!("Day 10 Part 1: {}", part_one(&input));
}

#[test]
fn test_part_one() {
    let input = read_input("test");
    assert_eq!(part_one(&input), 26397);
}
