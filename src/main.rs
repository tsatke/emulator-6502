use emulator_6502::cpu::{Cpu, CODE_START};
use emulator_6502::mem::Memory;

fn main() {
    let mut mem = Memory::new();

    [
        0xA9, 0x01, // LDA #0x01
        0x20, 0x08, 0xC0, // JSR 0xC008
        0x4C, 0x0B, 0xC0, // JMP 0xC00B
        0xA2, 0x02, // LDX #0x02
        0x60, // RTS
        0xA0, 0x03, // LDY #0x03
    ]
    .into_iter()
    .enumerate()
    .for_each(|(i, b)| {
        mem[CODE_START as usize + i] = b;
    });

    let mut cpu = Cpu::new(mem);
    cpu.run(Some(6));
}
