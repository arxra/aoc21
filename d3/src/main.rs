use anyhow::Result;

type File = Vec<String>;

fn first(file: &File) -> usize {
    let mut counters: Vec<usize> = Vec::with_capacity(12);
    let mut size = 0;

    file.iter().for_each(|line| {
        size += 1;
        line.chars().enumerate().for_each(|(i, c)| {
            if c == '1' {
                if i >= counters.len() {
                    counters.push(1)
                } else {
                    counters[i] += 1
                }
            }
        })
    });
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for i in 0..counters.len() {
        if counters[i] < size / 2 {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0')
        }
    }
    println!("counters: {:?} ", counters);
    println!("size: {:?} ", size);
    println!(
        "Gamma: {} - {}",
        gamma,
        usize::from_str_radix(&gamma, 2).unwrap()
    );
    println!(
        "epsilon: {} - {}",
        epsilon,
        usize::from_str_radix(&epsilon, 2).unwrap()
    );
    usize::from_str_radix(&gamma, 2).unwrap() * usize::from_str_radix(&epsilon, 2).unwrap()
}

fn count(file: &File, pos: usize) -> (usize, usize) {
    let mut count = 0;
    let size = file.len();

    file.iter().for_each(|line| {
        line.chars().nth(pos).map(|c| {
            if c == '1' {
                count +=1;
            }
        });
    });
    (size, count)
}
fn second(file: File) -> usize {
    let mut o2 = file.clone();
    let mut pos = 0;
    let mut clear_list = Vec::new();
    while o2.len() > 1 {
        let (size, count) = count(&o2, pos);
        let delim = size / 2;
        clear_list.clear();
        o2.iter().enumerate().for_each(|(i, line)| {
            let c = line.as_bytes()[pos] as char;
            if c == '0' && (size - count) <= delim {
                clear_list.push(i);
            } else if c == '1' && count < delim {
                clear_list.push(i);
            }
        });
        for index in clear_list.iter().take(o2.len()-1).rev() {
            o2.remove(*index);
        }
        pos += 1;
    }

    let mut co2 = file;
    pos = 0;
    while co2.len() > 1 {
        let (size, count) = count(&co2, pos);
        let delim = size / 2;
        clear_list.clear();
        co2.iter().enumerate().for_each(|(i, line)| {
            let c = line.as_bytes()[pos] as char;
            if c == '0' && size - count > delim {
                clear_list.push(i);
            } else if c == '1' && count >= delim {
                clear_list.push(i);
            }
        });
        for index in clear_list.iter().take(co2.len()-1).rev() {
            co2.remove(*index);
        }
        pos += 1;
    }

    usize::from_str_radix(&o2[0], 2).unwrap() * usize::from_str_radix(&co2[0], 2).unwrap()

}

fn main() -> Result<()> {
    let file: File = std::fs::read_to_string("input")?
        .lines()
        .map(|f| f.to_string())
        .collect();

    println!("s1: {}", first(&file));
    println!("s2: {}", second(file));
    Ok(())
}
