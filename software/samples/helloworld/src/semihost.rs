//! Semihosting通信模块

/// Semihosting操作类型
#[repr(u32)]
pub enum SemihostingOperation {
    SYS_WRITE0 = 0x04,    // 输出字符串
    SYS_REPORTEXC = 0x18, // 报告异常
}

/// 执行semihosting调用（ARM模式）
#[inline(always)]
pub unsafe fn semihost_call(
    op: SemihostingOperation,
    arg: *const u8
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

/// 输出字符串（自动添加空终止符）
pub fn print_str(s: &str) {
    // 将Rust字符串转换为C字符串（自动添加空终止符）
    let mut buffer = heapless::Vec::<u8, 128>::new();
    buffer.extend_from_slice(s.as_bytes()).unwrap();
    buffer.push(0).unwrap();

    unsafe {
        semihost_call(
            SemihostingOperation::SYS_WRITE0,
            buffer.as_ptr()
        );
    }
}
