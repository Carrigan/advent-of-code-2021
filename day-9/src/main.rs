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

fn find_neighbors(index: usize, columns: usize, rows: usize) -> Vec<usize> {
    let mut neighbors = Vec::new();

    if (index % columns) != 0 { neighbors.push(index - 1) }
    if (index % columns) != (columns - 1) { neighbors.push(index + 1) }
    if (index / columns) != 0 { neighbors.push(index - columns) }
    if (index / columns) != (rows - 1) { neighbors.push(index + columns) }

    neighbors
}

fn find_mins(heights: &Vec<usize>, columns: usize) -> Vec<usize> {
    let rows = heights.len() / columns as usize;

    (0..heights.len())
        .filter(|&i| {
            find_neighbors(i, columns, rows)
                .iter()
                .all(|&ni| heights[ni] > heights[i])
        })
        .collect()
}

fn part_one(heights: &Vec<usize>, columns: usize) -> usize {
    find_mins(&heights, columns)
        .iter()
        .map(|&i| heights[i] + 1)
        .sum()
}

fn traverse_basin(heights: &Vec<usize>, columns: usize, low_index: usize) -> Vec<usize> {
    let rows = heights.len() / columns as usize;
    let mut to_search = vec!(low_index);
    let mut basin = Vec::new();

    while to_search.len() > 0 {
        let index = to_search.remove(0);

        if basin.contains(&index) || to_search.contains(&index) {
            continue;
        }

        basin.push(index);

        find_neighbors(index, columns, rows)
            .iter()
            .filter(|&&adj_index| heights[adj_index] > heights[index] && heights[adj_index] < 9)
            .for_each(|&adj_index| { to_search.push(adj_index) });
    }

    basin
}

fn part_two(heights: &Vec<usize>, columns: usize) -> usize {
    let mut basins: Vec<usize> = find_mins(heights, columns)
        .iter()
        .map(|&min_index| traverse_basin(heights, columns, min_index).len())
        .collect();

    basins.sort_by(|a, b| b.cmp(a));

    basins.iter().take(3).product()
}

fn main() {
    let input = read_input("input");
    println!("Day 9 Part 1: {}", part_one(&input, 100));
    println!("Day 9 Part 2: {}", part_two(&input, 100));
}

#[test]
fn test_part_one() {
    let input = read_input("test");
    assert_eq!(part_one(&input, 10), 15);
}


#[test]
fn test_part_two() {
    let input = read_input("test");
    assert_eq!(part_two(&input, 10), 1134);
}