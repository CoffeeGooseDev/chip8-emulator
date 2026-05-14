use crate::{
    key_state::{KeyState, KeyStateView},
    keys::{Chip8Keys, Keys},
    memory::{Chip8Ram, Memory},
    screen::{Chip8Screen, Screen},
};

// u8 and u16 genereric indo data and adress?

/// The communication interface between the CPU and the system hardware.
///
/// The Bus mediates access to memory, display, and input devices,
/// ensuring that the CPU follows the correct hardware protocols.
pub(crate) trait Bus {
    /// Reads a byte of data from the system RAM at the specified address.
    /// Address is `u16` because Chip-8 addresses are 12-bit (0x000 - 0xFFF).
    fn fetch_memory(&self, addr: u16) -> u8;

    /// Writes a byte of data to the system RAM at the specified address.
    fn write_memory(&mut self, addr: u16, val: u8);

    /// Commands the display hardware to clear all pixels to the "off" state.
    fn clear_screen(&mut self);

    /// Checks the current state of a specific key on the 16-key hex keypad.
    /// `key_id` is a `u8` representing keys 0x0 through 0xF.
    fn is_key_pressed(&self, key_id: u8) -> bool;
}

/// The concrete implementation of the system bus for the Chip-8.
///
/// This struct uses short-lived mutable references to "plug in" the
/// various hardware components for the duration of a CPU cycle.
pub struct Chip8Bus<'a> {
    /// Reference to the 4KB system RAM.
    pub ram: &'a mut Chip8Ram,
    /// Reference to the display buffer.
    pub screen: &'a mut Chip8Screen,
    /// A read-only view of the keypad state.
    pub keys: KeyStateView<'a, Chip8Keys>,
}

impl<'a> Chip8Bus<'a> {
    /// Connects the various hardware components to create a functional communication path.
    ///
    /// Note: `keys_ref` is taken as a read-only reference because the CPU
    /// should not be able to "press" keys, only check their state.
    pub fn new(
        ram: &'a mut Chip8Ram,
        screen: &'a mut Chip8Screen,
        keys_ref: &'a Chip8Keys,
    ) -> Self {
        Self {
            ram,
            screen,
            keys: KeyStateView { inner: keys_ref },
        }
    }
}

impl<'a> Bus for Chip8Bus<'a> {
    fn fetch_memory(&self, addr: u16) -> u8 {
        self.ram.read(addr)
    }

    fn write_memory(&mut self, addr: u16, val: u8) {
        self.ram.write(addr, val);
    }

    fn clear_screen(&mut self) {
        self.screen.clear_screen();
    }

    fn is_key_pressed(&self, key_id: u8) -> bool {
        self.keys.is_pressed(key_id as usize)
    }
}
