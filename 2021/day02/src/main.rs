use std::fs::File;
use std::io::prelude::*;

struct Entry(String, i64);

fn parse_line(line: &str) -> Entry {
    let pair: Vec<&str> = line.split(" ").collect();
    Entry(pair[0].to_string(), pair[1].parse::<i64>().unwrap())
}


fn part1(entries: &[Entry]) -> i64 {
    let mut depth = 0;
    let mut horizontal = 0;
    for Entry(s, u) in entries {
        match (s.as_str(), u) {
            ("forward", val) => horizontal += val,
            ("up", val) => depth -= val,
            ("down", val) => depth += val,
            _ => panic!("yolo"),
        }
    }

    depth * horizontal
}

fn part2(entries: &[Entry]) -> i64 {

    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    for Entry(s, u) in entries {
        match (s.as_str(), u) {
            ("forward", val) => { horizontal += val; depth += val * aim },
            ("up", val) => aim -= val,
            ("down", val) => aim += val,
            _ => panic!("yolo"),
        }
    }

    depth * horizontal
}

fn main() -> std::io::Result<()> {

    let mut file = File::open("day02.txt")?;
    let mut inp = String::new();
    file.read_to_string(&mut inp)?;

    let entries: Vec<Entry> = inp[..inp.len()- 1].split("\n").map(|l| parse_line(l)).collect();

    let r1 = part1(&entries);
    println!("PART1 : {}", r1);

    let r2 = part2(&entries);
    println!("PART2 : {}", r2);
    Ok(())

}
