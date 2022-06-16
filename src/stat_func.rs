
pub(crate) fn summary(data: &Vec<i32>) {
    let mut count = 1;
    let mut i = 1;
    //Cannot retrieve length of frequencies
    let mut unique_count = 0;
    let mut frequencies: Vec<(i32, i32)> = Vec::new();
    while i < data.len() {
        if data[i] == data[i - 1] {
            count = count + 1;
        } else {
            frequencies.push((data[i - 1], count));
            count = 1;
            unique_count = unique_count + 1;
        }
        if i == data.len() - 1 {
            frequencies.push((data[i], count));
            unique_count = unique_count + 1;
        }
        i = i + 1;
    }
    println!("Summary (value: frequency)");
    for j in frequencies {
        println!("{}: {}", j.0, j.1);
    }
    println!();
    println!("count = {}", unique_count);
}

pub fn sum(data: &Vec<i32>){
    let mut summation = 0;
    for i in data{
        summation = summation + *i;
    }
    println!("sum = {}", summation);
}