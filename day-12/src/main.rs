use std::fs;

#[derive(Clone)]
struct Edge {
    point_one: String,
    point_two: String
}

type Path = Vec<String>;

fn read_input(path: &str) -> Vec<Edge> {
    fs::read_to_string(path)
        .expect("File path must be valid")
        .lines()
        .map(|line| {
            let mut split = line.split("-");
            Edge {
                point_one: split.next().unwrap().to_string(),
                point_two: split.next().unwrap().to_string()
            }
        })
        .collect()
}

fn is_small_cave(cave: &str) -> bool {
    cave.chars().all(|c| match c { 'a'..='z' => true, _ => false })
}

fn permutate_path(edges: &Vec<Edge>, path: &Path) -> Vec<Path> {
    let last_visit = path.last().unwrap();

    edges
        .iter()
        .filter_map(|edge| {
            match (&edge.point_one, &edge.point_two) {
                (_lv, next_visit) if _lv == last_visit => Some(next_visit),
                (next_visit, _lv) if _lv == last_visit=> Some(next_visit),
                _ => None
            }
        })
        .filter(|node| !is_small_cave(node) || !path.contains(node))
        .map(|node| {
            let mut new_path = path.clone();
            new_path.push(node.clone());

            new_path
        })
        .collect()
}

fn part_one(edges: &Vec<Edge>) -> usize {
    let initial_path = vec!("start".to_string());
    let mut paths_in_progress = vec!(initial_path);
    let mut finished_paths = Vec::new();

    while paths_in_progress.len() > 0 {
        let path = paths_in_progress.remove(0);
        let permutations = permutate_path(edges, &path);

        for permutation in permutations {
            match permutation.last().unwrap().as_str() {
                "end" => finished_paths.push(permutation),
                _ => paths_in_progress.push(permutation)
            }
        }
    }

    finished_paths.len()
}

fn main() {
    let edges = read_input("input");
    println!("Day 12 Part 1: {}", part_one(&edges));
}

#[test]
fn test_part_one() {
    let edges = read_input("test1");
    assert_eq!(part_one(&edges), 10);

    let edges = read_input("test2");
    assert_eq!(part_one(&edges), 19);

    let edges = read_input("test3");
    assert_eq!(part_one(&edges), 226);
}
