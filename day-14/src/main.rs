use std::collections::HashMap;
use std::fs;

fn read_input(path: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let file  = fs::read_to_string(path)
        .expect("File path must be valid");
    let mut lines = file.lines();

    let polymer_template: Vec<char> = lines
        .next()
        .unwrap()
        .chars()
        .collect();

    lines.next();

    let mut rules = HashMap::new();

    for line in lines {
        let mut split = line.split(" -> ");
        let mut insertion_start = split.next().unwrap().chars();
        let insertion_end = split.next().unwrap();

        rules.insert(
        (insertion_start.next().unwrap(), insertion_start.next().unwrap()),
        insertion_end.chars().nth(0).unwrap()
        );
    }

    (polymer_template, rules)
}

fn iterate(polymer: Vec<char>, rules: &HashMap<(char, char), char>) -> Vec<char> {
    let firsts = &polymer.as_slice()[0..polymer.len() - 1];
    let seconds = &polymer.as_slice()[1..polymer.len()];
    let final_char = *polymer.last().unwrap();

    firsts
        .iter()
        .zip(seconds.iter())
        .flat_map(|(&first, &second)| vec!(first, rules[&(first, second)]))
        .chain(vec!(final_char))
        .collect()
}

fn part_one(mut polymer: Vec<char>, rules: &HashMap<(char, char), char>) -> usize {
    for _ in 0..10 { polymer = iterate(polymer, &rules); }


    let mut counts = HashMap::new();
    for entry in polymer.iter() {
        let count = match counts.get(entry) { Some(c) => c + 1, None => 1 };
        counts.insert(entry, count);
    }

    let max = counts.iter().map(|(_k, v)| *v).max().unwrap();
    let min = counts.iter().map(|(_k, v)| *v).min().unwrap();

    max - min
}

fn main() {
    let (polymer, rules) = read_input("input");
    println!("Day 14 Part 1: {}", part_one(polymer, &rules));
}

#[test]
fn test_part_one() {
    let (polymer, rules) = read_input("test");
    assert_eq!(part_one(polymer, &rules), 1588);
}
