fn main() {
    let mut increases = 0;
    let input = String::from_utf8(std::fs::read("inputs/01_part_2.txt").unwrap()).unwrap();

    let input: Vec<i32> = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    let mut prev_sum = input[0] + input[1] + input[2];

    for i in 1..input.len() - 2 {
        let new_sum = input[i] + input[i + 1] + input[i + 2];
        if new_sum > prev_sum {
            increases += 1;
        }
        prev_sum = new_sum;
    }

    println!("output: {}", increases);
}
