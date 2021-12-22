use tracing::{debug, info};

#[derive(Debug, Clone)]
struct Packet {
    version: usize,
    ty: usize,
    result: usize,
    len: usize,
    subs: Vec<Self>,
}

impl Packet {
    fn new(packet: &str) -> Self {
        let version = usize::from_str_radix(&packet[0..3], 2).unwrap();
        let ty = usize::from_str_radix(&packet[3..6], 2).unwrap();
        let mut subs = Vec::new();
        let mut result = 0;
        let mut len = 6;
        info!(?version, ?ty, ?packet);

        if ty == 4 {
            debug!("Parsing literal!");
            (result, len) = parse_literal(packet[6..].as_bytes());
            debug!(result);
            len += 6;
        } else {
            debug!("Parsing operator");
            // 7 instrad of 6 for type bit compensation
            if packet.as_bytes()[len] as char == '0' {
                len += 1;
                // 15 bits for the number of contained bits
                let mut bits = usize::from_str_radix(&packet[len..len + 15], 2).unwrap();
                debug!("bits: {}", &packet[len..len + 15]);
                len += 15;
                debug!("Parsing ({}) bits: {}-{}", bits, len, len + bits);
                bits += len;
                while len < bits {
                    let pack = Packet::new(&packet[len..]);
                    len += pack.len;
                    debug!(?pack, len);
                    subs.push(pack);
                }
            } else {
                // 11 bits for the number of packets contained.
                len += 1;
                let packets = usize::from_str_radix(&packet[len..len + 11], 2).unwrap();
                debug!("### Parsing packets: {:?}", packets);
                len += 11;
                for _ in 0..packets {
                    let pack = Packet::new(&packet[len..]);
                    len += pack.len;
                    debug!(?pack, len);
                    subs.push(pack);
                }
                debug!("### Finished parsing packets")
            }
        }

        Self {
            version,
            ty,
            result,
            len,
            subs,
        }
    }
    fn versions(&self) -> usize {
        debug!(?self);
        self.subs
            .iter()
            .fold(self.version, |res, item| res + item.versions())
    }
}

fn parse_literal(packet: &[u8]) -> (usize, usize) {
    let mut point = 0;
    let mut res = String::new();

    while point < packet.len() {
        res.push_str(&(std::str::from_utf8(&packet[point + 1..=point + 4]).unwrap()));
        if packet[point] as char == '0' {
            return (
                usize::from_str_radix(&res, 2).unwrap(),
                point+5
            );
        }
        point += 5;
    }
    unreachable!("no value parsed correctly");
}

fn solve(data: &mut String) -> Packet {
    let packet: String = data
        .chars()
        .filter(|c| *c != ' ' && *c != '\n')
        .map(to_bin)
        .collect();
    info!("packet: {:?}: {}", packet, packet.len());

    Packet::new(&packet)
}

fn to_bin(c: char) -> &'static str {
    match c {
        '0' => "0000", //0000
        '1' => "0001", //0001
        '2' => "0010", //0010
        '3' => "0011", //0011
        '4' => "0100", //0100
        '5' => "0101", //0101
        '6' => "0110", //0110
        '7' => "0111", //0111
        '8' => "1000", //1000
        '9' => "1001", //1001
        'A' => "1010", //1010
        'B' => "1011", //1011
        'C' => "1100", //1100
        'D' => "1101", //1101
        'E' => "1110", //1110
        'F' => "1111", //1111
        c => unreachable!("{} is not a hexadecimal char", c),
    }
}

fn file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}

#[test_log::test]
fn test_small_1() {
    let mut f = file("small/d16");
    assert_eq!(solve(&mut f).versions(), 16);
}

#[test_log::test]
fn test_small_2() {
    let mut f = file("small/d16-2");
    assert_eq!(solve(&mut f).versions(), 31);
}

#[test_log::test]
fn test_small_3() {
    let mut f = file("small/d16-3");
    assert_eq!(solve(&mut f).versions(), 0);
}
// #[test]
// fn test_small_2() {
//     let mut f = file("small/d16");
//     assert_eq!(solve(&mut f), 0);
// }

#[test]
fn s1() {
    let mut f = file("input/d16");
    let ans = solve(&mut f);
    println!("d16-1: {}", ans.versions());
    // assert_eq!(ans, 5252);
}
// #[test]
// fn s2() {
//     let mut f = file("input/d16");
//     println!("d16-2: {}", solve(&mut f));
// }
