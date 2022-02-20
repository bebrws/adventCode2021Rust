use std::vec::Vec;
use std::string::String;
use std::iter::{Iterator, Enumerate};
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines};
use std::option::Option;
use std::collections::VecDeque;


struct ThreeByIter {
    // file: File,
    // buf_reader: BufReader<File>,
    line_iter: Enumerate<Lines<BufReader<File>>>,
    deq: VecDeque<u64>,
}

impl ThreeByIter {
    fn new() -> ThreeByIter {
        let file = File::open("input").unwrap();
        let buf_reader = BufReader::new(file);
        let line_iter = buf_reader.lines().enumerate();
        let mut deq = VecDeque::new();
        ThreeByIter { line_iter, deq }
    }
}

impl Iterator for ThreeByIter {
    // type Item = Vec<u64>;
    type Item = [u64; 3];
    fn next(&mut self) -> Option<Self::Item> {
        if self.deq.len() != 3 {
            for i in 0..3 {
                let line_option = self.line_iter.next();
                if line_option.is_none() { return None; }
                let line_u64 = line_option.unwrap().1.unwrap().parse::<u64>().unwrap();
                self.deq.push_back(line_u64);
            }
        } else {
            let _old_u64 =self.deq.pop_front();
            let line_option = self.line_iter.next();
            if line_option.is_none() { return None; }
            let line_u64 = line_option.unwrap().1.unwrap().parse::<u64>().unwrap();
            self.deq.push_back(line_u64);
        }

        println!("self.deq: {:?}", self.deq);

        Some([self.deq.get(0).unwrap().clone(), self.deq.get(1).unwrap().clone(), self.deq.get(2).unwrap().clone()])
    }
}

fn main() -> std::io::Result<()> {

    let mut iter = ThreeByIter::new();

    let [f1, f2, f3] = iter.next().unwrap();
    let mut last_depth = f1 + f2 + f3;
    let mut depth_inc_count = 0;
    for [d1, d2, d3] in iter {
        let cur_depth = d1 + d2 + d3;
        if cur_depth > last_depth {
            depth_inc_count += 1;
        }
        last_depth = cur_depth;
        // println!("{} {} {}", d1, d2, d3);
    }
    println!("depth_inc_count: {}", depth_inc_count);

    Ok(())
}
