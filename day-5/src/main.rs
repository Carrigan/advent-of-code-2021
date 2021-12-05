use std::fs;

#[derive(Debug)]
struct Line {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32
}

impl Line {
    fn horizontal(&self) -> bool {
        !((self.x1 == self.x2) || (self.y1 == self.y2))
    }

    fn contains(&self, x: u32, y: u32) -> bool {
        match self.horizontal() {
            true => self.contains_horizontal(x, y),
            false => self.contains_vert_hor(x, y)
        }
    }

    fn contains_horizontal(&self, x: u32, y: u32) -> bool {
        let left = if self.x1 < self.x2 { (self.x1, self.y1) } else { (self.x2, self.y2) };
        let right = if self.x1 < self.x2 { (self.x2, self.y2) } else { (self.x1, self.y1) };
        let y_mult: i32 = if left.1 < right.1 { 1 } else { -1 };

        (left.0..=right.0).enumerate().any(|(i, line_x)| {
            let line_y = (left.1 as i32 + (y_mult * i as i32)) as u32;

            line_x == x && line_y == y
        })
    }

    fn contains_vert_hor(&self, x: u32, y: u32) -> bool {
        let lower_x = *[self.x1, self.x2].iter().min().unwrap();
        let upper_x = *[self.x1, self.x2].iter().max().unwrap();
        let lower_y = *[self.y1, self.y2].iter().min().unwrap();
        let upper_y = *[self.y1, self.y2].iter().max().unwrap();

        lower_x <= x && upper_x >= x && lower_y <= y && upper_y >= y
    }
}

fn find_puzzle_bounds(lines: &Vec<Line>) -> (u32, u32, u32, u32) {
    let mut x_min = 1000;
    let mut y_min = 1000;
    let mut x_max = 0;
    let mut y_max = 0;

    for line in lines {
        x_min = *[x_min, line.x1, line.x2].iter().min().unwrap();
        y_min = *[y_min, line.y1, line.y2].iter().min().unwrap();
        x_max = *[x_max, line.x1, line.x2].iter().max().unwrap();
        y_max = *[y_max, line.y1, line.y2].iter().max().unwrap();
    }

    (x_min, y_min, x_max, y_max)
}

fn read_input(path: &str) -> Vec<Line> {
    let input = fs::read_to_string(path)
        .expect("File path must be valid");

    input
        .lines()
        .map(|line| {
            let mut split = line.split(" -> ");
            let mut split_left = split.next().unwrap().split(",");
            let mut split_right = split.next().unwrap().split(",");

            Line {
                x1: split_left.next().unwrap().parse().unwrap(),
                y1: split_left.next().unwrap().parse().unwrap(),
                x2: split_right.next().unwrap().parse().unwrap(),
                y2: split_right.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

fn run_part_one(lines: &Vec<Line>, bounds: (u32, u32, u32, u32)) -> u32 {
    count_vents(lines, bounds, false)
}

fn run_part_two(lines: &Vec<Line>, bounds: (u32, u32, u32, u32)) -> u32 {
    count_vents(lines, bounds, true)
}

fn count_vents(lines: &Vec<Line>, (x_min, y_min, x_max, y_max): (u32, u32, u32, u32), process_all: bool) -> u32 {
    (y_min..=y_max)
        .map(|y|
            (x_min..=x_max)
                .filter(|&x|
                    lines
                        .iter()
                        .filter(|line| (process_all || !line.horizontal()) && line.contains(x, y))
                        .count() > 1
                )
                .count() as u32
        )
        .sum()
}

fn main() {
    let lines = read_input("input");
    let bounds = find_puzzle_bounds(&lines);
    println!("Day 5 Part 1: {}", run_part_one(&lines, bounds));
    println!("Day 5 Part 2: {}", run_part_two(&lines, bounds));
}

#[test]
fn test_part_one() {
    let lines = read_input("test");
    let bounds = find_puzzle_bounds(&lines);
    assert_eq!(5, run_part_one(&lines, bounds));
}

#[test]
fn test_part_two() {
    let lines = read_input("test");
    let bounds = find_puzzle_bounds(&lines);
    assert_eq!(12, run_part_two(&lines, bounds));
}
