mod stat_func;
mod tests;


use std::io;
use std::io::Read;
//use std::io::{BufReader};

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

    print!("Reading column {} from {}", column_name, filename);
    let mut vector:Vec<i32> = Vec::new();
    print!("Name: {}", filename);
    //let name: &str = &*std::fs::read_to_string(&mut filename).expect("unable to read");
    //assert_eq!(filename, "data.txt");
    let name: &str = &filename;
    let mut file = std::fs::File::open(name).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    for s in contents.lines() {
        if !s.is_empty() {
            vector.push(s.parse::<i32>().unwrap());
        }
    }
    //_vector = std::fs::read("data.txt").expect("Error with reading input file.");
    println!("{:?}", vector);
    //let mut vector: Vec<i32> = _vec.iter().map(|&i| i as i32).collect();
    //let mut vector = vec![4, 51, 0, -3, 0, 0, 124, -3, -3, -3];

        vector.sort();
        stat_func::summary(&vector);
        println!("sum = {}", stat_func::sum(&vector));
        println!("mean = {}", stat_func::mean(&vector));
        println!("stdev = {}", stat_func::stdev(&vector));
        println!("median = {}", stat_func::median(&vector));
        println!("mode = {}", stat_func::mode(&vector));
        println!("min = {}", stat_func::min(&vector));
        println!("max = {}", stat_func::max(&vector));
        stat_func::percentile(&vector);

        /*playground, commented code is a necessary evil.
        let file = std::fs::File::open(filename);
        let read = BufReader::new(file);
        let mut vector: Vec<i32> = Vec::new();
        for value in read.lines() {
            for s in value.split_whitespace() {
                let s = s.parse();
                vector.push(s);
            }
        }*/
    }
