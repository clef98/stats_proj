mod stat_func;
mod tests;


use std::io;
use std::io::Read;

fn main() {
    println!("Enter a filename: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error with input.");
    let filename = input.trim();
    println!("Enter a column name: ");
    let mut column_name = String::new();
    io::stdin()
        .read_line(&mut column_name)
        .expect("Error with input.");

    println!("Reading column {} from {}", column_name.trim(), filename);
    let mut vector:Vec<i32> = Vec::new();
    let name: &str = &filename;
    let mut file = std::fs::File::open(name).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    for s in contents.lines() {
        if !s.is_empty() {
            vector.push(s.parse::<i32>().unwrap());
        }
    }
        stat_func::summary(&mut vector);
        println!("sum = {}", stat_func::sum(&vector));
        println!("mean = {}", stat_func::mean(&vector));
        println!("stdev = {}", stat_func::stdev(&vector));
        println!("median = {}", stat_func::median(&mut vector));
        println!("mode = {}", stat_func::mode(&vector));
        println!("min = {}", stat_func::min(&vector));
        println!("max = {}", stat_func::max(&vector));
        stat_func::percentile(&mut vector);
    }
