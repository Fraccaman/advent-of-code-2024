use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve_part_one() {
    let input_filepath = "inputs/day1/data.txt";
    let file = File::open(input_filepath).unwrap();
    let reader = BufReader::new(file);

    let mut first_column = Vec::new();
    let mut second_column = Vec::new();

    for line in reader.lines() {
        match line.unwrap().split_once(' ') {
            Some((first, second)) => {
                let first = first.trim().parse::<i64>().unwrap();
                let second = second.trim().parse::<i64>().unwrap();
                first_column.push(first);
                second_column.push(second);
            }
            None => std::process::exit(1),
        }
    }

    first_column.sort_unstable();
    second_column.sort_unstable();

    let mut sum_diffs = 0;
    for element in 0..first_column.len() {
        sum_diffs += (first_column[element] - second_column[element]).abs()
    }

    println!("Result day 1/1: {}", sum_diffs)
}

pub fn solve_part_two() {
    let input_filepath = "inputs/day1/data.txt";
    let file = File::open(input_filepath).unwrap();
    let reader = BufReader::new(file);

    let mut first_column = Vec::new();
    let mut second_column = HashMap::new();

    for line in reader.lines() {
        match line.unwrap().split_once(' ') {
            Some((first, second)) => {
                let first = first.trim().parse::<i64>().unwrap();
                let second = second.trim().parse::<i64>().unwrap();
                first_column.push(first);
                second_column
                    .entry(second)
                    .and_modify(|value| *value += 1)
                    .or_insert(1);
            }
            None => std::process::exit(1),
        }
    }

    let mut score = 0;
    for key in first_column.iter() {
        let occurence = second_column.get(key).unwrap_or(&0);
        score += key * occurence;
    }

    println!("Result day 1/2: {}", score);
}
