use std::fs::File;
use std::io::prelude::*;

fn tick(code: &mut [u32], ip: usize) -> usize {
    match code[ip] {
        99 => {
            println!("Exiting .. code[0] is {}",  code[0]);
            std::process::exit(0);
        },
        1 => {
            let op1 = code[code[ip + 1] as usize];
            let op2 = code[code[ip + 2] as usize];
            let res = code[ip + 3];

            code[res as usize] = op1 + op2;
            println!("{} : {} * {} = {}, code is now {:?}", res ,op1, op2, op1 * op2, code);
        },
        2 => {
            let op1 = code[code[ip + 1] as usize];
            let op2 = code[code[ip + 2] as usize];
            let res = code[ip + 3];

            code[res as usize] = op1 * op2;
            println!("{} : {} * {} = {}, code is now {:?}", res ,op1, op2, op1 * op2, code);
        },
        _ => {
            panic!("Unkown opcode {}", code[ip]);
        },
    }

    return ip + 4;
}

fn main() -> std::io::Result<()> {

    let mut file = File::open("input_day_2.txt")?;
    let mut inp = String::new();
    file.read_to_string(&mut inp)?;

    let mut code: Vec<u32> = inp[..inp.len()-1].split(",").map(|x| x.parse::<u32>().expect("Failed to parse u32.")).collect();
    let mut ip :usize = 0;
    
    code[1] = 12;
    code[2] = 2;

    loop {
        ip = tick(&mut code, ip);
    }

    Ok(())

}
