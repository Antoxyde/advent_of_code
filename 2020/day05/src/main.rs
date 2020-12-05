use std::fs::File;
use std::io::prelude::*;

fn parse_bp(bpstr: &str) -> u32 {

    let row = u32::from_str_radix(&bpstr[..bpstr.len() - 3].replace("F", "0").replace("B", "1"), 2).unwrap();
    let col = u32::from_str_radix(&bpstr[bpstr.len() - 3..].replace("R", "1").replace("L", "0"), 2).unwrap();
    row * 8 + col
}


fn part1(bps: &mut [u32]) -> u32 {
    bps.sort();
    bps[bps.len() - 1]
}

fn part2(bps: &mut [u32]) -> u32 {

    for i in 1..1024 {
        if !bps.contains(&i) {

            if i >> 8 != 0b11 && i >> 8 != 0 {
                return i;
            }
        }
    }
    0
}

fn main() -> std::io::Result<()> {

    let mut file = File::open("day05.txt")?;
    let mut inp = String::new();
    file.read_to_string(&mut inp)?;


    let mut bps: Vec<u32> = Vec::new();

    for bp in inp[..inp.len()-1].split("\n") {
        let num = parse_bp(&bp);
        bps.push(num);
    }

    let r1 = part1(&mut bps);
    println!("PART1 : {}", r1);

    let r2 = part2(&mut bps);
    println!("PART2 : {}", r2);
    Ok(())

}
