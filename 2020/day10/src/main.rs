use std::fs::File;
use std::io::prelude::*;


fn part1(nums: &mut [u64]) -> usize {

    let mut counts = vec![0,0,0];

    nums.sort();

    counts[nums[0] as usize - 1] += 1;
    counts[2] += 1; // outlet

    for i in 0..nums.len() - 1 {
        let diff = nums[i + 1] - nums[i];
        counts[diff as usize - 1] += 1;
    }

    counts[0] * counts[2]
}

fn part2(nums: &[u64])  -> u64 {


    let mut counts = vec![0; nums[nums.len() - 1] as usize + 1];
    counts[0] = 1;

    for &n in nums {
        for i in 1..4 {
            if n >= i {
                counts[n as usize] += counts[(n - i) as usize] ;
            }
        }
    }

    counts[nums[nums.len() - 1] as usize]
}

fn main() -> std::io::Result<()> {

    let mut file = File::open("day10.txt")?;
    let mut inp = String::new();
    file.read_to_string(&mut inp)?;

    let mut nums: Vec<u64> = inp[..inp.len()-1].split("\n").map(|x| x.parse::<u64>().unwrap()).collect();

    let r1 = part1(&mut nums);
    println!("PART1 : {}", r1);

    nums.push(nums[nums.len() - 1] + 3);

    let r2 = part2(&mut nums);
    println!("PART2 : {}", r2);
    Ok(())

}
