use std::mem::transmute;

fn main() {
    let a: u16 = 50115;
    let b: i16 = -15421;

    println!("a: {:016b} {}", a, a);
    println!("b: {:016b} {}", b, b);

    let af: f32 = 42.42;
    let ftype: u32 = unsafe { transmute(af) };
    println!("ft: {:032b} {}", ftype, ftype);
    let bf: f32 = unsafe { transmute(ftype) };
    println!("bf: {}", bf);
    assert_eq!(af, bf);

    let b_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let l_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];

    let a: i32 = unsafe { transmute(b_endian) };
    let b: i32 = unsafe { transmute(l_endian) };

    println!("{} vs {}", a, b);
    // (-1**0)*1.325625*(2**(132-127)) = 42.42
    let n: f32 = 42.42;
    let n_bits: u32 = n.to_bits();
    println!("  n_bits: {:032b} {}", n_bits, n_bits);
    let sign_bit = n_bits >> 31;
    println!("sign_bit: {:032b} {}", sign_bit, sign_bit);
    let exponent_ = n_bits >> 23;
    let exponent_ = exponent_ & 0xff;
    let exponent = (exponent_ as i32) - 127;
    println!("exponent: {:032b} {}", exponent, exponent);
    let mut mantissa: f32 = 1.0;

    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = n_bits & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }
    println!("mantissa: {:032b} {}", mantissa.to_bits(), mantissa);
}
