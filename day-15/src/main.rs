use std::fs;

fn read_input(path: &str) -> Vec<usize> {
    fs::read_to_string(path)
        .expect("File path must be valid")
        .lines()
        .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).map(|n| n as usize))
        .collect()
}

fn rot(i: usize) -> usize {
    match i {
        i if i > 9 => i - 9,
        i => i
    }
}

fn extend_map(map: &Vec<usize>, width: usize) -> Vec<usize> {
    let mut extended_right_map = Vec::new();
    let mut extended_map = Vec::new();

    let initial_row_count = map.len() / width;

    // Tile 5 times to the right
    for y in 0..initial_row_count {
        for copy in 0..5 {
            for x in 0..width {
                extended_right_map.push(rot(map[y * width + x] + copy));
            }
        }
    }

    // Tile 5 times down
    for copy in 0..5 {
        extended_right_map.iter().for_each(|i| extended_map.push(rot(*i + copy)));
    }

    extended_map
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
    shortest_path_cost(map, width)
}

fn part_two(map: &Vec<usize>, width: usize) -> usize {
    let extended = extend_map(map, width);
    shortest_path_cost(&extended, width * 5)
}

fn shortest_path_cost(map: &Vec<usize>, width: usize) -> usize {
    let map_size = map.len();
    let destination = map_size - 1;
    let mut unvisited_indeces: Vec<usize> = (0..map_size).collect();
    let mut tentative_distances: Vec<usize> = (0..map_size).map(|_| usize::MAX).collect();
    tentative_distances[0] = 0;

    while unvisited_indeces.len() > 0 {
        let (unvisited_index, &current_node_index) = unvisited_indeces
            .iter()
            .enumerate()
            .min_by(|(_, &i1), (_, &i2)| {
                tentative_distances[i1].cmp(&tentative_distances[i2])
            })
            .unwrap();

        unvisited_indeces.remove(unvisited_index);

        let current_node_cost = tentative_distances[current_node_index];

        for neighbor_index in neighboring_indeces(current_node_index, width, map_size) {
            let neighbor_cost = current_node_cost + map[neighbor_index];

            if neighbor_index == destination {
                return neighbor_cost;
            }

            if neighbor_cost < tentative_distances[neighbor_index] {
                tentative_distances[neighbor_index] = neighbor_cost;
            }
        }
    }

    panic!("Path not found")
}

fn main() {
    let map = read_input("input");
    println!("Day 5 Part One: {}", part_one(&map, 100));
    println!("Day 5 Part Two: {}", part_two(&map, 100));
}

#[test]
fn test_part_one() {
    let map = read_input("test");
    assert_eq!(part_one(&map, 10), 40);
}

#[test]
fn test_part_two() {
    let map = read_input("test");
    assert_eq!(part_two(&map, 10), 315);
}
