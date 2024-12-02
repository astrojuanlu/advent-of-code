use std::fs;

// use aoc24::similarity_score;
// use aoc24::total_distance;
use aoc24::count_safe_reports;

pub fn parse_input_01(contents: String) -> (Vec<usize>, Vec<usize>) {
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

pub fn parse_input_02(contents: String) -> Vec<Vec<isize>> {
    let lines = contents.lines();
    let mut reports: Vec<Vec<isize>> = Vec::new();
    for line in lines {
        let mut report: Vec<isize> = Vec::new();
        for num_str in line.split_whitespace() {
            report.push(num_str.parse::<isize>().unwrap());
        }
        reports.push(report);
    }

    return reports;
}

fn main() {
    let contents = fs::read_to_string("input02.txt").expect("File 'input02.txt' should be present");
    println!("{contents}");

    let reports = parse_input_02(contents);

    let result = count_safe_reports(&reports, false);

    println!("Safe reports: {result}");
}
