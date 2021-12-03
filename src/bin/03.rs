fn main() {
    let input = String::from_utf8(std::fs::read("inputs/03.txt").unwrap()).unwrap();
    let num_bits = input.split_once("\n").unwrap().0.len();

    let mut bit_counts: Vec<usize> = vec![0; num_bits];

    let mut total_numbers = 0usize;

    for line in input.lines() {
        total_numbers += 1;
        for (idx, char) in line.chars().enumerate() {
            if char == '1' {
                bit_counts[idx] += 1;
            }
        }
    }

    let mut out_string = String::with_capacity(num_bits);

    bit_counts.iter().for_each(|bit_count| {
        if *bit_count > total_numbers / 2 {
            out_string.push('1');
        } else {
            out_string.push('0');
        }
    });

    let gamma_rate = u64::from_str_radix(&out_string, 2).unwrap();

    // since we invert the whole u64, we create a lot of high, high bits,
    // shifting left and shifting right clears those
    let epsilon_rate = (!gamma_rate << (64 - num_bits)) >> (64 - num_bits);

    println!("output: {}", gamma_rate * epsilon_rate);
}
