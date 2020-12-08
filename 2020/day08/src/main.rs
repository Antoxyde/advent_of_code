use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
struct Inst {
    val: i32,
    opcode: String,
}

fn parse_line(l: &str) -> Inst {
    let data : Vec<&str> = l.split(" ").collect();

    Inst { val: data[1].parse().unwrap(),  opcode: data[0].to_owned() }
}


fn exec(code: &[Inst]) -> (i32, i32) {

    let mut acc = 0i32;
    let mut ip = 0i32;
    let mut done : Vec<i32> = Vec::new();

    while ip < code.len() as i32 && !done.contains(&ip)  {

        done.push(ip);
        //println!("executing {} : {} {}", ip, code[ip.abs() as usize].opcode, code[ip.abs() as usize].val);

        match code[ip as usize].opcode.as_str() {
            "acc" => {acc += code[ip.abs() as usize].val; ip += 1; },
            "jmp" => ip += code[ip.abs() as usize].val,
            "nop" | _ => ip += 1,
        }

    }

    (ip,acc)
}

fn part1(code: &[Inst]) -> i32 {
    exec(code).1
}

fn part2(code: &[Inst]) -> i32 {

    for ins in 0..code.len() {
        let mut modified = Vec::from(code);
        match modified[ins].opcode.as_str() {
            "jmp" => modified[ins].opcode = "nop".to_owned(),
            "nop" => modified[ins].opcode = "jmp".to_owned(),
            _ => (),
        }

        let (ip,acc) =  exec(&modified);

        if ip == code.len() as i32 {
            return acc;
        }
    }

    0
}

fn main() -> std::io::Result<()> {

    let mut file = File::open("day08.txt")?;
    let mut inp = String::new();
    file.read_to_string(&mut inp)?;

    let code: Vec<Inst> = inp[..inp.len()-1].split("\n").map(parse_line).collect();

    let r1 = part1(&code);
    println!("PART1 : {}", r1);

    let r2 = part2(&code);
    println!("PART2 : {}", r2);
    Ok(())

}
