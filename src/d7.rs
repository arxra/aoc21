type File = Vec<usize>;

fn median(array: &Vec<usize>) -> f64 {
    if (array.len() % 2) == 0 {
        let ind_left = array.len() / 2 - 1;
        let ind_right = array.len() / 2;
        (array[ind_left] + array[ind_right]) as f64 / 2.0
    } else {
        array[(array.len() / 2)] as f64
    }
}

fn solve(file: File, p1: bool) -> usize {
    let med = if p1 {
        median(&file)
    } else {
        let sum: f64 = file.iter().sum::<usize>() as f64;
        println!(
            "sum:{}, len:{}, avg: {}",
            sum,
            file.len(),
            (sum / file.len() as f64).round() as usize
        );
        (sum / file.len() as f64).round()
    };

    let mut res = 0.0;
    if p1 {
        for f in file {
            // println!("res ({})+: {} - {}", res, med, f);
            res += (med - f as f64).abs()
        }
    } else {
        res = f64::MAX;
        let max: usize = file
            .iter()
            .fold(0, |res, item| if *item > res { *item } else { res });
         (1..=max).for_each(|i| {
            file
                .iter()
                .map(|f| {
                    (i.min(*f)..=i.max(*f))
                        .into_iter()
                        .fold(0, |res, item| res + item.abs_diff(i))
                })
                .fold(0, |res, item| res + item) as f64;
        });
    }
    res as usize
}

fn file(path: &str) -> File {
    let mut f: File = std::fs::read_to_string(path)
        .unwrap()
        .split(',')
        .filter_map(|v| v.parse().ok())
        .collect();
    f.sort();
    f
}

#[test]
fn test_small_1() {
    let f = file("small/d7");
    assert_eq!(solve(f, true), 37);
}

#[test]
fn test_small_2() {
    let f = file("small/d7");
    assert_eq!(solve(f, false), 168);
}

#[test]
fn s1() {
    let f = file("input/d7");
    println!("d7-1: {}", solve(f, true));
}
#[test]
fn s2() {
    let f = file("input/d7");
    println!("d7-2: {}", solve(f, false));
}
