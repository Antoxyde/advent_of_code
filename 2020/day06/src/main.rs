use std::fs::File;
use std::io::prelude::*;
use itertools::Itertools;

fn part1(ans: &[&str]) -> usize {
    ans.iter()
        .map(|s|
             s.chars()
             .filter(|x| *x != '\n')
             .unique()
             .count())
        .sum()
}

fn part2(ans: &[&str]) -> usize {

    ans.iter()
        .map(|s|
             s.chars()
            .unique()
            .filter(|x| *x != '\n')
            .filter(|c| s.chars()
                        .filter(|c2| c2 == c)
                        .count() == s.chars()
                                    .filter(|c2| *c2 == '\n')
                                    .count() + 1)
            .count())
        .sum()
}



fn main() -> std::io::Result<()> {

    let mut file = File::open("day06.txt")?;
    let mut inp = String::new();
    file.read_to_string(&mut inp)?;

    let mut groups = Vec::new();

    for group in inp[..inp.len()-1].split("\n\n") {
        groups.push(group);
    }

    let r1 = part1(&groups);
    println!("PART1 : {}", r1);

    let r2 = part2(&groups);
    println!("PART2 : {}", r2);
    Ok(())

}
