use bitflags::bitflags;

use crate::mem::Memory;
use crate::opcode::*;

pub type Byte = u8;
pub type Word = u16;
pub type DoubleWord = u32;

pub const CODE_START: Word = 0xA000;
pub const STACK_START: Word = 0x0100;
pub const STACK_END: Word = 0x01FF;
pub const RESET_VECTOR: Word = 0xFFFC;

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct ProcessorStatus : u8 {
        const Carry = 0b0000_0001;
        const Zero = 0b0000_0010;
        const InterruptDisable = 0b0000_0100;
        const DecimalMode = 0b0000_1000;
        const Break = 0b0001_0000;
        const _Unused = 0b0010_0000;
        const Overflow = 0b0100_0000;
        const Negative = 0b1000_0000;
    }
}

pub enum CycleRestriction {
    None,
    Some(usize),
}

#[derive(Debug)]
pub struct Cpu {
    pub memory: Memory,

    pub pc: Word,
    pub sp: Byte,
    pub a: Byte,
    pub x: Byte,
    pub y: Byte,
    pub status: ProcessorStatus,
}

impl Cpu {
    pub fn new(memory: Memory) -> Self {
        Self {
            memory,

            pc: CODE_START,
            sp: 0xFF,
            a: 0,
            x: 0,
            y: 0,
            status: ProcessorStatus::empty(),
        }
    }

    pub fn run(&mut self, instruction_limit: Option<usize>) {
        if let Some(limit) = instruction_limit {
            for _ in 0..limit {
                self.execute_next_instruction();
            }
        } else {
            loop {
                self.execute_next_instruction();
            }
        }
    }

    fn execute_next_instruction(&mut self) {
        let opcode = self.fetch_and_advance_pc();
        let instruction = Instruction::try_from(opcode);
        let instruction = match instruction {
            Ok(instruction) => instruction,
            Err(_) => {
                self.invalid_opcode();
                return;
            }
        };

        let handler = match instruction.opcode {
            Opcode::Adc => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::And => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Asl => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Bcc => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Bcs => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Beq => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Bit => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Bmi => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Bne => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Bpl => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Brk => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Bvc => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Bvs => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Clc => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Cld => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Cli => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Clv => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Cmp => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Cpx => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Cpy => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Dec => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Dex => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Dey => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Eor => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Inc => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Inx => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Iny => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Jmp => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Jsr => Self::execute_jsr,
            Opcode::Lda => Self::execute_lda,
            Opcode::Ldx => Self::execute_ldx,
            Opcode::Ldy => Self::execute_ldy,
            Opcode::Lsr => Self::execute_lsr,
            Opcode::Nop => Self::execute_nop,
            Opcode::Ora => Self::execute_ora,
            Opcode::Pha => Self::execute_pha,
            Opcode::Php => Self::execute_php,
            Opcode::Pla => Self::execute_pla,
            Opcode::Plp => Self::execute_plp,
            Opcode::Rol => Self::execute_rol,
            Opcode::Ror => Self::execute_ror,
            Opcode::Rti => Self::execute_rti,
            Opcode::Rts => Self::execute_rts,
            Opcode::Sbc => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Sec => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Sed => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Sei => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Sta => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Stx => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Sty => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Tax => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Tay => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Tsx => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Txa => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Txs => todo!("{:?} not yet implemented", instruction.opcode),
            Opcode::Tya => todo!("{:?} not yet implemented", instruction.opcode),
        };
        handler(self, instruction.addressing_mode);
    }

    fn execute_jsr(&mut self, addressing_mode: AddressingMode) {
        debug_assert_eq!(addressing_mode, AddressingMode::Absolute);

        let low_byte = self.fetch_and_advance_pc();
        let high_byte = self.fetch_and_advance_pc();
        let address = (high_byte as Word) << 8 | (low_byte as Word);
        let return_address = self.pc - 1;
        self.push((return_address >> 8) as Byte);
        self.push((return_address & 0xFF) as Byte);
        self.pc = address;
    }

