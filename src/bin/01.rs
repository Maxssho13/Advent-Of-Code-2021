fn main() {
    let mut increases = 0;
    let inputs = String::from_utf8(std::fs::read("inputs/01.txt").unwrap()).unwrap();

    let mut lines = inputs.lines();
    let mut previous_num = lines.next().unwrap().parse::<i32>().unwrap();

    for line in lines {
        let num = line.parse::<i32>().unwrap();

        if num > previous_num {
            increases += 1;
        }
        previous_num = num;
    }

    println!("output: {}", increases);
}
