use crate::Bus;
use crate::Cpu;
use rand::random;

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
    DrawSprite(usize, usize, usize),
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
        let digit1 = (op & 0xF000) >> 12;
        let digit2 = (op & 0x0F00) >> 8;
        let digit3 = (op & 0x00F0) >> 4;
        let digit4 = op & 0x000F;

        match (digit1, digit2, digit3, digit4) {
            (0, 0, 0, 0) => Instruction::Nop,
            (0, 0, 0xE, 0) => Instruction::ClearScreen,
            (0, 0, 0xE, 0xE) => Instruction::Return,
            (1, _, _, _) => Instruction::Jump(op & 0xFFF),
            (2, _, _, _) => Instruction::Call(op & 0xFFF),
            (3, _, _, _) => Instruction::SkipIfI(((op & 0xF00) >> 8) as usize, (op & 0xFF) as u8),
            (4, _, _, _) => {
                Instruction::SkipIfNotI(((op & 0xF00) >> 8) as usize, (op & 0xFF) as u8)
            }
            (5, _, _, 0) => {
                Instruction::SkipIfR(((op & 0xF00) >> 8) as usize, ((op & 0xF0) >> 4) as usize)
            }
            (6, _, _, _) => Instruction::AsignI(((op & 0xF00) >> 8) as usize, (op & 0xFF) as u8),
            (7, _, _, _) => Instruction::AddI(((op & 0xF00) >> 8) as usize, (op & 0xFF) as u8),
            (8, _, _, 0) => {
                Instruction::AsignR(((op & 0xF00) >> 8) as usize, ((op & 0xF0) >> 4) as usize)
            }
            (8, _, _, 1) => {
                Instruction::Or(((op & 0xF00) >> 8) as usize, ((op & 0xF0) >> 4) as usize)
            }
            (8, _, _, 2) => unimplemented!("Unimplemented opcode: {}", op),
            (8, _, _, 3) => unimplemented!("Unimplemented opcode: {}", op),
            (8, _, _, 4) => {
                Instruction::AddR(((op & 0xF00) >> 8) as usize, ((op & 0xF0) >> 4) as usize)
            }
            (8, _, _, 5) => {
                Instruction::SubR(((op & 0xF00) >> 8) as usize, ((op & 0xF0) >> 4) as usize)
            }
            (8, _, _, 6) => {
                Instruction::SRL(((op & 0xF00) >> 8) as usize, ((op & 0xF0) >> 4) as usize)
            }
            (8, _, _, 7) => {
                Instruction::SubRReversed(((op & 0xF00) >> 8) as usize, ((op & 0xF0) >> 4) as usize)
            }
            (8, _, _, 0xE) => {
                Instruction::SLL(((op & 0xF00) >> 8) as usize, ((op & 0xF0) >> 4) as usize)
            }
            (9, _, _, 0) => {
                Instruction::SkipIfNotR(((op & 0xF00) >> 8) as usize, ((op & 0xF0) >> 4) as usize)
            }
            (0xA, _, _, _) => Instruction::IRegAsign(op & 0xFFF),
            (0xB, _, _, _) => Instruction::JumpV0(op & 0xFFF),
            (0xC, _, _, _) => Instruction::Rand(((op & 0xF00) >> 8) as usize, (op & 0xFF) as u8),
            (0xD, _, _, _) => Instruction::DrawSprite(
                ((op & 0xF00) >> 8) as usize,
                ((op & 0xF0) >> 4) as usize,
                (op & 0xF) as usize,
            ),
            (0xE, _, 9, 0xE) => Instruction::SkipIfKeyPressed(((op & 0xF00) >> 8) as usize),
            (0xE, _, 0xA, 1) => Instruction::SkipIfKeyNotPressed(((op & 0xF00) >> 8) as usize),
            (0xF, _, 0, 7) => Instruction::SetRDt(((op & 0xF00) >> 8) as usize),
            (0xF, _, 0, 0xA) => Instruction::WaitKeyPress(((op & 0xF00) >> 8) as usize),
            (0xF, _, 1, 5) => Instruction::SetDtR(((op & 0xF00) >> 8) as usize),
            (0xF, _, 1, 8) => Instruction::SetStR(((op & 0xF00) >> 8) as usize),
            (0xF, _, 1, 0xE) => Instruction::IRegAddR(((op & 0xF00) >> 8) as usize),
            (0xF, _, 2, 9) => Instruction::IFontAddress(((op & 0xF00) >> 8) as usize),
            (0xF, _, 3, 3) => Instruction::IBCD(((op & 0xF00) >> 8) as usize),
            (0xF, _, 5, 5) => Instruction::StoreRtoI(((op & 0xF00) >> 8) as usize),
            (0xF, _, 6, 5) => Instruction::LoadItoR(((op & 0xF00) >> 8) as usize),
            (_, _, _, _) => unimplemented!("No instruction for opcode: {}", op),
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
            Self::AddI(x, nn) => {
                let new_vx = cpu.get_register(x).wrapping_add(nn);
                cpu.asign_register(x, new_vx);
            }
            Self::AsignR(x, y) => cpu.asign_register(x, cpu.get_register(y)),
            Self::Or(x, y) => {
                cpu.asign_register(x, cpu.get_register(x) | cpu.get_register(y));
            }
            // TODO bit operation
            // TODO bit operation
            Self::AddR(x, y) => {
                cpu.asign_register(x, cpu.get_register(x) + cpu.get_register(y));
            }
            Self::AddR(x, y) => {
                let (new_vx, carry) = cpu.get_register(x).overflowing_add(cpu.get_register(y));
                let new_vf = if carry { 1 } else { 0 };
                cpu.asign_register(x, new_vx);
                cpu.asign_register(0xF, new_vf);
            }
            Self::SubR(x, y) => {
                let (new_vx, borrow) = cpu.get_register(x).overflowing_sub(cpu.get_register(y));
                let new_vf = if borrow { 0 } else { 1 };
                cpu.asign_register(x, new_vx);
                cpu.asign_register(0xF, new_vf);
            }
            Self::SRL(x, y) => {
                let current_vx = cpu.get_register(x);
                let lsb = current_vx & 1;
                let new_vx = current_vx >> 1;
                cpu.asign_register(x, new_vx);
                cpu.asign_register(0xF, lsb);
            }
            Self::SubRReversed(x, y) => {
                let (new_vx, borrow) = cpu.get_register(y).overflowing_sub(cpu.get_register(y));
                let new_vf = if borrow { 0 } else { 1 };
                cpu.asign_register(x, new_vx);
                cpu.asign_register(0xF, new_vf);
            }
            Self::SLL(x, y) => {
                let current_vx = cpu.get_register(x);
                let msb = (current_vx >> 7) & 1;
                let new_vx = current_vx << 1;
                cpu.asign_register(x, new_vx);
                cpu.asign_register(0xF, msb);
            }
            Self::SkipIfNotR(x, y) => {
                if cpu.get_register(x) != cpu.get_register(y) {
                    cpu.advance_pc();
                }
            }
            Self::IRegAsign(nnn) => cpu.set_i_register(nnn),
            Self::JumpV0(nnn) => cpu.set_pc(cpu.get_register(0) as u16 + nnn),
            Self::Rand(x, nn) => {
                let rng: u8 = random();
                cpu.asign_register(x, rng & nn);
            }
            Self::DrawSprite(usize, usize, usize) => {}
            Self::SkipIfKeyPressed(x) => {
                // TODO
            }
            Self::SkipIfKeyNotPressed(x) => {
                // TODO
            }
            Self::SetRDt(x) => {
                // TODO
            }
            Self::WaitKeyPress(x) => {
                // TODO
            }
            Self::SetDtR(x) => {
                let c = cpu.get_register(x);
                cpu.set_dt_register(c)
            }
            Self::SetStR(x) => {
                // TODO
            }
            Self::IRegAddR(x) => {
                // TODO
            }
            Self::IFontAddress(x) => {
                let c = cpu.get_register(x);
                cpu.set_i_register(c * 5);
            }
            Self::IBCD(x) => {
                // TODO
            }
            Self::StoreRtoI(x) => {
                // TODO
            }
            Self::LoadItoR(x) => {
                // TODO
            }
        }
    }
}
