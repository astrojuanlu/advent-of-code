use std::fs;

use aoc24::total_distance;

pub fn parse_input(contents: String) -> (Vec<usize>, Vec<usize>) {
    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();

    let lines = contents.lines();
    for line in lines {
        let mut numbers = line.split_whitespace();
        let left_number = numbers
            .next()
            .expect("Left value should not be empty")
            .parse::<usize>()
            .unwrap();
        let right_number = numbers
            .next()
            .expect("Right value should not be empty")
            .parse::<usize>()
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

    let result = total_distance(&left, &right);

    println!("Total distance: {result}");
}
