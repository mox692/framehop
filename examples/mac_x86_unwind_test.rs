//! macos(x86) で, stack unwindがどのようにできるかテスト
//!
//! * 実行スレッドの情報(レジスタの値とか)を取得

#[cfg(target_arch = "x86_64")]
use framehop::x86_64::UnwindRegsX86_64;
#[cfg(target_arch = "x86_64")]
use framehop::x86_64::{CacheX86_64, UnwinderX86_64};
use framehop::Unwinder;
use std::arch::asm;

fn main() {
    foo();
}
fn foo() {
    bar()
}
fn bar() {
    baz()
}
fn baz() {
    let mut cache = CacheX86_64::<_>::new();
    let unwinder: UnwinderX86_64<Vec<u8>> = UnwinderX86_64::new();

    let mut read_stack = |addr| {
        if addr % 8 != 0 {
            // Unaligned address
            return Err(());
        }
        // MEMO: シンプルに addr で渡ってきてるメモリの値を読んでるだけ.
        Ok(read_memory(addr))
    };

    // get value of registers
    let (rip, regs) = {
        let mut rip = 0;
        let mut rsp = 0;
        let mut rbp = 0;
        unsafe { asm!("lea {}, [rip]", out(reg) rip) };
        unsafe { asm!("mov {}, rsp", out(reg) rsp) };
        unsafe { asm!("mov {}, rbp", out(reg) rbp) };
        (rip, UnwindRegsX86_64::new(rip, rsp, rbp))
    };

    println!("rip: {:?}", rip as usize as *const ());
    println!("regs: {:?}", &regs);

    let mut iter = unwinder.iter_frames(rip, regs, &mut cache, &mut read_stack);

    let mut frames = Vec::new();
    // print frame
    while let Ok(Some(frame)) = iter.next() {
        println!(
            "********** addr: {:?}",
            frame.address() as usize as *const ()
        );
        frames.push(frame);
    }

    println!("all frame: {:?}", frames.len());
}

fn read_memory(address: u64) -> u64 {
    // 指定されたメモリ位置から値を読み込む
    let value: usize;
    unsafe {
        value = *(address as *const usize);
    }
    value as u64
}
