use std::fmt::Debug;

use crate::display_utils::HexNum;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct UnwindRegsX86_64 {
    sp: u64,
    bp: u64,
}

impl UnwindRegsX86_64 {
    pub fn new(sp: u64, bp: u64) -> Self {
        Self { sp, bp }
    }

    #[inline(always)]
    pub fn sp(&self) -> u64 {
        self.sp
    }
    #[inline(always)]
    pub fn set_sp(&mut self, sp: u64) {
        self.sp = sp
    }

    #[inline(always)]
    pub fn bp(&self) -> u64 {
        self.bp
    }
    #[inline(always)]
    pub fn set_bp(&mut self, bp: u64) {
        self.bp = bp
    }
}

impl Debug for UnwindRegsX86_64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnwindRegsX86_64")
            .field("sp", &HexNum(self.sp))
            .field("bp", &HexNum(self.bp))
            .finish()
    }
}
