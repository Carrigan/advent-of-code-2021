use std::fs;
use itertools::Itertools;

#[derive(PartialEq, Eq, Hash)]
struct Point { x: usize, y: usize }

impl Point {
    fn reflect_over(&mut self, fold: &Fold) {
        match fold {
            Fold::X(x) => {
                if self.x < *x { return; }
                self.x = 2 * x - self.x;
            },
            Fold::Y(y) => {
                if self.y < *y { return; }
                self.y = 2 * y - self.y;
            }
        }
    }
}
enum Fold { X(usize), Y(usize) }

fn read_input(path: &str) -> (Vec<Point>, Vec<Fold>) {
    let file  = fs::read_to_string(path)
        .expect("File path must be valid");
    let lines = file.lines();
    let mut processing_points = true;

    let mut points = Vec::new();
    let mut folds = Vec::new();

    for line in lines {
        if line.is_empty() { processing_points = false; continue; }

        if processing_points {
            let mut split = line.split(",");
            points.push(Point { x: split.next().unwrap().parse().unwrap(), y: split.next().unwrap().parse().unwrap() });
        } else {
            let mut split = line.split("=");
            match split.next().unwrap().chars().last().unwrap() {
                'x' => folds.push(Fold::X(split.next().unwrap().parse().unwrap())),
                'y' => folds.push(Fold::Y(split.next().unwrap().parse().unwrap())),
                _ => panic!()
            }
        }
    }

    (points, folds)
}

fn part_one(points: &mut Vec<Point>, folds: &Vec<Fold>) -> usize {
    let fold = folds.first().unwrap();
    points.iter_mut().for_each(|p| p.reflect_over(fold));

    points.iter().unique().count()
}

fn part_two(points: &mut Vec<Point>, folds: &Vec<Fold>) {
    for fold in folds {
        points.iter_mut().for_each(|p| p.reflect_over(fold));
    }

    let x_max = points.iter().map(|p| p.x).max().unwrap();
    let y_max = points.iter().map(|p| p.y).max().unwrap();

    for y in 0..=y_max {
        for x in 0..=x_max {
            match points.iter().any(|p| p.x == x && p.y == y) {
                true => print!("X"),
                false => print!(" "),
            }
        }

        println!("");
    }
}

fn main() {
    let (mut points, folds) = read_input("input");
    println!("Day 13 Part 1: {}", part_one(&mut points, &folds));
    println!("Day 13 Part 2:");
    part_two(&mut points, &folds);
}

#[test]
fn test_part_one() {
    let (mut points, folds) = read_input("test");
    assert_eq!(part_one(&mut points, &folds), 17);
}
