use std::fs::File;
use std::io::prelude::*;

fn parse_line(line: &str) -> i64 {
    i64::from_str_radix(line, 2).unwrap()
}

fn part1(entries: &[i64]) -> i64 {
    let mut gamma = 0; 
    for i in (0..=11).rev() {
        gamma <<= 1;
        let t = entries.iter().map(|x| ((x >> i) & 1) as usize).sum::<usize>();
        if t > entries.len() - t {
            gamma |= 1;
        }
    }

    gamma * ((!gamma) & 0b1111_1111_1111)
}

fn part2(entries: &[i64]) -> i64 {
    
    let mut ox = Vec::from(entries);
    let mut c2 = Vec::from(entries);

    for i in (0..=11).rev() {
        let t = ox.iter().map(|x| ((x >> i) & 1) as usize).sum::<usize>();
        let b = match t >= ox.len() - t {
            true => 1,
            false => 0,
        };

        if ox.len() > 1 { ox = ox.iter().filter(|x| (**x >> i ) & 1 == b).cloned().collect(); }

        let t = c2.iter().map(|x| ((x >> i) & 1) as usize).sum::<usize>();
        let b = match t >= c2.len() - t {
            true => 0,
            false => 1,
        };

        if c2.len() > 1 { c2 = c2.iter().filter(|x| (**x >> i ) & 1 == b).cloned().collect(); }

    }

    ox[0] * c2[0]
}

fn main() -> std::io::Result<()> {

    let mut file = File::open("day03.txt")?;
    let mut inp = String::new();
    file.read_to_string(&mut inp)?;

    let entries: Vec<i64> = inp.lines().map(|l| parse_line(l)).collect();

    let r1 = part1(&entries);
    println!("PART1 : {}", r1);

    let r2 = part2(&entries);
    println!("PART2 : {}", r2);
    Ok(())

}
