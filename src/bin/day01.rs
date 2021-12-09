use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("inputs/day01.txt").expect("Couldn't open file");
    let input = BufReader::new(f);
    let depths: Vec<i32> = input
        .lines()
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .collect();
    let increase_count = depths.windows(2).filter(|x| x[1] > x[0]).count();
    dbg!(increase_count);

    let three_increase_count = depths.windows(4).filter(|x| x[3] > x[0]).count();
    dbg!(three_increase_count);
}
