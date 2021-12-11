use std::io::BufRead;

fn bin_string_to_int(bin: String) -> isize {
    isize::from_str_radix(bin.as_str(), 2).unwrap()
}

fn main() {
    let inputs = aoc2021::get_inputs(3);
    let lines: Vec<String> = inputs.lines().map(|x| x.unwrap()).collect();
    let line_len = lines.first().unwrap().len();
    let line_count = lines.len();
    let mut bin_counter: Vec<isize> = vec![0; line_len];

    for input in lines {
        let split: Vec<isize> = input
            .chars()
            .map(|x| x.to_string().parse::<isize>().unwrap())
            .collect();

        for i in 0..line_len {
            bin_counter[i] += split[i];
        }
    }

    let mut gamma_vec = vec![""; line_len];
    let mut epsilon_vec = vec![""; line_len];

    for i in 0..line_len {
        let half = (line_count / 2) as isize;
        let one_bit_common = bin_counter[i] > half;

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
