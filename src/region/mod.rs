mod region_map;

//check if the region code is valid
pub fn validate_code(code: &str) -> bool {
    match region_map::REGION_MAP.get(code) {
        Some(_name) => true,
        None => false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_code_test() {
        assert_eq!(validate_code("640000"), true);
        assert_eq!(validate_code("649999"), false);
    }
}
