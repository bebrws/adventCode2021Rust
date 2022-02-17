use std::vec::Vec;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
  let file = File::open("input")?;
  let buf_reader = BufReader::new(file);
  let lines_vec: Vec<String> = buf_reader.lines().filter(|l| l.is_ok()).map(|l| l.unwrap()).collect();
  let mut last_depth: u64 = lines_vec.get(0).unwrap().parse::<u64>().unwrap();
  let mut depth_inc_count: u64 = 0;
  for line in lines_vec[1..lines_vec.len()].into_iter() {
    println!("line: {}", line);
    let line_uint = line.parse::<u64>().unwrap();

    if line_uint > last_depth {
      depth_inc_count += 1;
    }

    last_depth = line_uint;
  }

  println!("depth_inc_count: {}", depth_inc_count);
  
  Ok(())
}