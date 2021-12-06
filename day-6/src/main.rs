use std::fs;

fn iterate(fish: Vec<u8>) -> Vec<u8> {
    let mut add_count = 0;
    let mut new_fish: Vec<u8> = fish
        .iter()
        .map(|&n| if n == 0 { add_count += 1; 6 } else { n - 1 })
        .collect();

    (0..add_count).for_each(|_| new_fish.push(8));

    new_fish
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
    let mut fish = read_input("input");
    for _ in 0..80 { fish = iterate(fish); };
    println!("Day 6 Part 1: {}", fish.len());
}

#[test]
fn test_part_one() {
    let mut fish = read_input("test");
    for _ in 0..80 { fish = iterate(fish); };
    assert_eq!(fish.len(), 5934);
}
