#![feature(test)]
extern crate test;
use test::Bencher;
extern crate id_card as id_card_number;


#[bench]
fn bench_validate(b: &mut Bencher) {
    b.iter(|| id_card_number::validate("440524188001010014"));
}