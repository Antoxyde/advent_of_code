use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, VecDeque};

type Rules = HashMap<String, Vec<(u32, String)>>;

fn parse_line(line: &str) -> (String, Vec<(u32, String)>) {
    let datas: Vec<&str> = line.split(" bags contain ").collect();
    let mut contains = Vec::new();

    if !datas[1].contains("no other") {
        for bags in datas[1].trim().split(",") {
            let bag : Vec<&str> = bags.trim().split(" ").collect();
            contains.push((bag[0].parse::<u32>().unwrap(),
                        format!("{} {}", bag[1],bag[2])));
        }
    }


    (datas[0].to_owned(), contains)
}

fn part1(rules: &Rules) -> usize {
    let mut done = vec!["shiny gold"];
    let mut queue = VecDeque::new();
    queue.push_back("shiny gold");

    while !queue.is_empty() {

        let vertice = queue.pop_front().unwrap();

        for rule in rules.keys() { // mmh thats dirty
            for (_,col) in &rules[rule] {
                if col == vertice {
                    queue.push_back(rule);
                    if !done.contains(&rule.as_str()) {
                        done.push(&rule);
                    }
                }
            }
        }
    }

    done.len() - 1
}


fn part2(rules: &Rules, root: &str) -> u32 {

    rules[root].iter()
        .map(|(n,col)| n + n * part2(rules, &col))
        .sum()
}

fn main() -> std::io::Result<()> {

    let mut file = File::open("day07.txt")?;
    let mut inp = String::new();
    file.read_to_string(&mut inp)?;

    let rules: Rules = inp[..inp.len()- 1].split("\n").map(|l| parse_line(l)).collect();

    let r1 = part1(&rules);
    println!("PART1 : {}", r1);

    let r2 = part2(&rules, "shiny gold");
    println!("PART2 : {}", r2);

    Ok(())
}
