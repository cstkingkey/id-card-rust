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

    let mut chars = id_number.chars().rev();
    let check_code = chars.next().unwrap();
    //transform the chars except the last to u32
    let items = chars.map(|c| c.to_digit(10).unwrap()).rev();
    let factors: [u32; 17] = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
    let s: u32 = items.zip(&factors).map(|(x, y)| x * y).sum();
    let verify_code_expected = ['1', '0', 'X', '9', '8', '7', '6', '5', '4', '3', '2'];
    let modulo = s % 11;
    verify_code_expected[modulo as usize] == check_code.to_ascii_uppercase()
}


pub fn validate_code(credit_code_number: &str, _validate_region: bool) -> bool {
    static ID_PATTERN: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"[0-9A-HJ-NPQRTUWXY]{2}\d{6}[0-9A-HJ-NPQRTUWXY]{10}").unwrap());
    if !ID_PATTERN.is_match(credit_code_number) {
        return false;
    }

    let mut chars = credit_code_number.chars().rev();
    let check_code = chars.next().unwrap();
    //transform the chars except the last to u32
    let items = chars.map(|c| char_to_number(c)).rev();
    let factors: [u32; 17] =  [1, 3, 9, 27, 19, 26, 16, 17, 20, 29, 25, 13, 8, 24, 10, 30, 28];
    let s: u32 = items.zip(&factors).map(|(x, y)| x * y).sum();
    let modulo = 31 -s % 31;
    modulo == char_to_number(check_code)
}

fn char_to_number(c: char) -> u32 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'A' => 10,
        'B' => 11,
        'C' => 12,
        'D' => 13,
        'E' => 14,
        'F' => 15,
        'G' => 16,
        'H' => 17,
        'J' => 18,
        'K' => 19,
        'L' => 20,
        'M' => 21,
        'N' => 22,
        'P' => 23,
        'Q' => 24,
        'R' => 25,
        'T' => 26,
        'U' => 27,
        'W' => 28,
        'X' => 29,
        'Y' => 30,
        _ => 0,
    }
}
