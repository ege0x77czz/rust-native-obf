const fn utf8_char_len(first: u8) -> usize {
    if first & 0x80 == 0x00 { 1 }
    else if first & 0xe0 == 0xc0 { 2 }
    else if first & 0xf0 == 0xe0 { 3 }
    else if first & 0xf8 == 0xf0 { 4 }
    else { 1 }
}

const fn decode_utf8(bytes: &[u8]) -> Option<(u32, usize)> {
    if bytes.len() == 0 { return None; }
    let len = utf8_char_len(bytes[0]);
    if bytes.len() < len { return None; }
    
    let chr = match len {
        1 => bytes[0] as u32,
        2 => ((bytes[0] & 0x1f) as u32) << 6 | (bytes[1] & 0x3f) as u32,
        3 => ((bytes[0] & 0x0f) as u32) << 12 | ((bytes[1] & 0x3f) as u32) << 6 | (bytes[2] & 0x3f) as u32,
        4 => ((bytes[0] & 0x07) as u32) << 18 | ((bytes[1] & 0x3f) as u32) << 12 | ((bytes[2] & 0x3f) as u32) << 6 | (bytes[3] & 0x3f) as u32,
        _ => return None,
    };
    Some((chr, len))
}

pub const fn utf16_len(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut len = 0;
    let mut i = 0;
    while i < bytes.len() {
        if let Some((chr, step)) = decode_utf8(&bytes[i..]) {
            len += if chr >= 0x10000 { 2 } else { 1 };
            i += step;
        } else {
            i += 1;
        }
    }
    len
}

pub const fn encode_utf16<const N: usize>(s: &str) -> [u16; N] {
    let bytes = s.as_bytes();
    let mut result = [0u16; N];
    let mut i = 0;
    let mut pos = 0;
    while i < bytes.len() {
        if let Some((chr, step)) = decode_utf8(&bytes[i..]) {
            if chr >= 0x10000 {
                let code = chr - 0x10000;
                result[pos] = (0xd800 + (code >> 10)) as u16;
                result[pos + 1] = (0xdc00 + (code & 0x3ff)) as u16;
                pos += 2;
            } else {
                result[pos] = chr as u16;
                pos += 1;
            }
            i += step;
        } else {
            i += 1;
        }
    }
    result
}

#[macro_export]
macro_rules! wide_str {
    ($s:expr) => {{
        const _LEN: usize = $crate::wide::utf16_len($s);
        const _DATA: [u16; _LEN] = $crate::wide::encode_utf16::<_LEN>($s);
        &_DATA
    }};
}

pub const fn gen_wide_keys<const N: usize>(seed: u32) -> [u16; N] {
    let mut keys = [0u16; N];
    let mut state = seed;
    let mut i = 0;
    while i < N {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        keys[i] = state as u16;
        i += 1;
    }
    keys
}

pub const fn encrypt_wide<const N: usize>(data: &[u16], keys: &[u16; N]) -> [u16; N] {
    let mut result = [0u16; N];
    let mut i = 0;
    while i < N {
        result[i] = data[i] ^ keys[i];
        i += 1;
    }
    result
}

pub fn decrypt_wide<const N: usize>(data: &[u16; N], keys: &[u16; N]) -> [u16; N] {
    let mut result = [0u16; N];
    let mut i = 0;
    while i < N {
        result[i] = data[i] ^ keys[i];
        i += 1;
    }
    result
}

#[macro_export]
macro_rules! obf_wide {
    ($s:expr) => {{
        const _LEN: usize = $crate::wide::utf16_len($s);
        const _SRC: [u16; _LEN] = $crate::wide::encode_utf16::<_LEN>($s);
        const _KEYS: [u16; _LEN] = $crate::wide::gen_wide_keys::<_LEN>(
            $crate::ct_rand!(u32, stringify!($s)));
        static _ENC: [u16; _LEN] = $crate::wide::encrypt_wide::<_LEN>(&_SRC, &_KEYS);
        $crate::wide::decrypt_wide::<_LEN>(&_ENC, &_KEYS)
    }};
}

