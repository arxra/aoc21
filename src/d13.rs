type File = Vec<Vec<usize>>;
type Map = Vec<usize>;

#[allow(unused_variables)]
fn solve(file: &mut File, folds: usize) -> usize {
    0
}

fn file(path: &str) -> File {
    let mut points = Vec::new();
    let mut folds = Vec::new();

    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.split(',').collect())
        .for_each(|a: Vec<&str>| match a.len() {
            2 => {
                points.push((
                    a[0].parse::<usize>().unwrap(),
                    a[1].parse::<usize>().unwrap(),
                ));
            }
            1 => {
                // folds.push(a.split_at(11).1.dbg!)
            }
            _ => (),
        });
    folds
}

#[test]
fn test_small_1() {
    let mut f = file("small/d13");
    assert_eq!(solve(&mut f, 1), 17);
}

#[test]
fn test_small_2() {
    let mut f = file("small/d13");
    assert_eq!(solve(&mut f, 0), 36);
}

#[test]
fn s1() {
    let mut f = file("input/d12");
    let ans = solve(&mut f, 1);
    println!("d12-1: {}", ans);
    // assert_eq!(ans, 5252);
}
#[test]
fn s2() {
    let mut f = file("input/d12");
    println!("d12-2: {}", solve(&mut f, 1));
}
