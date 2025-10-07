pub mod rng;
pub mod hash;
pub mod string;
pub mod flow;
pub mod encoding;
pub mod pointer;
pub mod wide;

pub use rng::{xxhash_mix, siphash_seed, gen_entropy, GLOBAL_SEED};
pub use hash::{xxhash32, sdbm_hash};

pub fn opaque_true() -> bool {
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    (t | 1) != 0
}

pub fn opaque_identity<T>(val: T) -> T {
    if opaque_true() { val } else { unreachable!() }
}

pub fn stack_trash() {
    let mut buf = [0u8; 512];
    for i in 0..512 {
        buf[i] = ((i * 73) ^ 0xaa) as u8;
    }
    std::hint::black_box(&buf);
}

#[macro_export]
macro_rules! hidden_call {
    ($expr:expr) => {{
        if $crate::opaque_true() {
            $crate::stack_trash();
            $expr
        } else {
            unreachable!()
        }
    }};
}

pub fn noise_loop(count: usize) -> u64 {
    let mut acc = 0u64;
    for i in 0..count {
        acc = acc.wrapping_add(i as u64);
        acc ^= 0xdeadbeef;
        acc = acc.rotate_left(7);
    }
    acc
}

pub fn fake_compute(input: u64) -> u64 {
    let mut val = input;
    val ^= 0xa5a5a5a5a5a5a5a5;
    val = val.wrapping_mul(0x517cc1b727220a95);
    val = val.rotate_left(23);
    val ^= val >> 17;
    val
}

#[macro_export]
macro_rules! obf_if {
    ($cond:expr, $then:block, $else:block) => {{
        let c = $cond;
        $crate::stack_trash();
        if c { $then } else { $else }
    }};
    ($cond:expr, $then:block) => {{
        let c = $cond;
        $crate::stack_trash();
        if c { $then }
    }};
}

pub fn xor_stream(data: &mut [u8], key: &[u8]) {
    for (i, byte) in data.iter_mut().enumerate() {
        *byte ^= key[i % key.len()];
    }
}

pub fn cascade_encrypt(data: &[u8], rounds: usize) -> Vec<u8> {
    let mut result = data.to_vec();
    for round in 0..rounds {
        let mut key = (0xb7u8).wrapping_add(round as u8);
        for byte in result.iter_mut() {
            *byte = byte.wrapping_add(key);
            *byte ^= key;
            key = key.wrapping_mul(17).wrapping_add(83);
        }
    }
    result
}

pub fn cascade_decrypt(data: &[u8], rounds: usize) -> Vec<u8> {
    let mut result = data.to_vec();
    for round in (0..rounds).rev() {
        let mut key = (0xb7u8).wrapping_add(round as u8);
        for byte in result.iter_mut() {
            *byte ^= key;
            *byte = byte.wrapping_sub(key);
            key = key.wrapping_mul(17).wrapping_add(83);
        }
    }
    result
}

pub struct ObfuscatedValue<T> {
    data: Vec<u8>,
    key: u64,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Sized> ObfuscatedValue<T> {
    pub fn new(val: T) -> Self {
        let bytes = unsafe {
            std::slice::from_raw_parts(&val as *const T as *const u8, std::mem::size_of::<T>())
        };
        let key = noise_loop(100);
        let data = bytes.iter().enumerate()
            .map(|(i, &b)| b ^ ((key >> (i % 8 * 8)) as u8))
            .collect();
        std::mem::forget(val);
        Self { data, key, _marker: std::marker::PhantomData }
    }

    pub fn get(&self) -> T {
        let mut bytes = vec![0u8; self.data.len()];
        for (i, &b) in self.data.iter().enumerate() {
            bytes[i] = b ^ ((self.key >> (i % 8 * 8)) as u8);
        }
        unsafe { std::ptr::read(bytes.as_ptr() as *const T) }
    }
}

pub fn tamper_check(expected: u32) -> bool {
    let actual = ct_xxhash!(b"tamper_check");
    actual == expected
}

pub fn anti_debug() -> bool {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            extern "system" {
                fn IsDebuggerPresent() -> i32;
            }
            IsDebuggerPresent() == 0
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        true
    }
}

#[macro_export]
macro_rules! debug_trap {
    () => {
        if !$crate::anti_debug() {
            panic!("debugger detected");
        }
    };
}
