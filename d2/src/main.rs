use anyhow::Result;

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
            a => {
                println!("got direction {}, unknown", a);
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

fn main() -> Result<()> {
    let file: File = std::fs::read_to_string("input")?
        .lines()
        .map(|f| {
            println!("f: {}", f);
            (
                Dir::new(&f[0..f.find(' ').unwrap()]),
                f[f.find(' ').unwrap() + 1..].parse::<isize>().unwrap(),
            )
        })
        .collect();

    println!("s1: {}", first(file.clone()));
    println!("s2: {}", second(file));
    Ok(())
}
