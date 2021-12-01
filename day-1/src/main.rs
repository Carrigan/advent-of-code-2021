use std::fs;

fn read_input(path: &str) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();

    let input = fs::read_to_string(path).unwrap();
    for entry in input.lines() {
        numbers.push(entry.parse::<u32>().unwrap());
    }
    
    numbers
}

fn count_increases<I: Iterator<Item = u32>>(mut number_list: I) -> u32 {
    let mut last_value = number_list.next().unwrap();
    
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
}

#[test]
fn test_part_one() {
    let values = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(count_increases(values.into_iter()), 7);
}
