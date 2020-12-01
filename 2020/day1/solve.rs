use std::fs::File;
use std::io::prelude::*;


fn part1(nums: &[u32]) -> u32 {
    for n1 in nums {
        for n2 in nums {
            if n1 + n2 == 2020 {
                return n1 * n2;
            }
        }
    }

    0
}

fn part2(nums: &[u32]) -> u32 {

    for n1 in nums {
        for n2 in nums {
            for n3 in nums {
                if n1 + n2 + n3 == 2020 {
                    return n1 * n2 * n3;
                }
            }
        }
    }

    0
}


fn main() -> std::io::Result<()> {

    let mut file = File::open("day1.txt")?;
    let mut inp = String::new();
    file.read_to_string(&mut inp)?;

    let nums: Vec<u32> = inp[..inp.len()-1].split("\n").map(|x| x.parse::<u32>().expect("parsing failed")).collect();

    let r1 = part1(&nums);
    println!("PART1 : {}", r1);

    let r2 = part2(&nums);
    println!("PART2 : {}", r2);
    Ok(())

}
