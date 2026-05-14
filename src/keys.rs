/// The fundamental hardware interface for a keypad device.
///
/// This trait defines both the ability to read and write key states,
/// which is later split into restricted views ([Input] and [KeyState]).
pub trait Keys {
    /// The type used to index or identify specific keys.
    type KeyId: ?Sized;

    /// Clears all key states, effectively releasing all buttons.
    fn reset(&mut self);
    /// Returns the current state of a key (true for pressed).
    fn is_key_pressed(&self, ident: Self::KeyId) -> bool;
    /// Directly updates the state of a key.
    fn set_key(&mut self, ident: Self::KeyId, pressed: bool);
}

/// The standard Chip-8 hexadecimal keypad.
///
/// This hardware consists of 16 keys, traditionally mapped to
/// a 4x4 grid. The state is stored as a simple boolean array.
pub struct Chip8Keys {
    /// The state of the 16 keys (0x0 through 0xF).
    keys: [bool; 16],
}

impl Chip8Keys {
    /// Creates a new keypad with all keys in the released (false) state.
    pub fn new() -> Self {
        Self { keys: [false; 16] }
    }
}

impl Keys for Chip8Keys {
    /// Uses a simple index-based identifier for keys.
    type KeyId = usize;

    /// Re-initializes the key array to all `false`.
    fn reset(&mut self) {
        self.keys = [false; 16];
    }

    /// Accesses the internal array to check a key's state.
    fn is_key_pressed(&self, ident: Self::KeyId) -> bool {
        self.keys[ident]
    }

    /// Updates the internal array for a specific key index.
    fn set_key(&mut self, ident: Self::KeyId, pressed: bool) {
        self.keys[ident] = pressed;
    }
}
