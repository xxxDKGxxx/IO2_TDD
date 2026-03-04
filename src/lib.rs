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
        if num == value {
            return String::from(symbol);
        }
    }
    return String::new();
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
}
