use std::fs::File;
use std::io::prelude::*;


fn part1(nums: &[u64]) -> usize {
    nums.windows(2)
        .filter(|&p| p[0] < p[1])
        .count()
}

fn part2(nums: &[u64]) -> usize {
    nums.windows(4)
        .filter(|&p| p[0] < p[3])
        .count()
}

fn main() -> std::io::Result<()> {

    let mut file = File::open("day01.txt")?;
    let mut inp = String::new();
    file.read_to_string(&mut inp)?;

    let nums: Vec<u64> = inp[..inp.len()-1].split("\n").map(|x| x.parse::<u64>().unwrap()).collect();

    let r1 = part1(&nums);
    println!("PART1 : {}", r1);

    let r2 = part2(&nums);
    println!("PART2 : {}", r2);
    Ok(())

}
