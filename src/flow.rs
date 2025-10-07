pub const fn gen_flow_keys<const N: usize>(base: u32, stmts: &[&'static str; N]) -> [u32; N] {
    let mut keys = [0u32; N];
    let mut state = base;
    let mut i = 0;
    while i < N {
        let stmt_bytes = stmts[i].as_bytes();
        let mut j = 0;
        while j < stmt_bytes.len() {
            state ^= (stmt_bytes[j] as u32).wrapping_shl(((j & 3) * 8) as u32);
            state = state.wrapping_mul(0x27d4eb2d);
            j += 1;
        }
        keys[i] = state;
        state = state.rotate_left(13);
        i += 1;
    }
    keys
}

#[macro_export]
macro_rules! obf_block {
    ($code:block) => {{
        let choice = $crate::ct_rand!(u8) % 3;
        match choice {
            0 => {
                $crate::noise_loop(20);
                $code
            }
            1 => {
                $crate::stack_trash();
                $code
            }
            _ => {
                $crate::fake_compute(42);
                $code
            }
        }
    }};
}

