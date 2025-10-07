use core::ptr::{read_volatile, write};

const fn pcg_step(mut state: u32) -> u32 {
    state = state.wrapping_mul(747796405).wrapping_add(2891336453);
    let word = ((state >> ((state >> 28) + 4)) ^ state).wrapping_mul(277803737);
    (word >> 22) ^ word
}

pub const fn gen_keystream<const N: usize>(seed: u32) -> [u8; N] {
    let mut keys = [0u8; N];
    let mut state = seed;
    let mut i = 0;
    while i < N {
        state = pcg_step(state);
        let bytes = state.to_ne_bytes();
        let mut j = 0;
        while j < 4 && i + j < N {
            keys[i + j] = bytes[j];
            j += 1;
        }
        i += 4;
    }
    keys
}

pub const fn encrypt_bytes<const N: usize>(data: &[u8], keys: &[u8; N]) -> [u8; N] {
    let mut result = [0u8; N];
    let mut i = 0;
    while i < N {
        result[i] = data[i] ^ keys[i];
        i += 1;
    }
    result
}

pub fn decrypt_bytes<const N: usize>(data: &[u8; N], keys: &[u8; N]) -> [u8; N] {
    let mut result = [0u8; N];
    let mut i = 0;
    unsafe {
        let src = data.as_ptr();
        let dst = result.as_mut_ptr();
        #[cfg(target_pointer_width = "64")]
        while i + 8 <= N {
            let enc = read_volatile(src.add(i) as *const u64);
            let key = u64::from_ne_bytes([
                keys[i], keys[i+1], keys[i+2], keys[i+3],
                keys[i+4], keys[i+5], keys[i+6], keys[i+7],
            ]);
            write(dst.add(i) as *mut u64, enc ^ key);
            i += 8;
        }
        while i + 4 <= N {
            let enc = read_volatile(src.add(i) as *const u32);
            let key = u32::from_ne_bytes([keys[i], keys[i+1], keys[i+2], keys[i+3]]);
            write(dst.add(i) as *mut u32, enc ^ key);
            i += 4;
        }
        while i < N {
            let enc = read_volatile(src.add(i));
            write(dst.add(i), enc ^ keys[i]);
            i += 1;
        }
    }
    result
}

#[inline(always)]
pub fn bytes_to_str(bytes: &[u8]) -> &str {
    #[cfg(debug_assertions)]
    return core::str::from_utf8(bytes).unwrap();
    #[cfg(not(debug_assertions))]
    return unsafe { core::str::from_utf8_unchecked(bytes) };
}

#[macro_export]
macro_rules! obf_str {
    ($s:expr) => {{
        const _SRC: &[u8] = $s.as_bytes();
        const _LEN: usize = _SRC.len();
        const _KEYS: [u8; _LEN] = $crate::string::gen_keystream::<_LEN>(
            $crate::ct_rand!(u32, stringify!($s)));
        static _ENC: [u8; _LEN] = $crate::string::encrypt_bytes::<_LEN>(_SRC, &_KEYS);
        $crate::string::bytes_to_str(&$crate::string::decrypt_bytes::<_LEN>(&_ENC, &_KEYS))
    }};
}

#[macro_export]
macro_rules! obf_bytes {
    ($s:expr) => {{
        const _SRC: &[u8] = $s;
        const _LEN: usize = _SRC.len();
        const _KEYS: [u8; _LEN] = $crate::string::gen_keystream::<_LEN>(
            $crate::ct_rand!(u32, stringify!($s)));
        static _ENC: [u8; _LEN] = $crate::string::encrypt_bytes::<_LEN>(_SRC, &_KEYS);
        $crate::string::decrypt_bytes::<_LEN>(&_ENC, &_KEYS)
    }};
}

