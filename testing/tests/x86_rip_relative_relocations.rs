use dynasmrt::dynasm;
use dynasmrt::{DynasmApi, DynasmLabelApi};

// x64 and x86 are fairly similar, but differ in their handling of 32-bit displacements, which are RIP relative on x64, but absolute on x86
// we actually try to support rip-relative ones on x86, so there's some fun underneath

// special test cases for the absolute-to-relative relocations we generate for x86 rip-relative addressing
#[test]
fn x86_eip_relative_addressing() {
    let mut ops = dynasmrt::VecAssembler::<dynasmrt::x86::X86Relocation>::new(0x1234_0000usize);
    dynasm!(ops
        ; .arch x86
        ; lea eax, [0x1234_5678]
        ; lea eax, [eip + 0x5678]
        ; lea eax, [eip - 0x5678]
    );
    let buf = ops.finalize().unwrap();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex: String = hex.join(",");
    assert_eq!(hex,
               "8D,05,78,56,34,12,8D,05,78,56,34,12,8D,05,88,A9,33,12",
               "x86 lea reg, [eip + offset] absolute-to-relative relocations");
}

// we need 32-bits address space to validate working wraparound
#[cfg(target_pointer_width="32")]
#[test]
fn x86_eip_relative_addressing_wraparound() {
    let mut ops = dynasmrt::VecAssembler::<dynasmrt::x86::X86Relocation>::new(0x1234_0000usize);
    dynasm!(ops
        ; .arch x86
        ; lea eax, [0x9234_5678u32 as i32]
        ; lea eax, [eip + 0x7FFF_FFFF]
        ; lea eax, [eip - 0x8000_0000]
    );
    let buf = ops.finalize().unwrap();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex: String = hex.join(",");
    assert_eq!(hex,
               "8D,05,78,56,34,92,8D,05,FF,FF,33,92,8D,05,00,00,34,92",
               "x86 lea reg, [eip + offset] absolute-to-relative relocations");
}
