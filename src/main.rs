use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::{self,BufReader,BufWriter};

fn main() {
  let args: Vec<String> = env::args().collect();
  let f0 = File::open(&args[1]).expect("Unable to open file 1");
  let f1 = File::open(&args[2]).expect("Unable to open file 2");
  let mut i0 = BufReader::new(f0).bytes();
  let mut i1 = BufReader::new(f1).bytes();
  let mut w = BufWriter::new(io::stdout());
  loop {
    let o0 = i0.next();
    let o1 = i1.next();
    if !o0.is_some() && !o1.is_some() {
      break;
    }
    let r0 = o0.unwrap_or(Ok(0));
    let r1 = o1.unwrap_or(Ok(0));
    if !r0.is_ok() && !r1.is_ok() {
      break;
    }
    let b0 = r0.unwrap_or(0);
    let b1 = r1.unwrap_or(0);
    w.write_all(&[b0 ^ b1]);
  }
  w.flush();
}
