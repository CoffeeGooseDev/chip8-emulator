/// The hardware interface for the emulator's display.
///
/// This trait provides low-level access to the pixel buffer,
/// allowing the system to manipulate the 64x32 monochrome grid.
pub trait Screen {
    /// Resets the screen hardware to its initial state.
    fn reset(&mut self);
    /// Turns off every pixel on the display.
    fn clear_screen(&mut self);
    /// Returns the state of a pixel at the given (x, y) coordinates.
    fn get_pixel(&self, x: usize, y: usize) -> bool;
    /// Returns the resolution of the screen as (width, height).
    fn dimensions(&self) -> (usize, usize);
}

/// A concrete implementation of the original Chip-8 monochrome display.
///
/// The screen is 64 pixels wide and 32 pixels high. Pixel data is stored
/// in a flattened 1D array to optimize memory layout.
pub struct Chip8Screen {
    screen: [bool; 64 * 32],
}

impl Chip8Screen {
    pub fn new() -> Self {
        Self {
            screen: [false; 64 * 32],
        }
    }
}

impl Screen for Chip8Screen {
    /// Clears all internal pixel data.
    fn reset(&mut self) {
        self.screen = [false; 64 * 32];
    }

    /// Retrieves a pixel value from the flattened array.
    ///
    /// # formula
    /// `index = (y * 64) + x`
    fn get_pixel(&self, x: usize, y: usize) -> bool {
        unimplemented!("Unimplemented function")
    }

    /// Sets all pixels in the buffer to `false`.
    fn clear_screen(&mut self) {
        self.screen.fill(false);
    }

    /// Returns the standard Chip-8 resolution: 64x32.
    fn dimensions(&self) -> (usize, usize) {
        (64, 32)
    }
}