    fn execute_lda(&mut self, addressing_mode: AddressingMode) {
        let value = self.resolve_argument_value(addressing_mode);
        self.set_zero_and_negative_flags(value);
        self.a = value;
    }

    fn execute_ldx(&mut self, addressing_mode: AddressingMode) {
        let value = self.resolve_argument_value(addressing_mode);
        self.set_zero_and_negative_flags(value);
        self.x = value;
    }

    fn execute_ldy(&mut self, addressing_mode: AddressingMode) {
        let value = self.resolve_argument_value(addressing_mode);
        self.set_zero_and_negative_flags(value);
        self.y = value;
    }

    fn execute_lsr(&mut self, addressing_mode: AddressingMode) {
        assert_ne!(addressing_mode, AddressingMode::Immediate);

        let lsr = |cpu: &mut Cpu, value: Byte| -> Byte {
            cpu.status
                .set(ProcessorStatus::Carry, value & 0b0000_0001 > 0);
            let new_value = value >> 1;
            cpu.set_zero_and_negative_flags(new_value);
            new_value
        };

        if addressing_mode == AddressingMode::Accumulator {
            let value = self.a;
            self.a = lsr(self, value);
            return;
        }

        let address = self.resolve_argument_address(addressing_mode);
        let value = self.memory.read(address);
        let new_value = lsr(self, value);
        self.memory.write(address, new_value);
    }

    fn execute_nop(&mut self, _: AddressingMode) {
        // do nothing
    }

    fn execute_ora(&mut self, addressing_mode: AddressingMode) {
        let value = self.resolve_argument_value(addressing_mode);
        self.a |= value;
        self.set_zero_and_negative_flags(self.a);
    }

    fn execute_pha(&mut self, addressing_mode: AddressingMode) {
        debug_assert_eq!(addressing_mode, AddressingMode::Implicit);
        self.push(self.a);
    }

    fn execute_php(&mut self, addressing_mode: AddressingMode) {
        debug_assert_eq!(addressing_mode, AddressingMode::Implicit);
        self.push(self.status.bits());
    }

    fn execute_pla(&mut self, addressing_mode: AddressingMode) {
        debug_assert_eq!(addressing_mode, AddressingMode::Implicit);
        self.a = self.pop();
        self.set_zero_and_negative_flags(self.a);
    }

    fn execute_plp(&mut self, addressing_mode: AddressingMode) {
        debug_assert_eq!(addressing_mode, AddressingMode::Implicit);
        self.status = ProcessorStatus::from_bits_truncate(self.pop());
    }

    fn execute_rol(&mut self, addressing_mode: AddressingMode) {
        let rol = |cpu: &mut Cpu, value: Byte| -> Byte {
            let mut new_value = value << 1;
            if cpu.status.contains(ProcessorStatus::Carry) {
                new_value |= 1;
            }
            cpu.set_zero_and_negative_flags(new_value);
            cpu.a = new_value;
            cpu.status
                .set(ProcessorStatus::Carry, value & 0b1000_0000 > 0);
            new_value
        };

        if addressing_mode == AddressingMode::Accumulator {
            let value = self.a;
            self.a = rol(self, value);
            return;
        }

        let address = self.resolve_argument_address(addressing_mode);
        let value = self.memory.read(address);
        let new_value = rol(self, value);
        self.memory.write(address, new_value);
    }

    fn execute_ror(&mut self, addressing_mode: AddressingMode) {
        let ror = |cpu: &mut Cpu, value: Byte| -> Byte {
            let mut new_value = value >> 1;
            if cpu.status.contains(ProcessorStatus::Carry) {
                new_value |= 0b1000_0000;
            }
            cpu.set_zero_and_negative_flags(new_value);
            cpu.a = new_value;
            cpu.status
                .set(ProcessorStatus::Carry, value & 0b0000_0001 > 0);
            new_value
        };

        if addressing_mode == AddressingMode::Accumulator {
            let value = self.a;
            self.a = ror(self, value);
            return;
        }

        let address = self.resolve_argument_address(addressing_mode);
        let value = self.memory.read(address);
        let new_value = ror(self, value);
        self.memory.write(address, new_value);
    }

