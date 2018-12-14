#![allow(non_snake_case)]

use std::fs;
use std::collections::HashMap;

pub fn solve() {
    println!("\n*************   Day - 2 ******************");
    let txtPath = [env!("CARGO_MANIFEST_DIR"), "/inputs/day2.input1.txt"].join("");
    let contents = fs::read_to_string(txtPath).expect("err reading file");
    let arr = contents.split("\n");
    let arr: Vec<&str> = arr.collect();
    let arr2 = arr.clone();
    let arr3 = arr.clone();

    // Part 1
    let mut twiceCount = 0;
    let mut thriceCount = 0;
    for word in arr {
        let mut countMap: HashMap<char, i32> = HashMap::new();
        for c in word.chars() {
            countMap.entry(c).and_modify(|e| {*e += 1}).or_insert(1);
        }
        let mut b2 = true;
        let mut b3 = true;
        for (_key, value) in countMap {
            if b2 && value == 2 {
                twiceCount+=1;
                b2 = false;
            }
            if b3 && value == 3 {
                thriceCount+=1;
                b3 = false;
            }
        }
    }
    println!("Checksum is: {}", twiceCount * thriceCount);

    // Part 2
    'outer: for a in &arr2 {
        // let a: &String = &a.to_string();
        'inner: for b in &arr3 {
            // let b: &String = &b.to_string();
            let matchCount = a.chars().zip(b.chars()).filter(|&(a, b)| a == b).count();
            if a.chars().count() - matchCount == 1 {
                let s: String = a.chars().zip(b.chars()).filter(|&(a, b)| a == b).map(|(a, _b)| a).collect();
                println!("Common letters are: {}", s);
                break 'outer;
            }
        }
    }
}