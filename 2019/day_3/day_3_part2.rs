use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::iter::FromIterator;

fn gen_way(insts: &[&str]) -> Vec<(i32,i32)> {
    
    let mut way = Vec::new();
    let mut actual_point : (i32, i32) = (0,0);
    //way.push(actual_point);

    for inst in insts {

        let dir = inst.chars().nth(0).unwrap();
        let dist = inst[1..].parse::<i32>().unwrap();

        match dir {
            'R' => {
                let mut pts: Vec<(i32,i32)> = (1..dist + 1).map(|x| (actual_point.0 + x, actual_point.1)).collect();
                way.append(&mut pts);
                actual_point = (actual_point.0 + dist, actual_point.1);
            },
            'L' => {
                let mut pts: Vec<(i32,i32)> = (1..dist + 1).map(|x| (actual_point.0 - x, actual_point.1)).collect();
                way.append(&mut pts);
                actual_point = (actual_point.0 - dist, actual_point.1);
            },
            'U' => {
                let mut pts: Vec<(i32,i32)> = (1..dist + 1).map(|x| (actual_point.0 , actual_point.1 + x)).collect();
                way.append(&mut pts);
                actual_point = (actual_point.0, actual_point.1 + dist);
            },

            'D' => {
                let mut pts: Vec<(i32,i32)> = (1..dist + 1).map(|x| (actual_point.0 , actual_point.1 - x)).collect();
                way.append(&mut pts);
                actual_point = (actual_point.0, actual_point.1 - dist);
            },
            _ => panic!("Yo wtf"),
        }
    }

    return way;
}


fn main() -> std::io::Result<()> {

    let mut file = File::open("input_day_3.txt")?;
    let mut inp = String::new();
    file.read_to_string(&mut inp)?;
    let mut sets : Vec<HashSet<(i32, i32)>> = Vec::new();
    let mut vecs : Vec<Vec<(i32,i32)>> = Vec::new();
    for way in inp.lines() {

        let insts : Vec<&str> = way.split(",").collect();
        let w = gen_way(&insts);
        vecs.push(w.clone());
        sets.push(HashSet::from_iter(w.iter().cloned()));
    }
    
    let result = sets[0].intersection(&sets[1]).map(|p| vecs[0].iter().position(|r| p == r).unwrap() +  vecs[1].iter().position(|r| r == p).unwrap() + 2).min();
    
    println!("Result is {}", result.unwrap());
    Ok(())

}
