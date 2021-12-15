use std::fs;

struct VisitedNode {
    index: usize,
    parent: Option<usize>,
    total_cost: usize
}

struct VisitedNodeIterator<'a> {
    nodes: &'a Vec<VisitedNode>,
    index: Option<usize>
}

impl <'a> Iterator for VisitedNodeIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let node = &self.nodes[self.index?];
        self.index = node.parent;

        Some(node.index)
    }
}

fn read_input(path: &str) -> Vec<usize> {
    fs::read_to_string(path)
        .expect("File path must be valid")
        .lines()
        .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).map(|n| n as usize))
        .collect()
}

fn neighboring_indeces(index: usize, width: usize, size: usize) -> Vec<usize> {
    let mut neighbors = Vec::new();

    if index % width != 0 { neighbors.push(index - 1); }
    if index % width != width - 1 { neighbors.push(index + 1); }
    if index > width { neighbors.push(index - width); }
    if index < (size - width) { neighbors.push(index + width); }

    neighbors
}

fn part_one(map: &Vec<usize>, width: usize) -> usize {
    let mut nodes = vec!(VisitedNode { index: 0, parent: None, total_cost: 0 });
    let mut to_visit = vec!((0, 0)); // (node_index, node_cost)
    let map_size = map.len();
    let destination = map_size - 1;

    while to_visit.len() > 0 {
        let to_visit_index = to_visit
            .iter()
            .enumerate()
            .min_by(|(_, (_, c1)), (_, (_, c2))| c1.cmp(c2))
            .unwrap()
            .0;

        let lowest_cost_index = to_visit.remove(to_visit_index).0;
        let lowest_cost = nodes[lowest_cost_index].total_cost;

        for neighboring_map_index in neighboring_indeces(nodes[lowest_cost_index].index, width, map_size) {
            let path_cost = lowest_cost + map[neighboring_map_index];

            if neighboring_map_index == destination {
                return path_cost;
            }

            let already_visited = VisitedNodeIterator { nodes: &nodes, index: Some(lowest_cost_index) }
                .any(|node_map_index| node_map_index == neighboring_map_index);

            if already_visited { continue; }

            let insertion_index = nodes.len();
            nodes.push(VisitedNode {
                index: neighboring_map_index,
                parent: Some(lowest_cost_index),
                total_cost: path_cost
            });
            to_visit.push((insertion_index, path_cost));
        }
    }

    panic!("No solution was found")
}

fn main() {
    let map = read_input("input");
    println!("Day 5 Part One: {}", part_one(&map, 100));
}

#[test]
fn test_part_one() {
    let map = read_input("test");
    assert_eq!(part_one(&map, 10), 40);
}
