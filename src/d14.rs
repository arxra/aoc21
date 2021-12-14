use hashbrown::HashMap;

type File = HashMap<String, (char, usize)>;
type Count = HashMap<char, usize>;

#[allow(unused_variables)]
fn solve(file: &mut File, counts: &mut Count, iter: usize) -> usize {
    for i in 0..iter {
        for (key, (c, count)) in file.clone().iter().filter(|(_, (_, count))| *count > 0) {
            let start = format!("{}{}", key.chars().nth(0).unwrap(), c);
            let end = format!("{}{}", c, key.chars().nth(1).unwrap());

            if !start.eq(key) {
                file.get_mut(&start).unwrap().1 += count;
            }
            if !end.eq(key) && !start.eq(&end) {
                file.get_mut(&end).unwrap().1 += count;
            }

            if !end.eq(key) && !start.eq(key) {
                file.get_mut(key).unwrap().1 -= count;
            }

            match counts.get_mut(&c) {
                Some(a) => *a += count,
                None => {
                    counts.insert(*c, *count);
                }
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

    // println!("Max: {}\tMin: {}: {}", max, min, max - min);
    max - min
}

fn file(path: &str) -> (File, Count) {
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
    let mut count = HashMap::new();

    for ele in rows {
        if ele.len() > 1 {
            map.insert(ele[0].clone(), (ele[1].clone().chars().last().unwrap(), 0));
        }
    }
    for index in 0..start.len() - 1 {
        map.get_mut(&start[index..=index + 1].to_string())
            .unwrap()
            .1 += 1;
    }
    start.chars().for_each(|c| match count.get_mut(&c) {
        Some(a) => *a += 1,
        None => {
            count.insert(c, 1);
        }
    });
    println!("{:?}", count);

    (map, count)
}

#[test]
fn test_small_1() {
    let (mut f, mut count) = file("small/d14");
    assert_eq!(solve(&mut f, &mut count, 10), 1588);
}

#[test]
fn test_small_2() {
    let (mut f, mut count) = file("small/d14");
    assert_eq!(solve(&mut f, &mut count, 40), 2188189693529);
}

#[test]
fn s1() {
    let (mut f, mut count) = file("input/d14");
    let ans = solve(&mut f, &mut count, 10);
    println!("d14-1: {}", ans);
    assert_eq!(ans, 3408);
}
#[test]
fn s2() {
    let (mut f, mut count) = file("input/d14");
    println!("d14-2: {}", solve(&mut f, &mut count, 40));
    // 3056708530994 too low
}