    fn execute_rti(&mut self, addressing_mode: AddressingMode) {
        debug_assert_eq!(addressing_mode, AddressingMode::Implicit);

        self.status = ProcessorStatus::from_bits_truncate(self.pop());
        let low_byte = self.pop();
        let high_byte = self.pop();
        self.pc = (high_byte as Word) << 8 | (low_byte as Word);
    }

    fn execute_rts(&mut self, addressing_mode: AddressingMode) {
        debug_assert_eq!(addressing_mode, AddressingMode::Implicit);

        let low_byte = self.pop();
        let high_byte = self.pop();
        self.pc = (high_byte as Word) << 8 | (low_byte as Word);
        self.pc += 1;
    }

    fn push(&mut self, byte: Byte) {
        let address = STACK_START + self.sp as Word;
        self.memory.write(address, byte);
        self.sp = self.sp.checked_sub(1).expect("stack overflow");
    }

    fn pop(&mut self) -> Byte {
        self.sp = self.sp.checked_add(1).expect("stack underflow");
        let address = STACK_START + self.sp as Word;
        self.memory.read(address)
    }

    fn resolve_argument_address(&mut self, addressing_mode: AddressingMode) -> Word {
        match addressing_mode {
            AddressingMode::Accumulator | AddressingMode::Implicit | AddressingMode::Immediate => {
                unreachable!(
                    "{:?} addressing mode does not have an address",
                    addressing_mode
                );
            }
            AddressingMode::ZeroPage => self.fetch_and_advance_pc() as Word,
            AddressingMode::ZeroPageX => {
                let address = self.fetch_and_advance_pc();
                address.wrapping_add(self.x) as Word
            }
            AddressingMode::ZeroPageY => {
                let address = self.fetch_and_advance_pc();
                address.wrapping_add(self.y) as Word
            }
            AddressingMode::Absolute => {
                let low_byte = self.fetch_and_advance_pc();
                let high_byte = self.fetch_and_advance_pc();
                (high_byte as Word) << 8 | (low_byte as Word)
            }
            AddressingMode::AbsoluteX => {
                let low_byte = self.fetch_and_advance_pc();
                let high_byte = self.fetch_and_advance_pc();
                let address = (high_byte as Word) << 8 | (low_byte as Word);
                address.wrapping_add(self.x as Word)
            }
            AddressingMode::AbsoluteY => {
                let low_byte = self.fetch_and_advance_pc();
                let high_byte = self.fetch_and_advance_pc();
                let address = (high_byte as Word) << 8 | (low_byte as Word);
                address.wrapping_add(self.y as Word)
            }
            _ => unimplemented!("addressing mode {:?} not implemented", addressing_mode),
        }
    }

    fn resolve_argument_value(&mut self, addressing_mode: AddressingMode) -> Byte {
        if addressing_mode == AddressingMode::Immediate {
            return self.fetch_and_advance_pc();
        } else if addressing_mode == AddressingMode::Accumulator {
            return self.a;
        }

        let address = self.resolve_argument_address(addressing_mode);
        self.memory.read(address)
    }

    fn set_zero_and_negative_flags(&mut self, value: Byte) {
        self.status.set(ProcessorStatus::Zero, value == 0);
        self.status
            .set(ProcessorStatus::Negative, value & 0b1000_0000 > 0);
    }

    fn fetch_and_advance_pc(&mut self) -> Byte {
        let byte = self.memory.read(self.pc);
        self.pc += 1;
        byte
    }

    pub fn invalid_opcode(&mut self) {
        let original_pc = self.pc - 1; // we've already advanced the pc by one, so we need to subtract one to get the original pc
        panic!(
            "Invalid opcode {:#02x}\npc: {:#02x}\nsp: {:#02x}\na: {:#02x}\nx: {:#02x}\ny: {:#02x}\nstatus: {:?}", 
                self.memory.read(original_pc),
                original_pc,
                self.sp,
                self.a,
                self.x,
                self.y,
                self.status,
        );
    }
}
