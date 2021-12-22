const BOARD_DIM: usize = 1000;

fn main() {
    let input = String::from_utf8(std::fs::read("inputs/05.txt").unwrap()).unwrap();

    let mut points = vec![vec![0; BOARD_DIM]; BOARD_DIM];

    input.lines().for_each(|line| {
        let (point1, point2) = line.split_once("->").unwrap();
        let point1 = point1.trim();
        let point2 = point2.trim();

        let (x1, y1) = point1.split_once(',').unwrap();
        let (x2, y2) = point2.split_once(',').unwrap();

        let x1: usize = x1.parse().unwrap();
        let y1: usize = y1.parse().unwrap();
        let x2: usize = x2.parse().unwrap();
        let y2: usize = y2.parse().unwrap();

        if x1 == x2 || y1 == y2 {
            #[allow(clippy::needless_range_loop)]
            for i in x1.min(x2)..=x1.max(x2) {
                for j in y1.min(y2)..=y1.max(y2) {
                    points[i][j] += 1;
                }
            }
        }
    });

    let output = points
        .iter()
        .flatten()
        .fold(0, |acc, x| if *x >= 2 { acc + 1 } else { acc });

    println!("points with at least two overlap {}", output);
}
