use std::fs::File;
// use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

struct BitColAndRows {
    rows: Vec<Vec<u8>>,
}

impl BitColAndRows {
    fn new() -> BitColAndRows {
        BitColAndRows { rows: Vec::new() }
    }

    fn add_row_from_str(&mut self, str: &str) {
        self.rows.push(str.as_bytes().into());
    }

    fn get_col(&self, col_index: usize) -> Vec<u8> {
        self.rows
            .iter()
            .map(
                |row| row.get(col_index).unwrap().clone(), //.collect::<Vec<u8>>()).collect::<Vec<Vec<u8>>>();
            )
            .collect()
    }

    fn remove_row(&mut self, row_index: usize) {
        self.rows.remove(row_index);
    }

    fn remove_rows(&mut self, row_indexes: Vec<usize>) {
        self.rows = self
            .rows
            .iter()
            .enumerate()
            .filter_map(|(index, val)| match row_indexes.contains(&index) {
                true => None,
                false => Some(val.clone()),
            })
            .collect();
    }

    fn get_row(&self, row_index: usize) -> Vec<u8> {
        self.rows.get(row_index).unwrap().clone()
    }

    fn get_row_length(&self) -> usize {
        self.rows.len()
    }

    fn get_col_length(&self) -> usize {
        self.rows.get(0).unwrap().len()
    }
}

impl std::fmt::Display for BitColAndRows {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BitColAndRows: rows are {:?}", self.rows)
    }
}

#[cfg(test)]
mod test {
    use super::BitColAndRows;

    #[test]
    fn test_adding_and_removing_all_in_one() {
        let mut bcr = BitColAndRows::new();
        bcr.add_row_from_str("111");
        bcr.add_row_from_str("110");
        bcr.add_row_from_str("100");
        bcr.add_row_from_str("000");
        assert_eq!(bcr.get_row_length(), 4);
        println!("The bcr is {}", bcr);
        bcr.remove_row(2);
        println!("The bcr is {}", bcr);
        assert_eq!(bcr.get_row_length(), 3);
        assert_eq!(bcr.get_row(0), "111".as_bytes());
        assert_eq!(bcr.get_row(1), "110".as_bytes());
        assert_eq!(bcr.get_row(2), "000".as_bytes());

        println!("The bcr is {}", bcr);
        assert_eq!(bcr.get_row_length(), 3);
        assert_eq!(bcr.get_row(2), "000".as_bytes());
    }
}

fn get_most_common_bit(bit_list: &Vec<u8>) -> u8 {
    let list_length = bit_list.len();
    let one_len = bit_list
        .into_iter()
        .filter(|&c| *c == 1)
        .collect::<Vec<&u8>>()
        .len();
    let zero_len = list_length - one_len;
    match one_len >= zero_len {
        true => 1,
        false => 0,
    }
}

fn get_least_common_bit(bit_list: &Vec<u8>) -> u8 {
    let list_length = bit_list.len();
    let one_len = bit_list
        .into_iter()
        .filter(|&c| *c == 1)
        .collect::<Vec<&u8>>()
        .len();
    match one_len < list_length / 2 {
        true => 1,
        false => 0,
    }
}

fn get_row_from_columns(columns: &Vec<Vec<u8>>, row: &usize) -> Vec<u32> {
    let mut row_value: Vec<u32> = Vec::new();
    columns
        .iter()
        .for_each(|col| row_value.push(col.get(*row).unwrap().clone() as u32));
    row_value
}

