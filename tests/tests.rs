extern crate cn_id_card as id_card_number;

#[test]
fn correct_id_card_number() {
    assert!(id_card_number::validate("440524188001010014", true));
    assert!(id_card_number::validate("11010519491231002x", true));
    assert!(id_card_number::validate("11010519491231002X", true));
}

#[test]
fn wrong_verify_code() {
    assert!(!id_card_number::validate("440524188001010018", true));
    assert!(!id_card_number::validate("110105194912310020", true));
}

#[test]
fn wrong_special_chars() {
    assert!(!id_card_number::validate("a40524188001010y14", true));
    assert!(!id_card_number::validate("a40524188001010游42", true));
    assert!(!id_card_number::validate("游40524188001010014'", true));
}

#[test]
fn wrong_region_code() {
    assert!(!id_card_number::validate("449994188001010014", true));
}

#[test]
fn wrong_birth_date() {
    assert!(!id_card_number::validate("449994188002290014", true));
}
