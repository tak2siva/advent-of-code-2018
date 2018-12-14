#![allow(non_snake_case)]
#[macro_use]

use std::fs;
use regex::Regex;
use std::path::PathBuf;

pub fn solve() {
    println!("\n*************   Day - 3 ******************");
    let txtPath = [env!("CARGO_MANIFEST_DIR"), "/inputs/day3.input1.txt"].join("");
    let contents = fs::read_to_string(txtPath).expect("err reading file");
    let arr: Vec<&str> = contents.split("\n").collect();
    let mut fabric: [[i32; 1000]; 1000] = [[0; 1000]; 1000];
    let mut totalSquareInches = 0;

    for line in arr {
        let re = Regex::new(r"@ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        for cap in re.captures_iter(&line) {
            x = cap[1].to_string().parse::<i32>().unwrap(); 
            y = cap[2].to_string().parse::<i32>().unwrap(); 
            width = cap[3].to_string().parse::<i32>().unwrap();
            height = cap[4].to_string().parse::<i32>().unwrap();
            // println!("Res: {:?}, {:?}", width, height);
        }
        
        for i in x..(x+width) {
            for j in y..(y+height) {
                fabric[i as usize][j as usize] += 1;
                if fabric[i as usize][j as usize] == 2 {
                    totalSquareInches += 1;
                }
            }
        }
    }

    // for i in 0..10 {
    //     for j in 0..10 {
    //         print!("{} ", fabric[i as usize][j as usize]);
    //     }
    //     println!("")
    // }
    println!("Total Square Inches: {}",  totalSquareInches);
}