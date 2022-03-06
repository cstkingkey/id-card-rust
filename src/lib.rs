// #![deny(missing_docs)]
extern crate regex;
#[macro_use]
extern crate lazy_static;

mod region;

use chrono::NaiveDate;
use regex::Regex;

pub fn validate(id_number: &str, validate_region: bool) -> bool {
    let id_pattern = Regex::new(r"[1-9][0-9]{14}[0-9]{2}[0-9Xx]").unwrap();
    if id_pattern.is_match(id_number) == false {
        return false;
    }
    //check region code
    // let region_code: String = id_number.chars().take(6).collect();
    if validate_region && !region::validate_code(&id_number[0..6]) {
        return false;
    }
    //check date
    let birth_date = NaiveDate::parse_from_str(&id_number[6..14], "%Y%m%d");
    if !birth_date.is_ok() {
        return false;
    }

    let mut id_number_string = id_number.to_string();
    let check_code = id_number_string.pop().unwrap();
    //transform the chars except the last to u32
    let items: Vec<u32> = id_number_string
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let factors: [u32; 17] = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
    let mut sum = 0u32;
    for (&x, &y) in items.iter().zip(&factors) {
        sum += x * y;
    }
    let verify_code_expected = ['1', '0', 'X', '9', '8', '7', '6', '5', '4', '3', '2'];
    let modulo = sum % 11;
    return verify_code_expected[modulo as usize] == check_code.to_ascii_uppercase();
}
