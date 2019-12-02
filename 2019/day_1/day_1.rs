use std::fs::File;
use std::io::prelude::*;


fn compute_fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn main() -> std::io::Result<()> {

    let mut file = File::open("input_day_1.txt")?;
    let mut masses = String::new();
    file.read_to_string(&mut masses)?;

    let mut total_fuel: i32 = 0;
    
    for mass in masses.lines() {
         total_fuel += compute_fuel(mass.parse::<i32>().unwrap());
    }

    println!("Result : {}", total_fuel);

    Ok(())

}
