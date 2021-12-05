#![feature(drain_filter)]

fn main() {
    let input = String::from_utf8(std::fs::read("inputs/03_part_2.txt").unwrap()).unwrap();
    let input_lines: Vec<&str> = input.lines().collect();

    let mut oxygen_numbers = input_lines.clone();
    let mut c02_numbers = input_lines;

    let mut current_bit = 0;
    while oxygen_numbers.len() > 1 {
        let (common_bit, _) = bit_frequency(current_bit, &oxygen_numbers);

        oxygen_numbers
            .drain_filter(|number| number.chars().nth(current_bit).unwrap() != common_bit);

        current_bit += 1;
    }

    let mut current_bit = 0;
    while c02_numbers.len() > 1 {
        let (_, least_common_bit) = bit_frequency(current_bit, &c02_numbers);

        c02_numbers
            .drain_filter(|number| number.chars().nth(current_bit).unwrap() != least_common_bit);

        current_bit += 1;
    }

    let oxygen_rating = usize::from_str_radix(oxygen_numbers[0], 2).unwrap();
    let c02_rating = usize::from_str_radix(c02_numbers[0], 2).unwrap();

    println!("output: {}", oxygen_rating * c02_rating);
}

/// (most_common, least_common)
fn bit_frequency(idx: usize, input_lines: &[&str]) -> (char, char) {
    let mut one_count = 0;
    let mut zero_count = 0;
    for number in input_lines {
        if number.chars().nth(idx).unwrap() == '1' {
            one_count += 1;
        } else {
            zero_count += 1;
        }
    }

    if one_count >= zero_count {
        ('1', '0')
    } else {
        ('0', '1')
    }
}
