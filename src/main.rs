extern crate rand;
use rand::seq::IteratorRandom; use core::panic;
// 0.7.3
use std::{fs::File, io::{BufRead, BufReader}};
extern crate clap;
use clap::{Arg, App};
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
            .arg(Arg::new("file")
                .short('f')
                .about("The file to read from")
                .default_value("./wordlist.txt"))
        .get_matches();

        let string = cli.value_of("quantity").unwrap().to_string();
        let quantity: u8 = string.parse().unwrap();

        let filepath = cli.value_of("file").unwrap().to_string();

        for _i in 0..quantity {
            let output = find_word(filepath.clone());
            print!("{}", output);
            print!(" ")
        }
        
    }

// find a line from the specified file

fn find_word(filepath: String) -> String {
    let file = File::open(filepath).unwrap_or_else(|e| panic!("{} : file could not be found, see --help for info.", e)); //File::open(FILENAME)
    
    let f = BufReader::new(file);

    let lines = f.lines().map(|l| l.expect("Couldn't read line"));
    
    lines
        .choose(&mut rand::thread_rng())
        .expect("Wordlist was compiled empty...")


}
