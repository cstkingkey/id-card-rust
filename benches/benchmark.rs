#![feature(test)]
extern crate test;
use test::Bencher;
extern crate cn_id_card as id_card_number;


#[bench]
fn bench_validate(b: &mut Bencher) {
    b.iter(|| id_card_number::validate("440524188001010014", true));
}

#[bench]
fn bench_validate_no_region(b: &mut Bencher) {
    b.iter(|| id_card_number::validate("440524188001010014", false));
}
