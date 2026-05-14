use crate::screen::Screen;

/// A read-only interface for a display buffer.
///
/// This trait allows frontends to query pixel states and dimensions
/// without needing access to the underlying emulator state or hardware.
pub trait Frame {
    /// Returns `true` if the pixel at the given coordinates is "on".
    fn get_pixel(&self, x: usize, y: usize) -> bool;
    /// Returns the width and height of the display (typically 64x32).
    fn dimensions(&self) -> (usize, usize);
}

/// A lightweight wrapper that provides a [Frame] view of a screen.
///
/// `FrameView` uses a lifetime `'a` to ensure it cannot outlive the
/// hardware it is borrowing from. The `?Sized` bound allows this view
/// to wrap either a concrete screen or a `dyn Screen` trait object.
pub struct FrameView<'a, T: Screen + ?Sized> {
    inner: &'a T,
}

impl<'a, T: Screen + ?Sized> Frame for FrameView<'a, T> {
    /// Forwards the pixel query to the underlying screen hardware.
    fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.inner.get_pixel(x, y)
    }

    /// Forwards the dimension query to the underlying screen hardware.
    fn dimensions(&self) -> (usize, usize) {
        self.inner.dimensions()
    }
}

impl<'a, T: Screen + ?Sized> FrameView<'a, T> {
    /// Creates a new view from a reference to a type that implements [Screen].
    ///
    /// This is typically called by the [Emulator] to hand a temporary
    /// display handle to the rendering engine.
    pub(crate) fn new(inner: &'a T) -> Self {
        Self { inner }
    }
}
