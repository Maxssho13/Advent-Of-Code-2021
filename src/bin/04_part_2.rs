#![feature(drain_filter)]

fn main() {
    let input = String::from_utf8(std::fs::read("inputs/04_part_2.txt").unwrap()).unwrap();

    let (called_numbers, boards) = input.split_once('\n').unwrap();

    let called_numbers: Vec<i32> = called_numbers
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    let mut boards: Vec<Board> = boards
        .split("\n\n")
        .map(|board| {
            board
                .split_whitespace()
                .map(|num| num.trim().parse().unwrap())
                .collect::<Board>()
        })
        .collect();

    'outer: for number in called_numbers {
        boards.iter_mut().for_each(|board| {
            board.call_number(number);
        });

        for i in (0..boards.len()).rev() {
            if boards.len() == 1 && boards[0].found_bingo {
                let sum = boards[0].sum_uncalled();
                let output = sum * number;
                println!("last bingo: {}", output);
                break 'outer;
            }
            if boards.len() > 1 && boards[i].found_bingo {
                boards.remove(i);
            }
        }
    }
}

#[derive(Default, Debug)]
struct BoardNumber {
    data: i32,
    called: bool,
}

#[derive(Default, Debug)]
struct Board {
    grid: [[BoardNumber; 5]; 5],
    found_bingo: bool,
}

impl Board {
    fn call_number(&mut self, number: i32) {
        for row in 0..5 {
            for col in 0..5 {
                if self.grid[row][col].data == number {
                    self.grid[row][col].called = true;
                    let found_bingo = self.has_column_bingo(col) || self.has_row_bingo(row);
                    self.found_bingo = found_bingo;
                }
            }
        }
    }

    fn has_row_bingo(&self, row: usize) -> bool {
        let mut has_bingo = true;
        for col in 0..5 {
            has_bingo &= self.grid[row][col].called;
        }
        has_bingo
    }

    fn has_column_bingo(&self, col: usize) -> bool {
        let mut has_bingo = true;
        for row in 0..5 {
            has_bingo &= self.grid[row][col].called;
        }
        has_bingo
    }

    fn sum_uncalled(&self) -> i32 {
        let mut out = 0;
        for row in 0..5 {
            for col in 0..5 {
                if !self.grid[row][col].called {
                    out += self.grid[row][col].data;
                }
            }
        }
        out
    }
}

impl FromIterator<i32> for Board {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        let mut board = Board::default();
        let mut iter = iter.into_iter();

        for row in 0..5 {
            for col in 0..5 {
                board.grid[row][col].data = iter.next().expect("must be at least 5x5");
            }
        }
        board
    }
}
