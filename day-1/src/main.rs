use std::fs;

struct Average<I: Iterator<Item = u32>> {
    iterator: I,
    first: u32,
    second: u32,
    third: u32
}

impl <I: Iterator<Item = u32>> Average <I> {
    fn new(mut iterator: I) -> Average<I> {
        let first = 0;
        let error = "Running average iterator must start with at least two items";
        let second = iterator.next().expect(error);
        let third = iterator.next().expect(error);
        
        Average { iterator, first, second, third }
    }
}

impl <I: Iterator<Item = u32>> Iterator for Average <I> {
    type Item = u32;
    
    fn next(&mut self) -> Option<u32> {  
        let val = self.iterator.next()?;
        
        self.first = self.second;
        self.second = self.third;
        self.third = val;
        
        Some(self.first + self.second + self.third)
    }
}

fn read_input(path: &str) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();

    let input = fs::read_to_string(path).expect("File path must be valid");
    for entry in input.lines() {
        numbers.push(entry.parse::<u32>().expect("Lines must be parsable to u32"));
    }
    
    numbers
}

fn count_increases<I: Iterator<Item = u32>>(mut number_list: I) -> u32 {
    let mut last_value = number_list.next().expect("Increase list must be at least 2 values long");
    
    number_list.fold(0, |acc, item| {
        let current_last_val = last_value;
        last_value = item;
        
        if item > current_last_val { acc + 1 } else { acc }
    })
}

fn main() {
    let numbers = read_input("input-p1.txt");
    let increases = count_increases(numbers.into_iter());
    println!("Part One: {}", increases);
    
    let numbers = read_input("input-p1.txt");
    let average = Average::new(numbers.into_iter());
    let increases = count_increases(average.into_iter());
    println!("Part Two: {}", increases);
}

#[test]
fn test_part_one() {
    let values = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(count_increases(values.into_iter()), 7);
}

#[test]
fn test_part_two() {
    let values = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let average = Average::new(values.into_iter());
    assert_eq!(count_increases(average), 5);
}