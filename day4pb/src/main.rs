use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
struct BingoBoard {
    rows: Vec<Vec<u32>>,
    marked: Vec<Vec<u32>>,
}

impl BingoBoard {
    fn new() -> BingoBoard {
        let mut marked = Vec::new();
        for _i in 0..5 {
            marked.push(vec![0; 5]);
        }

        BingoBoard {
            rows: Vec::new(),
            marked,
        }
    }
    fn add_row(&mut self, row: Vec<u32>) {
        self.rows.push(row);
    }

    fn mark_pos(&mut self, col: usize, row: usize) {
        let mut marked_el = self.marked.get_mut(row).unwrap().get_mut(col).unwrap();
        *marked_el = 1;
    }

    fn num_called(&mut self, num: u32) {
        for row in 0..self.rows.len() {
            let col = self.rows.get(row).unwrap();
            for col in 0..col.len() {
                let pos_num = self.rows.get(row).unwrap().get(col).unwrap();
                if *pos_num == num {
                    self.mark_pos(col, row);
                }
            }
        }
    }

    fn get_unmarked_sum(&self) -> u32 {
        let mut sum = 0;
        self.marked.iter().enumerate().for_each(|(row_index, row)| {
            row.iter().enumerate().for_each(|(col_index, col_val)| {
                if *col_val == 0 {
                    sum += self.rows.get(row_index).unwrap().get(col_index).unwrap();
                }
            });
        });
        return sum;
    }

    fn is_winner(&self) -> bool {
        let row_wins = self
            .marked
            .iter()
            .filter_map(|row| match row.iter().sum::<u32>() == 5 {
                true => Some(true),
                false => None,
            })
            .collect::<Vec<bool>>()
            .len();
        if row_wins > 0 {
            return true;
        }

        for col in 0..5 {
            let rows_in_col_marked = self
                .marked
                .iter()
                .filter_map(|row| match *row.get(col).unwrap() == 1 {
                    true => Some(true),
                    false => None,
                })
                .collect::<Vec<bool>>()
                .len();
            if rows_in_col_marked == 5 {
                return true;
            }
        }

        return false;
    }
}

impl std::fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Board rows:\n")?;
        for row in &self.rows {
            for num in row {
                write!(f, "{} ", num)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for BingoBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Board rows:\n")?;
        for row in &self.rows {
            for num in row {
                write!(f, "{} ", num)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
fn main() {
    let file = File::open("input").unwrap();
    let buf_reader = BufReader::new(file);
    let mut lines_iter = buf_reader.lines();
    let numbers_line = lines_iter.next().unwrap().unwrap();
    let bingo_numbers: Vec<u32> = numbers_line
        .split(",")
        .map(|val| val.parse().unwrap())
        .collect();

    let mut boards = Vec::new();
    loop {
        let empty_line = lines_iter.next(); // Next line is empty
        if empty_line.is_none() {
            break;
        }

        let mut board_rows: Vec<Vec<u32>> = Vec::new();
        for _i in 0..5 {
            let line_read = lines_iter.next();
            if line_read.is_none() {
                break;
            }
            let next_bingo_board_line = line_read.unwrap().unwrap();
            let board_line_numbers: Vec<u32> = next_bingo_board_line
                .split(" ")
                .filter_map(|val| match val.parse() {
                    Ok(val) => Some(val),
                    Err(_) => None,
                })
                .collect();
            board_rows.push(board_line_numbers);
        }

        let mut board = BingoBoard::new();
        if board_rows.len() != 5 {
            break;
        }
        for i in 0..board_rows.len() {
            board.add_row(board_rows.get(i).unwrap().clone());
        }
        println!("Board: {}", board);
        boards.push(board);
    }
    println!("Bingo numbers: {:?}", bingo_numbers);

    for num in bingo_numbers {
        let mut final_value = 0;
        boards.iter_mut().for_each(|board| {
            board.num_called(num);
            if board.is_winner() {
                println!("Winner: {}", board);
            }
        });

        if boards.len() == 1 && boards.get(0).unwrap().is_winner() {
            final_value = boards.get(0).unwrap().get_unmarked_sum() * num;
        }

        boards.retain(|board| !board.is_winner());

        dbg!(boards.clone());

        if final_value != 0 {
            println!("final value: {}", final_value);
            break;
        }
    }
}
