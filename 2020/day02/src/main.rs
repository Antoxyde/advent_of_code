use std::fs::File;
use std::io::prelude::*;

fn part1(entries: &[PasswordEntry]) -> usize {

    let mut result = 0;

    for entry in entries {

        let count = entry.password.
            chars()
            .filter(|x| *x == entry.letter)
            .count();

        if count >= entry.min && count <= entry.max {
            result += 1;
        }
    }

    result
}

fn part2(entries: &[PasswordEntry]) -> usize {

    let mut result = 0;

    for entry in entries {

        if (entry.password.chars().nth(entry.min - 1).unwrap() == entry.letter) ^ (entry.password.chars().nth(entry.max - 1).unwrap() == entry.letter) {
            result += 1;
        }
    }

    result
}

#[derive(Debug, Default)]
struct PasswordEntry {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}


fn main() -> std::io::Result<()> {

    let mut file = File::open("day2.txt")?;
    let mut inp = String::new();
    file.read_to_string(&mut inp)?;

    let mut entries : Vec<PasswordEntry> = Vec::new();

    for line in inp[..inp.len()-1].split("\n") {
        let mut entry = PasswordEntry::default();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let minmax: Vec<&str> = parts[0].split("-").collect();
        entry.min = minmax[0].parse::<usize>().expect("parsing failed");
        entry.max = minmax[1].parse::<usize>().expect("parsing failed");
        entry.letter = parts[1].chars().nth(0).unwrap();
        entry.password = parts[2].to_string();

        entries.push(entry);

    }

    let r1 = part1(&entries);
    println!("PART1 : {}", r1);

    let r2 = part2(&entries);
    println!("PART2 : {}", r2);
    Ok(())

}
