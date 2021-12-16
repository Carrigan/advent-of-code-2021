use std::fs;

#[derive(Debug)]
enum Packet {
    Operator {
        version: usize,
        length_type_id: usize,
        contents: Vec<Packet>
    },
    Literal {
        version: usize,
        literal: usize,
        length: usize
    }
}

impl Packet {
    fn len(&self) -> usize {
        match self {
            Packet::Operator { length_type_id, contents, .. } => {
                let length_field = match length_type_id {
                    0 => 15,
                    1 => 11,
                    _ => panic!()
                };
                let content_length: usize = contents.iter().map(|packet| packet.len()).sum();

                7 + length_field + content_length
            },
            Packet::Literal { length, .. } => 6 + length
        }
    }
}

fn decode_hex(hex: char) -> Vec<usize> {
    match hex {
        'F' => vec!(1, 1, 1, 1),
        'E' => vec!(1, 1, 1, 0),
        'D' => vec!(1, 1, 0, 1),
        'C' => vec!(1, 1, 0, 0),
        'B' => vec!(1, 0, 1, 1),
        'A' => vec!(1, 0, 1, 0),
        '9' => vec!(1, 0, 0, 1),
        '8' => vec!(1, 0, 0, 0),
        '7' => vec!(0, 1, 1, 1),
        '6' => vec!(0, 1, 1, 0),
        '5' => vec!(0, 1, 0, 1),
        '4' => vec!(0, 1, 0, 0),
        '3' => vec!(0, 0, 1, 1),
        '2' => vec!(0, 0, 1, 0),
        '1' => vec!(0, 0, 0, 1),
        '0' => vec!(0, 0, 0, 0),
        _ => panic!("Invalid hex character")
    }
}

fn read_input(path: &str) -> Packet {
    let binary: Vec<usize> = fs::read_to_string(path)
        .expect("File path must be valid")
        .lines()
        .next()
        .unwrap()
        .chars()
        .flat_map(|hex_char| decode_hex(hex_char))
        .collect();

    binary_to_packet(&binary[..])
}

fn binary_to_packet(binary: &[usize]) -> Packet {
    let version = binary_to_int(&binary[0..3]);

    match binary_to_int(&binary[3..6]) {
        4 => {
            let (literal, length) = binary_to_literal(&binary[6..]);
            Packet::Literal { version, literal, length }
        },
        _ => {
            let length_type_id = binary[6];
            let mut contents = Vec::new();

            match length_type_id {
                0 => {
                    let total_bit_length = binary_to_int(&binary[7..22]);
                    let mut starting_index = 22;
                    let mut bits_digested = 0;

                    while bits_digested < total_bit_length {
                        let packet = binary_to_packet(&binary[starting_index..]);
                        let packet_length = packet.len();

                        starting_index += packet_length;
                        bits_digested += packet_length;

                        contents.push(packet);
                    }
                },
                1 => {
                    let packet_count = binary_to_int(&binary[7..18]);
                    let mut starting_index = 18;

                    for _ in 0..packet_count {
                        let packet = binary_to_packet(&binary[starting_index..]);
                        let packet_length = packet.len();

                        starting_index += packet_length;
                        contents.push(packet);
                    }
                },
                _ => panic!()
            };

            Packet::Operator { version, length_type_id, contents }
        }
    }
}

fn binary_to_literal(binary: &[usize]) -> (usize, usize) {
    let mut i = 0;
    let mut output = 0;

    while binary[i] == 1 {
        output = (output << 4) + binary_to_int(&binary[(i+1)..(i+5)]);
        i += 5;
    }

    ((output << 4) + binary_to_int(&binary[(i+1)..(i+5)]), i + 5)
}

fn binary_to_int(binary: &[usize]) -> usize {
    binary.iter().fold(0, |acc, x| (acc << 1) + x)
}

fn sum_versions(packet: &Packet) -> usize {
    match packet {
        Packet::Operator { version, contents, .. } =>
            *version + contents.iter().map(|p| sum_versions(p)).sum::<usize>(),
        Packet::Literal { version, .. } => *version
    }
}

fn main() {
    let input = read_input("input");
    println!("Day 16 Part 1: {}", sum_versions(&input));
}

#[test]
fn test_part_one() {
    let input_one = read_input("test1");
    assert_eq!(sum_versions(&input_one), 16);

    let input_two = read_input("test2");
    assert_eq!(sum_versions(&input_two), 12);

    let input_three = read_input("test3");
    assert_eq!(sum_versions(&input_three), 23);

    let input_four = read_input("test4");
    assert_eq!(sum_versions(&input_four), 31);
}
