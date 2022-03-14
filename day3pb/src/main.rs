use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;
use std::vec::Vec;

#[derive(Clone)]
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
            .map(|row| row.get(col_index).unwrap().clone())
            .collect()
    }

    fn remove_row(&mut self, row_index: usize) {
        self.rows.remove(row_index);
    }

    fn remove_rows(&mut self, row_indexes: &Vec<usize>) {
        let mut row_indexes_to_removed = row_indexes.clone();
        // We always stop if there is only one row left
        if row_indexes.len() >= self.rows.len() {
            row_indexes_to_removed =
                row_indexes_to_removed[0..row_indexes_to_removed.len() - 2].to_vec();
        }
        self.rows = self
            .rows
            .iter()
            .enumerate()
            .filter_map(
                |(index, val)| match row_indexes_to_removed.contains(&index) {
                    true => None,
                    false => Some(val.clone()),
                },
            )
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
    fn len(&self) -> usize {
        self.rows.len()
    }

    fn get_rows(&self) -> Vec<Vec<u8>> {
        self.rows.clone()
    }
}

impl std::fmt::Display for BitColAndRows {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n")?;
        for row in &self.rows {
            let row_str = str::from_utf8(row).unwrap();
            write!(f, "{}\n", row_str)?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for BitColAndRows {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n")?;
        for row in &self.rows {
            let row_str = str::from_utf8(row).unwrap();
            write!(f, "{}\n", row_str)?;
        }
        Ok(())
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

fn get_most_common_bit(col: &Vec<u8>) -> u8 {
    let list_length = col.len();
    let one_len = col
        .iter()
        .filter(|&c| *c == '1' as u8)
        .collect::<Vec<&u8>>()
        .len();
    let zero_len = list_length - one_len;
    dbg!(zero_len);
    dbg!(one_len);
    match one_len >= zero_len {
        true => '1' as u8,
        false => '0' as u8,
    }
}

fn get_least_common_bit(col: &Vec<u8>) -> u8 {
    let list_length = col.len();
    let one_len = col
        .iter()
        .filter(|&c| *c == '1' as u8)
        .collect::<Vec<&u8>>()
        .len();
    let zero_len = list_length - one_len;
    match one_len < zero_len {
        true => '1' as u8,
        false => '0' as u8,
    }
}
fn get_row_index_with_bit(bcr: &BitColAndRows, get_bit: fn(&Vec<u8>) -> u8) -> String {
    let mut bcr_rows_kept_by_column = bcr.clone();
    let mut cur_column = 0;

    println!("The bcr is {}", bcr_rows_kept_by_column);
    loop {
        let cur_col = bcr_rows_kept_by_column.get_col(cur_column);
        let common_bit = get_bit(&cur_col);

        dbg!(common_bit as char);

        let rows_to_remove: Vec<usize> = bcr_rows_kept_by_column
            .get_col(cur_column)
            .iter()
            .enumerate()
            .filter_map(|(row_index, val)| {
                println!("val {} != {}", val, common_bit);
                if *val != common_bit as u8 {
                    Some(row_index)
                } else {
                    None
                }
            })
            .collect();
        dbg!(rows_to_remove.clone());
        bcr_rows_kept_by_column.remove_rows(&rows_to_remove);
        dbg!(bcr_rows_kept_by_column.clone());

        if bcr_rows_kept_by_column.len() == 1 {
            return str::from_utf8(&bcr_rows_kept_by_column.get_row(0))
                .unwrap()
                .to_owned();
        }
        cur_column += 1;
    }
}

// Learned about: u32::from_str_radix
// fn decimal_from_bit_vec(bit_vec: Vec<u32>) -> u32 {
//     let mut value: u32 = 0;
//     bit_vec.iter().rev().enumerate().for_each(|(index, val)| {
//         value += u32::pow(2, index as u32) * val;
//     });
//     value
// }
fn main() -> std::io::Result<()> {
    let mut bcr: BitColAndRows = BitColAndRows::new();

    let input_file = File::open("input")?;
    let buf_rd = BufReader::new(input_file);

    for (_ln_no, line_res) in buf_rd.lines().enumerate() {
        let line = line_res.unwrap();
        bcr.add_row_from_str(line.as_str());
    }

    let final_row = get_row_index_with_bit(&bcr, get_most_common_bit);
    let oxygen_rating = u32::from_str_radix(&final_row, 2).unwrap();
    println!("Oxygen rating is: {:?}", oxygen_rating);

    let final_co2_row = get_row_index_with_bit(&bcr, get_least_common_bit);
    let co2_rating = u32::from_str_radix(&final_co2_row, 2).unwrap();
    println!("CO2 rating is: {:?}", co2_rating);

    print!("Final answer is: {}", co2_rating * oxygen_rating);
    Ok(())
}
