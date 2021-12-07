type File = Vec<Data>;
type Data = (Dir, isize);

#[derive(Debug, Clone)]
enum Dir {
    Forward,
    Down,
    Up,
}

impl Dir {
    fn new(input: &str) -> Self {
        match input {
            "forward" => Self::Forward,
            "down" => Self::Down,
            "up" => Self::Up,
            _ => {
                todo!()
            }
        }
    }
}

fn first(file: File) -> isize {
    let mut x = 0;
    let mut y = 0;
    for ele in file {
        match ele.0 {
            Dir::Forward => x += ele.1,
            Dir::Down => y += ele.1,
            Dir::Up => y -= ele.1,
        }
    }
    x * y
}
fn second(file: File) -> isize {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut aim: isize = 0;
    for ele in file {
        match ele.0 {
            Dir::Forward => {
                y += aim * ele.1;
                x += ele.1
            }
            Dir::Down => aim += ele.1,
            Dir::Up => aim -= ele.1,
        }
    }
    x * y
}


fn file() -> File{
    std::fs::read_to_string("input/d2")
        .unwrap()
        .lines()
        .map(|f| {
            (
                Dir::new(&f[0..f.find(' ').unwrap()]),
                f[f.find(' ').unwrap() + 1..].parse::<isize>().unwrap(),
            )
        })
        .collect()
}

#[test]
fn p1() {
    println!("d2-1: {}", first(file()));
}
#[test]
fn p2() {
    println!("d2-2: {}", second(file()));
}
