use crate::keys::Keys;

/// An internal interface for querying the current state of the keypad.
///
/// Unlike the public `Input` trait, `KeyState` is designed for the
/// emulator's internal components (like the CPU) to check which
/// keys are currently active.
pub(crate) trait KeyState {
    /// The identifier type for keys, typically a numeric index.
    type KeyId: ?Sized;

    /// Returns `true` if the specified key is currently being held down.
    ///
    /// # Constraints
    /// The `KeyId` must be `Sized` to be passed into this method.
    fn is_pressed(&self, id: Self::KeyId) -> bool
    where
        Self::KeyId: Sized;
}

/// A read-only snapshot-like view of the keypad hardware.
///
/// This struct is typically used by the [Bus] to grant the CPU
/// restricted, read-only access to the keyboard state during
/// instruction execution.
pub struct KeyStateView<'a, T: Keys + ?Sized> {
    /// Reference to the underlying keypad hardware.
    pub(crate) inner: &'a T,
}

impl<'a, T: Keys + ?Sized> KeyState for KeyStateView<'a, T> {
    type KeyId = T::KeyId;

    /// Queries the underlying hardware to see if a specific key is pressed.
    fn is_pressed(&self, id: Self::KeyId) -> bool
    where
        Self::KeyId: Sized,
    {
        self.inner.is_key_pressed(id)
    }
}
