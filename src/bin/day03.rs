use std::io::BufRead;

fn bin_string_to_int(bin: String) -> isize {
    isize::from_str_radix(bin.as_str(), 2).unwrap()
}

fn main() {
    let inputs = aoc2021::get_inputs(3);
    let mut bin_counter: Vec<i32> = vec![0; 12];
    let mut bin_len = 0;
    let mut line_counter = 0;

    for input in inputs.lines() {
        let input = input.unwrap();
        let split: Vec<i32> = input
            .chars()
            .map(|x| x.to_string().parse::<i32>().unwrap())
            .collect();

        line_counter += 1;
        if bin_len == 0 {
            bin_len = split.len();
        }

        for i in 0..bin_len {
            bin_counter[i] += split[i];
        }
    }

    let mut gamma_vec = vec![""; bin_len];
    let mut epsilon_vec = vec![""; bin_len];

    for i in 0..bin_len {
        let half = line_counter / 2;
        let one_bit_common = if bin_counter[i] > half { true } else { false };

        match one_bit_common {
            true => {
                gamma_vec[i] = "1";
                epsilon_vec[i] = "0";
            }
            false => {
                epsilon_vec[i] = "1";
                gamma_vec[i] = "0";
            }
        }
    }

    let epsilon = bin_string_to_int(epsilon_vec.concat());
    let gamma = bin_string_to_int(gamma_vec.concat());
    let power_consumption = gamma * epsilon;
    dbg!(power_consumption);
}
