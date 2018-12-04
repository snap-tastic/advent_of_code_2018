use std::env;
use std::fs;
use std::collections::HashMap;

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
    let mut single_mistake;

    'outer: for index in 0..lines_vec.len() {
        'inner: for inner_index in index + 1..lines_vec.len(){
            single_mistake = false;
            let mut combined_chars = lines_vec[index].chars().zip(lines_vec[inner_index].chars());
            'last: for (char1, char2) in combined_chars {
                if char1 != char2 {
                    if single_mistake == true {
                        continue 'inner;
                    }
                    single_mistake = true;
                }           
            }
            print!("{}\n{}\n", lines_vec[index], lines_vec[inner_index]);
            let winner : String = lines_vec[index].chars().zip(lines_vec[inner_index].chars()).into_iter()
                                                                                 .filter(| &(char1, char2) | char1 == char2)
                                                                                 .map(| (c, _) | c)
                                                                                 .into_iter()
                                                                                 .collect();
            println!("Winner Winner {:?}", winner);

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

    day2_part1(&contents);
    day2_part2(&contents)
}