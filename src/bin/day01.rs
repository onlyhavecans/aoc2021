fn increases_by_window(depths: &[usize], window: usize) -> usize {
    let fence = window + 1;
    depths.windows(fence).filter(|x| x[window] > x[0]).count()
}

fn main() {
    let input = aoc2021::get_inputs(1);
    let depths = aoc2021::inputs_to_int(input);

    let increase_count = increases_by_window(&depths, 1);
    dbg!(increase_count);

    let three_increase_count = increases_by_window(&depths, 3);
    dbg!(three_increase_count);
}
