use std::fs;

enum Instruction {
    Forward(i32),
    Up(i32),
    Down(i32)
}

impl From<&str> for Instruction {
    fn from(st: &str) -> Self {
        let mut split = st.split(" ");
        match (split.next(), split.next()) {
            (Some("forward"), Some(n)) => 
                Instruction::Forward(n.parse().unwrap()),
            (Some("up"), Some(n)) => 
                Instruction::Up(n.parse().unwrap()),
            (Some("down"), Some(n)) => 
                Instruction::Down(n.parse().unwrap()),
            _ => panic!()
        }
    }
}

fn read_input(path: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    let input = fs::read_to_string(path).expect("File path must be valid");
    for entry in input.lines() {
        instructions.push(Instruction::from(entry));
    }
    
    instructions
}

fn run_instructions(instructions: &Vec<Instruction>) -> (i32, i32) {
    let mut x = 0;
    let mut z = 0;
    
    for instruction in instructions {
        match instruction {
            Instruction::Forward(amount) => x += amount,
            Instruction::Up(amount) => z -= amount,
            Instruction::Down(amount) => z += amount,
        }
    }
    
    (x, z)
}

fn run_instructions_part_two(instructions: &Vec<Instruction>) -> (i32, i32) {
    let mut x = 0;
    let mut z = 0;
    let mut aim = 0;
    
    for instruction in instructions {
        match instruction {
            Instruction::Forward(amount) => {
                x += amount;
                z += amount * aim;
            },
            Instruction::Up(amount) => aim -= amount,
            Instruction::Down(amount) => aim += amount,
        }
    }
    
    (x, z)
}

fn main() {
    let instructions = read_input("input");
    let (x, z) = run_instructions(&instructions);
    println!("Day 2 Part 1: {}", x * z);
    
    let (x, z) = run_instructions_part_two(&instructions);
    println!("Day 2 Part 2: {}", x * z);
}

#[test]
fn test_part_one() {
    let instructions = read_input("test");
    let (x, z) = run_instructions(&instructions);
    assert_eq!(x * z, 150);
}

#[test]
fn test_part_two() {
    let instructions = read_input("test");
    let (x, z) = run_instructions_part_two(&instructions);
    assert_eq!(x * z, 900);
}
