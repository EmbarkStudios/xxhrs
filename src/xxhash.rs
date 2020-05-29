use std::default::Default;
use std::hash::Hasher;
use std::mem::MaybeUninit;
use std::os::raw::c_void;

use crate::xxhash_bindings as C;
// XXH32, XXH32_reset, XXH32_update, XXH32_digest,
// XXH64, XXH64_reset, XXH64_update, XXH64_digest,

#[derive(Clone)]
pub struct XXH32 {
    state: C::XXH32_state_t,
}

impl Default for XXH32 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl XXH32 {
    #[inline]
    pub fn hash(bytes: &[u8]) -> u32 {
        XXH32::hash_with_seed(0, bytes)
    }

    #[inline]
    pub fn hash_with_seed(seed: u32, bytes: &[u8]) -> u32 {
        unsafe { C::XXH32(bytes.as_ptr() as *const c_void, bytes.len(), seed) }
    }

    #[inline]
    pub fn new() -> XXH32 {
        XXH32::with_seed(0)
    }

    #[inline]
    pub fn with_seed(seed: u32) -> XXH32 {
        unsafe {
            let mut r = MaybeUninit::<C::XXH32_state_t>::uninit();
            C::XXH32_reset(r.as_mut_ptr() as *mut C::XXH32_state_t, seed);
            XXH32 {
                state: r.assume_init(),
            }
        }
    }

    #[inline]
    pub fn write(&mut self, bytes: &[u8]) {
        unsafe {
            C::XXH32_update(
                &mut self.state,
                bytes.as_ptr() as *const c_void,
                bytes.len(),
            );
        }
    }

    #[inline]
    pub fn finish(&self) -> u32 {
        unsafe { C::XXH32_digest(&self.state) }
    }
}

#[derive(Clone)]
pub struct XXH64 {
    state: C::XXH64_state_t,
}

impl Default for XXH64 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl XXH64 {
    #[inline]
    pub fn hash(bytes: &[u8]) -> u64 {
        XXH64::hash_with_seed(0, bytes)
    }

    #[inline]
    pub fn hash_with_seed(seed: u64, bytes: &[u8]) -> u64 {
        unsafe { C::XXH64(bytes.as_ptr() as *const c_void, bytes.len(), seed) }
    }

    #[inline]
    pub fn new() -> XXH64 {
        XXH64::with_seed(0)
    }

    #[inline]
    pub fn with_seed(seed: u64) -> XXH64 {
        unsafe {
            let mut r = MaybeUninit::<C::XXH64_state_t>::uninit();
            C::XXH64_reset(r.as_mut_ptr() as *mut C::XXH64_state_t, seed);
            XXH64 {
                state: r.assume_init(),
            }
        }
    }
}

impl Hasher for XXH64 {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        unsafe {
            C::XXH64_update(
                &mut self.state,
                bytes.as_ptr() as *const c_void,
                bytes.len(),
            );
        }
    }

    #[inline]
    fn finish(&self) -> u64 {
        unsafe { C::XXH64_digest(&self.state) }
    }
}
