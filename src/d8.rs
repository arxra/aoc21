type File = Vec<Vec<Vec<String>>>;
type Found = Vec<Vec<String>>;

fn sub_word(s1: &mut String, s2: &Vec<String>, expected: usize) -> bool {
    for s in s2 {
        let mut org = s1.clone();
        for pat in s.chars() {
            org.remove_matches(pat)
        }
        if org.len() == expected {
            return true;
        }
    }
    false
}

fn solve(file: File, p1: bool) -> usize {
    let mut res = 0;
    if p1 {
        file.iter().for_each(|line| {
            line[1].iter().for_each(|word| match word.len() {
                // 1, 4, 7, or 8 have these many chars in them
                2 | 4 | 3 | 7 => res += 1,
                _ => (),
            })
        });
        res
    } else {
        file.iter().for_each(|line| {
            let mut found: Found = vec![Vec::new(); 10];
            let mut fives = Vec::new();
            let mut sixes = Vec::new();

            // We know some words just straight from the number of chars.
            for word in line[0].iter().chain(line[1].iter()) {
                match word.len() {
                    2 => found[1].push(word.clone()),
                    3 => found[7].push(word.clone()),
                    4 => found[4].push(word.clone()),
                    5 => fives.push(word),
                    6 => sixes.push(word),
                    7 => found[8].push(word.clone()),
                    _ => (),
                }
            }

            for word in sixes.into_iter() {
                if sub_word(&mut word.clone(), &found[1], 5) {
                    found[6].push((*word).clone());
                } else if sub_word(&mut word.clone(), &found[4], 3) {
                    found[0].push((*word).clone());
                } else if sub_word(&mut word.clone(), &found[4], 2) {
                    found[9].push((*word).clone());
                } else {
                    todo!("Not handled number encountered")
                }
            }

            for word in fives.into_iter() {
                if sub_word(&mut word.clone(), &found[7], 2) {
                    // println!("found the 5: {}", word);
                    found[3].push((*word).clone());
                } else if sub_word(&mut word.clone(), &found[4], 3) {
                    found[2].push(word.clone());
                } else if sub_word(&mut word.clone(), &found[4], 2) {
                    found[5].push(word.clone());
                } else {
                    todo!("Not handled number encountered")
                }
            }

            let mut ans = String::new();
            'out: for out in line[1].clone() {
                for i in 0..10 {
                    for f in found[i].clone() {
                        if out.len() == f.len() && sub_word(&mut out.clone(), &found[i], 0) {
                            ans.extend(i.to_string().chars());
                            continue 'out;
                        }
                    }
                }
                todo!("not found number: {}", out);
            }
            res += ans.parse::<usize>().unwrap();
        });
        res
    }
}

fn file(path: &str) -> File {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| {
            l.split('|')
                .map(|a| a.split_whitespace().map(|word| word.to_string()).collect())
                .collect()
        })
        .collect()
}

#[test]
fn test_small_1() {
    let f = file("small/d8");
    assert_eq!(solve(f, true), 26);
}

#[test]
fn test_small_2() {
    let f = file("small/d8");
    assert_eq!(solve(f, false), 61229);
}

#[test]
fn s1() {
    let f = file("input/d8");
    println!("d8-1: {}", solve(f, true));
}
#[test]
fn s2() {
    let f = file("input/d8");
    println!("d8-2: {}", solve(f, false));
}
