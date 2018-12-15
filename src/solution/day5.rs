use std::fs;
use regex::Regex;
use std::collections::HashMap;
use std::vec::Vec;

pub fn solve() {
    println!("\n*************   Day - 5 ******************");

    let txtPath = [env!("CARGO_MANIFEST_DIR"), "/inputs/day5.input1.txt"].join("");
    let contents = fs::read_to_string(txtPath).expect("err reading file");
    let mut stack: Vec<char> = Vec::new();
    let stackRef = &mut stack;
    for ch in contents.chars() {
        if !stackRef.is_empty() && (((*stackRef.last().unwrap() as i32) - (ch as i32)).abs() == 32) {
            stackRef.pop();
        } else {
            stackRef.push(ch);
        }
    }

    // Part 1
    println!("Size of remaining units {}", stackRef.len());

    stackRef.clear();
    static ASCII_LOWER: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
                                    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
                                    's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let mut minPolymer: i32 = i32::max_value();
    for alpha in ASCII_LOWER.iter() {
        let regexStr = format!("({}|{})", alpha, alpha.to_uppercase());
        let regexReplace = Regex::new(&regexStr).unwrap();
        let newStr = regexReplace.replace_all(&contents, "");
        for ch in newStr.chars() {
            if !stackRef.is_empty() && (((*stackRef.last().unwrap() as i32) - (ch as i32)).abs() == 32) {
                stackRef.pop();
            } else {
                stackRef.push(ch);
            }
        }
        if (stackRef.len() as i32) < minPolymer {
            minPolymer = stackRef.len() as i32;
        }
        stackRef.clear();
    }

    // Part 2
    println!("Size of shortest polymer {}", minPolymer);
}