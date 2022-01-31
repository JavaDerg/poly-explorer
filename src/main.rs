use crate::xor_lfsr::Lfsr;

mod xor_lfsr;

const POLYS: [u32; 32] = [
    0x0001D258, 0x00017E04,
    0x0001FF6B, 0x00013F67,
    0x0001B9EE, 0x000198D1,
    0x000178C7, 0x00018A55,
    0x00015777, 0x0001D911,
    0x00015769, 0x0001991F,
    0x00012BD0, 0x0001CF73,
    0x0001365D, 0x000197F5,
    0x000194A0, 0x0001B279,
    0x00013A34, 0x0001AE41,
    0x000180D4, 0x00017891,
    0x00012E64, 0x00017C72,
    0x00019C6D, 0x00013F32,
    0x0001AE14, 0x00014E76,
    0x00013C97, 0x000130CB,
    0x00013750, 0x0001CB8D,
];

fn main() {
    if let Some(arg) = std::env::args().skip(1).next() {
        if arg == "rand" {
            let poly = POLYS[fastrand::usize(0..POLYS.len())];
            let mut lfsr = Lfsr::<17>::new(poly, 1);
            for _ in 0..fastrand::usize(0..2 << 17) {
                lfsr.next();
            }
            for bits in 15..=30 {
                println!("{}: {:0bits$b}", bits, b = lfsr.get_n(bits));
            }
            return;
        }
        let target = u32::from_str_radix(arg.trim(), 2).unwrap();
        let bits = arg.trim().len();
        // let mask = xor_lfsr::mask(bits);
        println!("bits={bits}");

        for (poly, pi) in POLYS.into_iter().zip(0..) {
            let mut lfsr = Lfsr::<17>::new(poly, 1);
            for i in 0.. {
                if lfsr.get_n(bits) == target {
                    println!("MATCH {i:06}, poly=0x{poly:08X} ch={} bit={}", pi / 2 + 1, pi & 0x01);
                    break;
                }
                if lfsr.next().is_none() {
                    break;
                }
            }
        }
    }
}
