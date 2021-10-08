use std::fmt::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::string;

fn main() {
    let filename = "wordlist.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);


    for  (count,line) in reader.lines().enumerate() {       
        println!("{}", line.unwrap())
    }
}
