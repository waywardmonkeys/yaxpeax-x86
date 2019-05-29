extern crate yaxpeax_arch;
extern crate yaxpeax_x86;

use std::fmt::Write;

use yaxpeax_arch::Decodable;
use yaxpeax_x86::{Instruction, Opcode, decode_one};

fn decode(bytes: &[u8]) -> Option<Instruction> {
    let mut instr = Instruction::invalid();
    match decode_one(bytes.iter().map(|x| *x).take(16).collect::<Vec<u8>>(), &mut instr) {
        Some(()) => Some(instr),
        None => None
    }
}

fn test_display(data: &[u8], expected: &'static str) {
    let mut hex = String::new();
    for b in data {
        write!(hex, "{:02x}", b);
    }
    match Instruction::decode(data.into_iter().map(|x| *x)) {
        Some(instr) => {
            let text = format!("{}", instr);
            assert!(
                text == expected,
                "display error for {}:\n  decoded: {:?}\n displayed: {}\n expected: {}\n",
                hex,
                instr,
                text,
                expected
            );
        },
        None => {
            assert!(false, "decode error for {}:\n  expected: {}\n", hex, expected);
        }
    }
}

#[test]
fn test_system() {
    test_display(&[0x45, 0x0f, 0x22, 0xc8], "mov cr9, r8");
    test_display(&[0x45, 0x0f, 0x20, 0xc8], "mov r8, cr9");
    test_display(&[0x40, 0x0f, 0x22, 0xc8], "mov cr1, rax");
    test_display(&[0x0f, 0x22, 0xc8], "mov cr1, rax");
    test_display(&[0x44, 0x0f, 0x22, 0xcf], "mov cr9, rdi");
    test_display(&[0x0f, 0x22, 0xcf], "mov cr1, rdi");
    test_display(&[0x0f, 0x20, 0xc8], "mov rax, cr1");

    test_display(&[0x45, 0x0f, 0x23, 0xc8], "mov dr9, r8");
    test_display(&[0x45, 0x0f, 0x21, 0xc8], "mov r8, dr9");
    test_display(&[0x40, 0x0f, 0x23, 0xc8], "mov dr1, rax");
    test_display(&[0x0f, 0x23, 0xc8], "mov dr1, rax");
    test_display(&[0x0f, 0x21, 0xc8], "mov rax, dr1");
}

#[test]
fn test_arithmetic() {
    test_display(&[0x81, 0xec, 0x10, 0x03, 0x00, 0x00], "sub esp, 0x310");
}

#[test]
fn test_E_decode() {
    test_display(&[0xff, 0x75, 0xb8], "push [rbp - 0x48]");
    test_display(&[0xff, 0x75, 0x08], "push [rbp + 0x8]");
}

// SETLE, SETNG, ...

#[test]
fn test_mov() {
    // test_display(&[0xa1, 0x93, 0x62, 0xc4, 0x00, 0x12, 0x34, 0x12, 0x34], "mov eax, [0x3412341200c46293]");
    // RCT.exe 32bit version, TODO: FIX
    test_display(&[0xa1, 0x93, 0x62, 0xc4, 0x00], "mov eax, [0xc46293]");
    test_display(&[0x48, 0xc7, 0x04, 0x24, 0x00, 0x00, 0x00, 0x00], "mov [rsp], 0x0");
    test_display(&[0x48, 0x89, 0x44, 0x24, 0x08], "mov [rsp + 0x8], rax");
    test_display(&[0x48, 0x89, 0x43, 0x18], "mov [rbx + 0x18], rax");
    test_display(&[0x48, 0xc7, 0x43, 0x10, 0x00, 0x00, 0x00, 0x00], "mov [rbx + 0x10], 0x0");
    test_display(&[0x49, 0x89, 0x4e, 0x08], "mov [r14 + 0x8], rcx");
    test_display(&[0x48, 0x8b, 0x32], "mov rsi, [rdx]");
    test_display(&[0x49, 0x89, 0x46, 0x10], "mov [r14 + 0x10], rax");
    test_display(&[0x4d, 0x0f, 0x43, 0xec, 0x49], "cmovnb r13, r12");
    test_display(&[0x0f, 0xb6, 0x06], "movzx eax, byte [rsi]");
    test_display(&[0x0f, 0xb7, 0x06], "movzx eax, word [rsi]");
    test_display(&[0x89, 0x55, 0x94], "mov [rbp - 0x6c], edx");
    test_display(&[0x65, 0x4c, 0x89, 0x04, 0x25, 0xa8, 0x01, 0x00, 0x00], "mov gs:[0x1a8], r8");
}

#[test]
fn test_stack() {
    test_display(&[0x66, 0x41, 0x50], "push r8w");
}

#[test]
fn test_prefixes() {
    test_display(&[0x66, 0x41, 0x31, 0xc0], "xor r8w, ax");
    test_display(&[0x66, 0x41, 0x32, 0xc0], "xor al, r8b");
    test_display(&[0x40, 0x32, 0xc5], "xor al, bpl");
}

#[test]
fn test_control_flow() {
    test_display(&[0x73, 0x31], "jnb 0x31");
    test_display(&[0x72, 0x5a], "jb 0x5a");
    test_display(&[0x0f, 0x86, 0x8b, 0x01, 0x00, 0x00], "jna 0x18b");
    test_display(&[0x74, 0x47], "jz 0x47");
    test_display(&[0xff, 0x15, 0x7e, 0x72, 0x24, 0x00], "call [rip + 0x24727e]");
    test_display(&[0xc3], "ret");
}

#[test]
fn test_test_cmp() {
    test_display(&[0x48, 0x3d, 0x01, 0xf0, 0xff, 0xff], "cmp rax, -0xfff");
    test_display(&[0x3d, 0x01, 0xf0, 0xff, 0xff], "cmp eax, -0xfff");
    test_display(&[0x48, 0x83, 0xf8, 0xff], "cmp rax, -0x1");
    test_display(&[0x48, 0x39, 0xc6], "cmp rsi, rax");
}

#[test]
#[ignore]
// VEX prefixes are not supported at the moment, in any form
fn test_avx() {
    test_display(&[0xc5, 0xf8, 0x10, 0x00], "vmovups xmm0, xmmword [rax]");
}

#[test]
fn test_push_pop() {
    test_display(&[0x5b], "pop rbx");
    test_display(&[0x41, 0x5e], "pop r14");
    test_display(&[0x68, 0x7f, 0x63, 0xc4, 0x00], "push 0xc4637f");
}

#[test]
fn test_misc() {
    test_display(&[0x48, 0x8d, 0xa4, 0xc7, 0x20, 0x00, 0x00, 0x12], "lea rsp, [rdi + rax * 8 + 0x12000020]");
    test_display(&[0x33, 0xc0], "xor eax, eax");
    test_display(&[0x48, 0x8d, 0x53, 0x08], "lea rdx, [rbx + 0x8]");
    test_display(&[0x31, 0xc9], "xor ecx, ecx");
    test_display(&[0x48, 0x29, 0xc8], "sub rax, rcx");
    test_display(&[0x48, 0x03, 0x0b], "add rcx, [rbx]");
    test_display(&[0x48, 0x8d, 0x0c, 0x12], "lea rcx, [rdx + rdx]");
    test_display(&[0xf6, 0xc2, 0x18], "test dl, 0x18");
}
