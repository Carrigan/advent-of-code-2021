use std::fs;

#[derive(Debug)]
enum Packet {
    Operator {
        version: usize,
        packet_type: usize,
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
    let file = fs::read_to_string(path)
        .expect("File path must be valid");

    let hex_string = file
        .lines()
        .next()
        .unwrap();

    hex_to_packet(hex_string)
}

fn hex_to_packet(input: &str) -> Packet {
    let binary: Vec<usize> = input
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
        packet_type => {
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

            Packet::Operator { version, length_type_id, contents, packet_type }
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

fn compute_packets(packet: &Packet) -> usize {
    match packet {
        Packet::Operator { packet_type, contents, .. } =>
            match packet_type {
                0 => contents.iter().map(|p| compute_packets(p)).sum(),
                1 => contents.iter().map(|p| compute_packets(p)).product(),
                2 => contents.iter().map(|p| compute_packets(p)).min().unwrap(),
                3 => contents.iter().map(|p| compute_packets(p)).max().unwrap(),
                5 => if compute_packets(&contents[0]) > compute_packets(&contents[1]) { 1 } else { 0 },
                6 => if compute_packets(&contents[0]) < compute_packets(&contents[1]) { 1 } else { 0 },
                7 => if compute_packets(&contents[0]) == compute_packets(&contents[1]) { 1 } else { 0 }
                _ => panic!()
            }
        Packet::Literal { literal, .. } => *literal
    }
}

fn main() {
    let input = read_input("input");
    println!("Day 16 Part 1: {}", sum_versions(&input));
    println!("Day 16 Part 1: {}", compute_packets(&input));
}

#[test]
fn test_part_one() {
    fn version_assert(hex_string: &str, val: usize) {
        let packet = hex_to_packet(hex_string);
        assert_eq!(sum_versions(&packet), val);
    }

    version_assert("8A004A801A8002F478", 16);
    version_assert("620080001611562C8802118E34", 12);
    version_assert("C0015000016115A2E0802F182340", 23);
    version_assert("A0016C880162017C3686B18A3D4780", 31);
}

#[test]
fn test_part_two() {
    fn compute_assert(hex_string: &str, val: usize) {
        let packet = hex_to_packet(hex_string);
        assert_eq!(compute_packets(&packet), val);
    }

    compute_assert("C200B40A82", 3);
    compute_assert("04005AC33890", 54);
    compute_assert("880086C3E88112", 7);
    compute_assert("CE00C43D881120", 9);
    compute_assert("D8005AC2A8F0", 1);
    compute_assert("F600BC2D8F", 0);
    compute_assert("9C005AC2F8F0", 0);
    compute_assert("9C0141080250320F1802104A08", 1);
}
