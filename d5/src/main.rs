#![feature(int_abs_diff)]
use std::collections::BTreeMap;

type Cord = Vec<Vec<usize>>;
type Map = BTreeMap<(usize, usize), usize>;

fn safe_add(map: &mut Map, x: usize, y: usize) {
    if let Some(e) = map.get_mut(&(x, y)) {
        *e += 1;
    } else {
        map.insert((x, y), 1);
    }
}

fn first(file: Cord) -> usize {
    let mut map = Map::new();
    let mut it = file.into_iter();
    while let Some(line) = it.next() {
        let mut line = line.into_iter().peekable();

        match (line.next(), line.next(), line.next(), line.next()) {
            (Some(x1), Some(y1), Some(x2), Some(y2)) if x1 == x2 => {
                for y in y1.min(y2)..=y2.max(y1) {
                    safe_add(&mut map, x1, y)
                }
            }
            (Some(x1), Some(y1), Some(x2), Some(y2)) if y1 == y2 => {
                for x in x1.min(x2)..=x2.max(x1) {
                    safe_add(&mut map, x, y1)
                }
            }
            _ => (),
        }
    }
    map.values().filter(|v| **v > 1).count()
}

fn second(file: Cord) -> usize {
    let mut map = Map::new();
    let mut it = file.into_iter();
    while let Some(line) = it.next() {
        let mut line = line.into_iter().peekable();

        match (line.next(), line.next(), line.next(), line.next()) {
            (Some(x1), Some(y1), Some(x2), Some(y2)) if x1 == x2 => {
                for y in y1.min(y2)..=y2.max(y1) {
                    safe_add(&mut map, x1, y)
                }
            }
            (Some(x1), Some(y1), Some(x2), Some(y2)) if y1 == y2 => {
                for x in x1.min(x2)..=x2.max(x1) {
                    safe_add(&mut map, x, y1)
                }
            }
            (Some(x1), Some(y1), Some(x2), Some(y2)) => {
                if let x = x1 > x2 {
                    (x2..=x1).rev()
                } else {
                    (x1..=x2)
                };
                for index in 0..=x1.abs_diff(x2) {
                    safe_add(&mut map, x, y)
                }
            }
            a => println!("unhandled line: {:?}", a),
        }
    }
    map.values().filter(|v| **v > 1).count()
}
fn main() {
    let file = std::fs::read_to_string("small").unwrap();
    let file: Cord = file
        .lines()
        .map(|l| {
            l.split(&[',', ' '][..])
                .filter_map(|f| f.parse().ok())
                .collect()
        })
        .collect();

    println!("s1: {}", first(file.clone()));
    println!("s2: {}", second(file));
}
