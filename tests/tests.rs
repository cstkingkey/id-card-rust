extern crate id_card as  id_card_number;

#[test]
fn correct_id_card_number() {
    assert_eq!(id_card_number::validate("440524188001010014"), true);
    assert_eq!(id_card_number::validate("11010519491231002x"), true);
    assert_eq!(id_card_number::validate("11010519491231002X"), true);
}

#[test]
fn wrong_verify_code() {
    assert_eq!(id_card_number::validate("440524188001010018"), false);
    assert_eq!(id_card_number::validate("110105194912310020"), false);
}

#[test]
fn wrong_special_chars() {
    assert_eq!(id_card_number::validate("a40524188001010y14"), false);
    assert_eq!(id_card_number::validate("a40524188001010游42"), false);
    assert_eq!(id_card_number::validate("游40524188001010014'"), false);
}

#[test]
fn wrong_region_code() {
    assert_eq!(id_card_number::validate("449994188001010014"), false);
}

#[test]
fn wrong_birth_date() {
    assert_eq!(id_card_number::validate("449994188002290014"), false);
}
