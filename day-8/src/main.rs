use std::fs;
use std::collections::HashMap;

fn read_input(path: &str) -> Vec<(Vec<String>, Vec<String>)> {
    fs::read_to_string(path)
        .expect("File path must be valid")
        .lines()
        .fold(Vec::new(), |mut acc, line| {
            let mut split = line.split("|");

            acc.push((
                split.next().unwrap().trim().split(" ").map(str::to_string).collect(),
                split.next().unwrap().trim().split(" ").map(str::to_string).collect()
            ));

            acc
        })
}

fn distinguishable_by_length(digit: &&String) -> bool {
    match digit.len() {
        2 | 3 | 4 | 7 => true,
        _ => false
    }
}

fn part_one_count(entries: &Vec<(Vec<String>, Vec<String>)>) -> usize {
    entries
        .iter()
        .map(|(_signals, outputs)| {
            outputs
                .iter()
                .filter(distinguishable_by_length)
                .count()
        })
        .sum()
}

fn decode_entry((signals, outputs): &(Vec<String>, Vec<String>)) -> usize {
    let mappings = determine_mappings(signals);

    outputs
        .iter()
        .map(|output| apply_mappings(&mappings, output))
        .fold(0, |acc, n| acc * 10 + n)
}

fn determine_mappings(signals: &Vec<String>) -> HashMap<char, char> {
    let mut output = HashMap::new();

    // Find 1 and 7
    let one = signals.iter().find(|s| s.len() == 2).unwrap();
    let seven = signals.iter().find(|s| s.len() == 3).unwrap();

    // The diff between them is 'a'
    let a = seven.chars().find(|&c| !one.contains(c)).unwrap();
    output.insert(a, 'a');

    // Find 6
    let one_first = one.chars().nth(0).unwrap();
    let one_second = one.chars().nth(1).unwrap();
    let six = signals.iter().find(|s| {
        s.len() == 6 && (!s.contains(one_first) || !s.contains(one_second))
    }).unwrap();

    // The missing piece can find which one character is which
    let (c, f) = match six.contains(one_first) {
        false => (one_first, one_second),
        true => (one_second, one_first),
    };

    output.insert(c, 'c');
    output.insert(f, 'f');

    // Find 4, mark b and d (though we don't know which is which)
    let four = signals.iter().find(|s| s.len() == 4).unwrap();

    // Find 0, the missing piece is d
    let zero = signals.iter().find(|s| {
        s.len() == 6 && !four.chars().filter(|&chr| chr != c).all(|c| s.contains(c))
    }).unwrap();

    let d = "abcdefg".chars().find(|&c| !zero.contains(c)).unwrap();
    output.insert(d, 'd');

    // In 4, the unknown piece is b
    let known_letters = [c, d, f];
    let b = four.chars().find(|c| !known_letters.contains(&c)).unwrap();
    output.insert(b, 'b');

    // Find 3, the unknown piece is g
    let known_letters = [a, c, d, f];
    let three = signals.iter().find(|s| {
        s.len() == 5 && known_letters.iter().all(|&l| s.contains(l))
    }).unwrap();
    let g = three.chars().find(|c| !known_letters.contains(&c)).unwrap();
    output.insert(g, 'g');

    // The only character left is e
    let e = "abcdefg".chars().find(|c| !output.keys().any(|v| v == c)).unwrap();
    output.insert(e, 'e');

    output
}

fn apply_mappings(mappings: &HashMap<char, char>, output: &String) -> usize {
    let applied: String = output.chars().map(|c| mappings[&c]).collect();

    match applied.len() {
        2 => 1,
        3 => 7,
        4 => 4,
        7 => 8,
        6 => match (applied.contains('d'), applied.contains('e')) {
            (false, _) => 0,
            (true, false) => 9,
            (true, true) => 6
        },
        5 => match (applied.contains('b'), applied.contains('c'), applied.contains('e')) {
            (true, false, false) => 5,
            (true, true, false) => 9,
            (false, true, true) => 2,
            (false, true, false) => 3,
            _ => panic!()
        },
        _ => panic!()
    }
}

fn part_two(entries: &Vec<(Vec<String>, Vec<String>)>) -> usize {
    entries.iter().map(decode_entry).sum()
}

fn main() {
    let entries = read_input("input");
    println!("Day 8 Part 1: {}", part_one_count(&entries));
    println!("Day 8 Part 2: {}", part_two(&entries));
}

#[test]
fn test_part_one() {
    let entries = read_input("test");
    assert_eq!(26, part_one_count(&entries));
}

#[test]
fn test_part_two() {
    let entries = read_input("test");
    assert_eq!(61229, part_two(&entries));
}

