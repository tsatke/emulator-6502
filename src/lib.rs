pub mod cpu;
pub mod mem;
pub mod opcode;

#[cfg(test)]
mod tests {
    use crate::cpu::{Cpu, ProcessorStatus, CODE_START};
    use crate::mem::Memory;

    fn run_code(code: &[u8], instruction_count: usize) -> Cpu {
        let mut mem = Memory::new();

        code.iter().enumerate().for_each(|(i, &b)| {
            mem[CODE_START as usize + i] = b;
        });

        let mut cpu = Cpu::new(mem);
        assert_eq!(cpu.pc, CODE_START);
        assert_eq!(cpu.sp, 0x0100);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.status, ProcessorStatus::empty());

        cpu.run(Some(instruction_count));
        cpu
    }

    #[test]
    fn test_lda() {
        let state = run_code(
            &[
                0xA9, 0x11, // LDA #0x11
            ],
            1,
        );
        assert_eq!(state.pc, CODE_START + 2);
        assert_eq!(state.a, 0x11);
        assert_eq!(state.status, ProcessorStatus::empty());
    }

    #[test]
    fn test_lda_zero() {
        let state = run_code(
            &[
                0xA9, 0x0, // LDA #0x0
            ],
            1,
        );
        assert_eq!(state.pc, CODE_START + 2);
        assert_eq!(state.a, 0x0);
        assert_eq!(state.status, ProcessorStatus::Zero);
    }

    #[test]
    fn test_lda_neg() {
        let state = run_code(
            &[
                0xA9, 0xFF, // LDA #0xFF
            ],
            1,
        );
        assert_eq!(state.pc, CODE_START + 2);
        assert_eq!(state.a, 0xFF);
        assert_eq!(state.status, ProcessorStatus::Negative);
    }

    #[test]
    fn test_ldx() {
        let state = run_code(
            &[
                0xA2, 0x11, // LDX #0x11
            ],
            1,
        );
        assert_eq!(state.pc, CODE_START + 2);
        assert_eq!(state.x, 0x11);
    }

    #[test]
    fn test_ldy() {
        let state = run_code(
            &[
                0xA0, 0x11, // LDY #0x11
            ],
            1,
        );
        assert_eq!(state.pc, CODE_START + 2);
        assert_eq!(state.y, 0x11);
    }
}
