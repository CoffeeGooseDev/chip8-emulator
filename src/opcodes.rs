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

        let nnn = op & 0xFFF;
        let nn = (op & 0xFF) as u8;
        let x = ((op & 0xF00) >> 8) as usize;
        let y = ((op & 0xF0) >> 4) as usize;
        let z = (op & 0xF) as usize;

        match (digit1, digit2, digit3, digit4) {
            (0, 0, 0, 0) => Instruction::Nop,
            (0, 0, 0xE, 0) => Instruction::ClearScreen,
            (0, 0, 0xE, 0xE) => Instruction::Return,
            (1, _, _, _) => Instruction::Jump(nnn),
            (2, _, _, _) => Instruction::Call(nnn),
            (3, _, _, _) => Instruction::SkipIfI(x, nn),
            (4, _, _, _) => Instruction::SkipIfNotI(x, nn),
            (5, _, _, 0) => Instruction::SkipIfR(x, y),
            (6, _, _, _) => Instruction::AsignI(x, nn),
            (7, _, _, _) => Instruction::AddI(x, nn),
            (8, _, _, 0) => Instruction::AsignR(x, y),
            (8, _, _, 1) => Instruction::Or(x, y),
            (8, _, _, 2) => unimplemented!("Unimplemented opcode: {}", op),
            (8, _, _, 3) => unimplemented!("Unimplemented opcode: {}", op),
            (8, _, _, 4) => Instruction::AddR(x, y),
            (8, _, _, 5) => Instruction::SubR(x, y),
            (8, _, _, 6) => Instruction::SRL(x, y),
            (8, _, _, 7) => Instruction::SubRReversed(x, y),
            (8, _, _, 0xE) => Instruction::SLL(x, y),
            (9, _, _, 0) => Instruction::SkipIfNotR(x, y),
            (0xA, _, _, _) => Instruction::IRegAsign(nnn),
            (0xB, _, _, _) => Instruction::JumpV0(nnn),
            (0xC, _, _, _) => Instruction::Rand(x, nn),
            (0xD, _, _, _) => Instruction::DrawSprite(x, y, z),
            (0xE, _, 9, 0xE) => Instruction::SkipIfKeyPressed(x),
            (0xE, _, 0xA, 1) => Instruction::SkipIfKeyNotPressed(x),
            (0xF, _, 0, 7) => Instruction::SetRDt(x),
            (0xF, _, 0, 0xA) => Instruction::WaitKeyPress(x),
            (0xF, _, 1, 5) => Instruction::SetDtR(x),
            (0xF, _, 1, 8) => Instruction::SetStR(x),
            (0xF, _, 1, 0xE) => Instruction::IRegAddR(x),
            (0xF, _, 2, 9) => Instruction::IFontAddress(x),
            (0xF, _, 3, 3) => Instruction::IBCD(x),
            (0xF, _, 5, 5) => Instruction::StoreRtoI(x),
            (0xF, _, 6, 5) => Instruction::LoadItoR(x),
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
            Self::ClearScreen => bus.clear_screen(),
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
            Self::AsignI(x, nn) => cpu.asign_register(x, nn),
            Self::AddI(x, nn) => {
                let new_vx = cpu.get_register(x).wrapping_add(nn);
                cpu.asign_register(x, new_vx);
            }
            Self::AsignR(x, y) => cpu.asign_register(x, cpu.get_register(y)),
            Self::Or(x, y) => cpu.asign_register(x, cpu.get_register(x) | cpu.get_register(y)),
            // TODO bit operation
            // TODO bit operation
            Self::AddR(x, y) => cpu.asign_register(x, cpu.get_register(x) + cpu.get_register(y)),
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
            Self::DrawSprite(x, y, z) => {
                // Get the (x, y) coords for our sprite
                let x_coord = cpu.get_register(x);
                let y_coord = cpu.get_register(y);
                // The last digit determines how many rows high our sprite is
                let num_rows = z;

                // Keep track if any pixels were flipped
                let mut flipped = false;
                // Iterate over each row of our sprite
                for y_line in 0..num_rows {
                    // Determine which memory address our row's data is stored
                    let addr = cpu.get_i_register() + y_line as u16;
                    let pixels = bus.fetch_memory(addr as usize);

                    // Iterate over each column in our row
                    for x_line in 0..8 {
                        // Use a mask to fetch current pixel's bit. Only flip if a 1
                        if (pixels & (0b1000_0000 >> x_line)) != 0 {
                            // TODO change SCREEN_WIDTH for something else
                            // Sprites should wrap around screen, so apply modulo
                            let x = (x_coord + x_line) as usize % SCREEN_WIDTH;
                            let y = (y_coord + y_line) as usize % SCREEN_HEIGHT;
                            // Get our pixel's index for our 1D screen array
                            let idx = x + SCREEN_WIDTH * y;
                            let pixel = bus.get_pixel(idx);
                            // Check if we're about to flip the pixel and set
                            flipped |= pixel;
                            bus.set_pixel(idx, pixel ^ true);
                        }
                    }
                }
                // Populate VF register
                if flipped {
                    cpu.asign_register(0xF, 1);
                } else {
                    cpu.asign_register(0xF, 0);
                }
            }
            Self::SkipIfKeyPressed(x) => {
                let vx = cpu.get_register(x);
                let key = bus.is_key_pressed(vx as usize);
                if key {
                    cpu.advance_pc();
                }
            }
            Self::SkipIfKeyNotPressed(x) => {
                let vx = cpu.get_register(x);
                let key = bus.is_key_pressed(vx as usize);
                if !key {
                    cpu.advance_pc();
                }
            }
            Self::SetRDt(x) => {
                cpu.asign_register(x, cpu.get_dt_register());
            }
            Self::WaitKeyPress(x) => {
                let mut pressed = false;
                // get number of keys
                for i in 0..self.keys.len() {
                    if bus.is_key_pressed(i) {
                        cpu.asign_register(x, i as u8);
                        pressed = true;
                        break;
                    }
                }
                if !pressed {
                    // Redo opcode
                    let current_pc = cpu.get_pc();
                    cpu.set_pc(current_pc - 2);
                }
            }
            Self::SetDtR(x) => {
                let c = cpu.get_register(x);
                cpu.set_dt_register(c);
            }
            Self::SetStR(x) => {
                let c = cpu.get_register(x);
                cpu.set_st_register(c);
            }
            Self::IRegAddR(x) => {
                cpu.set_i_register(cpu.get_i_register().wrapping_add(cpu.get_register(x)))
            }
            Self::IFontAddress(x) => {
                let c = cpu.get_register(x);
                cpu.set_i_register(c * 5);
            }
            Self::IBCD(x) => {
                let vx = cpu.get_register(x) as f32;
                // Fetch the hundreds digit by dividing by 100 and tossing the decimal
                let hundreds = (vx / 100.0).floor() as u8;
                // Fetch the tens digit by dividing by 10, tossing the ones digit and the decimal
                let tens = ((vx / 10.0) % 10.0).floor() as u8;
                // Fetch the ones digit by tossing the hundreds and the tens
                let ones = (vx % 10.0) as u8;
                let i_reg = cpu.get_i_register();
                bus.write_memory(i_reg as usize, hundreds);
                bus.write_memory((i_reg + 1) as usize, tens);
                bus.write_memory((i_reg + 2) as usize, ones);
            }
            Self::StoreRtoI(x) => {
                let i = cpu.get_i_register() as usize;
                for idx in 0..=x {
                    let x_reg = cpu.get_register(idx);
                    bus.write_memory(i + idx, x_reg);
                }
            }
            Self::LoadItoR(x) => {
                let i = cpu.get_i_register() as usize;
                for idx in 0..=x {
                    let i_reg = bus.fetch_memory(i + idx);
                    cpu.asign_register(idx, i_reg);
                }
            }
        }
    }
}
