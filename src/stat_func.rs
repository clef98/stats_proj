use std::collections::HashMap;

//Reasoning for summary and percentile functions to print directly: abstraction from main,
// otherwise would require loops and multiple calls to print the "equivalent" statistic.
pub(crate) fn summary(data: &mut [i32]) {
    data.sort();
    let mut count = 1;
    let mut i = 1;
    let mut frequencies: Vec<(i32, u32)> = Vec::new();
    while i < data.len() {
        if data[i] == data[i - 1] {
            count = count + 1;
        } else {
            frequencies.push((data[i - 1], count));
            count = 1;
        }
        if i == data.len() - 1 {
            frequencies.push((data[i], count));
        }
        i = i + 1;
    }
    let unique_count = frequencies.len() - 1;
    println!("Summary (value: frequency)");
    for j in frequencies {
        println!("{}: {}", j.0, j.1);
    }
    println!();
    println!("count = {}", unique_count);
}

pub fn sum(data: &[i32]) -> i32{
    let mut summation = 0;
    for i in data{
        summation = summation + *i;
    }
    summation
}

pub fn mean(data: &[i32]) -> f32{
    let summation = sum(&data);
    let size: f32 = data.len() as f32;
    let mean:f32 = summation as f32 / size;
    mean
}

pub fn stdev(data: &[i32]) -> f32{
    let mut ssd: f32 = 0.0;
    let mean = mean(&data);
    for i in data{
        ssd += (*i as f32 - mean).powf(2.0);
    }
    //N-1 used, N is also a common formula for standard deviation.
    let mut ssd_root: f32 = ssd / (data.len() - 1) as f32;
    ssd_root = ssd_root.sqrt();
    ssd_root
}

pub fn median(data: &mut [i32]) -> f32{
    data.sort();
    let median: f32;
    if data.len() % 2 == 1{
        median = data[(data.len()-1)/2] as f32;
    } else{
        median = (data[(data.len())/2] + data[(data.len())/2 - 1] ) as f32 /2.0;
    }
    median
}

//In the event of a tie, the first mode that appears in the vector is select.
pub fn mode(data: &[i32]) -> i32{
    let mut frequencies = HashMap::new();
    for i in data{
        *frequencies.entry(i).or_insert(0)+=1;
    }
    *frequencies.into_iter().max_by_key(|&(_, count) | count).map(|(val, _) | val).unwrap()
}

pub fn min(data: &[i32]) -> i32{
    let mut minimum: i32 = data[0];
    for i in data{
        if *i < minimum{
            minimum = *i;
        }
    }
    minimum
}

pub fn max(data: &[i32]) -> i32{
    let mut maximum: i32 = data[0];
    for i in data{
        if *i > maximum{
            maximum = *i;
        }
    }
    maximum
}

pub fn percentile(data: &mut [i32]){
    data.sort();
    let perc_0 = data[0];
    let mut perc = (0.25* (data.len() as f32)).floor();
    let perc_25 = data[perc as usize];
    perc = (0.5* (data.len() as f32)).floor();
    let perc_50 = data[perc as usize];
    perc = (0.75* (data.len() as f32)).floor();
    let perc_75 = data[perc as usize];
    let perc_100 = data[data.len()-1];
    println!("  0th percentile = {}", perc_0);
    println!(" 25th percentile = {}", perc_25);
    println!(" 50th percentile = {}", perc_50);
    println!(" 75th percentile = {}", perc_75);
    println!("100th percentile = {}", perc_100);
}

/* IF PERCENTILES ARE USER INPUTTED (Not recommended)
pub fn percentile_p(data: &[i32], p: f32){
    let mut perc = (p* (data.len() as f32)).floor();
    perc = data[perc as usize] as f32;
    println!("{} percentile = {}", p, perc);
}*/