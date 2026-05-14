/// Defines the behavior for a Chip-8 addressable memory space.
pub(crate) trait Memory {
    /// The total capacity of the memory in bytes.
    const RAM_SIZE: usize;

    /// Creates a new memory instance, typically initialized to zero.
    fn new() -> Self;

    /// Reads a single byte from the specified 16-bit address.
    ///
    /// # Panics
    /// May panic if `addr` exceeds [Self::RAM_SIZE].
    fn read(&self, addr: u16) -> u8;

    /// Writes a single byte to the specified 16-bit address.
    fn write(&mut self, addr: u16, val: u8);

    /// Clears all memory back to its initial state.
    fn reset(&mut self);
}

// There isn't a standard on how many memory Chip-8 should have but it was designed to be implemented
// on computers with 4KB of ram

/// A 4KB contiguous block of RAM representing the Chip-8 memory space.
///
/// # Memory Map
/// * `0x000 - 0x1FF`: Reserved for the interpreter (historically held the fontset).
/// * `0x200 - 0xFFF`: Program ROM and work RAM.
pub(crate) struct Chip8Ram {
    storage: [u8; Self::RAM_SIZE],
}

impl Memory for Chip8Ram {
    /// The standard 4096 bytes of memory for a Chip-8 system.
    const RAM_SIZE: usize = 4096;

    /// Initializes RAM and loads the system fontset into the interpreter reserved area.
    fn new() -> Self {
        let mut ram = Self {
            storage: [0; Self::RAM_SIZE],
        };
        // ram.load_fontset(); // Future implementation
        ram
    }

    /// Reads a byte from the specified memory address.
    ///
    /// # Panics
    /// This implementation currently panics if the address is outside
    /// the range `0x000` to `0xFFF`.
    fn read(&self, addr: u16) -> u8 {
        self.storage[addr as usize]
    }

    /// Writes a byte to the specified memory address.
    ///
    /// # Safety/Constraints
    /// While the Chip-8 has no memory protection, care should be taken
    /// not to overwrite the fontset stored in the `0x000-0x1FF` range
    /// during runtime.
    fn write(&mut self, addr: u16, val: u8) {
        self.storage[addr as usize] = val;
    }

    /// Clears all RAM to zero and reloads the system fontset.
    ///
    /// This is used to return the memory state to the "factory default"
    /// before a new program is loaded.
    fn reset(&mut self) {
        self.storage = [0; Self::RAM_SIZE];
        // TODO: Re-copy FONTSET here to ensure the interpreter
        // area is restored for programs that rely on it.
    }
}

// TODO FONTSET DATA:
//
//
// ram initialization
// ram: [0; Self::RAM_SIZE],
// new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
//
// Ram reset
// self.ram = [0; Self::RAM_SIZE];
// self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);

// 3. Include your fontset data
//const FONTSET: &[u8] = include_bytes!("fontset.bin");

//const FONTSET_SIZE: usize = 80;
