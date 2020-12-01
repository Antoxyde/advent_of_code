use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {

    let mut file = File::open("dayXX.txt")?;
    let mut inp = String::new();
    file.read_to_string(&mut inp)?;

    // let mut nums: Vec<u32> = inp[..inp.len()-1].split(",").map(|x| x.parse::<u32>().expect("Failed to parse u32.")).collect();
    // let mut nums: Vec<u32> = inp[..inp.len()-1].split("\n").map(|x| x.parse::<u32>().expect("Failed to parse u32.")).collect();

    Ok(())

}
