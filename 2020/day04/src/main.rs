use std::fs::File;
use std::io::prelude::*;

extern crate regex;
use regex::Regex;

#[derive(Default, Debug)]
struct PassportEntry {
    ecl: Option<String>,
    pid: Option<String>,
    eyr: Option<String>,
    hcl: Option<String>,
    byr: Option<String>,
    iyr: Option<String>,
    cid: Option<String>,
    hgt: Option<String>,
}

impl PassportEntry {
    pub fn validate_ecl(&self) -> bool {
        if let Some(v) = &self.ecl {
            return ["amb","blu","brn","gry","grn","hzl","oth"].iter().any(|&i| i == v);
        }
        false
    }

    pub fn validate_pid(&self) -> bool {
        let reg = Regex::new(r"^\d{9}$").unwrap();
        if let Some(v) = &self.pid {
            return reg.is_match(&v);
        }
        false
    }

    pub fn validate_iyr(&self) -> bool {
        if let Some(v) = &self.iyr {
            return match v.parse::<i32>() {
                Err(_) => false,
                Ok(n) => n >= 2010 && n <= 2020,
            };
        }
        false
    }


    pub fn validate_hcl(&self) -> bool {
        let reg = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        if let Some(v) = &self.hcl {
            return reg.is_match(&v);
        }
        false
    }

    pub fn validate_byr(&self) -> bool {
        if let Some(v) = &self.byr {
            return match v.parse::<i32>() {
                Err(_) => false,
                Ok(n) => n >= 1920 && n <= 2002,
            };
        }
        false
    }

    pub fn validate_eyr(&self) -> bool {
        if let Some(v) = &self.eyr {
            return match v.parse::<u32>() {
                Err(_) => false,
                Ok(n) => n >= 2020 && n <= 2030,
            };
        }
        false
    }

    pub fn validate_hgt(&self) -> bool {
        if let Some(v) = &self.hgt {
            return match &v[v.len() - 2..] {
                "in" =>
                    match v[..v.len() - 2].parse::<i32>() {
                        Err(_) => false,
                        Ok(n) => n >= 59 && n <= 76,
                    },
                "cm" => match v[..v.len() - 2].parse::<i32>() {
                        Err(_) => false,
                        Ok(n) => n >= 150 && n <= 193,
                    },
                &_ => false,
            };
        }
        false
    }

    pub fn validate_all(&self) -> bool {
        self.validate_hgt() &&
        self.validate_hcl() &&
        self.validate_eyr() &&
        self.validate_iyr() &&
        self.validate_byr() &&
        self.validate_pid() &&
        self.validate_ecl()
    }

}

fn part1(pps: &[PassportEntry]) -> u32 {
    let mut result = 0;

    for pass in pps {
        if !(pass.ecl.is_none() ||
            pass.pid.is_none() ||
            pass.eyr.is_none() ||
            pass.hcl.is_none() ||
            pass.byr.is_none() ||
            pass.iyr.is_none() ||
            pass.hgt.is_none() ) {
            result += 1;
        }
    }

    result
}

fn part2(pps: &[PassportEntry]) -> u32 {
    pps.iter().filter(|p| p.validate_all()).count() as u32
}

fn main() -> std::io::Result<()> {

    let mut file = File::open("day04.txt")?;
    let mut inp = String::new();
    file.read_to_string(&mut inp)?;

    let mut passes: Vec<PassportEntry> = Vec::new();

    for line in inp[..inp.len()-1].split("\n\n") {

        let mut pass = PassportEntry::default();
        let data  = line.replace("\n", " ");

        for carac in data.split(" ") {
            let vals: Vec<&str> = carac.split(":").collect();
            let n = vals[0];
            let v = vals[1].to_string();
            match n {
                "ecl" => pass.ecl = Some(v),
                "pid" => pass.pid = Some(v),
                "eyr" => pass.eyr = Some(v),
                "hcl" => pass.hcl = Some(v),
                "byr" => pass.byr = Some(v),
                "iyr" => pass.iyr = Some(v),
                "cid" => pass.cid = Some(v),
                "hgt" => pass.hgt = Some(v),
                &_ => unreachable!(),
            };
        }
        passes.push(pass);
    }

    let r1 = part1(&passes);
    println!("PART1 : {}", r1);

    let r2 = part2(&passes);
    println!("PART2 : {}", r2);
    Ok(())

}
