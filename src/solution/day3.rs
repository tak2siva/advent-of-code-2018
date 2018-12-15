#![allow(non_snake_case)]
#[macro_use]

use std::fs;
use regex::Regex;
use std::collections::HashMap;

pub fn solve() {
    println!("\n*************   Day - 3 ******************");

    let txtPath = [env!("CARGO_MANIFEST_DIR"), "/inputs/day3.input1.txt"].join("");
    let contents = fs::read_to_string(txtPath).expect("err reading file");
    let arr: Vec<&str> = contents.split("\n").collect();
    let mut fabric: [[i32; 1000]; 1000] = [[0; 1000]; 1000];
    let mut fabricOverlap: [[bool; 1000]; 1000] = [[false; 1000]; 1000];
    let mut totalSquareInches = 0;
    let mut overlapMap = HashMap::new();
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    for line in &arr {
        let mut id: i32 = -1;
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        for cap in re.captures_iter(&line) {
            id = cap[1].to_string().parse::<i32>().unwrap(); 
            x = cap[2].to_string().parse::<i32>().unwrap(); 
            y = cap[3].to_string().parse::<i32>().unwrap(); 
            width = cap[4].to_string().parse::<i32>().unwrap();
            height = cap[5].to_string().parse::<i32>().unwrap();
            // println!("Res: {:?}, {:?}", cap, height);
        }
        for i in x..(x+width) {
            for j in y..(y+height) {
                let val = fabric[i as usize][j as usize];
                if val > 0 {
                    overlapMap.entry(val).or_insert(true);
                    overlapMap.entry(id).or_insert(true);

                    if !fabricOverlap[i as usize][j as usize] {
                        totalSquareInches += 1;
                        fabricOverlap[i as usize][j as usize] = true;
                    }
                } 
                fabric[i as usize][j as usize] = id;
            }
        }
    }

    // Part 1
    println!("Total Square Inches: {}",  totalSquareInches);

    // Part 2
    for i in 1..arr.len() {
        if !overlapMap.contains_key(&(i as i32)) {
            println!("Claim without any overlap: {}",  i);
        }
    }
}