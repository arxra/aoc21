fn solve(list: &Vec<usize>, days: usize) -> usize {
    let mut vals: Vec<usize> = vec![0; 10];
    for a in list {
        vals[*a] += 1;
    }
    for _ in 0..days {
        for v in 0..vals.len() {
            vals[v] = match v {
                0 => {
                    vals[9] = vals[0];
                    vals[7] += vals[0];
                    vals[1]
                }
                9 => 0,
                i => vals[i + 1],
            }
        }
    }
    vals.into_iter().sum()
}
fn file() -> Vec<usize> {
    std::fs::read_to_string("input/d6")
        .unwrap()
        .split(',')
        .filter_map(|v| v.parse::<usize>().ok())
        .collect()
}

#[test]
fn p1() {
    println!("d6-1: {}", solve(&mut file(), 80));
}

#[test]
fn p2() {
    println!("d6-2: {}", solve(&mut file(), 256));
}
