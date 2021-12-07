use std::fs;

fn alignment_fuel_cost(positions: &Vec<u32>) -> u32 {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    (min..max)
        .map(|n| fuel_cost(positions, n))
        .min()
        .unwrap()
}

fn fuel_cost(positions: &Vec<u32>, alignment: u32) -> u32 {
    positions.iter().map(|&n| (n as i32 - alignment as i32).abs() as u32).sum()
}

fn read_input(path: &str) -> Vec<u32> {
    fs::read_to_string(path)
        .expect("File path must be valid")
        .trim()
        .split(",")
        .map(|n| n.parse().expect("input must be numbers"))
        .collect()
}

fn main() {
    let positions = read_input("input");
    println!("Day 7 Part 1: {}", alignment_fuel_cost(&positions));
}

#[test]
fn test_part_one() {
    let positions = read_input("test");
    assert_eq!(37, alignment_fuel_cost(&positions));
}
