use std::fs::File;
use std::io::prelude::*;


fn count_trees(map: &[&str], right: usize, down: usize) -> u32 {

    let (mut r, mut c, mut count) = (0, 0, 0);

    while r < map.len() {

        if map[r].chars().nth(c) == Some('#') {
            count += 1;
        }

        c = (c + right) % map[0].len();
        r += down;
    }

    count
}

fn part1(map: &[&str]) -> u32 {
    count_trees(map, 3, 1)
}

fn part2(map: &[&str]) -> u32 {
    [(1,1), (3,1),(5,1),(7,1),(1,2)]
        .iter()
        .map(|(r,d)| count_trees(map, *r, *d))
        .fold(1, |acc, i| acc * i)
}



fn main() -> std::io::Result<()> {

    let mut file = File::open("day03.txt")?;
    let mut inp = String::new();
    file.read_to_string(&mut inp)?;

    let map: Vec<&str>= inp[..inp.len()-1].split("\n").collect();

    let r1 = part1(&map);

    println!("PART1 : {}", r1);

    let r2 = part2(&map);
    println!("PART2 : {}", r2);
    Ok(())

}
