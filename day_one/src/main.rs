use std::env;
use std::fs;

fn loop_list(mut counter: i32, mut dupe: i32, mut frequency: Vec<i32>, contents: String) -> (i32, i32){
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
        loop_list(counter, dupe, frequency, contents);
    } 

    (counter, dupe)
}

fn main() {
    let counter = 0;
    let mut frequency : Vec<i32> = Vec::new();
    frequency.push(0);
    let dupe = 0;
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading file: {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let (counter, dupe) = loop_list(counter, dupe, frequency, contents);

    println!("Total: {:?}", counter);
    println!("First dupe: {:?}", dupe)
}
