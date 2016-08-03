#![feature(test)]
#![allow(dead_code)]
extern crate test;
extern crate quickcheck;

// Ugly hack to allow benchmarking private modules
#[path = "../src/bit_iterator.rs"] mod bit_iterator;
#[path = "../src/packet.rs"] mod packet;
use test::Bencher;
use packet::{Packet, TryFrom};

#[bench]
fn bench_decode(b: &mut Bencher) {
    let buf = [0x21, 0x00, 0x41, 0xa8, 0x99, 0x2f, 0xd0, 0x2a, 0x9f, 0x4a,
               0x26, 0x21, 0x00, 0x10, 0x00, 0x00, 0x3a, 0xf2, 0x6c, 0x79,
               0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a];
    b.iter(|| { let _ = test::black_box(Packet::try_from(&buf)); });
}

#[bench]
fn bench_encode(b: &mut Bencher) {
    let packet = Packet::with_payload(&[1, 2, 3, 4, 5, 6]);
    b.iter(|| { let _ = test::black_box(packet.as_ref()); });
}

#[bench]
fn bench_extract_payload(b: &mut Bencher) {
    let buf = [0x21, 0x01, 0x41, 0xa7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
               0x00, 0x00, 0x00, 0x00, 0x05, 0xdc, 0xab, 0x53, 0x3a, 0xf5,
               0xff, 0x04, 0x01, 0x02, 0x03, 0x04, // First extension
               0x00, 0x04, 0x05, 0x06, 0x07, 0x08, // Second extension, followed by data
               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let packet = Packet::try_from(&buf).unwrap();
    b.iter(|| { let _ = test::black_box(packet.payload()); });
}

#[bench]
fn bench_extract_extensions(b: &mut Bencher) {
    let buf = [0x21, 0x01, 0x41, 0xa7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
               0x00, 0x00, 0x00, 0x00, 0x05, 0xdc, 0xab, 0x53, 0x3a, 0xf5,
               0xff, 0x04, 0x01, 0x02, 0x03, 0x04, // First extension
               0x00, 0x04, 0x05, 0x06, 0x07, 0x08, // Second extension, followed by data
               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
               0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let packet = Packet::try_from(&buf).unwrap();
    b.iter(|| { let _ = test::black_box(packet.extensions().count()); });
}
