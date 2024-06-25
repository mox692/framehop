//! macos(x86) で, stack unwindがどのようにできるかテスト
//!
//! * 実行スレッドの情報(レジスタの値とか)を取得

#[cfg(target_arch = "x86_64")]
use framehop::x86_64::UnwindRegsX86_64;
#[cfg(target_arch = "x86_64")]
use framehop::x86_64::{CacheX86_64, UnwinderX86_64};
use framehop::Unwinder;
use std::arch::asm;
use std::path::Path;
use wholesym::{SymbolManager, SymbolManagerConfig, SymbolMap};

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

    let symbol_map = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(create_symbol_manager());

    // print frame
    while let Ok(Some(frame)) = iter.next() {
        for i in (0..8).into_iter() {
            let addr = frame.address() as usize as *const ();
            // println!(
            //     "********** {}  addr: {:?}, relative sym: {:?}, {:?}, {:?}",
            //     i,
            //     addr,
            //     symbol_map.lookup_sync(LookupAddress::Relative(addr as usize as u32 - i)),
            //     symbol_map.lookup_sync(LookupAddress::FileOffset(addr as usize as u64 - i as u64)),
            //     symbol_map.lookup_sync(LookupAddress::Svma(addr as usize as u64 - i as u64)),
            // );
        }
        // println!("");
        println!("addr: {:?}", frame.address() as u32 as *const ());
    }
}

async fn create_symbol_manager() -> SymbolMap {
    let config = SymbolManagerConfig::default();
    let symbol_manager = SymbolManager::with_config(config);

    let binary_path =
        Path::new("/Users/s15255/work/framehop/target/debug/examples/mac_x86_unwind_and_wholesym");
    let symbol_map: SymbolMap = symbol_manager
        .load_symbol_map_for_binary_at_path(binary_path, None)
        .await
        .unwrap();

    for (addr, (idx, sym)) in symbol_map.iter_symbols().enumerate() {
        if idx % 1000 == 0 {
            println!("addr: {:?}, sym: {:?}", addr as *const (), sym);
        }
    }

    symbol_map
}

fn read_memory(address: u64) -> u64 {
    // 指定されたメモリ位置から値を読み込む
    let value: usize;
    unsafe {
        value = *(address as *const usize);
    }
    value as u64
}
