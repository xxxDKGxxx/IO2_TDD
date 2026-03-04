const ROMAN_NUMERALS_CONVERSION_TABLE: [(i32, &str); 7] = [
    (1000, "M"),
    (500, "D"),
    (100, "C"),
    (50, "L"),
    (10, "X"),
    (5, "V"),
    (1, "I"),
];
pub fn convert(mut num: i32) -> String {
    let mut result = String::new();

    for &(value, symbol) in ROMAN_NUMERALS_CONVERSION_TABLE.iter() {
        while num >= value {
            result.push_str(symbol);
            num -= value
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_symbols_test() {
        let test_data = vec![1, 5, 10, 50, 100, 500, 1000];
        let expected_results: Vec<String> = vec![
            "I".into(),
            "V".into(),
            "X".into(),
            "L".into(),
            "C".into(),
            "D".into(),
            "M".into(),
        ];

        for (test_input, expected_result) in test_data.iter().zip(expected_results) {
            let result = convert(*test_input);
            assert!(
                result == expected_result,
                "Expected {:?} for arab number {:?}, got {:?}",
                expected_result,
                test_input,
                result
            );
        }
    }

    #[test]
    fn basic_addition_symbols_tests() {
        let test_data = vec![2, 6, 20, 60, 110, 150, 600, 1500, 2000];
        let expected_results: Vec<String> = vec![
            "II".into(),
            "VI".into(),
            "XX".into(),
            "LX".into(),
            "CX".into(),
            "CL".into(),
            "DC".into(),
            "MD".into(),
            "MM".into(),
        ];

        for (test_input, expected_result) in test_data.iter().zip(expected_results) {
            let result = convert(*test_input);
            assert!(
                result == expected_result,
                "Expected {:?} for arab number {:?}, got {:?}",
                expected_result,
                test_input,
                result
            );
        }
    }

    #[test]
    fn advanced_addition_symbols_tests() {
        let test_data = vec![3, 8, 27, 68, 126, 273, 666, 1723, 2533];
        let expected_results: Vec<String> = vec![
            "III".into(),
            "VIII".into(),
            "XXVII".into(),
            "LXVIII".into(),
            "CXXVI".into(),
            "CCLXXIII".into(),
            "DCLXVI".into(),
            "MDCCXXIII".into(),
            "MMDXXXIII".into(),
        ];

        for (test_input, expected_result) in test_data.iter().zip(expected_results) {
            let result = convert(*test_input);
            assert!(
                result == expected_result,
                "Expected {:?} for arab number {:?}, got {:?}",
                expected_result,
                test_input,
                result
            );
        }
    }
}
