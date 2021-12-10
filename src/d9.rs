use std::collections::HashMap;

type File = Vec<Vec<u32>>;
type Map = HashMap<(usize, usize), (usize, usize)>;

fn check(x: usize, y: usize, map: &mut Map, file: &File) -> Option<u32> {
    if map.contains_key(&(y, x)) {
        return None;
    }

    if file[y][x] == 9 {
        return None;
    }
    match (x, y) {
        (0, 0) => {
            if file[0][1] < file[0][0] {
                return check(x + 1, y, map, &file);
            } else if file[1][0] < file[0][0] {
                return check(x, y + 1, map, &file);
            }
        }
        (x, 0) if file[0].len() > x + 1 => {
            if file[0][x + 1] < file[0][x] {
                return check(x + 1, y, map, &file);
            } else if file[0][x - 1] < file[0][x] {
                return check(x - 1, y, map, &file);
            } else if file[1][x] < file[0][x] {
                return check(x, 1, map, &file);
            }
        }
        (x, 0) => {
            if file[0][x - 1] < file[0][x] {
                return check(x - 1, y, map, &file);
            } else if file[y + 1][x] < file[0][x] {
                return check(x, y + 1, map, &file);
            }
        }
        (0, y) if file.len() > y + 1 => {
            if file[y + 1][0] < file[y][0] {
                return check(x, y + 1, map, &file);
            } else if file[y - 1][0] < file[y][0] {
                return check(x, y - 1, map, &file);
            } else if file[y][1] < file[y][0] {
                return check(x + 1, y, map, &file);
            }
        }
        (0, y) => {
            if file[y - 1][0] < file[y][0] {
                return check(x, y - 1, map, &file);
            }
        }
        (x, y) if file[0].len() > x + 1 && file.len() > y + 1 => {
            if file[y + 1][x] < file[y][x] {
                return check(x, y + 1, map, &file);
            } else if file[y - 1][x] < file[y][x] {
                return check(x, y - 1, map, &file);
            } else if file[y][x - 1] < file[y][x] {
                return check(x - 1, y, map, &file);
            } else if file[y][x + 1] < file[y][x] {
                return check(x + 1, y, map, &file);
            }
        }
        (x, y) if file[0].len() > x + 1 => {
            if file[y - 1][x] < file[y][x] {
                return check(x, y - 1, map, &file);
            } else if file[y][x - 1] < file[y][x] {
                return check(x - 1, y, map, &file);
            } else if file[y][x + 1] < file[y][x] {
                return check(x + 1, y, map, &file);
            }
        }
        (x, y) if file.len() > y + 1 => {
            if file[y + 1][x] < file[y][x] {
                return check(x, y + 1, map, &file);
            } else if file[y - 1][x] < file[y][x] {
                return check(x, y - 1, map, &file);
            } else if file[y][x - 1] < file[y][x] {
                return check(x - 1, y, map, &file);
            }
        }
        (x, y) => {
            if file[y - 1][x] < file[y][x] {
                return check(x, y - 1, map, &file);
            } else if file[y][x - 1] < file[y][x] {
                return check(x - 1, y, map, &file);
            }
        }
    }
    map.insert((y, x), (y, x));
    Some(file[y][x])
}

#[allow(unused_variables)]
fn solve(file: File, p1: bool) -> u32 {
    let mut visited: Map = HashMap::new();
    let mut found: Vec<u32> = Vec::new();

    let y = file.len();
    let x = file[0].len();

    for row in 0..y {
        for v in 0..x {
            if let Some(v) = check(v, row, &mut visited, &file) {
                println!("Found down point: {}", v);
                found.push(v);
            }
        }
    }
    if p1 {
        found.into_iter().fold(0, |res, item| res + item + 1)
    } else {
        let mut cm: HashMap<(usize, usize), u32> = HashMap::new();
        visited
            .into_values()
            .for_each(|pair| *cm.entry(pair).or_insert(0) += 1);
        let mut v = cm.into_values().collect::<Vec<u32>>();
        v.sort();
        println!("v: {:?}", v);
        v.into_iter().rev().take(3).fold(1, |res, item| res * item)
    }
}

fn file(path: &str) -> File {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.chars().filter_map(|a| a.to_digit(10)).collect())
        .collect()
}

#[test]
fn test_small_1() {
    let f = file("small/d9");
    assert_eq!(solve(f, true), 15);
}

#[test]
fn test_small_2() {
    let f = file("small/d9");
    assert_eq!(solve(f, false), 1134);
}

#[test]
fn s1() {
    let f = file("input/d9");
    println!("d9-1: {}", solve(f, true));
}
#[test]
fn s2() {
    let f = file("input/d9");
    println!("d9-2: {}", solve(f, false));
}
