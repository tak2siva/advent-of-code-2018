use std::fs;
use std::collections::HashMap;

pub fn solve() {
    println!("\n*************   Day - 1 ******************");
    let txtPath = [env!("CARGO_MANIFEST_DIR"), "/inputs/day1.input.txt"].join("");
    let contents = fs::read_to_string(txtPath).expect("err reading file");
    let arr = contents.split("\n");
    let numbers: Vec<i32> = arr.map(|s| s.parse().unwrap_or(0)).collect();

    // Part 1
    let sum = numbers.iter().fold(0,|a, &b| a + b);
    println!("Sum is:{}", sum);
    part2()
}

// Part 2
fn part2() {
    let txtPath = [env!("CARGO_MANIFEST_DIR"), "/inputs/day1.input2.txt"].join("");
    let contents = fs::read_to_string(txtPath).expect("err reading file");
    let arr = contents.split("\n");
    let numbers: Vec<i32> = arr.map(|s| s.parse().unwrap_or(0)).collect();
    
    // let mut search = true;
    let mut frequency = 0;
    let mut frequencyMap = HashMap::new();
    frequencyMap.insert(0, true);
    'outer: loop {
        'inner: for n in &numbers {
            frequency = frequency + n;
            if frequencyMap.contains_key(&frequency) {
                println!("First twice is:{}", frequency);
                // search = false;
                break 'outer;
            } else {
                frequencyMap.insert(frequency, true);   
            }
        }
    }
}