use std::ops::Deref;

use super::unwind_rule::*;
use crate::cache::*;

pub struct CacheAarch64<D: Deref<Target = [u8]>, P: AllocationPolicy<D> = MayAllocateDuringUnwind>(
    pub Cache<D, UnwindRuleAarch64, P>,
);

impl<D: Deref<Target = [u8]>, P: AllocationPolicy<D>> CacheAarch64<D, P> {
    pub fn new() -> Self {
        Self(Cache::new())
    }
}

impl<D: Deref<Target = [u8]>, P: AllocationPolicy<D>> Default for CacheAarch64<D, P> {
    fn default() -> Self {
        Self::new()
    }
}