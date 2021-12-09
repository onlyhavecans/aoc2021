use std::io::BufRead;

fn main() {
    let inputs = aoc2021::get_inputs(2);

    let mut aim: isize = 0;
    let mut depth: isize = 0;
    let mut horizontal: isize = 0;

    for line in inputs.lines() {
        let line = line.unwrap();
        let (dir, distance) = line.split_once(" ").unwrap();
        let distance = distance.parse::<isize>().unwrap();
        match dir {
            "forward" => {
                horizontal += distance;
                depth += distance * aim;
            }
            "down" => aim += distance,
            "up" => aim -= distance,
            _ => panic!(),
        }
    }

    let multiply = horizontal * depth;
    dbg!(multiply);
}
