use std::fs::File;
// use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

fn main() -> std::io::Result<()> {
    let mut columns: Vec<Vec<u8>> = Vec::new();
    // for _i in 0..5 {
    //     let new_c: Vec<u8> = Vec::new();
    //     columns.push(new_c);
    // }
    let input_file = File::open("input")?;
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

    let mut most_common: Vec<u8> = Vec::new();
    for c_no in 0..columns.len() {
        let mut zero_count: u64 = 0;
        let mut one_count: u64 = 0;
        for (_b_no, b) in columns.get(c_no).unwrap().into_iter().enumerate() {
            println!("{:}", b);
            if *b == 0 {
                zero_count += 1;
            } else {
                one_count += 1;
            }
        }
        if zero_count > one_count {
            most_common.push(0);
        } else {
            most_common.push(1);
        }
    }
    
    let mut gamma_rate: u64 = 0;
    for (decimal_place, bit) in most_common.into_iter().rev().enumerate() {
        gamma_rate += u64::pow(2, decimal_place as u32) * bit as u64;
    }
    println!("gamma: {:?}", gamma_rate);

    let mut least_common: Vec<u8> = Vec::new();
    for c_no in 0..columns.len() {
        let mut zero_count: u64 = 0;
        let mut one_count: u64 = 0;
        for (_b_no, b) in columns.get(c_no).unwrap().into_iter().enumerate() {
            println!("{:}", b);
            if *b == 0 {
                one_count += 1;
            } else {
                zero_count += 1;
            }
        }
        if zero_count > one_count {
            least_common.push(0);
        } else {
            least_common.push(1);
        }
    }
    
    let mut epsilon_rate: u64 = 0;
    for (decimal_place, bit) in least_common.into_iter().rev().enumerate() {
        epsilon_rate += u64::pow(2, decimal_place as u32) * bit as u64;
    }
    println!("epsilon: {:?}", epsilon_rate);

    let power_consumption = epsilon_rate * gamma_rate;

    println!("power_consumption: {:?}", power_consumption);

    Ok(())
}
