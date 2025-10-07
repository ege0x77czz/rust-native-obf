pub const fn mba_encode_u32(val: u32, key: u32) -> u32 {
    let a = val ^ key;
    let b = a.wrapping_mul(0x9e3779b1);
    let c = b.rotate_left(13);
    let d = c ^ 0xdeadbeef;
    d
}

pub const fn mba_decode_u32(val: u32, key: u32) -> u32 {
    let d = val ^ 0xdeadbeef;
    let c = d.rotate_right(13);
    let b = c.wrapping_mul(0x61c88647);
    let a = b ^ key;
    a
}

pub const fn mba_encode_u64(val: u64, key: u64) -> u64 {
    let a = val ^ key;
    let b = a.wrapping_mul(0x517cc1b727220a95);
    let c = b.rotate_left(31);
    let d = c ^ 0xcafebabe12345678;
    d
}

pub const fn mba_decode_u64(val: u64, key: u64) -> u64 {
    let d = val ^ 0xcafebabe12345678;
    let c = d.rotate_right(31);
    let b = c.wrapping_mul(0xbf58476d1ce4e5b9);
    let a = b ^ key;
    a
}

#[macro_export]
macro_rules! obf_const {
    ($val:expr, u32) => {{
        const _KEY: u32 = $crate::ct_rand!(u32, stringify!($val));
        const _ENC: u32 = $crate::encoding::mba_encode_u32($val, _KEY);
        $crate::encoding::mba_decode_u32(_ENC, _KEY)
    }};
    ($val:expr, u64) => {{
        const _KEY: u64 = $crate::ct_rand!(u64, stringify!($val));
        const _ENC: u64 = $crate::encoding::mba_encode_u64($val, _KEY);
        $crate::encoding::mba_decode_u64(_ENC, _KEY)
    }};
}

pub fn runtime_encode(val: usize) -> usize {
    let key = 0x9e3779b97f4a7c15usize;
    val.wrapping_mul(key).rotate_left(7) ^ 0xdeadbeef
}

pub fn runtime_decode(val: usize) -> usize {
    let key_inv = 0xf1bbcdcbfa53dcc9usize;
    (val ^ 0xdeadbeef).rotate_right(7).wrapping_mul(key_inv)
}

