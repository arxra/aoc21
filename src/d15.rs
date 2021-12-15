use std::collections::BTreeMap;

type Map = Vec<Vec<usize>>;
type Path = Vec<(usize, usize)>;
type Nodes = BTreeMap<usize, Vec<(usize, usize)>>;

#[allow(unused_variables)]
fn solve(map: &mut Map) -> usize {
    let mut nodes = Nodes::new();
    nodes.insert(0, vec![(0, 0); 1]);
    let mut costs = vec![vec![usize::MAX; map[0].len()]; map.len()];

    while costs[map.len() - 1][map[0].len() - 1] == usize::MAX {
        djik(&mut nodes, &mut costs, &map);
    }

    costs[map.len() - 1][map[0].len() - 1]
}

fn file(path: &str) -> Map {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| {
            l.chars()
                .filter_map(|a| a.to_digit(10))
                .map(|a| a as usize)
                .collect()
        })
        .collect::<Map>()
        .into_iter()
        .filter(|a| a.len() > 0)
        .collect()
}
fn insert(nodes: &mut Nodes, map: &Map, value: usize, y: usize, x: usize) {
    let cost = value + map[y][x];
    if let Some(v) = nodes.get_mut(&(cost)) {
        v.push((y, x))
    } else {
        nodes.insert(cost, vec![(y, x); 1]);
    };
}

fn djik(nodes: &mut Nodes, costs: &mut Map, map: &Map){
    let (value, mut val) = nodes.pop_first().unwrap();
    let (y, x) = val.pop().unwrap();
    if val.len() != 0 {
        nodes.insert(value, val);
    }
    costs[y][x] = value;

    if y > 0 {
        if costs[y - 1][x] == usize::MAX {
            insert(nodes, map, value, y - 1, x);
        }
    }
    if x < map[y].len() - 1 {
        if costs[y][x + 1] == usize::MAX {
            insert(nodes, map, value, y, x + 1);
        }
    }
    if y < map.len() - 1 {
        if costs[y + 1][x] == usize::MAX {
            insert(nodes, map, value, y + 1, x);
        }
    }
    if x > 0 {
        if costs[y][x - 1] == usize::MAX {
            insert(nodes, map, value, y, x - 1);
        }
    }
}

#[test]
fn test_small_1() {
    let mut f = file("small/d15");
    assert_eq!(solve(&mut f), 40);
}

#[test]
fn test_small_2() {
    let mut f = file("small/d15");
    assert_eq!(solve(&mut f), 0);
}

#[test]
fn s1() {
    let mut f = file("input/d15");
    let ans = solve(&mut f);
    println!("d15-1: {}", ans);
    // assert_eq!(ans, 5252);
}
#[test]
fn s2() {
    let mut f = file("input/d15");
    println!("d15-2: {}", solve(&mut f));
}
