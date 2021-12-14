use std::collections::HashMap;
use std::fs;
use std::hash::Hash;

type Rules = HashMap<(char, char), [(char, char); 2]>;

fn read_input(path: &str) -> (Vec<char>, Rules) {
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
        let char_one = insertion_start.next().unwrap();
        let char_two = insertion_start.next().unwrap();
        let insertion = split.next().unwrap().chars().nth(0).unwrap();

        rules.insert((char_one, char_two), [(char_one, insertion), (insertion, char_two)]);
    }

    (polymer_template, rules)
}

fn increment<K: Eq + Hash>(map: &mut HashMap<K, usize>, key: K, n: usize) {
    let new_count = match map.get(&key) {
        Some(c) => *c + n,
        None => n
    };

    map.insert(key, new_count);
}

fn iterate_and_diff(polymer: &Vec<char>, rules: &Rules, count: usize) -> usize {
    let mut counts: HashMap<(char, char), usize> = HashMap::new();
    rules.keys().for_each(|&k| { counts.insert(k, 0); });

    // Load the counts
    for i in 0..polymer.len() - 1 {
        let pair = (polymer[i], polymer[i + 1]);
        increment(&mut counts, pair, 1);
    }

    // Iterate the counts
    for _ in 0..count {
        let mut next_counts: HashMap<(char, char), usize> = HashMap::new();

        for (key_pair, count) in counts {
            let [first_next_pair, second_next_pair] = rules.get(&key_pair).unwrap();
            increment(&mut next_counts, *first_next_pair, count);
            increment(&mut next_counts, *second_next_pair, count);
        }

        counts = next_counts;
    }

    // Sum everything
    let mut letter_counts = HashMap::new();

    for ((l1, l2), &count) in &counts {
        increment(&mut letter_counts, l1, count);
        increment(&mut letter_counts, l2, count);
    }

    // Add in the first and last letters since all things are doubled but them
    increment(&mut letter_counts, polymer.first().unwrap(), 1);
    increment(&mut letter_counts, polymer.last().unwrap(), 1);

    // Find the min and max
    let max = letter_counts
        .iter()
        .map(|(_, a)| *a)
        .max()
        .unwrap();

    let min = letter_counts
        .iter()
        .map(|(_, a)| *a)
        .min()
        .unwrap();


    (max - min) / 2
}

fn part_one(polymer: &Vec<char>, rules: &Rules) -> usize {
    iterate_and_diff(polymer, rules, 10)
}

fn part_two(polymer: &Vec<char>, rules: &Rules) -> usize {
    iterate_and_diff(polymer, rules, 40)
}

fn main() {
    let (polymer, rules) = read_input("input");
    println!("Day 14 Part 1: {}", part_one(&polymer, &rules));
    println!("Day 14 Part 2: {}", part_two(&polymer, &rules));
}

#[test]
fn test_part_one() {
    let (polymer, rules) = read_input("test");
    assert_eq!(part_one(&polymer, &rules), 1588);
}

#[test]
fn test_part_two() {
    let (polymer, rules) = read_input("test");
    assert_eq!(part_two(&polymer, &rules), 2188189693529);
}
