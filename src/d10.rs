type File = Vec<Vec<char>>;
type Map = Vec<usize>;

#[allow(unused_variables)]
fn solve(file: File, p1: bool) -> usize {
    let chunks = vec!['{', '(', '[', '<'];
    let mut error_line = Vec::new();
    let mut errors = Vec::new();
    let mut line_errors = Vec::new();

    'a: for line in file {
        let mut stack = Vec::new();
        for c in line {
            if chunks.contains(&c) {
                stack.push(c);
            } else if stack.ends_with(&['(']) && c == ')'
                || stack.ends_with(&['{']) && c == '}'
                || stack.ends_with(&['[']) && c == ']'
                || stack.ends_with(&['<']) && c == '>'
            {
                stack.pop();
            } else {
                error_line.push(c);
                continue 'a;
            }
        }
        while let Some(v) = stack.pop() {
            match v {
                '{' => errors.push('}'),
                '(' => errors.push(')'),
                '[' => errors.push(']'),
                '<' => errors.push('>'),
                _ => unreachable!(""),
            }
        }
        line_errors.push(errors.clone());
        errors.clear();
    }
    if p1 {
        error_line.into_iter().fold(0, |res, item| match item {
            ')' => res + 3,
            ']' => res + 57,
            '}' => res + 1197,
            '>' => res + 25137,
            _ => unreachable!(""),
        })
    } else {
        let mut error_scores: Vec<usize> = line_errors
            .into_iter()
            .map(|es| {
                es.into_iter().fold(0, |res: usize, item: char| match item {
                    ')' => 5 * res + 1,
                    ']' => 5 * res + 2,
                    '}' => 5 * res + 3,
                    '>' => 5 * res + 4,
                    _ => unreachable!(""),
                })
            })
            .collect::<Vec<usize>>();
        error_scores.sort();
        error_scores[error_scores.len() / 2]
    }
}

fn file(path: &str) -> File {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

#[test]
fn test_small_1() {
    let f = file("small/d10");
    assert_eq!(solve(f, true), 26397);
}

#[test]
fn test_small_2() {
    let f = file("small/d10");
    assert_eq!(solve(f, false), 288957);
}

#[test]
fn s1() {
    let f = file("input/d10");
    println!("d10-1: {}", solve(f, true));
}
#[test]
fn s2() {
    let f = file("input/d10");
    println!("d10-2: {}", solve(f, false));
}
