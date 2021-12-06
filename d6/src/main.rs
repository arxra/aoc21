use std::usize;

fn solve(list: &Vec<usize>, days: usize) -> usize {
    let mut vals: Vec<usize> = vec![0; 10];
    for a in list {
        vals[*a] += 1;
    }
    println!("vals: {:?}", vals);
    for d in 0..days {
        println!("Day {}, vals: {:?}", d, vals);
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

fn main() {
    let mut file = std::fs::read_to_string("input")
        .unwrap()
        .split(',')
        .filter_map(|v| v.parse::<usize>().ok())
        .collect();
    println!("input: {:?}", file);
    println!("s1: {}", solve(&mut file, 80));
    println!("s2: {}", solve(&mut file, 256));
}
