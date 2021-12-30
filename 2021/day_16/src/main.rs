use std::{collections::HashMap, vec};

const INPUT: &str = "620D79802F60098803B10E20C3C1007A2EC4C84136F0600BCB8AD0066E200CC7D89D0C4401F87104E094FEA82B0726613C6B692400E14A305802D112239802125FB69FF0015095B9D4ADCEE5B6782005301762200628012E006B80162007B01060A0051801E200528014002A118016802003801E2006100460400C1A001AB3DED1A00063D0E25771189394253A6B2671908020394359B6799529E69600A6A6EB5C2D4C4D764F7F8263805531AA5FE8D3AE33BEC6AB148968D7BFEF2FBD204CA3980250A3C01591EF94E5FF6A2698027A0094599AA471F299EA4FBC9E47277149C35C88E4E3B30043B315B675B6B9FBCCEC0017991D690A5A412E011CA8BC08979FD665298B6445402F97089792D48CF589E00A56FFFDA3EF12CBD24FA200C9002190AE3AC293007A0A41784A600C42485F0E6089805D0CE517E3C493DC900180213D1C5F1988D6802D346F33C840A0804CB9FE1CE006E6000844528570A40010E86B09A32200107321A20164F66BAB5244929AD0FCBC65AF3B4893C9D7C46401A64BA4E00437232D6774D6DEA51CE4DA88041DF0042467DCD28B133BE73C733D8CD703EE005CADF7D15200F32C0129EC4E7EB4605D28A52F2C762BEA010C8B94239AAF3C5523CB271802F3CB12EAC0002FC6B8F2600ACBD15780337939531EAD32B5272A63D5A657880353B005A73744F97D3F4AE277A7DA8803C4989DDBA802459D82BCF7E5CC5ED6242013427A167FC00D500010F8F119A1A8803F0C62DC7D200CAA7E1BC40C7401794C766BB3C58A00845691ADEF875894400C0CFA7CD86CF8F98027600ACA12495BF6FFEF20691ADE96692013E27A3DE197802E00085C6E8F30600010882B18A25880352D6D5712AE97E194E4F71D279803000084C688A71F440188FB0FA2A8803D0AE31C1D200DE25F3AAC7F1BA35802B3BE6D9DF369802F1CB401393F2249F918800829A1B40088A54F25330B134950E0";

#[derive(Debug, PartialEq)]
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
    version: usize,
    type_id: TypeId,
    operator: Option<Operator>,
    sub_packets: Vec<Packet>,
    number: Option<usize>,
}

impl Packet {
    fn from_binary(data: &[char]) -> (Self, usize) {
        let version = chars_to_num(&data.iter().take(3).copied().collect::<Vec<char>>());
        let type_id = chars_to_num(&data.iter().skip(3).take(3).copied().collect::<Vec<char>>());
        let type_id = match type_id {
            4 => TypeId::Literal,
            _ => TypeId::Operator,
        };

        dbg!(data);

        let mut next = 0;

        let (operator, sub_packets, number) = match type_id {
            TypeId::Literal => {
                let (number, new_next) = Self::get_number(&data[6..]);
                next = new_next;

                (None, vec![], Some(number))
            },
            TypeId::Operator => {
                let op = data[6];
                match op {
                    '0' => {
                        // 15 next bits = length
                        dbg!(&data);
                        let length = chars_to_num(&data[7..22]);

                        (Some(Operator::Length(length)), Self::get_packets(&data[22..], Operator::Length(length)), None)
                    }
                    '1' => {
                        // 11 next bits = number
                        let number = chars_to_num(&data[7..18]);

                        (Some(Operator::Number(number)), Self::get_packets(&data[18..], Operator::Number(number)), None)
                    }
                    _ => panic!("Invalid char"),
                }
            }
        };

        (Self {
            version,
            type_id,
            operator,
            sub_packets,
            number,
        },
        next)
    }

    fn get_packets(bits: &[char], operator: Operator) -> Vec<Self> {
        let mut packets = Vec::new();

        match operator {
            Operator::Length(n) => {
                // Length of bits
                let mut next = 0;

                while next < n {
                    let (packet, new_next) = Self::from_binary(&bits[next..]);
                    next += 6 + new_next; // Add version and type ID
                    // dbg!(&packet, next);

                    packets.push(packet);
                }
            }
            Operator::Number(n) => {
                // Number of sub packets
                let mut next = 0;

                for _ in 0..n {
                    let (packet, new_next) = Self::from_binary(&bits[next..]);
                    next += 6 + new_next; // Add version and type ID
                    // dbg!(&packet, next);

                    packets.push(packet);
                }
            }
        }
        
        packets
    }

    fn get_number(bits: &[char]) -> (usize, usize) {
        let mut header = bits.first().unwrap();
        let mut result = Vec::new();
        let mut index = 1;

        while *header != '0' {
            for _ in 0..4 {
                result.push(bits[index]);
                index += 1;
            }
            header = &bits[index];
            index += 1;
        }

        for _ in 0..4 {
            result.push(bits[index]);
            index += 1;
        }
        
        (chars_to_num(&result), index)
    }
}

fn chars_to_num(c: &[char]) -> usize {
    let c: String = c.iter().collect();
    usize::from_str_radix(&c, 2).unwrap()
}

fn main() {
    println!("Part 1 result: {}", part1(get_data(INPUT)));
    println!("Part 2 result: {}", part2(get_data(INPUT)));
}

fn part1(input: Vec<char>) -> usize {
    let (packet, _) = Packet::from_binary(&input);
    dbg!(&packet);

    let mut sum = 0;
    let mut list = Vec::new();
    list.push(packet);
    
    while let Some(packet) = list.pop() {
        sum += packet.version;

        if packet.type_id == TypeId::Operator {
            // TODO: Compute sub packets
        }
    }

    sum
}

fn part2(_input: Vec<char>) -> usize {
    0
}

#[test]
fn test_part1() {
    // assert_eq!(6, part1(get_data("D2FE28")));
    // assert_eq!(9, part1(get_data("38006F45291200")));
    // assert_eq!(14, part1(get_data("EE00D40C823060")));
    // assert_eq!(16, part1(get_data("8A004A801A8002F478")));

    // 01100010000000001000000000000000000101100001000101010110001011001000100000000010000100011000111000110100
    // v = 3, t = 0, number = 2 =>
    //   sub => 000 000 0 000000000010110 0001000101010110001011 | 001 000 1 00000000010 000 100 01100 011 100 01101 00
    //      v = 0, t = 0, length (000000000010110) = 22
    //         sub => 000 100 0 1010 || 101 100 0 1011
    //             v = 0, t = 4, value = 10
    //             v = 5, t = 4, value = 11
    //      v = 1, t = 0, number = 2
    //         sub => 000 100 0 1100 || 011 100 0 1101 || 00
    //             v = 0, t = 4, value = 12
    //             v = 3, t = 4, value = 13
    assert_eq!(12, part1(get_data("620080001611562C8802118E34")));
    assert_eq!(23, part1(get_data("C0015000016115A2E0802F182340")));
    assert_eq!(31, part1(get_data("A0016C880162017C3686B18A3D4780")));
    assert_eq!(0, part1(get_data(INPUT)));
}

#[test]
fn test_part2() {
    // assert_eq!(168, part2(get_data("test.txt")));
    // assert_eq!(97164301, part2(get_data(INPUT)));
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
