pub const fn xxhash32(data: &[u8], seed: u32) -> u32 {
    const PRIME1: u32 = 0x9e3779b1;
    const PRIME2: u32 = 0x85ebca77;
    const PRIME3: u32 = 0xc2b2ae3d;
    const PRIME4: u32 = 0x27d4eb2f;
    const PRIME5: u32 = 0x165667b1;

    let mut h = seed.wrapping_add(PRIME5);
    let mut i = 0;

    while i + 4 <= data.len() {
        let k = u32::from_le_bytes([data[i], data[i+1], data[i+2], data[i+3]]);
        h = h.wrapping_add(k.wrapping_mul(PRIME3));
        h = h.rotate_left(17).wrapping_mul(PRIME4);
        i += 4;
    }

    while i < data.len() {
        h = h.wrapping_add((data[i] as u32).wrapping_mul(PRIME5));
        h = h.rotate_left(11).wrapping_mul(PRIME1);
        i += 1;
    }

    h = h.wrapping_add(data.len() as u32);
    h ^= h >> 15;
    h = h.wrapping_mul(PRIME2);
    h ^= h >> 13;
    h = h.wrapping_mul(PRIME3);
    h ^= h >> 16;
    h
}

pub const fn sdbm_hash(s: &str) -> u32 {
    let bytes = s.as_bytes();
    let mut hash = 0u32;
    let mut i = 0;
    while i < bytes.len() {
        hash = (bytes[i] as u32)
            .wrapping_add(hash.wrapping_shl(6))
            .wrapping_add(hash.wrapping_shl(16))
            .wrapping_sub(hash);
        i += 1;
    }
    hash
}

#[macro_export]
macro_rules! ct_xxhash {
    ($data:expr) => {{
        const _H: u32 = $crate::hash::xxhash32($data, 0);
        _H
    }};
    ($data:expr, $seed:expr) => {{
        const _H: u32 = $crate::hash::xxhash32($data, $seed);
        _H
    }};
}

#[macro_export]
macro_rules! ct_sdbm {
    ($s:expr) => {{
        const _H: u32 = $crate::hash::sdbm_hash($s);
        _H
    }};
}

