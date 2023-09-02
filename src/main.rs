use std::io::prelude::*;
use std::fs;

fn main() {
    println!("Hello, world!");
    let mut file=fs::File::create("/helloworld/helloworld.txt").unwrap();
    write!(file,"Hello world!\n").unwrap();
}
