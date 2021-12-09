use std::collections::HashMap;

type File = Vec<Vec<u32>>;
type Map = HashMap<(usize, usize), bool>;

fn check(x: usize, y: usize, map: &mut Map, file: &File) -> Option<u32> {
    if map.contains_key(&(y, x)) {
        return None;
    } else {
        map.insert((y, x), true);
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
            } else {
                return Some(file[y][x]);
            }
        }
        (x, 0) if file[0].len() > x + 1 => {
            if file[0][x + 1] < file[0][x] {
                return check(x + 1, y, map, &file);
            } else if file[0][x - 1] < file[0][x] {
                return check(x - 1, y, map, &file);
            } else if file[1][x] < file[0][x] {
                return check(x, 1, map, &file);
            } else {
                println!("checking x,0 x in range");
                return Some(file[y][x]);
            }
        }
        (x, 0) => {
            if file[0][x - 1] < file[0][x] {
                return check(x - 1, y, map, &file);
            } else if file[y + 1][x] < file[0][x] {
                return check(x, y + 1, map, &file);
            } else {
                return Some(file[y][x]);
            }
        }
        (0, y) if file.len() > y + 1 => {
            if file[y + 1][0] < file[y][0] {
                return check(x, y + 1, map, &file);
            } else if file[y - 1][0] < file[y][0] {
                return check(x, y - 1, map, &file);
            } else if file[y][1] < file[y][0] {
                return check(x + 1, y, map, &file);
            } else {
                println!("checking 0,y y in range");
                return Some(file[y][x]);
            }
        }
        (0, y) => {
            if file[y - 1][0] < file[y][0] {
                return check(x, y - 1, map, &file);
            } else {
                println!("checking 0,y after all ranges");
                return Some(file[y][x]);
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
            } else {
                println!("checking x,y where x & y in range");
                return Some(file[y][x]);
            }
        }
        (x, y) if file[0].len() > x + 1 => {
            if file[y - 1][x] < file[y][x] {
                return check(x, y - 1, map, &file);
            } else if file[y][x - 1] < file[y][x] {
                return check(x - 1, y, map, &file);
            } else if file[y][x + 1] < file[y][x] {
                return check(x + 1, y, map, &file);
            } else {
                println!("checked x,y where x in range");
                return Some(file[y][x]);
            }
        }
        (x, y) if file.len() > y + 1 => {
            if file[y + 1][x] < file[y][x] {
                return check(x, y + 1, map, &file);
            } else if file[y - 1][x] < file[y][x] {
                return check(x, y - 1, map, &file);
            } else if file[y][x - 1] < file[y][x] {
                return check(x - 1, y, map, &file);
            } else {
                println!("checking x,y where y in range");
                return Some(file[y][x]);
            }
        }
        (x, y) => {
            if file[y - 1][x] < file[y][x] {
                return check(x, y - 1, map, &file);
            } else if file[y][x - 1] < file[y][x] {
                return check(x - 1, y, map, &file);
            } else {
                println!("checking x,y after all ranges");
                return Some(file[y][x]);
            }
        }
    }
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
    found.into_iter().fold(0, |res, item| res + item + 1)
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
