use std::fs::File;
use std::io::Read;

pub fn run() {
    let mut file = File::open("inputs/1.txt").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input);

    let numbers: Vec<u32> = input.split('\n').map(|s| s.parse().unwrap()).collect();

    let mut prev = 0;
    let mut count = 0;
    for i in 0..numbers.len() {
        if numbers[i] > prev && prev != 0 {
            count = count + 1;
        }
        prev = numbers[i];
    }

    println!("Day 1 part 1: {}", count);

    let mut prev = 0;
    let mut count = 0;
    for window in numbers.windows(3) {
        let sum = window.iter().sum();

        if sum > prev && prev != 0 {
            count = count + 1;
        }
        prev = sum;
    }

    println!("Day 1 part 2: {}", count);
}
