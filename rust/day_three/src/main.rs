extern crate regex;

use std::fs;
use std::env;
use regex::Regex;

fn day3_part1(contents: &str) {
    let mut cloth = [[0u8; 1000]; 1000];
    //println!("{:?}", contents);
    /*
    for i in 0..cloth.len(){
    	for j in 0..cloth.len(){
    		print!("{:?} ", cloth[i][j]);
    	}
    	println!("");
    }
    */
    let re = Regex::new(r"(\d+,\d+): (\d+x\d+)").unwrap();
    for line in contents.lines() {
    	let caps = re.captures(line).unwrap();
    	println!("{:?} and {:?}", caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str());
    }
    println!("{:?}", cloth.len());
}

fn main() {
	let args: Vec<String> = env::args().collect();
    let mut _frequency : Vec<i32> = Vec::new();
    if args.len() <= 1 {
        panic!("Need input file");
    }
    let filename = &args[1];

    println!("Reading file: {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    day3_part1(&contents);
}
