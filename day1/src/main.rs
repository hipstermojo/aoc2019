use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("Unable to open input file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Unable to read the file content");
    println!("Part 1: The total fuel needed is {}", part_one(&content));
    println!("Part 2: The total fuel of all requirements is {}", part_two(&content));
}

fn part_one(content: &str) -> i32 {
    content
        .split("\n")
        .filter_map(|mass_str| mass_str.parse::<i32>().ok())
        .map(get_fuel)
        .sum()
}

fn part_two(content: &str) -> i32 {
    content
        .split("\n")
        .filter_map(|mass_str| mass_str.parse::<i32>().ok())
        .map(get_total_fuel)
        .sum()
}

fn get_fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn get_total_fuel(mass: i32) -> i32 {
    let fuel = get_fuel(mass);
    if fuel > 0 {
        fuel + get_total_fuel(fuel)
    } else {
        0
    }
}
