extern crate rand;

use rand::seq::IteratorRandom; // 0.7.3
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

extern crate clap;
use clap::{Arg, App};

const FILENAME: &str = "wordlist.txt";
fn main() {
    
   

    //Set options for cli arguments

    let cli = App::new("Oracle-rs")
        .version("1.0")
        .author("github.com/1Michael23/oracle-rs")
        .about("A recreation of the Oracle by Terry A Davis.")
            .arg(Arg::new("quantity")
                .short('q')
                .about("The number of words to print.")
                .default_value("1"))
        .get_matches();

        let string = cli.value_of("quantity").unwrap().to_string();
        let quantity: u8 = string.parse().unwrap();

        for _i in 0..quantity {
            let output = find_word();
            print!("{}", output);
            print!(" ")
        }
        
    }

// find a line from the specified file

fn find_word() -> String {
    let f = File::open(FILENAME)
        .unwrap_or_else(|e| panic!("(;_;) file not found: {}: {}", FILENAME, e));
    let f = BufReader::new(f);

    let lines = f.lines().map(|l| l.expect("Couldn't read line"));

    lines
        .choose(&mut rand::thread_rng())
        .expect("File had no lines")


}
