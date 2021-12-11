use std::fs;

fn read_input(path: &str) -> Vec<usize> {
    fs::read_to_string(path)
        .expect("File path must be valid")
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize))
        .flatten()
        .collect()
}

fn neighbor_indeces(index: usize) -> Vec<usize> {
    match (index < 10, index >= 90, index % 10 == 0, index % 10 == 9) {
        (true, false, false, false) => vec!(index - 1, index + 1, index + 9, index + 10, index + 11),
        (true, false, true, false) => vec!(index + 1, index + 10, index + 11),
        (true, false, false, true) => vec!(index - 1, index + 9, index + 10),
        (false, true, false, false) => vec!(index - 11, index - 10, index - 9, index - 1, index + 1),
        (false, true, true, false) => vec!(index - 10, index - 9, index + 1),
        (false, true, false, true) => vec!(index - 11, index - 10, index - 1),
        (false, false, false, true) => vec!(index - 11, index - 10, index - 1, index + 9, index + 10),
        (false, false, true, false) => vec!(index - 10, index - 9, index + 1, index + 10, index + 11),
        _ => vec!(index - 11, index - 10, index - 9, index - 1, index + 1, index + 9, index + 10, index + 11)
    }
}

fn step(state: &mut Vec<usize>) -> usize {
    let mut add_queue: Vec<usize> = (0..state.len()).collect();

    while add_queue.len() > 0 {
        let i = add_queue.pop().unwrap();

        state[i] = state[i] + 1;
        if state[i] == 10 { neighbor_indeces(i).iter().for_each(|ni| add_queue.push(*ni)); }
    }

    (0..state.len())
        .map(|i| {
            match state[i] > 9 {
                true => { state[i] = 0; 1 }
                false => 0
            }
        })
        .sum()
}

fn part_one(state: &mut Vec<usize>) -> usize {
    (0..100)
        .map(|_| step(state))
        .sum()
}

fn main() {
    let input = read_input("input");
    println!("Day 11 Part 1: {}", part_one(&mut input.clone()));
}

#[test]
fn test_part_one() {
    let input = read_input("test");
    assert_eq!(part_one(&mut input.clone()), 1656);
}
