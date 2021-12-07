use std::fs;

enum FuelCalculation {
    Linear,
    Triangle
}

impl FuelCalculation {
    fn diff(&self, n1: u32, n2: u32) -> u32 {
        let linear_diff = (n1 as i32 - n2 as i32).abs() as u32;

        match self {
            FuelCalculation::Linear => linear_diff,
            FuelCalculation::Triangle => (linear_diff * (linear_diff + 1)) / 2,
        }
    }
}

fn alignment_fuel_cost(positions: &Vec<u32>, fuel_calculation: &FuelCalculation) -> u32 {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    (min..max)
        .map(|n| fuel_cost(positions, n, fuel_calculation))
        .min()
        .unwrap()
}

fn fuel_cost(positions: &Vec<u32>, alignment: u32, fuel_calculation: &FuelCalculation) -> u32 {
    positions
        .iter()
        .map(|&n| fuel_calculation.diff(n, alignment))
        .sum()
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
    println!("Day 7 Part 1: {}", alignment_fuel_cost(&positions, &FuelCalculation::Linear));
    println!("Day 7 Part 2: {}", alignment_fuel_cost(&positions, &FuelCalculation::Triangle));
}

#[test]
fn test_part_one() {
    let positions = read_input("test");
    assert_eq!(37, alignment_fuel_cost(&positions, &FuelCalculation::Linear));
}

#[test]
fn test_part_two() {
    let positions = read_input("test");
    assert_eq!(168, alignment_fuel_cost(&positions, &FuelCalculation::Triangle));
}

