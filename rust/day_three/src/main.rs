extern crate regex;

use std::fs;
use std::env;
use regex::Regex;

fn day3_part1(contents: &str) {
    let mut cloth = [[0u8; 1000]; 1000];
    let mut overlap = 0;

    let re = Regex::new(r"(\d+),(\d+): (\d+)x(\d+)").unwrap();
    for line in contents.lines() {
    	let caps = re.captures(line).unwrap();
        let x_axis: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        let x_movement: usize = caps.get(3).unwrap().as_str().parse().unwrap();
        let y_axis: usize = caps.get(2).unwrap().as_str().parse().unwrap();
        let y_movement: usize = caps.get(4).unwrap().as_str().parse().unwrap();
        for i in x_axis..(x_axis + x_movement) {
            for j in y_axis..(y_axis + y_movement) {
                cloth[i][j] += 1;
            }
        }
    }
    for i in 0..cloth.len() {
        for j in 0..cloth.len() {
            if cloth[i][j] >= 2 {
                overlap += 1;
            }
        }
    }
    println!("Overlap: {:?}", overlap);
}

fn day3_part2 (contents: &str) {
    let mut cloth = [[0u8; 1000]; 1000];
    let mut overlap;
    let mut golden_cloth = Vec::new();

    let re = Regex::new(r"(\d+),(\d+): (\d+)x(\d+)").unwrap();
    for line in contents.lines() {
        let caps = re.captures(line).unwrap();
        let x_axis: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        let x_movement: usize = caps.get(3).unwrap().as_str().parse().unwrap();
        let y_axis: usize = caps.get(2).unwrap().as_str().parse().unwrap();
        let y_movement: usize = caps.get(4).unwrap().as_str().parse().unwrap();
        for i in x_axis..(x_axis + x_movement) {
            for j in y_axis..(y_axis + y_movement) {
                cloth[i][j] += 1;
            }
        }
    }

    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    'line: for line in contents.lines() {
        let caps = re.captures(line).unwrap();
        let id: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        let x_axis: usize = caps.get(2).unwrap().as_str().parse().unwrap();
        let x_movement: usize = caps.get(4).unwrap().as_str().parse().unwrap();
        let y_axis: usize = caps.get(3).unwrap().as_str().parse().unwrap();
        let y_movement: usize = caps.get(5).unwrap().as_str().parse().unwrap();
        overlap = 0;
        'outer: for i in x_axis..(x_axis + x_movement) {
            'inner: for j in y_axis..(y_axis + y_movement) {
                if cloth[i][j] >= 2 {
                    overlap = 1;
                    break 'outer;
                }
            }
        }
        if overlap == 0 {
            golden_cloth.push(id);
        }
    }
    println!("Not overlapped: {:?}", golden_cloth);
}


fn main() {
	let args: Vec<String> = env::args().collect();
    let mut _frequency : Vec<i32> = Vec::new();
    if args.len() <= 1 {
        panic!("Need input file");
    }
    let filename = &args[1];

    //println!("Reading file: {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    day3_part1(&contents);
    day3_part2(&contents);
}
