use std::io::BufRead;
use vectrix::Matrix;

fn main() {
    let inputs = aoc2021::get_inputs(4);
    let lines: Vec<String> = inputs.lines().map(|x| x.unwrap()).collect();
    let numbers: Vec<usize> = lines
        .first()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    dbg!(numbers);

    let mut boards: Vec<Matrix<usize, 5, 5>> = Vec::new();
}
