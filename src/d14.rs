use hashbrown::HashMap;

type File = (String, HashMap<String, char>);
type Map = Vec<usize>;

#[allow(unused_variables)]
fn solve(file: &mut File, iter: usize) -> usize {
    for i in 0..iter {
        file.0.reserve(file.0.len());
        for index in (0..file.0.len() - 1).rev() {
            if let Some(ch) = file.1.get(&file.0[index..=index + 1]) {
                file.0.insert(index+1, *ch);
            }
        }
        println!("after step {}", i+1);
    }

    let mut counts: HashMap<char, usize> = HashMap::new();
    for a in file.0.chars() {
        match counts.get_mut(&a) {
            Some(a) => *a += 1,
            None => {
                counts.insert(a, 1);
            }
        }
    }
    let mut min = usize::MAX;
    let mut max = usize::MIN;
    counts.values().for_each(|v| {
        if *v < min {
            min = *v
        } else if *v > max {
            max = *v
        }
    });
    // println!("Max: {}\tMin: {}", max, min);

    max - min
}

fn file(path: &str) -> File {
    let rows: Vec<Vec<String>> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|a| a.to_string())
                .filter(|a| a != "->")
                .collect()
        })
        .collect();

    let start = rows.get(0).unwrap().get(0).unwrap().clone();
    let mut map = HashMap::new();

    for ele in rows {
        if ele.len() > 1 {
            map.insert(ele[0].clone(), ele[1].clone().chars().last().unwrap());
        }
    }

    (start.clone(), map)
}

#[test]
fn test_small_1() {
    let mut f = file("small/d14");
    assert_eq!(solve(&mut f, 10), 1588);
}

#[test]
fn test_small_2() {
    let mut f = file("small/d14");
    assert_eq!(solve(&mut f, 40), 2188189693529);
}

#[test]
fn s1() {
    let mut f = file("input/d14");
    let ans = solve(&mut f, 10);
    println!("d14-1: {}", ans);
    assert_eq!(ans, 3408);
}
#[test]
fn s2() {
    let mut f = file("input/d14");
    println!("d14-2: {}", solve(&mut f, 40));
}
