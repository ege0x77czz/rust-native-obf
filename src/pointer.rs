use core::hint::black_box;

const fn ptr_mangle<const SEED: u64>(offset: u32) -> usize {
    let mut val = offset as u64;
    val ^= SEED;
    val = val.wrapping_mul(0x517cc1b727220a95);
    val ^= val >> 27;
    val = val.wrapping_mul(0x94d049bb133111eb);
    (val & 0xffff) as usize
}

#[inline(never)]
fn restore_ptr<const SEED: u64>(base: *const u8, offset: u32) -> *const u8 {
    base.wrapping_add(ptr_mangle::<SEED>(offset))
}

pub fn obf_ref<T: ?Sized, const OFF: u32, const SEED: u64>(r: &'static T) -> &'static T {
    unsafe {
        let mut ptr: *const T = r;
        let base_ptr = (ptr as *const u8).wrapping_sub(ptr_mangle::<SEED>(OFF));
        let restored = restore_ptr::<SEED>(black_box(base_ptr), black_box(OFF));
        *(&mut ptr as *mut *const T as *mut *const u8) = restored;
        &*ptr
    }
}

pub fn obf_ref_mut<T: ?Sized, const OFF: u32, const SEED: u64>(r: &'static mut T) -> &'static mut T {
    unsafe {
        let mut ptr: *mut T = r;
        let base_ptr = (ptr as *mut u8).wrapping_sub(ptr_mangle::<SEED>(OFF));
        let restored = restore_ptr::<SEED>(black_box(base_ptr as *const u8), black_box(OFF)) as *mut u8;
        *(&mut ptr as *mut *mut T as *mut *mut u8) = restored;
        &mut *ptr
    }
}

#[macro_export]
macro_rules! obf_static_ref {
    ($e:expr) => {
        $crate::pointer::obf_ref::<_,
            { $crate::ct_rand!(u32, stringify!($e)) },
            { $crate::ct_rand!(u64, stringify!($e), "ptr") }>($e)
    };
}

#[macro_export]
macro_rules! obf_static_mut {
    ($e:expr) => {
        $crate::pointer::obf_ref_mut::<_,
            { $crate::ct_rand!(u32, stringify!($e)) },
            { $crate::ct_rand!(u64, stringify!($e), "ptr") }>($e)
    };
}

