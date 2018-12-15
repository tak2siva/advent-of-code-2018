use std::fs;
use regex::Regex;
use std::collections::HashMap;
use std::vec::Vec;

pub fn solve() {
    println!("\n*************   Day - 4 ******************");

    let txtPath = [env!("CARGO_MANIFEST_DIR"), "/inputs/day4.input1.txt"].join("");
    let contents = fs::read_to_string(txtPath).expect("err reading file");
    let arr: Vec<&str> = contents.split("\n").collect();

    let mut inputMap: HashMap<i64, String> = HashMap::new();
    let mut sleepLog: HashMap<String, Vec<i32>> = HashMap::new();
    let mut sleepDuration: HashMap<String, i32> = HashMap::new();
    let mut orderedLog: Vec<i64> = Vec::new();

    let regexSplitDate: Regex = Regex::new(r"\[(.*?)\] ([[:alpha:]].*)").unwrap();
    let regexReplace = Regex::new(r"(:|-| )").unwrap();

    for line in &arr {
        // lazy_static! {
            // static ref RE: Regex = Regex::new(r"\[(.*?)\] ([[:alpha:]].*)").unwrap();
        // }
        for cap in regexSplitDate.captures_iter(&line) {
            let dateStr = cap[1].to_string();
            let summary = cap[2].to_string();
            let dateStr2 = regexReplace.replace_all(&dateStr, "");
            let timeId: i64 = dateStr2.parse::<i64>().unwrap();
            orderedLog.push(timeId);
            inputMap.insert(timeId, summary.clone());
        }       
    }
    orderedLog.sort();
    let sleepLogRef = &mut sleepLog;
    let mut sleepDurationRef = &mut sleepDuration;
    let mut guardId: String = "".to_string();
    let mut sleepStart: i64 = 0;
    for (pos, timeSeq) in orderedLog.iter().enumerate() {
        let summary = inputMap.get(timeSeq).unwrap();
        lazy_static! {
            static ref GuardRotate: Regex = Regex::new(r"Guard #(\d+) ").unwrap();
        }
        if GuardRotate.is_match(&summary) {
            for cap in GuardRotate.captures_iter(&summary) {
                guardId = cap[1].to_string();
            }
            sleepLogRef.entry(guardId.clone()).or_insert(vec![0; 61]);
        } else if summary == "falls asleep" {
            sleepStart = timeSeq.clone();
        } else if summary == "wakes up" {
            let sleepEnd = timeSeq.clone();
            let duration = ((sleepEnd % 100) - (sleepStart % 100)) as i32;
            sleepDurationRef.entry(guardId.clone()).and_modify(|e| {*e+=duration}).or_insert(duration.clone());
            for x in sleepStart..sleepEnd {
                let idx = x % 100;
                sleepLogRef.entry(guardId.clone()).and_modify(|e| {e[idx as usize]+= 1});
            }
        } else {
            println!("Error");
        }
    }
    let mut maxSleeper = "";
    let mut maxVaule: i32 = -1;
    for (gId, value) in sleepDurationRef.into_iter() {
        if value.clone() > maxVaule {
            maxSleeper = gId;
            maxVaule = value.clone();
        }
    }   
    let mut bestTime: i32 = 0;
    let mut maxSleepOverlap = 0;
    for (pos, val) in sleepLogRef.get(maxSleeper).unwrap().iter().enumerate() {
        if val.clone() > maxSleepOverlap {
            bestTime = pos as i32;
            maxSleepOverlap = val.clone();
        }
    }

    let mut maxSleepOverlapGlobal = 0;
    let mut maxSleepOverlapGlobalPos = 0;
    let mut maxSleepOverlapGlobalGid: String = "".to_string();
    for (gId, arr) in sleepLogRef.into_iter() {
        for x in 0..61 {
            if arr[x].clone() > maxSleepOverlapGlobal {
                maxSleepOverlapGlobal = arr[x] as i32;
                maxSleepOverlapGlobalGid = gId.clone();
                maxSleepOverlapGlobalPos = x;
            }
        }
    }
    println!("Part 1 Solution: {}", maxSleeper.parse::<i32>().unwrap() * bestTime);
    println!("Part 2 Solution: {}", maxSleepOverlapGlobalGid.parse::<i32>().unwrap() * maxSleepOverlapGlobalPos as i32);
}