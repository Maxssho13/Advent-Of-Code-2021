fn main() {
    let input = String::from_utf8(std::fs::read("inputs/02.txt").unwrap()).unwrap();

    let mut depth = 0;
    let mut horizontal_location = 0;

    for instruction in input.lines() {
        let mut args = instruction.split_ascii_whitespace();
        let instruction = args.next().unwrap();
        let amount = args.next().unwrap().parse::<i32>().unwrap();

        match instruction {
            "forward" => {
                horizontal_location += amount;
            }
            "up" => depth -= amount,
            "down" => depth += amount,
            _ => panic!("parsing error"),
        }
    }

    println!("output: {}", depth * horizontal_location);
}
