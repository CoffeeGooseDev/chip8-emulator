use crate::Bus;
use crate::Cpu;

/// Represents the decoded Chip-8 instruction set.
///
/// `D` represents the Data type (typically u8) and `A` represents
/// the Address type (typically u16).
pub enum Instruction<D, A> {
    /// 0000 - No operation.
    Nop,
    /// 00E0 - CLS: Clears the display.
    ClearScreen,
    /// 00EE - RET: Returns from a subroutine.
    Return,
    /// 1nnn - JP addr: Jump to address `nnn`.
    Jump(A),
    /// 2nnn - CALL addr: Call subroutine at `nnn`.
    Call(A),
    /// 3xkk - SE Vx, byte: Skip next instruction if `Vx == kk`.
    SkipIfI(usize, D),
    /// 4xkk - SNE Vx, byte: Skip next instruction if `Vx != kk`.
    SkipIfNotI(usize, D),
    /// 5xy0 - SE Vx, Vy: Skip next instruction if `Vx == Vy`.
    SkipIfR(usize, usize),
    /// 6xkk - LD Vx, byte: Set `Vx = kk`.
    AsignI(usize, D),
    /// 7xkk - ADD Vx, byte: Set `Vx = Vx + kk`.
    AddI(usize, D),
    /// 8xy0 - LD Vx, Vy: Set `Vx = Vy`.
    AsignR(usize, usize),
    /// 8xy1 - OR Vx, Vy: Set `Vx = Vx OR Vy`.
    Or(usize, usize),

    /// 8xy4 - ADD Vx, Vy: Set `Vx = Vx + Vy`, set VF = carry.
    AddR(usize, usize),
    /// 8xy5 - SUB Vx, Vy: Set `Vx = Vx - Vy`, set VF = NOT borrow.
    SubR(usize, usize),
    SRL(usize, usize),
    SubRReversed(usize, usize),
    SLL(usize, usize),
    SkipIfNotR(usize, usize),
    IRegAsign(A),
    JumpV0(A),
    Rand(usize, D),
    /// Dxyb - DRW Vx, Vy, nibble: Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    DrawSprite(),
    SkipIfKeyPressed(usize),
    SkipIfKeyNotPressed(usize),
    SetRDt(usize),
    WaitKeyPress(usize),
    SetDtR(usize),
    SetStR(usize),
    IRegAddR(usize),
    IFontAddress(usize),
    IBCD(usize),
    StoreRtoI(usize),
    LoadItoR(usize),
}

impl Instruction<u8, u16> {
    /// Decodes a raw 16-bit opcode into a high-level [Instruction].
    ///
    /// This uses bit-masking to extract the instruction type and
    /// its arguments (x, y, kk, or nnn).
    pub fn decode(op: u16) -> Self {
        match (op & 0xF000) >> 12 {
            0x0 => match op & 0x00FF {
                0x00 => Instruction::Nop,
                0xE0 => Instruction::ClearScreen,
                0xEE => Instruction::Return,
                _ => unimplemented!("Unimplemented opcode: {}", op),
            },
            0x1 => Instruction::Jump(op & 0xFFF),
            0x2 => Instruction::Call(op & 0xFFF),
            0x3 => Instruction::SkipIfI(((op & 0xF00) >> 8) as usize, (op & 0xFF) as u8),
            0x4 => Instruction::SkipIfNotI(((op & 0xF00) >> 8) as usize, (op & 0xFF) as u8),
            0x5 => match op & 0xF {
                0x0 => {
                    Instruction::SkipIfR(((op & 0xF00) >> 8) as usize, ((op & 0xF0) >> 4) as usize)
                }
                _ => unimplemented!("Unimplemented opcode: {}", op),
            },
            0x6 => Instruction::AsignI(((op & 0xF00) >> 8) as usize, (op & 0xFF) as u8),
            0x7 => Instruction::AddI(((op & 0xF00) >> 8) as usize, (op & 0xFF) as u8),
            0x8 => match op & 0xF {
                0x0 => {
                    Instruction::AsignR(((op & 0xF00) >> 8) as usize, ((op & 0xF0) >> 4) as usize)
                }
                0x1 => {
                    return Instruction::Or(
                        ((op & 0xF00) >> 8) as usize,
                        ((op & 0xF0) >> 4) as usize,
                    );
                }
                0x2 => unimplemented!("Unimplemented opcode: {}", op),
                0x3 => unimplemented!("Unimplemented opcode: {}", op),
                0x4 => Instruction::AddR(((op & 0xF00) >> 8) as usize, ((op & 0xF0) >> 4) as usize),
                0x5 => Instruction::SubR(((op & 0xF00) >> 8) as usize, ((op & 0xF0) >> 4) as usize),
                0x6 => unimplemented!("Unimplemented opcode: {}", op),
                0x7 => unimplemented!("Unimplemented opcode: {}", op),
                0xE => unimplemented!("Unimplemented opcode: {}", op),
                _ => unimplemented!("Unimplemented opcode: {}", op),
            },
            // 0x9
            // 0xA
            // 0xC
            // 0xD
            // 0xE
            // 0xF
            _ => unimplemented!("Unimplemented opcode: {}", op),
        }
    }

    /// Performs the hardware state changes associated with the instruction.
    ///
    /// This method mediates between the [Cpu] and the [Bus] to modify registers,
    /// memory, or the display.
    pub fn execute<B: Bus, C: Cpu<Data = u8, Addr = u16>>(self, bus: &mut B, cpu: &mut C) {
        match self {
            Self::Nop => {}
            Self::ClearScreen => {
                bus.clear_screen();
            }
            Self::Return => {
                let return_addr = cpu.pop_stack();
                cpu.set_pc(return_addr);
            }
            Self::Jump(nnn) => {
                cpu.set_pc(nnn);
            }
            Self::Call(nnn) => {
                cpu.push_stack(cpu.get_pc());
                cpu.set_pc(nnn);
            }
            Self::SkipIfI(x, nn) => {
                if cpu.get_register(x) == nn {
                    cpu.advance_pc();
                }
            }
            Self::SkipIfNotI(x, nn) => {
                if cpu.get_register(x) != nn {
                    cpu.advance_pc();
                }
            }
            Self::SkipIfR(x, y) => {
                if cpu.get_register(x) == cpu.get_register(y) {
                    cpu.advance_pc();
                }
            }
            Self::AsignI(x, nn) => {
                cpu.asign_register(x, nn);
            }
            Self::AsignR(x, y) => cpu.asign_register(x, cpu.get_register(y)),
            Self::Or(x, y) => {
                cpu.asign_register(x, cpu.get_register(x) | cpu.get_register(y));
            }
            Self::AddR(x, y) => {
                cpu.asign_register(x, cpu.get_register(x) + cpu.get_register(y));
            }
            _ => unimplemented!("Unimplemented Instruction"),
        }
    }
}
