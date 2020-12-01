use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {

    let mut file = File::open("day1.txt")?;
    let mut inp = String::new();
    file.read_to_string(&mut inp)?;

    inp.truncate(inp.len() - 1);

    let nums: Vec<u32> = inp.split("\n").map(|x| x.parse::<u32>().expect("Failed to parse u32.")).collect();

    for n1 in &nums {
        for n2 in &nums {
            if n1 + n2 == 2020 {
                println!("{}", n1 * n2);
            }
        }
    }

    Ok(())

}
