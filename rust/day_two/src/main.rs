use std::env;
use std::fs;
use std::collections::HashMap;

#[allow(dead_code)]
fn day2_part1(contents: &str) {
    let mut two_count = 0;
    let mut three_count = 0;
    let mut results = Vec::new();
    let mut two_already = 0;
    let mut three_already = 0;

    for (index, line) in contents.lines().enumerate(){
        results.push(HashMap::new());
        for character in line.chars(){
            *results[index].entry(character).or_insert(0) += 1;
        }
    }

    for hashmap in results {
        for (_ , v) in hashmap
                        .iter()
                        .filter(|&(_, v)| *v > 1) {
                        match v {
                            2 => {
                                if two_already == 0 {
                                    two_count += 1;
                                    two_already = 1;
                                }
                            },
                            3 => {
                                if three_already == 0 {
                                    three_count += 1;
                                    three_already = 1;
                                }
                            },
                            _ => (),
                        }
                    }
        two_already = 0;
        three_already = 0;
    }

    println!("Two: {}, Three {}", two_count, three_count);
    println!("Checksum: {}", two_count * three_count);
}

fn day2_part2(contents: &str) {
    let lines_vec : Vec<&str> = contents.lines().collect();
    let mut single_mistake = false;

    for index in 0..lines_vec.len() {
        for inner_index in index + 1..lines_vec.len(){
            let mut combined_chars = lines_vec[index].chars().zip(lines_vec[inner_index].chars()).peekable();
            for (char1, char2) in combined_chars {
                if char1 != char2 {
                    if single_mistake == true {
                        break;
                    }
                    single_mistake = true;
                }
                if combined_chars.peek() != None && single_mistake == true{
                    println!("{} and {}", lines_vec[index], lines_vec[inner_index]);

                }
            }
        }
        
    }
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

    //day2_part1(&contents);
    day2_part2(&contents)
}