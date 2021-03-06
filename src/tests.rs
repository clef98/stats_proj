#[cfg(test)]
mod tests {
    use crate::stat_func;
    # [test]
    fn large_set_test(){
        let mut vector = vec![81, 96, 74, 20, 65, 70, 43, 43, 69, 100, 25, 64, 75, 11, 11, 77, 4, 82, 96, 96, 71, 9];
        //Tests multiple modes, split median.
        assert_eq!(stat_func::sum(&vector), 1282);
        assert_eq!(stat_func::mean(&vector), 58.272727);
        assert_eq!(stat_func::stdev(&vector), 31.87951);
        assert_eq!(stat_func::median(&mut vector), 69.5);
        assert_eq!(stat_func::mode(&vector), 96);
        assert_eq!(stat_func::min(&vector), 4);
        assert_eq!(stat_func::max(&vector), 100);
        stat_func::summary(&mut vector);
        stat_func::percentile(&mut vector);
    }
}