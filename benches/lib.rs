#![feature(test)]

extern crate test;
extern crate skip32;

use test::{Bencher, black_box};

const KEY: &'static [u8; 10] = &[0,0,0,0,0,0,0,0,0,0];

#[bench]
fn bench_encode(b: &mut Bencher) {
    b.iter(|| skip32::encode(KEY, black_box(0)))
}

#[bench]
fn bench_decode(b: &mut Bencher) {
    b.iter(|| skip32::decode(KEY, black_box(0)))
}
