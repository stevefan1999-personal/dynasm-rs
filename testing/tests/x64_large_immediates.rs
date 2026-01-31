//! Some extra x64 tests relating to 64-bit immediates, displacements and absolute relocations

#![allow(unused_imports)]

use dynasmrt::dynasm;
use dynasmrt::{DynasmApi, DynasmLabelApi, x64::X64Relocation};

include!("gen_x64/avx.rs.gen");

// 64-bit immediate move
#[test]
fn mov_64bit_imm() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch x64
        ; mov rax, 0xCC // normal 32-bit immediate mov
        ; mov rbx, 0x7FFF_FFFF // largest 32-bit normal immediate
        ; mov rcx, -0x8000_0000 // largest negative 32-bit normal immediate
        ; mov rdx, QWORD 0x0 // 64-bit immediate mov, forced size
        ; mov rsp, 0x8000_0000 // constant 64-bit immediate, auto promoted
        ; mov rbp, -0x8000_0001 // constant 64-bit immediate, auto promoted
        ; mov rsi, 0x7FFF_FFFF_FFFF_FFFF // largest 64-bit normal immediate
        ; mov rdi, -0x8000_0000_0000_0000 // largest negative 64-bit normal immediate
    );
    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex: String = hex.join(", ");
    assert_eq!(hex,
               "48, C7, C0, CC, 00, 00, 00, 48, C7, C3, FF, FF, FF, 7F, 48, C7, C1, 00, 00, 00, 80, 48, BA, 00, 00, 00, 00, 00, 00, 00, 00, 48, BC, 00, 00, 00, 80, 00, 00, 00, 00, 48, BD, FF, FF, FF, 7F, FF, FF, FF, FF, 48, BE, FF, FF, FF, FF, FF, FF, FF, 7F, 48, BF, 00, 00, 00, 00, 00, 00, 00, 80",
               "64-bit mov");
}

// 64-bit movabs: absolute address load/store, only to/from rax
#[test]
fn mov_64bit_disp() {
    let mut ops = dynasmrt::SimpleAssembler::new();
    dynasm!(ops
        ; .arch x64
        ; movabs rax, 0x0
        ; movabs 0x0, rax
        ; movabs rax, 0x7FFF_FFFF_FFFF_FFFF // largest 64-bit immediate
        ; movabs rax, -0x8000_0000_0000_0000 // largest negative 64-bit immediate
        ; movabs 0x7FFF_FFFF_FFFF_FFFF, rax // largest 64-bit immediate
        ; movabs -0x8000_0000_0000_0000, rax // largest negative 64-bit immediate
    );
    let buf = ops.finalize();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex: String = hex.join(", ");
    assert_eq!(hex,
               "48, A1, 00, 00, 00, 00, 00, 00, 00, 00, 48, A3, 00, 00, 00, 00, 00, 00, 00, 00, 48, A1, FF, FF, FF, FF, FF, FF, FF, 7F, 48, A1, 00, 00, 00, 00, 00, 00, 00, 80, 48, A3, FF, FF, FF, FF, FF, FF, FF, 7F, 48, A3, 00, 00, 00, 00, 00, 00, 00, 80",
               "64-bit mov");
}

// 64-bit absolute displacements
#[test]
fn mov_64bit_addr() {
    let mut ops = dynasmrt::VecAssembler::<X64Relocation>::new(0xAABB_CCDD_EEFF_0011);
    dynasm!(ops
        ; .arch x64
        ; mov r9, ->test_global_label
        ; movabs al, ->test_global_label
        ; movabs ax, ->test_global_label
        ; movabs eax, ->test_global_label
        ; movabs rax, ->test_global_label
        ; movabs ->test_global_label, al
        ; movabs ->test_global_label, ax
        ; movabs ->test_global_label, eax
        ; movabs ->test_global_label, rax
        ; ->test_global_label:
    );
    let buf = ops.finalize().unwrap();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex: String = hex.join(", ");
    assert_eq!(hex,
               "49, B9, 67, 00, FF, EE, DD, CC, BB, AA, A0, 67, 00, FF, EE, DD, CC, BB, AA, 66, A1, 67, 00, FF, EE, DD, CC, BB, AA, A1, 67, 00, FF, EE, DD, CC, BB, AA, 48, A1, 67, 00, FF, EE, DD, CC, BB, AA, A2, 67, 00, FF, EE, DD, CC, BB, AA, 66, A3, 67, 00, FF, EE, DD, CC, BB, AA, A3, 67, 00, FF, EE, DD, CC, BB, AA, 48, A3, 67, 00, FF, EE, DD, CC, BB, AA",
               "64-bit mov");
}

// tests the relocations in practice
#[cfg(target_arch = "x86_64")]
#[test]
fn large_code_model() {
    let mut ops = dynasmrt::Assembler::<X64Relocation>::new().unwrap();
    let start = ops.offset();
    dynasm!(ops
        ; .arch x64
        // the entry point
        ; mov rax, ->far_function
        ; call rax
        ; ret
        // the far function
        ; ->far_function:
        ; movabs rax, ->far_data
        ; ret
        // the data
        ; ->far_data:
        ; .u64 0x0123456789ABCDEF
    );
    let buf = ops.finalize().unwrap();
    let ptr = buf.ptr(start);
    let function: extern "sysv64" fn() -> u64 = unsafe { std::mem::transmute(ptr) };
    let result = function();
    assert_eq!(result, 0x0123456789ABCDEF, "x64 large code model support");
}
