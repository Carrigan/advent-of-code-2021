use std::fs;

#[derive(Clone)]
enum Cave {
    Large(String),
    Small(String)
}

impl Cave {
    fn name<'a>(&'a self) -> &'a String {
        match self {
            Cave::Large(name) => name,
            Cave::Small(name) => name
        }
    }
}

impl From<&str> for Cave {
    fn from(cave_string: &str) -> Self {
        match cave_string.chars().all(|c| match c { 'a'..='z' => true,  _ => false }) {
            true => Cave::Small(cave_string.to_string()),
            false => Cave::Large(cave_string.to_string())
        }
    }
}

struct Edge {
    point_one: Cave,
    point_two: Cave
}


struct Node {
    cave: Cave,
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
                point_one: Cave::from(split.next().unwrap()),
                point_two: Cave::from(split.next().unwrap())
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

        Some(&current_node.cave.name())
    }
}

fn validate_node(cave: &Cave, nodes: &Vec<Node>, node_index: usize, can_visit_small_cave_twice: bool) -> bool {
    if cave.name() == "start" { return false; }
    if let Cave::Large(_) = cave { return true; }

    let last_node = &nodes[node_index];
    if can_visit_small_cave_twice && !last_node.small_visited_twice { return true; }

    !PathIterator { nodes, node_index: Some(node_index) }.any(|path_node| path_node == cave.name())
}

fn permutate_path(edges: &Vec<Edge>, nodes: &Vec<Node>, node_index: usize, can_visit_small_cave_twice: bool) -> Vec<Node> {
    let last_visit = &nodes[node_index];

    edges
        .iter()
        .filter_map(|edge| {
            match (&edge.point_one, &edge.point_two) {
                (lv, next_visit) if lv.name() == last_visit.cave.name() => Some(next_visit),
                (next_visit, lv) if lv.name() == last_visit.cave.name() => Some(next_visit),
                _ => None
            }
        })
        .filter(|cave| validate_node(cave, nodes, node_index, can_visit_small_cave_twice))
        .map(|cave| {
            let mut small_visited_twice = last_visit.small_visited_twice;

            if can_visit_small_cave_twice && !small_visited_twice {
                small_visited_twice = match cave {
                    Cave::Large(_) => false,
                    Cave::Small(name) => PathIterator { nodes, node_index: Some(node_index) }.any(|n| n == name)
                };
            }

            Node {
                cave: cave.clone(),
                parent: Some(node_index),
                small_visited_twice
            }
        })
        .collect()
}

fn traverse_cave(edges: &Vec<Edge>, can_visit_small_cave_twice: bool) -> usize {
    let initial_node = Node { cave: Cave::Small("start".to_string()), parent: None, small_visited_twice: false };
    let mut nodes = vec!(initial_node);
    let mut node_indeces_in_progress = vec!(0);
    let mut finished_node_indeces = Vec::new();

    while node_indeces_in_progress.len() > 0 {
        let node_index = node_indeces_in_progress.remove(0);
        let permutations = permutate_path(edges, &nodes, node_index, can_visit_small_cave_twice);

        for permutation in permutations {
            let new_index = nodes.len();

            match permutation.cave.name().as_str() {
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
