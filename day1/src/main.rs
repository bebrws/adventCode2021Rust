use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
  let file = File::open("input")?;
  let buf_reader = BufReader::new(file);
  
  let mut line_iter = buf_reader.lines().enumerate();
  
  let mut last_depth: u64 = line_iter.next().unwrap().1.unwrap().parse::<u64>().unwrap();
  let mut depth_inc_count: u64 = 0;
  
  for (_index, line_res) in line_iter {
    let line_uint = line_res.unwrap().parse::<u64>().unwrap();
    println!("line: {}", line_uint);

    if line_uint > last_depth {
      depth_inc_count += 1;
    }

    last_depth = line_uint;
  }

  println!("depth_inc_count: {}", depth_inc_count);
  
  Ok(())
}