use std::fs::File;
use std::io::prelude::*;


fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn crt(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}


fn part1(nums: &[(i64, i64)], goal: i64) -> i64 {
    let mut sorted: Vec<(i64, i64)> = Vec::from(nums);
    sorted.sort_by_key(|(_,x)| x - (goal % x));
    sorted[0].1 * (sorted[0].1 - (goal % sorted[0].1))
}

fn part2(nums: &[(i64, i64)]) -> i64 {

    let residues = nums.iter().map(|&(i,v)| v - i).collect::<Vec<_>>();
    let modulis = nums.iter().map(|&(_,v)| v).collect::<Vec<_>>();
    crt(&residues, &modulis).unwrap()
}

fn main() -> std::io::Result<()> {

    let mut file = File::open("day13.txt")?;
    let mut inp = String::new();
    file.read_to_string(&mut inp)?;

    let lines: Vec<&str> = inp[..inp.len()-1].split("\n").collect();
    let goal = lines[0].parse::<i64>().expect("parsing failed");
    let nums: Vec<(i64, i64)> = lines[1].split(",").enumerate().filter(|(_,x)| x != &"x").map(|(i,x)| (i as i64, x.parse().unwrap())).collect();


    let r1 = part1(&nums, goal);
    println!("PART1 : {}", r1);

    let r2 = part2(&nums);
    println!("PART2 : {}", r2);
    Ok(())

}
