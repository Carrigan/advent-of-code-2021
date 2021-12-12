use std::fs;

#[derive(Clone)]
struct Edge {
    point_one: String,
    point_two: String
}


struct Path {
    nodes: Vec<String>,
    small_visited_twice: bool
}

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

fn validate_path(node: &String, path: &Path, can_visit_small_cave_twice: bool) -> bool {
    if node == "start" { return false; }
    if !is_small_cave(node) { return true; }
    if can_visit_small_cave_twice && !path.small_visited_twice { return true; }

    !path.nodes.contains(node)
}

fn permutate_path(edges: &Vec<Edge>, path: &Path, can_visit_small_cave_twice: bool) -> Vec<Path> {
    let last_visit = path.nodes.last().unwrap();

    edges
        .iter()
        .filter_map(|edge| {
            match (&edge.point_one, &edge.point_two) {
                (_lv, next_visit) if _lv == last_visit => Some(next_visit),
                (next_visit, _lv) if _lv == last_visit=> Some(next_visit),
                _ => None
            }
        })
        .filter(|node| validate_path(node, path, can_visit_small_cave_twice))
        .map(|node| {
            let mut new_path = path.nodes.clone();
            new_path.push(node.clone());

            let small_visited_twice = is_small_cave(node) && new_path.iter().filter(|&n| n == node).count() == 2;

            Path { nodes: new_path, small_visited_twice: path.small_visited_twice || small_visited_twice }
        })
        .collect()
}

fn traverse_cave(edges: &Vec<Edge>, can_visit_small_cave_twice: bool) -> usize {
    let initial_path = Path { nodes: vec!("start".to_string()), small_visited_twice: false };
    let mut paths_in_progress = vec!(initial_path);
    let mut finished_paths = Vec::new();

    while paths_in_progress.len() > 0 {
        let path = paths_in_progress.remove(0);
        let permutations = permutate_path(edges, &path, can_visit_small_cave_twice);

        for permutation in permutations {
            match permutation.nodes.last().unwrap().as_str() {
                "end" => finished_paths.push(permutation),
                _ => paths_in_progress.push(permutation)
            }
        }
    }

    finished_paths.len()
}

fn part_one(edges: &Vec<Edge>) -> usize {
    traverse_cave(edges, false)
}

fn part_two(edges: &Vec<Edge>) -> usize {
    traverse_cave(edges, true)
}

fn main() {
    let edges = read_input("input");
    println!("Day 12 Part 1: {}", part_one(&edges));
    println!("Day 12 Part 2: {}", part_two(&edges));
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

#[test]
fn test_part_two() {
    let edges = read_input("test1");
    assert_eq!(part_two(&edges), 36);

    let edges = read_input("test2");
    assert_eq!(part_two(&edges), 103);

    let edges = read_input("test3");
    assert_eq!(part_two(&edges), 3509);
}
