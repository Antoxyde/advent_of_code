use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
enum MaskMem {
    Mem((i64,i64)),
    Mask(String),
}

fn apply_mask(val: i64, mask: &str) -> i64 {
    let mut res = val;

    for (idx, bit) in mask.chars().enumerate() {
        match bit {
            '1' => res |= 1 << (35 - idx),
            '0' => res &= !(1 << (35 - idx)),
            'X' => (),
            _ => panic!(),
        };
    }

    res
}

fn apply_mask_v2(val: i64, mask: &str) -> Vec<i64> {

    let mut res = val;
    let mut values = Vec::new();
for (idx, bit) in mask.chars().enumerate() { match bit { '1' => res |= 1 << (35 - idx),
            '0' => (),
            'X' => (),
            _ => panic!(),
        };
    }

    for i in 0..i64::pow(2,mask.matches('X').count() as u32) {
        let mut tempval = res;
        let mut count = 0;
        for (idx,c) in mask.chars().enumerate() {
            if c == 'X' {
                let bit =  (i >> count) & 1;
                match bit {
                    1 => tempval |= 1 << (35 - idx),
                    0 => tempval &= !(1 << (35 - idx)),
                    _ => (),
                }
                count += 1;
            }
        }
        values.push(tempval);
    }

    values
}


fn part1(nums: &[MaskMem]) -> i64 {

    let mut mask = String::new();
    let mut storage = HashMap::new();

    for inst in nums {
        match inst {
            MaskMem::Mem((off,val)) => { storage.insert(off,apply_mask(*val, &mask)); () }
            MaskMem::Mask(m) => mask = String::from(m),
        };
    }

    storage.iter().map(|(_,v)| v).sum()
}


fn part2(nums: &[MaskMem]) -> i64 {

    let mut mask = String::new();
    let mut storage = HashMap::new();

    for inst in nums {
        match inst {
            MaskMem::Mem((off,val)) => {
                for key in apply_mask_v2(*off, &mask){
                    storage.insert(key,*val);
                }},
            MaskMem::Mask(m) => mask = String::from(m),
        };
    }

    storage.iter().map(|(_,v)| v).sum()
}


fn parse_line(line: &str) -> MaskMem {
    let words : Vec<&str> = line.split(" = ").collect();
    match words[0] {
        "mask" => MaskMem::Mask(words[1].to_string()),
        _ => {
            let offset = words[0][4..words[0].len() - 1].parse().unwrap();
            let value = words[1].parse().unwrap();
            MaskMem::Mem((offset, value))
            },
        }
}

fn main() -> std::io::Result<()> {

    let mut file = File::open("day14.txt")?;
    let mut inp = String::new();
    file.read_to_string(&mut inp)?;

    let inp : Vec<MaskMem> = inp[..inp.len()-1].split("\n").map(parse_line).collect();

    let r1 = part1(&inp);
    println!("PART1 : {}", r1);

    let r2 = part2(&inp);
    println!("PART2 : {}", r2);
    Ok(())

}
