use bitflags::bitflags;

use crate::mem::Memory;
use crate::opcode::*;

pub type Byte = u8;
pub type Word = u16;
pub type DoubleWord = u32;

pub const CODE_START: Word = 0xC000;
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
        #[cfg(debug_assertions)]
        {
            println!("addr op ins |AC XR YR SP|nv_bdizc|");
            println!("------------|-----------|--------|");
        }

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

        let m = instruction.addressing_mode;
        match instruction.opcode {
            Opcode::Adc => self.execute_adc(m),
            Opcode::And => self.execute_and(m),
            Opcode::Asl => self.execute_asl(m),
            Opcode::Bcc => self.execute_bcc(m),
            Opcode::Bcs => self.execute_bcs(m),
            Opcode::Beq => self.execute_beq(m),
            Opcode::Bit => self.execute_bit(m),
            Opcode::Bmi => self.execute_bmi(m),
            Opcode::Bne => self.execute_bne(m),
            Opcode::Bpl => self.execute_bpl(m),
            Opcode::Brk => self.execute_brk(m),
            Opcode::Bvc => self.execute_bvc(m),
            Opcode::Bvs => self.execute_bvs(m),
            Opcode::Clc => self.execute_clc(m),
            Opcode::Cld => self.execute_cld(m),
            Opcode::Cli => self.execute_cli(m),
            Opcode::Clv => self.execute_clv(m),
            Opcode::Cmp => self.execute_cmp(m),
            Opcode::Cpx => self.execute_cpx(m),
            Opcode::Cpy => self.execute_cpy(m),
            Opcode::Dec => self.execute_dec(m),
            Opcode::Dex => self.execute_dex(m),
            Opcode::Dey => self.execute_dey(m),
            Opcode::Eor => self.execute_eor(m),
            Opcode::Inc => self.execute_inc(m),
            Opcode::Inx => self.execute_inx(m),
            Opcode::Iny => self.execute_iny(m),
            Opcode::Jmp => self.execute_jmp(m),
            Opcode::Jsr => self.execute_jsr(m),
            Opcode::Lda => self.execute_lda(m),
            Opcode::Ldx => self.execute_ldx(m),
            Opcode::Ldy => self.execute_ldy(m),
            Opcode::Lsr => self.execute_lsr(m),
            Opcode::Nop => {}
            Opcode::Ora => self.execute_ora(m),
            Opcode::Pha => self.execute_pha(m),
            Opcode::Php => self.execute_php(m),
            Opcode::Pla => self.execute_pla(m),
            Opcode::Plp => self.execute_plp(m),
            Opcode::Rol => self.execute_rol(m),
            Opcode::Ror => self.execute_ror(m),
            Opcode::Rti => self.execute_rti(m),
            Opcode::Rts => self.execute_rts(m),
            Opcode::Sbc => self.execute_sbc(m),
            Opcode::Sec => self.execute_sec(m),
            Opcode::Sed => self.execute_sed(m),
            Opcode::Sei => self.execute_sei(m),
            Opcode::Sta => self.execute_sta(m),
            Opcode::Stx => self.execute_stx(m),
            Opcode::Sty => self.execute_sty(m),
            Opcode::Tax => self.execute_tax(m),
            Opcode::Tay => self.execute_tay(m),
            Opcode::Tsx => self.execute_tsx(m),
            Opcode::Txa => self.execute_txa(m),
            Opcode::Txs => self.execute_txs(m),
            Opcode::Tya => self.execute_tya(m),
        };

        #[cfg(debug_assertions)]
        {
            println!(
                "{:04X} {:02X} {:?} |{:02X} {:02X} {:02X} {:02X}|{:08b}|",
                self.pc - 1,
                opcode,
                instruction.opcode,
                self.a,
                self.x,
                self.y,
                self.sp,
                self.status.bits(),
            );
        }
    }

    fn execute_adc(&mut self, addressing_mode: AddressingMode) {
        let value = self.resolve_argument_value(addressing_mode);
        let (new_value, carry) = self.a.overflowing_add(value);
        self.status.set(ProcessorStatus::Carry, carry);
        self.status.set(
            ProcessorStatus::Overflow,
            (self.a ^ new_value) & (value ^ new_value) & 0x80 > 0,
        );
        self.a = new_value;
        self.set_zero_and_negative_flags(self.a);
    }

    fn execute_and(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_asl(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_bcc(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_bcs(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_beq(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_bit(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_bmi(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_bne(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_bpl(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_brk(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_bvc(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_bvs(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_clc(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_cld(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_cli(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_clv(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_cmp(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_cpx(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_cpy(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_dec(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_dex(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_dey(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_eor(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_inc(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_inx(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_iny(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_jmp(&mut self, addressing_mode: AddressingMode) {
        let address = self.resolve_argument_address(addressing_mode);
        self.pc = address;
    }

    fn execute_jsr(&mut self, addressing_mode: AddressingMode) {
        debug_assert_eq!(addressing_mode, AddressingMode::Absolute);

        let address = self.resolve_argument_address(addressing_mode);
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

    fn execute_sbc(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_sec(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_sed(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_sei(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_sta(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_stx(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_sty(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_tax(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_tay(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_tsx(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_txa(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_txs(&mut self, addressing_mode: AddressingMode) {
        todo!()
    }

    fn execute_tya(&mut self, addressing_mode: AddressingMode) {
        todo!()
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
            AddressingMode::Indirect => {
                let low_byte = self.fetch_and_advance_pc();
                let high_byte = self.fetch_and_advance_pc();
                let address = (high_byte as Word) << 8 | (low_byte as Word);
                let low_byte = self.memory.read(address);
                let high_byte = self.memory.read(address + 1);
                (high_byte as Word) << 8 | (low_byte as Word)
            }
            AddressingMode::IndexedIndirect => {
                let address = self.fetch_and_advance_pc();
                let address = address.wrapping_add(self.x) as Word;
                let low_byte = self.memory.read(address);
                let high_byte = self.memory.read(address + 1);
                (high_byte as Word) << 8 | (low_byte as Word)
            }
            AddressingMode::IndirectIndexed => {
                let address = self.fetch_and_advance_pc() as Word;
                let low_byte = self.memory.read(address);
                let high_byte = self.memory.read(address + 1);
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
