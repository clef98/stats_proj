//use std::io;
//use std::env;
//use std::fs;

/*
fn load_data(file_path: &str){
    let filename = String::new();
    println!("Enter a file name: {}", filename);
    let data_name = std::fs::File::open(filename);
}*/

use std::io::BufReader;

fn read_a_file(file_path: &str){
    let mut file = std::fs::File::open(file_path).expect("error in opening file");
    let reader = std::io::BufReader::new(file);
    let numbers: Vec<u8> = reader.lines()

}



fn main() {
    let mut line = String::new();
    println!("Enter a file name: ");
    let file_name = std::io::stdin().read_line(&mut line).unwrap();
    let mut data_name = std::fs::File::open(file_name).unwrap();
    //println!(data_name);
    println!("{}", filename);
    //println!("{}", data_name);
    //println!("Enter a column name: ");

}


