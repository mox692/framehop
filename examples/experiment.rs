fn main() {
    // fooのinstruction pointer
    let foo_ptr = foo as *mut ();
    println!("foo ptr:       {:?}", foo_ptr);
    println!("foo ptr + 8:   {:?}", unsafe { foo_ptr.byte_add(8) });
    let s = 3;
    let p = foo(s + s); // 0x0000000100001997

    // 0x1000019ac <+396>: movl   -0x138(%rbp), %edi
    // 0x1000019b2 <+402>: callq  0x1000019d0               ; hello::foo::h2d1768ddeef007d1 at hello.rs:10
    // 0x1000019b7 <+407>: addq   $0x140, %rsp              ; imm = 0x140

    // 0x00000001000019b7

    let b = 3; // 0x00000001000019b7
}
fn foo(x: i32) {
    bar(x + x);
}
fn bar(x: i32) {
    // rbp: このbarのframeの中にあるbpの位置. 値はfoo関数のrbpの位置が保存されている
    // = このアドレス+8がfooのreturnアドレスになるはず)

    // 現在のrbpの値(このbarの中にあるframe pointerの位置)
    println!("rpb:           {:?}", { read_register("rbp").unwrap() });

    // fooのframe pointerのaddr
    println!("read(rpb)    : {:?}", unsafe {
        read_memory(read_register("rbp").unwrap() as usize) as *const ()
    });

    // fooのframe pointerのaddr + 8byte のaddr (これがfooのreturn addrになる？)
    println!("read(read(rpb) + 8): {:?}", unsafe {
        read_memory(
            (read_memory(read_register("rbp").unwrap() as usize) as *const ()).byte_add(8) as usize,
        )
    });
}

fn read_memory(address: usize) -> usize {
    // 指定されたメモリ位置から値を読み込む
    let value: usize;
    unsafe {
        value = *(address as *const usize);
    }
    value
}

fn read_register(register: &str) -> Result<*const (), &str> {
    use std::arch::asm;

    let mut value: usize = 0;

    unsafe {
        match register {
            "rax" => asm!("mov {}, rax", out(reg) value),
            "rbx" => asm!("mov {}, rbx", out(reg) value),
            "rcx" => asm!("mov {}, rcx", out(reg) value),
            "rdx" => asm!("mov {}, rdx", out(reg) value),
            "rbp" => asm!("mov {}, rbp", out(reg) value),
            "rip" => asm!("lea {}, [rip]", out(reg) value),
            _ => return Err("Unsupported register"),
        }
    }

    Ok(value as *const ())
}
