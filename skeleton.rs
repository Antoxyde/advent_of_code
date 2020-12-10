use std::fs::File;
use std::io::prelude::*;


fn part1(nums: &[u64]) -> usize {
    0
}

fn main() -> std::io::Result<()> {

    let mut file = File::open("dayXX.txt")?;
    let mut inp = String::new();
    file.read_to_string(&mut inp)?;

    // let mut nums: Vec<u64> = inp[..inp.len()-1].split("\n").map(|x| x.parse::<u64>().unwrap()).collect();

    let r1 = part1();
    println!("PART1 : {}", r1);

    //let r2 = part2();
    //println!("PART2 : {}", r2);
    Ok(())

}
