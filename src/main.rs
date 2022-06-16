mod stat_func;

use std::io;
//use std::io::{BufRead};

fn main() {
    println!("Enter a filename: ");

    let mut filename = String::new();
    io::stdin()
        .read_line(&mut filename)
        .expect("Error with input.");


    println!("Enter a column name: ");
    let mut column_name = String::new();
    io::stdin()
        .read_line(&mut column_name)
        .expect("Error with input.");
    let vector = vec![4,51,0,-3,124];
    stat_func::summary(&vector);
/*playground
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