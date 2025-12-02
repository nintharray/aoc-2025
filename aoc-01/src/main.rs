use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut dial = 50;
    let mut zero_count = 0;
    let mut old_zero_count = 0;

    for line in content.lines() {
        let polarity = if line.chars().nth(0).expect("Should have a letter") == 'L' {
            -1
        } else {
            1
        };
        let magnitude: i32 = line[1..].to_string().parse().expect("Should have parsed int from string");

        let transform = polarity * magnitude;
        let result = dial + transform;
        let next_dial_pos = result.rem_euclid(100);

        if transform < 0 {
            zero_count += get_zeros_counterclockwise(dial, transform);
        };

        if transform > 0 {
            zero_count += get_zeros_clockwise(dial, transform);
        };

        dial = next_dial_pos;

        if next_dial_pos == 0 { old_zero_count += 1; };

        println!("Line: {}", line);
        println!("New dial pos: {}", dial);
        println!("Zero count: {}\n", zero_count);

    }

    println!("Final zero count: {}", zero_count);
    println!("Final old zero count: {}", old_zero_count);

}

/// Move dial. Get zeros.
///
/// Simple. Or so I thought.
///
/// * `start_pos` - Initial dial position. Between 0 and 99.
/// * `transform` - How far the dial is moved. Positive (clockwise)
pub fn get_zeros_clockwise(start_pos: i32, transform: i32) -> i32 {
    return (start_pos + transform) / 100;
}

/// Move dial. Get zeros.
///
/// ASSUMES THERE ARE NO "ZERO ROTATIONS"
///
/// * `start_pos` - Initial dial position. Between 0 and 99.
/// * `transform` - How far the dial is moved. Negative (counterclockwise)
pub fn get_zeros_counterclockwise(start_pos: i32, transform: i32) -> i32 {
    if (start_pos + transform) > 0 { return 0; }
    else if (start_pos + transform) == 0 { return 1; }
    else {
        // start_pos + transform < 0
        // we know we at least made it to 0.
        if start_pos == 0 {
            return (transform / 100).abs();
        };

        let normalized_transform = transform + start_pos;
        // if normalized_transform % 100 != 0 {
        return (normalized_transform / 100).abs() + 1;
        // } else {
        //     return (normalized_transform / 100).abs();
        // };
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_zero_c_0_small() {
        let result = get_zeros_clockwise(0, 1);
        assert_eq!(result, 0);
    }

    #[test]
    fn from_zero_c_0_big() {
        let result = get_zeros_clockwise(0, 99);
        assert_eq!(result, 0);
    }

    #[test]
    fn from_zero_c_1() {
        let result = get_zeros_clockwise(0, 100);
        assert_eq!(result, 1);
    }

    #[test]
    fn from_zero_c_1_extra() {
        let result = get_zeros_clockwise(0, 105);
        assert_eq!(result, 1);
    }

    #[test]
    fn from_zero_c_2() {
        let result = get_zeros_clockwise(0, 200);
        assert_eq!(result, 2);
    }

    #[test]
    fn from_zero_c_2_extra() {
        let result = get_zeros_clockwise(0, 205);
        assert_eq!(result, 2);
    }

    #[test]
    fn from_offset_c_0_small() {
        let result = get_zeros_clockwise(1, 1);
        assert_eq!(result, 0);
    }

    #[test]
    fn from_offset_c_0_big() {
        let result = get_zeros_clockwise(1, 99);
        assert_eq!(result, 1);
    }

    #[test]
    fn from_offset_c_1() {
        let result = get_zeros_clockwise(1, 100);
        assert_eq!(result, 1);
    }

    #[test]
    fn from_offset_c_1_extra() {
        let result = get_zeros_clockwise(1, 199);
        assert_eq!(result, 2);
    }

    #[test]
    fn from_offset_c_2() {
        let result = get_zeros_clockwise(1, 200);
        assert_eq!(result, 2);
    }

    #[test]
    fn from_offset_c_2_extra() {
        let result = get_zeros_clockwise(1, 205);
        assert_eq!(result, 2);
    }

    #[test]
    fn from_fifty_c_0() {
        let result = get_zeros_clockwise(50, 49);
        assert_eq!(result, 0);
    }

    #[test]
    fn from_fifty_c_1() {
        let result = get_zeros_clockwise(50, 50);
        assert_eq!(result, 1);
    }

    #[test]
    fn from_fifty_c_1_extra() {
        let result = get_zeros_clockwise(50, 51);
        assert_eq!(result, 1);
    }

    #[test]
    fn from_fifty_c_2() {
        let result = get_zeros_clockwise(50, 150);
        assert_eq!(result, 2);
    }

    #[test]
    fn from_zero_cc_0_small() {
        let result = get_zeros_counterclockwise(0, -1);
        assert_eq!(result, 0);
    }

    #[test]
    fn from_zero_cc_0_big() {
        let result = get_zeros_counterclockwise(0, -99);
        assert_eq!(result, 0);
    }

    #[test]
    fn from_zero_cc_1() {
        let result = get_zeros_counterclockwise(0, -100);
        assert_eq!(result, 1);
    }

    #[test]
    fn from_zero_cc_1_extra() {
        let result = get_zeros_counterclockwise(0, -105);
        assert_eq!(result, 1);
    }

    #[test]
    fn from_zero_cc_2() {
        let result = get_zeros_counterclockwise(0, -200);
        assert_eq!(result, 2);
    }

    #[test]
    fn from_zero_cc_2_extra() {
        let result = get_zeros_counterclockwise(0, -205);
        assert_eq!(result, 2);
    }

    #[test]
    fn from_offset_cc_0_small() {
        let result = get_zeros_counterclockwise(1, -1);
        assert_eq!(result, 1);
    }

    #[test]
    fn from_offset_cc_0_big() {
        let result = get_zeros_counterclockwise(1, -99);
        assert_eq!(result, 1);
    }

    #[test]
    fn from_offset_cc_1() {
        let result = get_zeros_counterclockwise(1, -100);
        assert_eq!(result, 1);
    }

    #[test]
    fn from_offset_cc_1_extra() {
        let result = get_zeros_counterclockwise(1, -199);
        assert_eq!(result, 2);
    }

    #[test]
    fn from_offset_cc_2() {
        let result = get_zeros_counterclockwise(1, -200);
        assert_eq!(result, 2);
    }

    #[test]
    fn from_offset_cc_2_extra() {
        let result = get_zeros_counterclockwise(1, -205);
        assert_eq!(result, 3);
    }

    #[test]
    fn from_fifty_cc_0() {
        let result = get_zeros_counterclockwise(50, -49);
        assert_eq!(result, 0);
    }

    #[test]
    fn from_fifty_cc_1() {
        let result = get_zeros_counterclockwise(50, -50);
        assert_eq!(result, 1);
    }

    #[test]
    fn from_fifty_cc_1_extra() {
        let result = get_zeros_counterclockwise(50, -51);
        assert_eq!(result, 1);
    }

    #[test]
    fn from_fifty_cc_2() {
        let result = get_zeros_counterclockwise(50, -150);
        assert_eq!(result, 2);
    }

}

