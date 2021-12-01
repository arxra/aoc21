use std::io::BufRead;

use anyhow::Result;

fn first(file: Vec<usize>) -> usize {
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
fn second(file: Vec<usize>) -> usize {
    let mut prev = 10000;
    let mut count = 0;
    for pos in 0..file.len()-2 {
        let window = file[pos] + file[pos+1] + file[pos+2];
        if window > prev {
            count += 1;
        }
        prev = window;
    }
    count
}

fn main() -> Result<()> {
    let file: Vec<usize> = std::fs::read("input")?
        .lines()
        .map(|f| f.unwrap().parse::<usize>().unwrap())
        .collect();

    println!("s1: {}", first(file.clone()));
    println!("s2: {}", second(file));
    Ok(())
}
