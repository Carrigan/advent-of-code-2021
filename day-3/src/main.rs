use std::fs;

fn read_input(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("File path must be valid")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn get_most_popular(numbers: &Vec<String>, i: usize) -> u32 {
    let count = numbers.iter().fold(0, |acc, n| {
        if n.chars().nth(i).unwrap() == '1' { acc + 1 } else { acc - 1 }
    });
    
    if count > 0 { 1 } else { 0 }
}

fn part_one(numbers: &Vec<String>) -> u32 {
    let length = numbers[0].len();
    
    let (normal, flipped) = (0..length)
        .map(|i| get_most_popular(numbers, i))
        .fold((0, 0), |(acc_norm, acc_flip), n| ((acc_norm * 2) + n, (acc_flip * 2) + 1 - n));

    normal * flipped
}

fn main() {
    let input = read_input("input");
    let output = part_one(&input);
    println!("Day 3, Part 1: {}", output);
}

#[test]
fn test_part_one() {
    let input = read_input("test");
    let output = part_one(&input);
    assert_eq!(output, 198);
}
