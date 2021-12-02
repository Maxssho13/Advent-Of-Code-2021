fn main() {
    let input = String::from_utf8(std::fs::read("inputs/02.txt").unwrap()).unwrap();
    let mut submarine = Submarine::new();

    let instructions = Instruction::from_string(&input);

    instructions
        .iter()
        .for_each(|instruction| submarine.execute_instruction(instruction));

    println!("output: {:?}", submarine.multiplied_coordinates());
}

#[derive(Debug)]
enum Instruction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl Instruction {
    fn from_string(input: &str) -> Vec<Instruction> {
        let mut output = Vec::with_capacity(1000);
        for instruction in input.lines() {
            let mut args = instruction.split_ascii_whitespace();
            let instruction = args.next().unwrap();
            let amount = args.next().unwrap().parse::<i32>().unwrap();

            output.push(match instruction {
                "forward" => Instruction::Forward(amount),
                "up" => Instruction::Up(amount),
                "down" => Instruction::Down(amount),
                _ => panic!("Invalid input"),
            });
        }

        output
    }
}

#[derive(Debug)]
struct Submarine {
    horizontal_position: i32,
    depth: i32,
}

impl Submarine {
    fn new() -> Self {
        Self {
            horizontal_position: 0,
            depth: 0,
        }
    }

    fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Forward(amount) => self.horizontal_position += amount,
            Instruction::Up(amount) => self.depth -= amount,
            Instruction::Down(amount) => self.depth += amount,
        }
    }

    fn multiplied_coordinates(&self) -> i32 {
        self.depth * self.horizontal_position
    }
}
