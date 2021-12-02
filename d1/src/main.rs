use std::io::BufRead;

use anyhow::Result;

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

fn main() -> Result<()> {
    let file: File = std::fs::read("input")?
        .lines()
        .map(|f| f.unwrap().parse::<usize>().unwrap())
        .collect();

    println!("s1: {}", first(file.clone()));
    println!("s2: {}", second(file));
    Ok(())
}
