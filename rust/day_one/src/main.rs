use std::env;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn day1_part1 (contents: &String) -> i32 {
    let count_sum = contents
        .lines()
        .map(|number| number.parse::<i32>().unwrap())
        .sum();

    count_sum
}

//Iterator solution
fn day1_part2(contents: &String) -> i32 {
    let mut index = 0;
    let mut counter = 0;
    let mut frequency = HashSet::new();
    let max_index = contents.trim().lines().count() as i64 - 1;

    let mut contents_hashmap: HashMap<i64, i32> = HashMap::new();
    for line in contents.lines() {
        contents_hashmap.insert(index, line.parse::<i32>().unwrap());
        index += 1;
    }

    index = 0;
    loop {
        counter += contents_hashmap[&index];
        if index == max_index {
            index = 0;
        } else {
            if frequency.insert(counter) == false {
                return counter;
            }
            index += 1;
        }
    }
}

//Recursion solution
#[allow(dead_code)]
fn day1_part2_recursion(mut counter: i32, mut dupe: i32, mut frequency: Vec<i32>, contents: &String) -> i32 {
    for line in contents.lines(){
        let change = line.parse::<i32>().unwrap();
        counter += change;
        if frequency.contains(&counter) {
            println!("Found {}", counter);
            dupe = counter;
            break;
        }
        frequency.push(counter);
        println!("Counter: {}", counter);
    }

    if dupe == 0 {
        day1_part2_recursion(counter, dupe, frequency, &contents);
    } 

    dupe
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

    let counter_sum = day1_part1(&contents);
    let dupe = day1_part2(&contents);
    //let dupe = day1_part2_recursion(0, 0, _frequency, &contents);

    println!("Total: {:?}", counter_sum);
    println!("First dupe: {:?}", dupe)
}
