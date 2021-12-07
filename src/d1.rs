type File = Vec<usize>;

fn first(file: File) -> usize {
    let mut prev = 10000;
    let mut count = 0;
    for ele in file {
        if ele > prev {
            count += 1;
        }
        prev = ele;
    }
    count
}
fn second(file: File) -> usize {
    let mut prev = 10000;
    let mut count = 0;
    file.windows(3).map(|w| w.iter().sum()).for_each(|window| {
        if window > prev {
            count += 1;
        }
        prev = window;
    });
    count
}

fn file() -> File {
    std::fs::read_to_string("input/d1")
        .unwrap()
        .lines()
        .map(|f| f.parse().unwrap())
        .collect()
}

#[test]
fn p1() {
    println!("d1-1: {}", first(file()));
}
#[test]
fn p2() {
    println!("d1-2: {}", second(file()));
}
