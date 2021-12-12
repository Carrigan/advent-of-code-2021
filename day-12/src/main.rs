use std::fs;

#[derive(Clone)]
struct Edge {
    point_one: String,
    point_two: String
}


struct Node {
    name: String,
    parent: Option<usize>,
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

struct PathIterator<'a> {
    node_index: Option<usize>,
    nodes: &'a Vec<Node>
}

impl <'a> Iterator for PathIterator<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        let current_node = &self.nodes[self.node_index?];
        self.node_index = current_node.parent;

        Some(&current_node.name)
    }
}

fn is_small_cave(cave: &str) -> bool {
    cave.chars().all(|c| match c { 'a'..='z' => true, _ => false })
}

fn validate_node(node_name: &String, nodes: &Vec<Node>, node_index: usize, can_visit_small_cave_twice: bool) -> bool {
    if node_name == "start" { return false; }
    if !is_small_cave(node_name) { return true; }

    let last_node = &nodes[node_index];
    if can_visit_small_cave_twice && !last_node.small_visited_twice { return true; }

    !PathIterator { nodes, node_index: Some(node_index) }.any(|path_node| path_node == node_name)
}

fn permutate_path(edges: &Vec<Edge>, nodes: &Vec<Node>, node_index: usize, can_visit_small_cave_twice: bool) -> Vec<Node> {
    let last_visit = &nodes[node_index];

    edges
        .iter()
        .filter_map(|edge| {
            match (&edge.point_one, &edge.point_two) {
                (_lv, next_visit) if _lv == &last_visit.name => Some(next_visit),
                (next_visit, _lv) if _lv == &last_visit.name => Some(next_visit),
                _ => None
            }
        })
        .filter(|node_name| validate_node(node_name, nodes, node_index, can_visit_small_cave_twice))
        .map(|node| {
            let mut small_visited_twice = last_visit.small_visited_twice;

            if can_visit_small_cave_twice && !small_visited_twice {
                small_visited_twice = is_small_cave(node) && PathIterator { nodes, node_index: Some(node_index) }.any(|n| n == node);
            }

            Node {
                name: node.clone(),
                parent: Some(node_index),
                small_visited_twice
            }
        })
        .collect()
}

fn traverse_cave(edges: &Vec<Edge>, can_visit_small_cave_twice: bool) -> usize {
    let initial_node = Node { name: "start".to_string(), parent: None, small_visited_twice: false };
    let mut nodes = vec!(initial_node);
    let mut node_indeces_in_progress = vec!(0);
    let mut finished_node_indeces = Vec::new();

    while node_indeces_in_progress.len() > 0 {
        let node_index = node_indeces_in_progress.remove(0);
        let permutations = permutate_path(edges, &nodes, node_index, can_visit_small_cave_twice);

        for permutation in permutations {
            let new_index = nodes.len();

            match permutation.name.as_str() {
                "end" => finished_node_indeces.push(new_index),
                _ => node_indeces_in_progress.push(new_index)
            }

            nodes.push(permutation);
        }
    }

    finished_node_indeces.len()
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
