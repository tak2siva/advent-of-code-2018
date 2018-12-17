use std::fs;
use regex::Regex;
use std::collections::HashMap;
use std::vec::Vec;
use std::ptr;
use std::fmt;


#[derive(Copy)]
// #[derive(Debug)]
struct Point {
    id: i32,
    distance: i32,
    conflict: bool,
}
impl Clone for Point {
    fn clone(&self) -> Point { *self }
}
impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.id == -1 {
            write!(f, "{}", ".")
        } else {
            write!(f, "{}", (65 + self.id as u8) as char)
        }
    
    }
}

pub fn solve() {
    println!("\n*************   Day - 6 ******************");
    let txtPath = [env!("CARGO_MANIFEST_DIR"), "/inputs/day6.input1.txt"].join("");
    let contents = fs::read_to_string(txtPath).expect("err reading file");
    let arr: Vec<&str> = contents.split("\n").collect();
    let mut coordinates: Vec<Vec<i32>> = Vec::new();
    let mut xMax = -1;
    let mut yMax = -1;
    let mut ignoreMap: HashMap<i32, i32> = HashMap::new();
    
    for (i, line) in arr.iter().enumerate() {
        let splitVal: Vec<&str> = line.split(", ").collect();
        let y = splitVal[0].parse::<i32>().unwrap();
        let x = splitVal[1].parse::<i32>().unwrap();      
        coordinates.push(Vec::new());
        coordinates[i].push(x);
        coordinates[i].push(y);
        if x > xMax {
            xMax = x;
        }
        if y > yMax {
            yMax = y;
        }
    }
    let mut grid: Vec<Vec<Option<Point>>> = Vec::new();
    for i in 0..xMax+2 {
        grid.push(vec![None::<Point>; (yMax + 2) as usize]);
        for j in 0..yMax+2 {     
            for (pos, c) in coordinates.iter().enumerate() {
                let distance = manhattanDistance(c[0], i, c[1], j);
                if grid[i as usize][j as usize].is_none() {
                    grid[i as usize][j as usize] = Some(Point {id: pos as i32 , distance: distance, conflict: false,});
                } else {
                    let mut prevPoint = grid[i as usize][j as usize].unwrap();
                    if distance < prevPoint.distance {
                        grid[i as usize][j as usize] = Some(Point {id: pos as i32, distance: distance, conflict: false});
                    } else if distance == prevPoint.distance {
                        grid[i as usize][j as usize] = Some(Point {id: -1, distance: distance, conflict: true});
                        if i == xMax+1 || j == yMax+1 {
                            ignoreMap.entry(pos as i32).or_insert(1);
                        }
                    }
                }
            }
        }
    }

    let mut minPoint = Point{id: -1, distance: -1, conflict: false};
    let mut countMap: HashMap<i32, i32> = HashMap::new();
    for i in 0..xMax+2 {
        ignoreMap.entry(grid[i as usize][0].unwrap().id).or_insert(1);
        ignoreMap.entry(grid[i as usize][(yMax) as usize].unwrap().id).or_insert(1);
    }
    for i in 0..yMax+2 {
        ignoreMap.entry(grid[0][i as usize].unwrap().id).or_insert(1);
        ignoreMap.entry(grid[(xMax) as usize][i as usize].unwrap().id).or_insert(1);
    }
    for i in 0..xMax+2 {
        for j in 0..yMax+2 {     
            let currPoint = grid[i as usize][j as usize].unwrap();
            if !ignoreMap.contains_key(&currPoint.id) && !currPoint.conflict && currPoint.id != -1 {
                countMap.entry(currPoint.id).and_modify(|e| {*e+=1}).or_insert(1);
            }
        }
    }
    // println!("map {:?}", ignoreMap);
    // println!("map {:?}", countMap);
    let mut res: i32 = -1;
    for (k,v) in countMap.iter() {
        if res < *v {
            res = v.clone();
        }
    }   
    // Part 1 Solution
    println!("{:?}", res);

    let mut regionSize: i32 = 0;
    for i in 0..xMax+2 {
        for j in 0..yMax+2 {     
            let mut combinedDistance: i32 = 0;
            for (pos, c) in coordinates.iter().enumerate() {
                combinedDistance += manhattanDistance(c[0], i, c[1], j);
            }
            if combinedDistance < 10000 {
                regionSize += 1;
            }
        }
    }
    // Part 2 Solution
    println!("{:?}", regionSize);
}

// Manhattan distance |x1 - x2| + |y1 - y2|
fn manhattanDistance(x1: i32, x2: i32, y1: i32, y2: i32) -> i32 {
    ((x1-x2).abs() + (y1-y2).abs())
}

// Might require if there are multiple region / clusters in Part 2
fn expandThePoint(i: i32, j: i32, map: &mut HashMap<String, String>, arr: &Vec<Vec<i32>>, xMax: i32, yMax: i32) {
    if i-1 >= 0 {
        expandThePoint(i-1, j, map, &arr, xMax, yMax);
    }
    if i+1 < yMax {
        expandThePoint(i+1, j, map, &arr, xMax, yMax);
    }
    if j-1 >= 0 {
        expandThePoint(i, j-1, map, &arr, xMax, yMax);
    }
    if j+1 >= 0 {
        expandThePoint(i, j+1, map, &arr, xMax, yMax);
    }
    if i-1 >= 0 && j-1 >= 0 {
        expandThePoint(i-1, j-1, map, &arr, xMax, yMax);   
    }
    if i+1 < xMax && j-1 >= 0 {
        expandThePoint(i+1, j-1, map, &arr, xMax, yMax);
    }
    if i-1 >= 0 && j+1 < yMax {
        expandThePoint(i-1, j+1, map, &arr, xMax, yMax);
    }
    if i+1 < yMax && j+1 < yMax {
        expandThePoint(i+1, j+1, map, &arr, xMax, yMax);
    }
}