use std::collections::HashMap;

pub(crate) fn summary(data: &Vec<i32>){
    let mut frequencies: HashMap<i32, i32> = HashMap::new();
    for value in data{
        *frequencies.entry(*value).or_default() += 1;
    }
}