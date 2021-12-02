fn main() {
    let input = String::from_utf8(std::fs::read("inputs/02_part_2.txt").unwrap()).unwrap();

    let mut depth = 0;
    let mut horizontal_location = 0;
    let mut aim = 0;

    for instruction in input.lines() {
        let mut args = instruction.split_ascii_whitespace();
        let instruction = args.next().unwrap();
        let amount = args.next().unwrap().parse::<i32>().unwrap();

        match instruction {
            "forward" => {
                horizontal_location += amount;
                depth += aim * amount;
            }
            "up" => aim -= amount,
            "down" => aim += amount,
            _ => panic!("parsing error"),
        }
    }

    println!("output: {}", depth * horizontal_location);
}
