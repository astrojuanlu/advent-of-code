use std::fs;

use aoc24::compute_total_distance;

pub fn parse_input(contents: String) -> (Vec<u32>, Vec<u32>) {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    let lines = contents.lines();
    for line in lines {
        let mut numbers = line.split_whitespace();
        let left_number = numbers
            .next()
            .expect("Left value should not be empty")
            .parse::<u32>()
            .unwrap();
        let right_number = numbers
            .next()
            .expect("Right value should not be empty")
            .parse::<u32>()
            .unwrap();

        left.push(left_number);
        right.push(right_number);
    }

    return (left, right);
}

fn main() {
    let contents = fs::read_to_string("input01.txt").expect("File 'input01.txt' should be present");
    println!("{contents}");

    let (left, right) = parse_input(contents);

    let result = compute_total_distance(left, right);

    println!("Total distance: {result}");
}