fn get_row_index_with_bit(columns: &Vec<Vec<u8>>, get_bit: fn(&Vec<u8>) -> u8) -> usize {
    let mut rows_kept_by_column: Vec<Vec<u8>> = columns.clone();
    let mut list_of_row_indexes: Vec<usize> =
        (0..rows_kept_by_column.get(0).unwrap().len()).collect();
    let mut cur_column = 0;
    loop {
        // Have to remove the index of the row to remove inside the column.get each loop
        let bitcol = dbg!(rows_kept_by_column.get(cur_column).unwrap());
        let most_common_bit = dbg!(get_bit(bitcol));
        let rows_to_remove: Vec<usize> = rows_kept_by_column
            .get(cur_column)
            .unwrap()
            .iter()
            .enumerate()
            .filter_map(|(row, val)| {
                if *val != most_common_bit {
                    Some(row)
                } else {
                    None
                }
            })
            .collect();
        dbg!(rows_to_remove.clone());
        rows_kept_by_column = rows_kept_by_column
            .iter()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .filter_map(|(index, val)| match rows_to_remove.contains(&index) {
                        true => None,
                        false => Some(val.clone()),
                    })
                    .collect()
            })
            .collect();

        list_of_row_indexes = list_of_row_indexes
            .iter()
            .enumerate()
            .filter_map(
                |(row_index, row)| match rows_to_remove.contains(&row_index) {
                    true => None,
                    false => Some(row.clone()),
                },
            )
            .collect();

        list_of_row_indexes.iter().for_each(|row_index| {
            println!("row_index: {}", row_index);
        });

        dbg!(list_of_row_indexes.len() == rows_kept_by_column.get(0).unwrap().len());

        // dbg!(rows_kept_by_column.clone());

        let row_count = rows_kept_by_column.get(0).unwrap().len();

        for i in 0..row_count {
            let row: Vec<u8> = rows_kept_by_column
                .iter()
                // .enumerate()
                .map(|row| {
                    // row.iter().enumerate().filter(|(col_index, val)| {
                    //     match col_index == i {
                    //         true => Some(val.clone()),
                    //         false => None,
                    //     }
                    // }).collect()
                    row.get(i).unwrap().clone()
                })
                .collect();
            println!("Row: {} is {:?}", i, row);
            //get(i).unwrap().remove(0);
        }

        dbg!(list_of_row_indexes.clone());
        if list_of_row_indexes.len() == 1 {
            break;
        }
        cur_column += 1;
    }
    // rows_kept.get(0).unwrap().clone()
    // Convert column in rows_kep to binary
    return list_of_row_indexes.get(0).unwrap().clone();
}

fn decimal_from_bit_vec(bit_vec: Vec<u32>) -> u32 {
    let mut value: u32 = 0;
    bit_vec.iter().rev().enumerate().for_each(|(index, val)| {
        value += u32::pow(2, index as u32) * val;
    });
    value
}
fn main() -> std::io::Result<()> {
    let mut columns: Vec<Vec<u8>> = Vec::new();

    let input_file = File::open("test_input")?;
    let buf_rd = BufReader::new(input_file);

    for (ln_no, line_res) in buf_rd.lines().enumerate() {
        let line = line_res.unwrap();
        for (c_no, c) in line.chars().enumerate() {
            if ln_no == 0 {
                let new_c: Vec<u8> = Vec::new();
                columns.push(new_c);
            }
            let du8 = c.to_digit(10).unwrap() as u8;
            columns.get_mut(c_no).unwrap().push(du8);
        }
    }

    // let rows_kept_clone = dbg!(rows_kept.clone()); // One thing I learned is that dbg! consumes the variable, so I need to clone it
    let final_row = get_row_index_with_bit(&columns, get_most_common_bit);
    let row = dbg!(get_row_from_columns(&columns, &final_row));
    let oxygen_rating = decimal_from_bit_vec(row);
    println!("Oxygen rating is: {:?}", oxygen_rating);

    let final_row = get_row_index_with_bit(&columns, get_least_common_bit);
    let row = dbg!(get_row_from_columns(&columns, &final_row));
    let co2_rating = decimal_from_bit_vec(row);
    println!("C02 Scrubber rating is: {:?}", co2_rating);

    println!("Answer is: {:?}", co2_rating * oxygen_rating);

    Ok(())
}
