use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_inputs(day: usize) -> BufReader<File> {
    let name = format!("inputs/day{:02}.txt", day);
    let f = File::open(&name).expect(&format!("Could not open file {}", &name));
    BufReader::new(f)
}

// I tried to do this by abusing generics but failed
pub fn inputs_to_int(input: BufReader<File>) -> Vec<usize> {
    input
        .lines()
        .map(|x| x.unwrap().parse::<usize>().unwrap())
        .collect()
}
