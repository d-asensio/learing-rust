fn scalar_types_integer () {
    println!("8 bits:");
    const SIGNED_8_BITS_INTEGER: i8 = 127;
    println!("Signed 8bit integer (-128...127): {}", SIGNED_8_BITS_INTEGER);

    const UNSIGNED_8_BITS_INTEGER: u8 = 255;
    println!("Signed 8bit integer (0...255): {}", UNSIGNED_8_BITS_INTEGER);

    println!("16 bits:");
    const SIGNED_16_BITS_INTEGER: i16 = 32_767;
    println!("Signed 16bit integer (-32768...32767): {}", SIGNED_16_BITS_INTEGER);

    const UNSIGNED_16_BITS_INTEGER: u16 = 65_535;
    println!("Signed 16bit integer (0...65535): {}", UNSIGNED_16_BITS_INTEGER);

    println!("32 bits:");
    const SIGNED_32_BITS_INTEGER: i32 = 214_483_647;
    println!("Signed 32bit integer (-2147483648...2147483647): {}", SIGNED_32_BITS_INTEGER);

    const UNSIGNED_32_BITS_INTEGER: u32 = 4_294_967_295;
    println!("Signed 32bit integer (0...4294967295): {}", UNSIGNED_32_BITS_INTEGER);

    println!("64 bits:");
    const SIGNED_64_BITS_INTEGER: i64 = 9_223_372_036_854_775_807;
    println!("Signed 64bit integer (-9223372036854775808...9223372036854775807): {}", SIGNED_64_BITS_INTEGER);

    const UNSIGNED_64_BITS_INTEGER: u64 = 18_446_744_073_709_551_615;
    println!("Signed 64bit integer (0...18446744073709551615): {}", UNSIGNED_64_BITS_INTEGER);

    println!("128 bits:");
    const SIGNED_128_BITS_INTEGER: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;
    println!("Signed 128bit integer (-170141183460469231731687303715884105728...170141183460469231731687303715884105727): {}", SIGNED_128_BITS_INTEGER);

    const UNSIGNED_128_BITS_INTEGER: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
    println!("Signed 128bit integer (0...340282366920938463463374607431768211455): {}", UNSIGNED_128_BITS_INTEGER);

    println!("Architecture (64bits in my case):");
    const SIGNED_ARCHITECTURE_INTEGER: isize = 9_223_372_036_854_775_807;
    println!("Unsigned architecture: {}", SIGNED_ARCHITECTURE_INTEGER);

    const UNSIGNED_ARCHITECTURE_INTEGER: usize = 18_446_744_073_709_551_615;
    println!("Signed architecture: {}", UNSIGNED_ARCHITECTURE_INTEGER);
}

fn floating_point () {
    const FLOATING_64: f64 = 2.1;
    const FLOATING_32: f32 = 3.2;

    println!("Floating point number 64 bits: {}", FLOATING_64);
    println!("Floating point number 32 bits: {}", FLOATING_32);
}

fn main() {
    println!("-- Scalar types --");
    println!("-- integer");
    scalar_types_integer();
    println!("-- floating point");
    floating_point();
}
