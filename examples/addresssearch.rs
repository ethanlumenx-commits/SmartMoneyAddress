use std::fs;
use std::io::{BufReader, BufRead};

fn main(){
    let file = fs::File::open("dune.csv").expect("msg");
    let reader = BufReader::new(file);

    let array = reader
        .lines()
        .filter_map(|line| {
            let line = line.ok()?;
            let line = line.trim();
            if line.len() > 20 {
                Some(line.to_string())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // println!("{:?}",&array[..20]);
    println!("{}",array.len());
}