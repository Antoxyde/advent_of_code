use std::fs::File;
use std::io::prelude::*;

fn run(c: &[u32]) -> u32 {

    let mut code = Vec::from(c);
    let mut ip = 0;

    loop {
        match code[ip] {

            99 => {
                // println!("Returning {}",  code[0]);
                return code[0];
            },
            1 => {
                let op1 = code[code[ip + 1] as usize];
                let op2 = code[code[ip + 2] as usize];
                let res = code[ip + 3];

                code[res as usize] = op1 + op2;
                // println!("{} : {} * {} = {}, code is now {:?}", res ,op1, op2, op1 * op2, code);
            },
            2 => {
                let op1 = code[code[ip + 1] as usize];
                let op2 = code[code[ip + 2] as usize];
                let res = code[ip + 3];

                code[res as usize] = op1 * op2;
                // println!("{} : {} * {} = {}, code is now {:?}", res ,op1, op2, op1 * op2, code);
            },
            _ => {
                panic!("Unkown opcode {}", code[ip]);
            },
        }

        ip += 4;
    }

}

fn main() -> std::io::Result<()> {

    let mut file = File::open("input_day_2.txt")?;
    let mut inp = String::new();
    file.read_to_string(&mut inp)?;

    let mut code: Vec<u32> = inp[..inp.len()-1].split(",").map(|x| x.parse::<u32>().expect("Failed to parse u32.")).collect();
    
    code[1] = 12;
    code[2] = 2;
    
    for noun in 0..100 {
        for verb in 0..100 {

            code[1] = noun;
            code[2] = verb;

            if run(&code) == 19690720 {
                println!("Found verb = {}, noun = {}.", noun, verb);
                println!("Result is {}", noun * 100 + verb);
            }
        }
    }

    Ok(())

}
