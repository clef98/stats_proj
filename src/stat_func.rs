use std::collections::HashMap;

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

pub fn sum(data: &Vec<i32>) -> i32{
    let mut summation = 0;
    for i in data{
        summation = summation + *i;
    }
    summation
}

pub fn mean(data: &Vec<i32>) -> f32{
    let summation = sum(&data);
    let size: f32 = data.len() as f32;
    let mean:f32 = summation as f32 / size;
    mean
}

pub fn stdev(data: &Vec<i32>) -> f32{
    let mut ssd: f32 = 0.0;
    let mean = mean(&data);
    for i in data{
        ssd += (*i as f32 - mean).powf(2.0);
    }
    let mut ssd_root: f32 = ssd / data.len() as f32;
    ssd_root = ssd_root.sqrt();
    ssd_root
}

pub fn median(data: &Vec<i32>) -> f32{
    let median: f32;
    if data.len() % 2 == 1{
        median = data[(data.len()-1)/2] as f32;
    } else{
        median = ((data[(data.len()-1)/2] + data[(data.len()-1)/2-1] )/2) as f32;
    }
    median
}

pub fn mode(data: &Vec<i32>) -> i32{
    let mut frequencies = HashMap::new();
    for i in data{
        *frequencies.entry(i).or_insert(0)+=1;
    }
    *frequencies.into_iter().max_by_key(|&(_, count) | count).map(|(val, _) | val).unwrap()
}
