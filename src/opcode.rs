use derive_more::{Constructor, Display, Error};

use crate::cpu::Byte;

#[derive(Constructor, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub addressing_mode: AddressingMode,
}

#[derive(Error, Display, Debug, Copy, Clone, Eq, PartialEq)]
pub struct DecodeError;

impl TryFrom<Byte> for Instruction {
    type Error = DecodeError;

    fn try_from(value: Byte) -> Result<Self, Self::Error> {
        Ok(match value {
            // ADC
            0x69 => Self::new(Opcode::Adc, AddressingMode::Immediate),
            0x65 => Self::new(Opcode::Adc, AddressingMode::ZeroPage),
            0x75 => Self::new(Opcode::Adc, AddressingMode::ZeroPageX),
            0x6D => Self::new(Opcode::Adc, AddressingMode::Absolute),
            0x7D => Self::new(Opcode::Adc, AddressingMode::AbsoluteX),
            0x79 => Self::new(Opcode::Adc, AddressingMode::AbsoluteY),
            0x61 => Self::new(Opcode::Adc, AddressingMode::IndexedIndirect),
            0x71 => Self::new(Opcode::Adc, AddressingMode::IndirectIndexed),
            // AND
            0x29 => Self::new(Opcode::And, AddressingMode::Immediate),
            0x25 => Self::new(Opcode::And, AddressingMode::ZeroPage),
            0x35 => Self::new(Opcode::And, AddressingMode::ZeroPageX),
            0x2D => Self::new(Opcode::And, AddressingMode::Absolute),
            0x3D => Self::new(Opcode::And, AddressingMode::AbsoluteX),
            0x39 => Self::new(Opcode::And, AddressingMode::AbsoluteY),
            0x21 => Self::new(Opcode::And, AddressingMode::IndexedIndirect),
            0x31 => Self::new(Opcode::And, AddressingMode::IndirectIndexed),
            // ASL
            0x0A => Self::new(Opcode::Asl, AddressingMode::Accumulator),
            0x06 => Self::new(Opcode::Asl, AddressingMode::ZeroPage),
            0x16 => Self::new(Opcode::Asl, AddressingMode::ZeroPageX),
            0x0E => Self::new(Opcode::Asl, AddressingMode::Absolute),
            0x1E => Self::new(Opcode::Asl, AddressingMode::AbsoluteX),
            // BCC
            0x90 => Self::new(Opcode::Bcc, AddressingMode::Relative),
            // BCS
            0xB0 => Self::new(Opcode::Bcs, AddressingMode::Relative),
            // BEQ
            0xF0 => Self::new(Opcode::Beq, AddressingMode::Relative),
            // BIT
            0x24 => Self::new(Opcode::Bit, AddressingMode::ZeroPage),
            0x2C => Self::new(Opcode::Bit, AddressingMode::Absolute),
            // BMI
            0x30 => Self::new(Opcode::Bmi, AddressingMode::Relative),
            // BNE
            0xD0 => Self::new(Opcode::Bne, AddressingMode::Relative),
            // BPL
            0x10 => Self::new(Opcode::Bpl, AddressingMode::Relative),
            // BRK
            0x00 => Self::new(Opcode::Brk, AddressingMode::Implicit),
            // BVC
            0x50 => Self::new(Opcode::Bvc, AddressingMode::Relative),
            // BVS
            0x70 => Self::new(Opcode::Bvs, AddressingMode::Relative),
            // CLC
            0x18 => Self::new(Opcode::Clc, AddressingMode::Implicit),
            // CLD
            0xD8 => Self::new(Opcode::Cld, AddressingMode::Implicit),
            // CLI
            0x58 => Self::new(Opcode::Cli, AddressingMode::Implicit),
            // CLV
            0xB8 => Self::new(Opcode::Clv, AddressingMode::Implicit),
            // CMP
            0xC9 => Self::new(Opcode::Cmp, AddressingMode::Immediate),
            0xC5 => Self::new(Opcode::Cmp, AddressingMode::ZeroPage),
            0xD5 => Self::new(Opcode::Cmp, AddressingMode::ZeroPageX),
            0xCD => Self::new(Opcode::Cmp, AddressingMode::Absolute),
            0xDD => Self::new(Opcode::Cmp, AddressingMode::AbsoluteX),
            0xD9 => Self::new(Opcode::Cmp, AddressingMode::AbsoluteY),
            0xC1 => Self::new(Opcode::Cmp, AddressingMode::IndexedIndirect),
            0xD1 => Self::new(Opcode::Cmp, AddressingMode::IndirectIndexed),
            // CPX
            0xE0 => Self::new(Opcode::Cpx, AddressingMode::Immediate),
            0xE4 => Self::new(Opcode::Cpx, AddressingMode::ZeroPage),
            0xEC => Self::new(Opcode::Cpx, AddressingMode::Absolute),
            // CPY
            0xC0 => Self::new(Opcode::Cpy, AddressingMode::Immediate),
            0xC4 => Self::new(Opcode::Cpy, AddressingMode::ZeroPage),
            0xCC => Self::new(Opcode::Cpy, AddressingMode::Absolute),
            // DEC
            0xC6 => Self::new(Opcode::Dec, AddressingMode::ZeroPage),
            0xD6 => Self::new(Opcode::Dec, AddressingMode::ZeroPageX),
            0xCE => Self::new(Opcode::Dec, AddressingMode::Absolute),
            0xDE => Self::new(Opcode::Dec, AddressingMode::AbsoluteX),
            // DEX
            0xCA => Self::new(Opcode::Dex, AddressingMode::Implicit),
            // DEY
            0x88 => Self::new(Opcode::Dey, AddressingMode::Implicit),
            // EOR
            0x49 => Self::new(Opcode::Eor, AddressingMode::Immediate),
            0x45 => Self::new(Opcode::Eor, AddressingMode::ZeroPage),
            0x55 => Self::new(Opcode::Eor, AddressingMode::ZeroPageX),
            0x4D => Self::new(Opcode::Eor, AddressingMode::Absolute),
            0x5D => Self::new(Opcode::Eor, AddressingMode::AbsoluteX),
            0x59 => Self::new(Opcode::Eor, AddressingMode::AbsoluteY),
            0x41 => Self::new(Opcode::Eor, AddressingMode::IndexedIndirect),
            0x51 => Self::new(Opcode::Eor, AddressingMode::IndirectIndexed),
            // INC
            0xE6 => Self::new(Opcode::Inc, AddressingMode::ZeroPage),
            0xF6 => Self::new(Opcode::Inc, AddressingMode::ZeroPageX),
            0xEE => Self::new(Opcode::Inc, AddressingMode::Absolute),
            0xFE => Self::new(Opcode::Inc, AddressingMode::AbsoluteX),
            // INX
            0xE8 => Self::new(Opcode::Inx, AddressingMode::Implicit),
            // INY
            0xC8 => Self::new(Opcode::Iny, AddressingMode::Implicit),
            // JMP
            0x4C => Self::new(Opcode::Jmp, AddressingMode::Absolute),
            0x6C => Self::new(Opcode::Jmp, AddressingMode::Indirect),
            // JSR
            0x20 => Self::new(Opcode::Jsr, AddressingMode::Absolute),
            // LDA
            0xA9 => Self::new(Opcode::Lda, AddressingMode::Immediate),
            0xA5 => Self::new(Opcode::Lda, AddressingMode::ZeroPage),
            0xB5 => Self::new(Opcode::Lda, AddressingMode::ZeroPageX),
            0xAD => Self::new(Opcode::Lda, AddressingMode::Absolute),
            0xBD => Self::new(Opcode::Lda, AddressingMode::AbsoluteX),
            0xB9 => Self::new(Opcode::Lda, AddressingMode::AbsoluteY),
            0xA1 => Self::new(Opcode::Lda, AddressingMode::IndexedIndirect),
            0xB1 => Self::new(Opcode::Lda, AddressingMode::IndirectIndexed),
            // LDX
            0xA2 => Self::new(Opcode::Ldx, AddressingMode::Immediate),
            0xA6 => Self::new(Opcode::Ldx, AddressingMode::ZeroPage),
            0xB6 => Self::new(Opcode::Ldx, AddressingMode::ZeroPageY),
            0xAE => Self::new(Opcode::Ldx, AddressingMode::Absolute),
            0xBE => Self::new(Opcode::Ldx, AddressingMode::AbsoluteY),
            // LDY
            0xA0 => Self::new(Opcode::Ldy, AddressingMode::Immediate),
            0xA4 => Self::new(Opcode::Ldy, AddressingMode::ZeroPage),
            0xB4 => Self::new(Opcode::Ldy, AddressingMode::ZeroPageX),
            0xAC => Self::new(Opcode::Ldy, AddressingMode::Absolute),
            0xBC => Self::new(Opcode::Ldy, AddressingMode::AbsoluteX),
            // LSR
            0x4A => Self::new(Opcode::Lsr, AddressingMode::Accumulator),
            0x46 => Self::new(Opcode::Lsr, AddressingMode::ZeroPage),
            0x56 => Self::new(Opcode::Lsr, AddressingMode::ZeroPageX),
            0x4E => Self::new(Opcode::Lsr, AddressingMode::Absolute),
            0x5E => Self::new(Opcode::Lsr, AddressingMode::AbsoluteX),
            // NOP
            0xEA => Self::new(Opcode::Nop, AddressingMode::Implicit),
            // ORA
            0x09 => Self::new(Opcode::Ora, AddressingMode::Immediate),
            0x05 => Self::new(Opcode::Ora, AddressingMode::ZeroPage),
            0x15 => Self::new(Opcode::Ora, AddressingMode::ZeroPageX),
            0x0D => Self::new(Opcode::Ora, AddressingMode::Absolute),
            0x1D => Self::new(Opcode::Ora, AddressingMode::AbsoluteX),
            0x19 => Self::new(Opcode::Ora, AddressingMode::AbsoluteY),
            0x01 => Self::new(Opcode::Ora, AddressingMode::IndexedIndirect),
            0x11 => Self::new(Opcode::Ora, AddressingMode::IndirectIndexed),
            // PHA
            0x48 => Self::new(Opcode::Pha, AddressingMode::Implicit),
            // PHP
            0x08 => Self::new(Opcode::Php, AddressingMode::Implicit),
            // PLA
            0x68 => Self::new(Opcode::Pla, AddressingMode::Implicit),
            // PLP
            0x28 => Self::new(Opcode::Plp, AddressingMode::Implicit),
            // ROL
            0x2A => Self::new(Opcode::Rol, AddressingMode::Accumulator),
            0x26 => Self::new(Opcode::Rol, AddressingMode::ZeroPage),
            0x36 => Self::new(Opcode::Rol, AddressingMode::ZeroPageX),
            0x2E => Self::new(Opcode::Rol, AddressingMode::Absolute),
            0x3E => Self::new(Opcode::Rol, AddressingMode::AbsoluteX),
            // ROR
            0x6A => Self::new(Opcode::Ror, AddressingMode::Accumulator),
            0x66 => Self::new(Opcode::Ror, AddressingMode::ZeroPage),
            0x76 => Self::new(Opcode::Ror, AddressingMode::ZeroPageX),
            0x6E => Self::new(Opcode::Ror, AddressingMode::Absolute),
            0x7E => Self::new(Opcode::Ror, AddressingMode::AbsoluteX),
            // RTI
            0x40 => Self::new(Opcode::Rti, AddressingMode::Implicit),
            // RTS
            0x60 => Self::new(Opcode::Rts, AddressingMode::Implicit),
            // SBC
            0xE9 => Self::new(Opcode::Sbc, AddressingMode::Immediate),
            0xE5 => Self::new(Opcode::Sbc, AddressingMode::ZeroPage),
            0xF5 => Self::new(Opcode::Sbc, AddressingMode::ZeroPageX),
            0xED => Self::new(Opcode::Sbc, AddressingMode::Absolute),
            0xFD => Self::new(Opcode::Sbc, AddressingMode::AbsoluteX),
            0xF9 => Self::new(Opcode::Sbc, AddressingMode::AbsoluteY),
            0xE1 => Self::new(Opcode::Sbc, AddressingMode::IndexedIndirect),
            0xF1 => Self::new(Opcode::Sbc, AddressingMode::IndirectIndexed),
            // SEC
            0x38 => Self::new(Opcode::Sec, AddressingMode::Implicit),
            // SED
            0xF8 => Self::new(Opcode::Sed, AddressingMode::Implicit),
            // SEI
            0x78 => Self::new(Opcode::Sei, AddressingMode::Implicit),
            // STA
            0x85 => Self::new(Opcode::Sta, AddressingMode::ZeroPage),
            0x95 => Self::new(Opcode::Sta, AddressingMode::ZeroPageX),
            0x8D => Self::new(Opcode::Sta, AddressingMode::Absolute),
            0x9D => Self::new(Opcode::Sta, AddressingMode::AbsoluteX),
            0x99 => Self::new(Opcode::Sta, AddressingMode::AbsoluteY),
            0x81 => Self::new(Opcode::Sta, AddressingMode::IndexedIndirect),
            0x91 => Self::new(Opcode::Sta, AddressingMode::IndirectIndexed),
            // STX
            0x86 => Self::new(Opcode::Stx, AddressingMode::ZeroPage),
            0x96 => Self::new(Opcode::Stx, AddressingMode::ZeroPageY),
            0x8E => Self::new(Opcode::Stx, AddressingMode::Absolute),
            // STY
            0x84 => Self::new(Opcode::Sty, AddressingMode::ZeroPage),
            0x94 => Self::new(Opcode::Sty, AddressingMode::ZeroPageX),
            0x8C => Self::new(Opcode::Sty, AddressingMode::Absolute),
            // TAX
            0xAA => Self::new(Opcode::Tax, AddressingMode::Implicit),
            // TAY
            0xA8 => Self::new(Opcode::Tay, AddressingMode::Implicit),
            // TSX
            0xBA => Self::new(Opcode::Tsx, AddressingMode::Implicit),
            // TXA
            0x8A => Self::new(Opcode::Txa, AddressingMode::Implicit),
            // TXS
            0x9A => Self::new(Opcode::Txs, AddressingMode::Implicit),
            // TYA
            0x98 => Self::new(Opcode::Tya, AddressingMode::Implicit),
            // Unknown
            _ => return Err(DecodeError),
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Opcode {
    Adc,
    And,
    Asl,
    Bcc,
    Bcs,
    Beq,
    Bit,
    Bmi,
    Bne,
    Bpl,
    Brk,
    Bvc,
    Bvs,
    Clc,
    Cld,
    Cli,
    Clv,
    Cmp,
    Cpx,
    Cpy,
    Dec,
    Dex,
    Dey,
    Eor,
    Inc,
    Inx,
    Iny,
    Jmp,
    Jsr,
    Lda,
    Ldx,
    Ldy,
    Lsr,
    Nop,
    Ora,
    Pha,
    Php,
    Pla,
    Plp,
    Rol,
    Ror,
    Rti,
    Rts,
    Sbc,
    Sec,
    Sed,
    Sei,
    Sta,
    Stx,
    Sty,
    Tax,
    Tay,
    Tsx,
    Txa,
    Txs,
    Tya,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AddressingMode {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
}
