use std::fs;
use std::collections::HashMap;

fn read_input(path: &str) -> Vec<usize> {
    fs::read_to_string(path)
        .expect("File path must be valid")
        .lines()
        .fold(Vec::new(), |mut acc, line| {
            line.trim().chars().for_each(|c| acc.push(c.to_digit(10).unwrap() as usize));
            acc
        })
}



fn part_one(heights: &Vec<usize>, columns: usize) -> usize {
    let rows = heights.len() / columns as usize;

    (0..heights.len())
        .filter(|&i| {
            let mut neighbors = Vec::new();
            if (i % columns) != 0 { neighbors.push(i - 1) }
            if (i % columns) != (columns - 1) { neighbors.push(i + 1) }
            if (i / columns) != 0 { neighbors.push(i - columns) }
            if (i / columns) != (rows - 1) { neighbors.push(i + columns) }

            neighbors.iter().all(|&ni| heights[ni] > heights[i])
        })
        .map(|i| heights[i] + 1)
        .sum()
}

fn main() {
    let input = read_input("input");
    println!("Day 9 Part 1: {}", part_one(&input, 100));
}

#[test]
fn test_part_one() {
    let input = read_input("test");
    assert_eq!(part_one(&input, 10), 15);
}
