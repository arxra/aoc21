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

fn solve(file: Cord, diags: bool) -> usize {
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
            (Some(x1), Some(y1), Some(x2), Some(y2)) if diags => {
                for i in 0..=x1.abs_diff(x2) {
                    if (x1 < x2) && y1 < y2 {
                        safe_add(&mut map, x1 + i, y1 + i)
                    } else if (x1 > x2) && y1 < y2 {
                        safe_add(&mut map, x1 - i, y1 + i)
                    } else if (x1 < x2) && y1 > y2 {
                        safe_add(&mut map, x1 + i, y1 - i)
                    } else if (x1 > x2) && y1 > y2 {
                        safe_add(&mut map, x1 - i, y1 - i)
                    }
                }
            }
            _ => (),
        }
    }
    map.values().filter(|v| **v > 1).count()
}

fn file() -> Cord {
    std::fs::read_to_string("input/d5")
        .unwrap()
        .lines()
        .map(|l| {
            l.split(&[',', ' '][..])
                .filter_map(|f| f.parse().ok())
                .collect()
        })
        .collect()
}

#[test]
fn p1() {
    println!("d5-1: {}", solve(file(), false));
}
#[test]
fn p2() {
    println!("d5-2: {}", solve(file(), true));
}
