use crate::keys::Keys;

/// A write-only interface for injecting user input into the emulator.
///
/// This trait abstracts over the keyboard hardware, allowing the frontend
/// to toggle key states without needing to know the underlying implementation.
pub trait Input {
    /// The type used to identify keys (typically a numeric index or enum).
    type KeyId: ?Sized;

    /// Sets the state of a specific key.
    ///
    /// # Arguments
    /// * `id` - The identifier for the key (0x0 to 0xF for Chip-8).
    /// * `pressed` - `true` if the key is down, `false` if it is up.
    fn set_key(&mut self, id: Self::KeyId, pressed: bool)
    where
        Self::KeyId: Sized;
}

/// A mutable handle that provides an [Input] interface to the keypad hardware.
///
/// `InputHandle` holds a unique mutable reference to the keys, ensuring that
/// only one part of the system (typically the UI thread) can modify input
/// state at a time.
pub struct InputHandle<'a, T: ?Sized> {
    /// Mutable reference to the internal keypad state.
    pub(crate) inner: &'a mut T,
}

impl<'a, T: Keys + ?Sized> Input for InputHandle<'a, T> {
    type KeyId = T::KeyId;

    /// Forwards the key event directly to the hardware's `set_key` implementation.
    fn set_key(&mut self, id: Self::KeyId, pressed: bool)
    where
        Self::KeyId: Sized,
    {
        self.inner.set_key(id, pressed);
    }
}

impl<'a, T: ?Sized> InputHandle<'a, T> {
    /// Creates a new input handle from a mutable reference to the keypad hardware.
    ///
    /// This is used by the [Emulator] to provide the frontend with a way
    /// to update key states during the frame cycle.
    pub fn new(keys: &'a mut T) -> Self {
        Self { inner: keys }
    }
}
