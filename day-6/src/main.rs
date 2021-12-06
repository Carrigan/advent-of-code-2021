use std::fs;

fn iterate_n(fish: &Vec<u8>, n: usize) -> usize {
    let mut counts = [0; 9];
    for f in fish.iter() { counts[*f as usize] += 1; }

    for _ in 0..n {
        let zeroes = counts[0];
        for x in 0..8 { counts[x] = counts[x + 1]; }
        counts[6] += zeroes;
        counts[8] = zeroes;
    }

    counts.iter().sum()
}

fn read_input(path: &str) -> Vec<u8> {
    fs::read_to_string(path)
        .expect("File path must be valid")
        .trim()
        .split(",")
        .map(|n| n.parse().expect("input must be numbers"))
        .collect()
}

fn main() {
    let fish = read_input("input");
    println!("Day 6 Part 1: {}", iterate_n(&fish, 80));
    println!("Day 6 Part 2: {}", iterate_n(&fish, 256));
}

#[test]
fn test_part_one() {
    let fish = read_input("test");
    assert_eq!(iterate_n(&fish, 80), 5934);
}

#[test]
fn test_part_two() {
    let fish = read_input("test");
    assert_eq!(iterate_n(&fish, 256), 26984457539);
}
