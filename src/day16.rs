pub fn run() {
    let input = puzzle_input();
    println!("day16.part1.solution = {}", solve_part1(input));
    println!("day16.part2.solution = {}", solve_part2(input));
}

fn solve_part1(input: &str) -> u64 {
    let packet = packet_from_hex(input);
    packet.sum_versions()
}

fn solve_part2(input: &str) -> u64 {
    let packet = packet_from_hex(input);
    packet.execute()
}

fn puzzle_input() -> &'static str {
    include_str!("../inputs/day16.txt").trim()
}

#[derive(Debug, PartialEq)]
enum Packet {
    Literal(u8, u8, u64),
    Operator(u8, u8, Vec<Packet>),
}

impl Packet {
    pub fn sum_versions(&self) -> u64 {
        match self {
            Packet::Literal(v, _, _) => *v as u64,
            Packet::Operator(v, _, packets) => {
                packets.iter().map(|p| p.sum_versions()).sum::<u64>() + (*v as u64)
            }
        }
    }

    pub fn execute(&self) -> u64 {
        match self {
            Packet::Literal(_, _, v) => *v,
            Packet::Operator(_, t, packets) => {
                let values: Vec<_> = packets.iter().map(|p| p.execute()).collect();

                match t {
                    0 => values.iter().sum(),
                    1 => values.iter().product(),
                    2 => *values.iter().min().unwrap(),
                    3 => *values.iter().max().unwrap(),
                    5 => (values[0] > values[1]) as u64,
                    6 => (values[0] < values[1]) as u64,
                    7 => (values[0] == values[1]) as u64,
                    _ => panic!("unknown operator type"),
                }
            }
        }
    }
}

fn packet_from_hex(input: &str) -> Packet {
    let bits: Vec<bool> = input.chars().flat_map(hex_to_bits).collect();
    let (packet, _) = packet_from_binary(&bits);
    packet
}

fn packet_from_binary(bits: &[bool]) -> (Packet, usize) {
    let version = bits_to_int(&bits[0..3]) as u8;
    let id = bits_to_int(&bits[3..6]) as u8;

    if id == 4 {
        let mut n = 0;
        let mut offset = 6;

        while bits[offset] {
            n <<= 4;
            n |= bits_to_int(&bits[offset + 1..offset + 5]);
            offset += 5;
        }

        n <<= 4;
        n |= bits_to_int(&bits[offset + 1..offset + 5]);
        offset += 5;

        return (Packet::Literal(version, id, n), offset);
    }

    let contains_num_subpackets = bits[6];

    if !contains_num_subpackets {
        let subpacket_length = bits_to_int(&bits[7..(7 + 15)]) as usize;
        let mut offset: usize = 7 + 15;
        let end = subpacket_length + 7 + 15;

        let mut packets = vec![];

        while offset < end {
            let (packet, bits_read) = packet_from_binary(&bits[offset..end]);
            packets.push(packet);
            offset += bits_read;
        }

        (Packet::Operator(version, id, packets), offset)
    } else {
        let num_packets = bits_to_int(&bits[7..(7 + 11)]) as usize;
        let mut offset: usize = 7 + 11;
        let mut packets = vec![];

        for _ in 0..num_packets {
            let (packet, bits_read) = packet_from_binary(&bits[offset..]);
            packets.push(packet);
            offset += bits_read;
        }

        (Packet::Operator(version, id, packets), offset)
    }
}

fn bits_to_int(bits: &[bool]) -> u64 {
    let mut n = 0;
    for &bit in bits {
        n <<= 1;
        n |= bit as u64;
    }
    n
}

fn hex_to_bits(c: char) -> [bool; 4] {
    let mut bits = [false, false, false, false];
    let digit = c.to_digit(16).unwrap();

    for (idx, bit) in bits.iter_mut().enumerate() {
        *bit = (digit & (1 << (3 - idx))) > 0;
    }

    bits
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_int_from_bits() {
        assert_eq!(6, bits_to_int(&[true, true, false]));
    }

    #[test]
    fn parses_bits_from_hex() {
        assert_eq!([true, false, true, true], hex_to_bits('B'));
    }

    #[test]
    fn parses_literal() {
        assert_eq!(Packet::Literal(6, 4, 2021), packet_from_hex("D2FE28"));
    }

    #[test]
    fn parses_operators() {
        assert_eq!(
            Packet::Operator(
                1,
                6,
                vec![Packet::Literal(6, 4, 10), Packet::Literal(2, 4, 20)]
            ),
            packet_from_hex("38006F45291200")
        );
        assert_eq!(
            Packet::Operator(
                7,
                3,
                vec![
                    Packet::Literal(2, 4, 1),
                    Packet::Literal(4, 4, 2),
                    Packet::Literal(1, 4, 3)
                ]
            ),
            packet_from_hex("EE00D40C823060")
        );
    }

    #[test]
    fn verify_example_input_part1() {
        assert_eq!(16, solve_part1("8A004A801A8002F478"));
        assert_eq!(12, solve_part1("620080001611562C8802118E34"));
        assert_eq!(23, solve_part1("C0015000016115A2E0802F182340"));
        assert_eq!(31, solve_part1("A0016C880162017C3686B18A3D4780"));
    }

    #[test]
    fn verify_example_input_part2() {
        assert_eq!(3, solve_part2("C200B40A82"));
        assert_eq!(54, solve_part2("04005AC33890"));
        assert_eq!(7, solve_part2("880086C3E88112"));
        assert_eq!(9, solve_part2("CE00C43D881120"));
        assert_eq!(1, solve_part2("D8005AC2A8F0"));
        assert_eq!(0, solve_part2("F600BC2D8F"));
        assert_eq!(0, solve_part2("9C005AC2F8F0"));
        assert_eq!(1, solve_part2("9C0141080250320F1802104A08"));
    }
}
