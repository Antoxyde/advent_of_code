use std::fs::File;
use std::io::prelude::*;
use std::cmp::{min,max};


fn part1(nums: &[u64], size: usize) -> u64 {

    for i in size..nums.len() {
        let mut found = false;

        'inner: for j in i-size..i {
            for k in i-size..i  {
                if j != k && nums[j] + nums[k] == nums[i] {
                    found = true;
                    break 'inner;
                }
            }
        }

        if !found {
            return nums[i];
        }
    }

    0
}

fn part2(nums: &[u64]) -> u64 {

    let goal = part1(nums, 25);

    for i in 0..nums.len()  - 1 {
        let mut j = i + 1;
        let mut sum = nums[i] + nums[j];

        while j < nums.len() && sum < goal {
            j += 1;
            sum += nums[j];
        }

        if sum == goal {
            return nums[i..=j].iter().min().unwrap() + nums[i..=j].iter().max().unwrap();
        }
    }

    0
}

fn main() -> std::io::Result<()> {

    let mut file = File::open("day09.txt")?;
    let mut inp = String::new();
    file.read_to_string(&mut inp)?;

    let mut nums: Vec<u64> = inp[..inp.len()-1].split("\n").map(|x| x.parse::<u64>().unwrap()).collect();

    let r1 = part1(&nums, 25);
    println!("PART1 : {}", r1);

    let r2 = part2(&nums);
    println!("PART2 : {}", r2);
    Ok(())

}
