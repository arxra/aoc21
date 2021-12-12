type File = Vec<Vec<u32>>;
type Map = Vec<usize>;

#[allow(unused_variables)]
fn solve(file: &mut File, p1: bool, steps: usize) -> usize {
    let mut counter = 0;
    for s in 0..steps {
        for y in 0..file.len() {
            for x in 0..file[y].len() {
                file[y][x] += 1;
            }
        }
        let mut changed = true;
        let mut flashes = 0;

        while changed {
            let mut add = vec![vec![0; file[0].len()]; file.len()];
            changed = false;
            for y in 0..file.len() {
                for x in 0..file[y].len() {
                    if file[y][x] > 9 {
                        file[y][x] = 0;
                        changed = true;
                        counter += 1;
                        flashes += 1;

                        if y > 0 && x > 0 {
                            add[y - 1][x - 1] += 1;
                        }
                        if y > 0 {
                            add[y - 1][x] += 1;
                        }
                        if y > 0 && x < file[y].len() - 1 {
                            add[y - 1][x + 1] += 1;
                        }
                        if x < file[y].len() - 1 {
                            add[y][x + 1] += 1;
                        }
                        if x < file[y].len() - 1 && y < file.len() - 1 {
                            add[y + 1][x + 1] += 1;
                        }
                        if y < file.len() - 1 {
                            add[y + 1][x] += 1;
                        }
                        if y < file.len() - 1 && x > 0 {
                            add[y + 1][x - 1] += 1;
                        }
                        if x > 0 {
                            add[y][x - 1] += 1;
                        }
                    }
                }
            }
            if flashes == 100 && !p1 {
                return s + 1;
            }
            for y in 0..file.len() {
                for x in 0..file[y].len() {
                    if file[y][x] != 0 {
                        file[y][x] += add[y][x];
                    }
                }
            }
        }
    }

    counter
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
    let mut f = file("small/d11");
    assert_eq!(solve(&mut f, true, 100), 1656);
}

#[test]
fn test_small_3() {
    let mut f = file("small/d11");
    assert_eq!(solve(&mut f, true, 10), 204);
}

#[test]
fn test_small_2() {
    let mut f = file("small/d11");
    assert_eq!(solve(&mut f, false, 10000), 195);
}

#[test]
fn s1() {
    let mut f = file("input/d11");
    println!("d11-1: {}", solve(&mut f, true, 100));
}
#[test]
fn s2() {
    let mut f = file("input/d11");
    println!("d11-2: {}", solve(&mut f, false, 1000000));
}
