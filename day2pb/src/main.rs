use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines};

struct Command {
    cmd_str: String,
    len: u64,
}

impl Command {
    fn new(cmd_str: String, len: u64) -> Command {
        Command { cmd_str, len }
    }
}

fn command_from_str(s: String) -> std::io::Result<Command> {
    let s_trim = s.trim();
    let mut s_iter = s_trim.split(" ");
    let command = s_iter.next().ok_or(std::io::Error::new(std::io::ErrorKind::Other, "No command"))?;
    let len_str = s_iter.next().ok_or(std::io::Error::new(std::io::ErrorKind::Other, "No len"))?;
    let len = len_str.parse::<u64>().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("Parse error for len: {}", e)))?;
    Ok( Command { cmd_str: command.to_string(), len } )
}
fn main() -> std::io::Result<()> {
    // let test_str = r#"down 3
    // forward 3
    // up 4"#;

    // for l in test_str.lines() {
    //     let cmd = command_from_str(l.to_string())?;
    //     println!("cmd {} {}", cmd.cmd_str, cmd.len);
    // }

    let mut horizontal: u64 = 0;
    let mut depth: u64 = 0;
    let mut aim: u64 = 0;

    let f = File::open("input")?;
    let bf = BufReader::new(f);
    for (ln_no, l_result) in bf.lines().enumerate() {
        if l_result.is_err() {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error getting line"));
        }
        let cmd = command_from_str(l_result.unwrap())?;
        println!("cmd {} {}", cmd.cmd_str, cmd.len);
        if cmd.cmd_str == "down" {
            aim += cmd.len;
        } else if cmd.cmd_str == "up" {
            aim -= cmd.len;
        } else if cmd.cmd_str == "forward" {
            horizontal += cmd.len;
            depth += cmd.len * aim;
        }
    }

    println!("multiplied: {}", horizontal * depth);
    Ok(())
 }