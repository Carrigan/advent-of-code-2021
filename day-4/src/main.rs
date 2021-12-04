use std::{fs};

#[derive(Debug)]
struct BingoBoard {
    numbers: Vec<u32>,
    marked: Vec<usize>
}

impl BingoBoard {
    fn is_finished(&self) -> bool {
        (0..5).any(|i| {
            let row_filled = [5 * i, 5 * i + 1, 5 * i + 2, 5 * i + 3, 5 * i + 4]
                .iter()
                .all(|n| self.marked.contains(n));
                
            let col_filled = [i, i + 5, i + 10, i + 15, i + 20]
                .iter()
                .all(|n| self.marked.contains(n));
                
            row_filled | col_filled
        })
    }
    
    fn mark(&mut self, n: u32) {
        if let Some(pos) = self.numbers.iter().position(|&i| i == n) {
            self.marked.push(pos);
        }
    }
    
    fn score(&self) -> u32 {
        (0..25)
            .filter(|i| !self.marked.contains(i))
            .map(|i| self.numbers[i])
            .sum()
    }
}

fn read_input(path: &str) -> (Vec<u32>, Vec<BingoBoard>) {
    let input = fs::read_to_string(path)
        .expect("File path must be valid");
    
    let mut lines = input.lines().peekable();
    
    let mut bingos = Vec::new();
    let numbers = lines
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    
    lines.next(); // Discard the blank line
    
    while lines.peek().is_some() {
        let mut numbers = Vec::new();
        
        (0..5).for_each(|_| {
            lines
                .next()
                .unwrap()
                .split_whitespace()
                .for_each(|n| numbers.push(n.parse().unwrap()));
        });
        
        lines.next(); // Discard the blank line
        
        bingos.push(BingoBoard { numbers, marked: Vec::new() });
    }
    
    (numbers, bingos)
}

fn run_part_one(numbers: &Vec<u32>, bingos: &mut Vec<BingoBoard>) -> Option<u32> {
    for &n in numbers.iter() {
        bingos.iter_mut().for_each(|b| b.mark(n));
        
        if let Some(board) = bingos.iter().find(|b| b.is_finished()) {
            return Some(n * board.score());
        }
    }
    
    None
}

fn main() {
    let (numbers, mut bingos) = read_input("input");
    println!("Day 4 Part 1: {}", run_part_one(&numbers, &mut bingos).unwrap());
}

#[test]
fn test_part_one() {
    let (numbers, mut bingos) = read_input("test");
    assert_eq!(4512, run_part_one(&numbers, &mut bingos).unwrap());
}
