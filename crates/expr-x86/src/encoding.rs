//! x86-64 instruction encoding helpers.
//!
//! This module provides functions to encode x86-64 instructions as bytes.

pub fn ret() -> Vec<u8> {
    vec![0xC3]
}

pub fn push_rax() -> Vec<u8> {
    vec![0x50]
}

pub fn pop_rax() -> Vec<u8> {
    vec![0x58]
}

pub fn pop_rbx() -> Vec<u8> {
    vec![0x5B]
}

pub fn mov_rax_imm64(value: i64) -> Vec<u8> {
    let mut bytes = vec![0x48, 0xB8]; // REX.W + MOV RAX, imm64
    bytes.extend_from_slice(&value.to_le_bytes());
    bytes
}

pub fn add_rax_rbx() -> Vec<u8> {
    vec![0x48, 0x01, 0xD8] // REX.W + ADD rax, rbx
}

pub fn sub_rax_rbx() -> Vec<u8> {
    vec![0x48, 0x29, 0xD8] // REX.W + SUB rax, rbx
}

pub fn imul_rax_rbx() -> Vec<u8> {
    vec![0x48, 0x0F, 0xAF, 0xC3] // REX.W + IMUL rax, rbx
}

pub fn cqo() -> Vec<u8> {
    vec![0x48, 0x99] // REX.W + CQO
}

pub fn idiv_rbx() -> Vec<u8> {
    vec![0x48, 0xF7, 0xFB] // REX.W + IDIV rbx
}
