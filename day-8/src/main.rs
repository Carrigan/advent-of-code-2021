use std::fs;

fn read_input(path: &str) -> (Vec<String>, Vec<String>) {
    fs::read_to_string(path)
        .expect("File path must be valid")
        .lines()
        .fold((Vec::new(), Vec::new()), |(mut signals, mut outputs), line| {
            let mut split = line.split("|");
            split.next().unwrap().split(" ").for_each(|sig| signals.push(sig.to_string()));
            split.next().unwrap().split(" ").for_each(|out| outputs.push(out.to_string()));

            (signals, outputs)
        })
}

fn part_one_count(outputs: &Vec<String>) -> usize {
    outputs
        .iter()
        .filter(|out|
            match out.len() {
                2 | 3 | 4 | 7 => true,
                _ => false
            }
        )
        .count()
}

fn main() {
    let (_signals, outputs) = read_input("input");
    println!("Day 8 Part 1: {}", part_one_count(&outputs));
}

#[test]
fn test_part_one() {
    let (_signals, outputs) = read_input("test");
    assert_eq!(26, part_one_count(&outputs));
}

