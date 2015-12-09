extern crate skip32;
extern crate quickcheck;

static KEYS: &'static [&'static [u8; 10]] = &[
    b"\x42\x08\x81\xdf\xbe\xcf\x2f\x33\xb3\xdd",
    b"\x28\xf2\x4e\x89\x63\x92\x29\xdd\xfe\x74",
    b"\x0a\x12\x1d\xd0\xe8\xab\x95\x7d\xef\x30",
    b"\x25\xe7\xb3\xa9\xc2\x6a\xac\x7e\x53\x61",
    b"\x1c\xc3\xaa\x06\x96\x12\xea\x38\xb5\x63",
];

static VALUES: &'static [&'static [u32]] = &[
    &[0xba355ec9, 0xe0aab6ae, 0x850e1981, 0x3ef83325, 0x0a108f99, 0x97dfefb7, 0xb81882ec, 0x5a53eaad, 0x61697bc0, 0xcf66b16e],
    &[0x6b80e903, 0x1bc0e577, 0xb69e01cc, 0x28b0b730, 0x47fa1c70, 0x379a58b6, 0xd73da509, 0x67cb0fb2, 0x9da8aff9, 0x8fef8355],
    &[0x0cec9cf5, 0xc61c4549, 0xf0dc5ac7, 0x6f44d1dc, 0x9a40acd5, 0xfb469310, 0x53d99581, 0x7ad1419a, 0x09ddff92, 0x4eae4647],
    &[0xed16526a, 0x5006a750, 0x199e76e7, 0x1a37277f, 0x5271f6b1, 0xd8b946e7, 0x3e740dbd, 0x5c88b02d, 0xd9f72db9, 0x406eabac],
    &[0xd53199a6, 0x3f23a58a, 0x7800f978, 0x96294d9f, 0xe02c93c1, 0x33cc4407, 0xd6433128, 0x56839fe9, 0x993864e9, 0xcde7a768],
];

#[test]
fn predef() {
    for x in 0..5 {
        for y in 0..10 {
            let enc = skip32::encode(KEYS[x], KEYS[x][y] as u32);
            let dec = skip32::decode(KEYS[x], VALUES[x][y]);
            assert!(enc == VALUES[x][y]);
            assert!(dec == KEYS[x][y] as u32);
        }
    }
}

use quickcheck::quickcheck;
use quickcheck::Arbitrary;
use quickcheck::Gen;

#[derive(Copy, Clone, Debug)]
struct X10<T: Copy>{ a: [T; 10] }

impl<A: Arbitrary + Clone + Copy> Arbitrary for X10<A> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        unsafe {
            let mut x: X10<A> = X10 { a: std::mem::uninitialized() };
            for x in &mut x.a { *x = A::arbitrary(g); }
            x
        }
    }
}

#[test]
fn prop_id() {
    fn prop(k: X10<u8>, x: u32) -> bool {
        skip32::decode(&k.a, skip32::encode(&k.a, x)) == x
    }
    quickcheck(prop as fn(X10<u8>, u32) -> bool);
}
