//! # Chip-8 Core
//!
//! This crate provides a high-performance, platform-agnostic Chip-8 emulator core.
//! It uses a trait-based hardware abstraction layer (HAL) to allow easy integration
//! with various frontends (SDL2, WebAssembly, etc.).

mod bus;
mod cpu;
mod frame;
mod input;
mod key_state;
mod keys;
mod memory;
mod opcodes;
mod screen;

use crate::{
    bus::{Bus, Chip8Bus},
    cpu::{Chip8Cpu, Cpu},
    frame::{Frame, FrameView},
    input::{Input, InputHandle},
    key_state::KeyStateView,
    keys::{Chip8Keys, Keys},
    memory::Chip8Ram,
    memory::Memory,
    opcodes::Instruction,
    screen::Chip8Screen,
    screen::Screen,
};

/// This represents the main Chip-8 system
///
/// # The Life Cycle
/// A single [Emulator::tick] executes one full instruction cycle:
/// 1. **Fetch**: Read the opcode from memory.
/// 2. **Decode**: Interpret the instruction.
/// 3. **Execute**: Perform the operation via the internal bus.
pub trait Emulator {
    /// A read-only view of the display buffer.
    type FrameType<'a>: Frame
    where
        Self: 'a;

    /// A handle to send input events (keypresses) to the emulator.
    type InputType<'a>: Input
    where
        Self: 'a;

    /// Creates a new emulator instance with cleared memory and registers.
    fn new() -> Self;

    /// Hard-resets the CPU, RAM, display, and key states.
    fn reset(&mut self);

    /// Advances the system clock by one cycle.
    ///
    /// This includes instruction execution and internal timer updates.
    fn tick(&mut self);

    //fn turn_on();
    // Accessors for the Frontend

    /// Returns a wrapper to access current pixel data.
    ///
    /// The returned view borrows from the emulator, ensuring data consistency
    /// during a frame render.
    fn get_frame(&self) -> Self::FrameType<'_>;

    /// Returns a handle to modify key states.
    fn get_input(&mut self) -> Self::InputType<'_>;
    // Change into giving a wrapper for external input?
}

pub struct Chip8Emu {
    cpu: Chip8Cpu,
    ram: Chip8Ram,
    screen: Chip8Screen,
    keys: Chip8Keys,
}

impl Emulator for Chip8Emu {
    type FrameType<'a>
        = FrameView<'a, dyn Screen + 'a>
    // Added + 'a here
    where
        Self: 'a;

    type InputType<'a>
        = InputHandle<'a, Chip8Keys>
    where
        Self: 'a;

    fn get_input(&mut self) -> Self::InputType<'_> {
        InputHandle::new(&mut self.keys)
    }

    fn get_frame(&self) -> Self::FrameType<'_> {
        // 1. Don't use 'as'. Use an explicit type annotation.
        // This tells Rust to perform "Unsize Coercion" rather than a "Cast".
        let screen_dyn: &dyn Screen = &self.screen;

        // 2. Wrap it in your view
        FrameView::new(screen_dyn)
    }

    fn new() -> Self {
        Self {
            cpu: Chip8Cpu::new(),
            ram: Chip8Ram::new(),
            screen: Chip8Screen::new(),
            keys: Chip8Keys::new(),
        }
    }

    fn reset(&mut self) {
        self.cpu.reset();
        self.ram.reset();
        self.screen.reset();
        self.keys.reset();
    }

    fn tick(&mut self) {
        // We use a 'Bus' pattern here to satisfy the borrow checker.
        // The Bus borrows components from 'self' and presents them to the CPU.
        let mut bus = Chip8Bus {
            ram: &mut self.ram,                       // Bus borrows RAM
            screen: &mut self.screen,                 // Bus borrows Screen
            keys: KeyStateView { inner: &self.keys }, // Bus borrows Keys (Read-only)
        };
        // Fetch
        let op = self.cpu.fetch(&bus);
        // Decode & Execute
        let instr = Instruction::decode(op);
        // Execute
        instr.execute(&mut bus, &mut self.cpu);

        // Only tick timers if a certain amount of time has passed
        // or every Nth instruction.
        self.cpu.tick_timers();
    }
}
