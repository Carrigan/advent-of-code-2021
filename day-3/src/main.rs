use std::fs;

fn read_input(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("File path must be valid")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn get_most_popular<'a, I: ExactSizeIterator<Item = &'a String>>(numbers: I, i: usize) -> u32 {
    let count = numbers.fold(0, |acc, n| {
        if n.chars().nth(i).unwrap() == '1' { acc + 1 } else { acc - 1 }
    });
    
    if count < 0 { 0 } else { 1 }
}

fn part_one(numbers: &Vec<String>) -> u32 {
    let length = numbers[0].len();
    
    let (normal, flipped) = (0..length)
        .map(|i| get_most_popular(numbers.iter(), i))
        .fold((0, 0), |(acc_norm, acc_flip), n| ((acc_norm * 2) + n, (acc_flip * 2) + 1 - n));

    normal * flipped
}

fn part_two(numbers: &Vec<String>) -> u32 {
    let oxygen_generator_rating = iterate_bit_criteria(numbers, true, 0);
    let co2_scrubber_rating = iterate_bit_criteria(numbers, false, 0);

    oxygen_generator_rating * co2_scrubber_rating
}

fn iterate_bit_criteria(numbers: &Vec<String>, most_popular: bool, i: usize) -> u32 {
    if numbers.len() == 1 {
        return numbers.first().unwrap().chars()
            .map(|c| if c == '1' { 1 } else { 0 })
            .fold(0, |acc, n| (acc * 2) + n);
    }
    
    let popular_number = get_most_popular(numbers.iter(), i);
    let criteria = if most_popular { popular_number } else { 1 - popular_number };
    
    let filtered: Vec<String> = numbers
        .iter()
        .filter(|&n| n.chars().nth(i).unwrap().to_digit(10).unwrap() == criteria)
        .cloned()
        .collect();
        
    iterate_bit_criteria(&filtered, most_popular, i + 1)
}

fn main() {
    let input = read_input("input");
    println!("Day 3, Part 1: {}", part_one(&input));
    println!("Day 3, Part 2: {}", part_two(&input));
}

#[test]
fn test_part_one() {
    let input = read_input("test");
    let output = part_one(&input);
    assert_eq!(output, 198);
}

#[test]
fn test_part_two() {
    let input = read_input("test");
    let output = part_two(&input);
    assert_eq!(output, 230);
}
