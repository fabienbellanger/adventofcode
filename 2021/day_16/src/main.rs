use std::collections::HashMap;

#[derive(Debug)]
enum TypeId {
    Literal,
    Operator,
}

#[derive(Debug)]
enum Operator {
    Length(usize),
    Number(usize),
}

#[derive(Debug)]
struct Packet {
    Version: usize,
    TypeId: TypeId,
    Operator: Option<Operator>,
    SubPackets: Vec<char>,
}

impl Packet {
    fn from_binary(data: &[char]) -> Self {
        let version = chars_to_num(&data.iter().take(3).map(|c| *c).collect::<Vec<char>>());
        let type_id = chars_to_num(&data.iter().skip(3).take(3).map(|c| *c).collect::<Vec<char>>());
        let type_id = match type_id {
            4 => TypeId::Literal,
            _ => TypeId::Operator,
        };
        let (operator, sub_packets) = match type_id {
            TypeId::Literal => (None, &data[6..]),
            TypeId::Operator => {
                let op = data[6];
                match op {
                    '0' => {
                        // 15 next bits = length
                        (Some(Operator::Length(chars_to_num(&data[7..22]))), &data[22..])
                    }
                    '1' => {
                        // 11 next bits = number
                        (Some(Operator::Number(chars_to_num(&data[7..18]))), &data[18..])
                    }
                    _ => panic!("Invalid char"),
                }
            }
        };

        Self {
            Version: version,
            TypeId: type_id,
            Operator: operator,
            SubPackets: sub_packets.to_vec(),
        }
    }
}

fn chars_to_num(c: &[char]) -> usize {
    let c: String = c.iter().collect();
    usize::from_str_radix(&c, 2).unwrap()
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("")));
    // println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(input: Vec<char>) -> usize {
    let packet = Packet::from_binary(&input);
    dbg!(&packet);

    0
}

fn part2() -> usize {
    0
}

#[test]
fn test_part1() {
    assert_eq!(6, part1(get_data("D2FE28")));
    assert_eq!(9, part1(get_data("38006F45291200")));
    assert_eq!(14, part1(get_data("EE00D40C823060")));
    assert_eq!(16, part1(get_data("8A004A801A8002F478")));
    assert_eq!(12, part1(get_data("620080001611562C8802118E34")));
    assert_eq!(23, part1(get_data("C0015000016115A2E0802F182340")));
    assert_eq!(31, part1(get_data("A0016C880162017C3686B18A3D4780")));
    // assert_eq!(344297, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    // assert_eq!(168, part2(get_data("test.txt")));
    // assert_eq!(97164301, part2(get_data("input.txt")));
}

fn get_data(input: &str) -> Vec<char> {
    let input = input.trim();

    let mut hex_hash: HashMap<char, Vec<char>> = HashMap::with_capacity(16);
    hex_hash.insert('0', vec!['0', '0', '0', '0']);
    hex_hash.insert('1', vec!['0', '0', '0', '1']);
    hex_hash.insert('2', vec!['0', '0', '1', '0']);
    hex_hash.insert('3', vec!['0', '0', '1', '1']);
    hex_hash.insert('4', vec!['0', '1', '0', '0']);
    hex_hash.insert('5', vec!['0', '1', '0', '1']);
    hex_hash.insert('6', vec!['0', '1', '1', '0']);
    hex_hash.insert('7', vec!['0', '1', '1', '1']);
    hex_hash.insert('8', vec!['1', '0', '0', '0']);
    hex_hash.insert('9', vec!['1', '0', '0', '1']);
    hex_hash.insert('A', vec!['1', '0', '1', '0']);
    hex_hash.insert('B', vec!['1', '0', '1', '1']);
    hex_hash.insert('C', vec!['1', '1', '0', '0']);
    hex_hash.insert('D', vec!['1', '1', '0', '1']);
    hex_hash.insert('E', vec!['1', '1', '1', '0']);
    hex_hash.insert('F', vec!['1', '1', '1', '1']);

    input
        .chars()
        .map(|c| hex_hash.get(&c).unwrap().clone())
        .flatten()
        .collect()
}
