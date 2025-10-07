pub const fn xxhash_mix(mut x: u64) -> u64 {
    x ^= x >> 33;
    x = x.wrapping_mul(0xc2b2ae3d27d4eb4f);
    x ^= x >> 29;
    x = x.wrapping_mul(0x165667b19e3779f9);
    x ^= x >> 32;
    x
}

pub const fn siphash_seed(a: &str) -> u64 {
    let bytes = a.as_bytes();
    let mut h = 0x736f6d6570736575u64;
    let mut i = 0;
    while i < bytes.len() {
        h ^= (bytes[i] as u64).wrapping_shl(((i & 7) * 8) as u32);
        h = h.wrapping_mul(0x517cc1b727220a95);
        i += 1;
    }
    h
}

pub const fn gen_entropy(input: &str) -> u64 {
    let base = siphash_seed(input);
    xxhash_mix(base ^ GLOBAL_SEED)
}

pub const GLOBAL_SEED: u64 = match option_env!("NATIVE_OBF_SEED") {
    Some(s) => siphash_seed(s),
    None => xxhash_mix(0x1234567890abcdef),
};

#[macro_export]
macro_rules! ct_rand {
    ($ty:ident $(, $seed:expr)*) => {{
        const _RND: $ty = $crate::rng::cast_rand!($ty, 
            $crate::rng::gen_entropy(concat!(file!(), ":", line!(), ":", column!() $(, ":", $seed)*)));
        _RND
    }};
}

#[macro_export]
macro_rules! cast_rand {
    (u8, $v:expr) => { $v as u8 };
    (u16, $v:expr) => { $v as u16 };
    (u32, $v:expr) => { $v as u32 };
    (u64, $v:expr) => { $v };
    (usize, $v:expr) => { $v as usize };
    (i8, $v:expr) => { $v as i8 };
    (i16, $v:expr) => { $v as i16 };
    (i32, $v:expr) => { $v as i32 };
    (i64, $v:expr) => { $v as i64 };
    (isize, $v:expr) => { $v as isize };
    (bool, $v:expr) => { ($v & 1) == 1 };
}

