#![no_std]
#![no_main]
// #![feature(asm_experimental_arch)]

//! semihost通信模块
mod semihost {
    /// semihost操作类型
    #[repr(u32)]
    pub enum Action {
        #[warn(dead_code)]
        WriteChar   = 0x03, // 输出单个字符
        WriteString = 0x04, // 输出字符串
        Exit        = 0x18, // 带状态的退出
    }

    /// 执行semihost调用（ARM模式）
    #[inline(always)]
    pub unsafe fn exec(
        op: Action,
        arg: usize,
    ) -> i32 {
        let op_code = op as u32;
        let result: i32;
    
        // 注意：bkpt指令后的立即数在ARM模式下应为0xAB
        core::arch::asm!(
            "bkpt 0xAB",
            in("r0") op_code,
            in("r1") arg,
            lateout("r0") result,
            options(nostack, preserves_flags)
        );
    
        result
    }

    /// 输出字符
    pub fn outc(c: u8) {
        unsafe {
            exec(Action::WriteChar, c as usize);
        }
    }

    /// 输出字符串（用户保证s为\0结束的C-String）
    pub fn outs(s: *const u8) -> i32 {
        unsafe {
            exec(Action::WriteString, s as usize)
        }
    }

    // 输出字符串（Rust标准样式）TODO
    /*pub fn outstr(s: String) -> i32 {
        unsafe {
            exec(Action::WriteString, s as usize)
        }
    }*/
}

use core::panic::PanicInfo;
use semihost::outs;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    let status = main();
    exit(status);
}

fn main() -> i32 {
    let message = b"Hello, World!\n";
    outs(message.as_ptr())
}

// 系统关机（QEMU会结束运行）
fn exit(status : i32) -> !  {
    unsafe {
        semihost::exec(semihost::Action::Exit, status as usize);
    }
    loop {}
}

/// 定义向量表
// #[link_section = ".vector_table.reset_vector"]
// #[no_mangle]
// pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

/// Panic处理函数
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// 默认中断处理
#[no_mangle]
pub extern "C" fn DefaultHandler() -> ! {
    loop {}
}
