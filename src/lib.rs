// #![deny(missing_docs)]
extern crate regex;

#[cfg(feature = "region")]
pub mod region;

use chrono::NaiveDate;
use once_cell::sync::Lazy;
use regex::Regex;

pub fn validate(id_number: &str, validate_region: bool) -> bool {
    static ID_PATTERN: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"[1-9][0-9]{14}[0-9]{2}[0-9Xx]").unwrap());
    if !ID_PATTERN.is_match(id_number) {
        return false;
    }
    //check region code
    // let region_code: String = id_number.chars().take(6).collect();
    #[cfg(feature = "region")]
    if validate_region && !region::validate_code(&id_number[0..6]) {
        return false;
    }

    #[cfg(not(feature = "region"))]
    if validate_region {
        panic!("unsupported. To validate region, enable region feature.");
    }

    //check date
    let birth_date = NaiveDate::parse_from_str(&id_number[6..14], "%Y%m%d");
    if birth_date.is_err() {
        return false;
    }

    let mut id_number_string = id_number.to_string();
    let check_code = id_number_string.pop().unwrap();
    //transform the chars except the last to u32
    let items = id_number_string.chars().map(|c| c.to_digit(10).unwrap());
    let factors: [u32; 17] = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
    let mut sum = 0u32;
    for (x, &y) in items.zip(&factors) {
        sum += x * y;
    }
    let verify_code_expected = ['1', '0', 'X', '9', '8', '7', '6', '5', '4', '3', '2'];
    let modulo = sum % 11;
    verify_code_expected[modulo as usize] == check_code.to_ascii_uppercase()
}
