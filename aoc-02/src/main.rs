use std::env;
use std::fs;

pub struct Range {
    start: u64,
    end: u64,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let ranges: Vec<Range> = get_ranges(&content);

    let sum: u64 = process_ranges(ranges);

    println!("Sum: {}", sum);
}

pub fn get_ranges(input: &str) -> Vec<Range> {

    let mut output: Vec<Range> = Vec::new();

    for range_str in input.split(",") {
        let parts: Vec<&str> = range_str.split("-").collect();
        let start: u64 = parts[0]
            .parse()
            .expect("Should parse int successfully");

        let end: u64 = parts[1]
            .trim_end()
            .parse()
            .expect("Should parse int successfully");

        output.push(Range{start, end});
    }
    return output;
}

pub fn process_ranges(ranges: Vec<Range>) -> u64 {
    let mut sum: u64 = 0;

    for range in ranges {
        let mut i: u64 = 0;

        while range.start + i <= range.end {
            sum += get_invalid_id_sum(range.start + i);
            i += 1;
        };
    }
    return sum;
}

pub fn get_invalid_id_sum(num: u64) -> u64 {
    if num <= 10 { return 0; }; // length < 2 don't care; and 11 is the earliest match
    let str: String = (num).to_string();
    let factors: Vec<usize> = get_factors(str.len());
    for factor in factors {
        let substrings: Vec<&str> = get_factor_substrings(&str, factor);
        if is_all_same(substrings) {
            return num;
        };

    };
    return 0;
}

pub fn get_factors(num: usize) -> Vec<usize> {
    let mut i: usize = 2;
    let mut output: Vec<usize> = Vec::new();
    output.push(1);

    while i <= num / 2 {
        if num % i == 0 {
            output.push(i);
        };
        i += 1;
    }
    return output;
}

pub fn get_factor_substrings(str: &String, factor: usize) -> Vec<&str> {
    let mut i: usize = 1;
    let mut output: Vec<&str> = Vec::new();

    while i * factor <= str.len() {
        let split_index: usize = str.len() - (i * factor);
        // push factor length chunks of each num
        output.push(&str[split_index..(split_index + factor as usize)]);
        i += 1;
    };
    return output;
}

pub fn is_all_same(vec: Vec<&str>) -> bool {
    vec.windows(2).all(|w| w[0] == w[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_factors_1() {
        let result = get_factors(1);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn get_factors_2() {
        let result = get_factors(2);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn get_factors_3() {
        let result = get_factors(3);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn get_factors_4() {
        let result = get_factors(4);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn get_factors_6() {
        let result = get_factors(6);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn get_factors_7() {
        let result = get_factors(7);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn process_example_range_1() {
        let result = process_ranges(vec![Range{start: 11, end: 22}]);
        assert_eq!(result, 33);
    }

    #[test]
   fn process_example_range_2() {
        let result = process_ranges(vec![Range{start: 95, end: 115}]);
        assert_eq!(result, 210);
    }

    #[test]
   fn process_example_range_3() {
        let result = process_ranges(vec![Range{start: 998, end: 1012}]);
        assert_eq!(result, 2009);
    }

    #[test]
   fn process_example_range_last() {
        let result = process_ranges(vec![Range{start: 2121212118, end: 2121212124}]);
        assert_eq!(result, 2121212121);
    }
}

