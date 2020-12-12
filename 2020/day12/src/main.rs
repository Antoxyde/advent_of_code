use std::fs::File;
use std::io::prelude::*;
use std::mem;

type Inst = (char, i64);
type State = (char, i64, i64); // Dir, x, y

fn turn_left(state: &mut State, deg: i64) {
    let mut  n = deg;
    while n > 0 {
        n -= 90;
        state.0 = match state.0 {
            'N' => 'W',
            'S' => 'E',
            'E' => 'N',
            'W' => 'S',
            _ => unreachable!(),
        };

    }
}

fn forward(dir: char, n: i64, state: &mut State) {
    match dir {
        'N' => state.2 += n,
        'S' => state.2 -= n,
        'E' => state.1 += n,
        'W' => state.1 -= n,
        _ => unreachable!(),
    }
}

// x ->
// y ^
fn part1(insts: &[Inst]) -> i64 {
    let mut state: State =  ('E', 0, 0);

    for inst in insts {
        match inst {
            ('N', _) | ('S', _) | ('E', _) | ('W', _) => forward(inst.0, inst.1, &mut state),
            ('L', n) => turn_left(&mut state, *n),
            ('R', n) => turn_left(&mut state, -(*n) + 360),
            ('F', n) => forward(state.0, *n, &mut state),
            _ => unreachable!(),
        }
    }

   state.1.abs() + state.2.abs()
}

fn turn_waypoint(wp: &mut State, deg: i64) {

    let mut n = deg;
    while n > 0 {
        mem::swap(&mut wp.1, &mut wp.2);
        wp.1 = -wp.1;
        n -= 90;
    }
}

fn part2(insts: &[Inst]) -> i64 {
    let mut waypoint: State =  ('E', 10, 1);
    let mut ship: State =  ('E', 0, 0);

    for inst in insts {
        match inst {
            ('N', _) | ('S', _) | ('E', _) | ('W', _) => forward(inst.0, inst.1, &mut waypoint),
            ('L', n) => turn_waypoint(&mut waypoint, *n),
            ('R', n) => turn_waypoint(&mut waypoint, -(*n) + 360),
            ('F', n) => {
                ship.1 += waypoint.1 * n;
                ship.2 += waypoint.2 * n;
            },
            _ => unreachable!(),
        }
    }

   ship.1.abs() + ship.2.abs()
}

fn parse_line(line: &str) -> Inst {
    (line.chars().nth(0).unwrap(), line[1..].parse().unwrap())
}

fn main() -> std::io::Result<()> {

    let mut file = File::open("day11.txt")?;
    let mut inp = String::new();
    file.read_to_string(&mut inp)?;

    let insts: Vec<Inst> = inp[..inp.len()-1].split("\n").map(parse_line).collect();

    let r1 = part1(&insts);
    println!("PART1 : {}", r1);

    let r2 = part2(&insts);
    println!("PART2 : {}", r2);
    Ok(())

}
