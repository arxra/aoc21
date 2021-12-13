use hashbrown::HashMap;

type File = Vec<Vec<String>>;
type Map = Vec<usize>;

#[derive(Debug)]
struct Node {
    name: String,
    big: bool,
    neigbors: Vec<String>,
}

impl Node {
    fn new(name: String) -> Self {
        let neigbors: Vec<String> = Vec::new();
        let big = name.eq(&name.to_uppercase());
        Self {
            name,
            big,
            neigbors,
        }
    }
    fn connect(&mut self, other: &mut Node) {
        self.neigbors.push(other.name.clone());
        other.neigbors.push(self.name.clone())
    }
}

#[derive(Debug)]
struct Graph {
    nodes: HashMap<String, Node>,
}

impl Graph {
    fn new() -> Self {
        let nodes = HashMap::new();
        Self { nodes }
    }
    fn add_node_pair(&mut self, pair: &mut Vec<String>) {
        if pair.len() != 2 {
            eprintln!("pair: {:?}", pair);
        }
        // Create the nodes.
        for n in 0..=1 {
            if !self.nodes.contains_key(&pair[n]) {
                let v = Node::new(pair[n].clone());
                self.nodes.insert(pair[n].clone(), v);
            }
        }

        // Get the nodes
        let nodelist = match self.nodes.get_each_mut([&pair[0], &pair[1]]) {
            [Ok(n1), Ok(n2)] => (n1, n2),
            _ => unreachable!(),
        };

        // Pair the nodes
        nodelist.0.connect(nodelist.1);
    }
    fn node_by_name(&self, name: &String) -> &Node {
        self.nodes.get(name).unwrap()
    }
}

fn dfs(node: &Node, graph: &Graph, path: &Vec<String>, allow_small: bool) -> Vec<Vec<String>> {
    let mut res = Vec::with_capacity(5);

    let mut mypath = path.clone();
    mypath.push(node.name.clone());

    if node.name.eq(&"end".to_string()) {
        res.push(mypath);
        return res;
    }
    for n in node.neigbors.iter() {
        let neigh = graph.node_by_name(&n);
        if neigh.big || !path.contains(n) {
            res.append(&mut dfs(neigh, &graph, &mypath, allow_small));
        } else if allow_small && !(n.eq("start") || n.eq("end")) {
            res.append(&mut dfs(neigh, &graph, &mypath, false));
        }
    }
    res
}

#[allow(unused_variables)]
fn solve(file: &mut File, p2: bool) -> usize {
    // let mut paths: Vec<Vec<String>> = Vec::with_capacity(1000);
    let mut graph = Graph::new();
    file.into_iter()
        .filter(|pair| pair.len() == 2)
        .for_each(|pair| graph.add_node_pair(pair));
    // dbg!(graph);

    let start = graph.nodes.get("start").unwrap();
    let nopath: Vec<String> = Vec::new();
    let paths = dfs(&start, &graph, &nopath, p2);
    println!("{:?}", paths);
    paths.len()
}

fn file(path: &str) -> File {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.split('-').map(|a| a.to_string()).collect())
        .collect()
}

#[test]
fn test_small_1() {
    let mut f = file("small/d12");
    assert_eq!(solve(&mut f, false), 10);
}

#[test]
fn test_small_3() {
    let mut f = file("small/d12-2");
    assert_eq!(solve(&mut f, false), 226);
}

#[test]
fn test_small_2() {
    let mut f = file("small/d12");
    assert_eq!(solve(&mut f, true), 36);
}

#[test]
fn s1() {
    let mut f = file("input/d12");
    let ans = solve(&mut f, false);
    println!("d12-1: {}", ans);
    assert_eq!(ans, 5252);
}
#[test]
fn s2() {
    let mut f = file("input/d12");
    println!("d12-2: {}", solve(&mut f, true));
}
