type File = (Vec<usize>, Vec<Board>);

#[derive(Debug, Clone)]
struct Board {
    numbers: Vec<Vec<usize>>,
    unmarked: Vec<usize>,
}

impl Board {
    fn new(numbers: Vec<Vec<usize>>) -> Self {
        let unmarked = numbers.clone().into_iter().flatten().collect();
        Self { numbers, unmarked }
    }
    fn call_num(&mut self, value: usize, called_numbers: &[usize]) -> Option<usize> {
        match self.unmarked.iter().position(|x| *x == value) {
            Some(index) => self.unmarked.remove(index),
            None => return None,
        };

        for y in 0..self.numbers.len() {
            for x in 0..self.numbers.len() {
                if self.numbers[y][x] == value {
                    if self.check_winner(x, y, called_numbers) {
                        return Some(self.score(value));
                    }
                }
            }
        }
        None
    }

    fn check_winner(&self, x: usize, y: usize, called_numbers: &[usize]) -> bool {
        let mut winner = 5;
        for xx in 0..self.numbers.len() {
            if called_numbers.contains(&self.numbers[y][xx]) {
                winner -= 1;
            }
        }
        if winner == 0 {
            return true;
        }
        winner = 5;
        for yy in 0..self.numbers.len() {
            if called_numbers.contains(&self.numbers[yy][x]) {
                winner -= 1;
            }
        }

        if winner == 0 {
            true
        } else {
            false
        }
    }

    fn score(&self, value: usize) -> usize {
        self.unmarked.iter().sum::<usize>() * value
    }
}

fn first(file: &mut File) -> usize {
    let mut called_numbers = Vec::new();
    for n in file.0.clone() {
        called_numbers.push(n);
        for b in 0..file.1.len() {
            if let Some(winner) = file.1[b].call_num(n, &called_numbers) {
                return winner;
            }
        }
    }
    0
}

fn second(file: &mut File) -> usize {
    let mut called_numbers = Vec::new();
    for n in file.0.clone() {
        called_numbers.push(n);
        let mut rm = Vec::new();
        for b in 0..file.1.len() {
            if let Some(winner) = file.1[b].call_num(n, &called_numbers) {
                if file.1.len() - 1 == rm.len() {
                    return winner;
                } else {
                    rm.push(b);
                }
            }
        }
        for r in rm.iter().take(file.1.len() - 1).rev() {
            file.1.remove(*r);
        }
    }
    0
}

fn to_file(input: Vec<String>) -> File {
    let numbers: Vec<usize> = input[0]
        .split(',')
        .map(|a| a.parse::<usize>().unwrap())
        .collect();
    let mut boards = Vec::new();
    let mut file = input[1..].into_iter();
    while file.next().is_some() {
        let mut board: Vec<Vec<usize>> = Vec::new();
        for _ in 0..5 {
            if let Some(l) = file.next() {
                board.push(l.split_whitespace().map(|u| u.parse().unwrap()).collect());
            }
        }
        boards.push(Board::new(board));
    }

    (numbers, boards)
}

fn file() -> File{
    let input: Vec<String> = std::fs::read_to_string("input/d4").unwrap()
        .lines()
        .map(|f| f.to_string())
        .collect();
    to_file(input)
}

#[test]
fn p1(){
    println!("d4-1: {}", first(&mut file()));

}
#[test]
fn p2(){
    println!("d4-2: {}", second(&mut file()));
}
