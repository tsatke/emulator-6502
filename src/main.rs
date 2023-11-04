use emulator_6502::cpu::{Cpu, CODE_START};
use emulator_6502::mem::Memory;

fn main() {
    let mut mem = Memory::new();

    [
        0xA9, 0x11, // LDA #0x11
        0xA2, 0x20, // LDX #0x20
        0xA0, 0x40, // LDY #0x40
        0x4A, // LSR A
    ]
    .into_iter()
    .enumerate()
    .for_each(|(i, b)| {
        mem[CODE_START as usize + i] = b;
    });

    let mut cpu = Cpu::new(mem);
    cpu.run(Some(3));
    println!("{:#X?}", cpu);
    cpu.run(Some(1));
    println!("{:#X?}", cpu);
}
