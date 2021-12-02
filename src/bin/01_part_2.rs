fn main() {
    let mut increases = 0;
    let input = String::from_utf8(std::fs::read("inputs/01_part_2.txt").unwrap()).unwrap();

    let input: Vec<i32> = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    input.iter().zip(input.iter().skip(3)).for_each(|(a, b)| {
        if b > a {
            increases += 1
        }
    });

    println!("output: {}", increases);
}
