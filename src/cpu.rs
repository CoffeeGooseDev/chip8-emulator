use crate::Bus;

/// Defines the core requirements for a processor unit within the emulator.
///
/// This trait uses Associated Types to remain flexible across different
/// bit-widths (e.g., 8-bit data vs 16-bit addresses).
pub(crate) trait Cpu {
    /// The primary data type the CPU operates on (typically `u8`).
    type Data;
    /// The address type for memory mapping (typically `u16`).
    type Addr;

    /// The starting memory address where programs are loaded.
    const START_ADDR: u16;
    /// The total number of general-purpose registers available.
    const NUM_REGS: usize;

    /// Creates a new CPU with default register and pointer states.
    fn new() -> Self;

    /// Resets the CPU to its power-on state, including the Program Counter.
    fn reset(&mut self);

    /// Fetches the next instruction from memory via the [Bus].
    ///
    /// Standard Chip-8 instructions are 2 bytes wide. This method
    /// also handles the automatic advancement of the Program Counter.
    fn fetch<T: Bus>(&mut self, bus: &T) -> Self::Addr;

    /// Decrements the internal hardware timers (Delay and Sound) if they are non-zero.
    fn tick_timers(&mut self);

    // --- Stack Operations ---
    fn pop_stack(&mut self) -> Self::Addr;
    fn push_stack(&mut self, val: Self::Addr);

    // --- Register Access ---
    fn get_register(&self, r_num: usize) -> Self::Data;
    fn asign_register(&mut self, r_num: usize, val: Self::Data);

    // --- Program Counter Control ---
    fn get_pc(&self) -> Self::Addr;
    fn set_pc(&mut self, val: Self::Addr);
    fn advance_pc(&mut self);
}

/// A concrete implementation of a Chip-8 CPU.
///
/// Includes sixteen 8-bit general-purpose registers, a 16-bit index register,
/// and hardware timers for delay and sound.
pub(crate) struct Chip8Cpu {
    /// Program Counter: Points to the current instruction in RAM.
    pub(crate) pc: u16,
    /// General purpose registers (V0 through VF).
    pub(crate) v_reg: [u8; Self::NUM_REGS],
    /// Index Register (I): Used to store memory addresses for buffer operations.
    pub(crate) i_reg: u16,
    /// The subroutine call stack.
    pub(crate) stack: Stack,
    /// Delay Timer: Automatically decrements at 60Hz.
    pub(crate) dt: u8,
    /// Sound Timer: Automatically decrements at 60Hz; triggers a beep while > 0.
    pub(crate) st: u8,
}

impl Cpu for Chip8Cpu {
    type Data = u8;
    type Addr = u16;

    const START_ADDR: u16 = 0x200;
    const NUM_REGS: usize = 16;

    fn new() -> Self {
        Self {
            pc: Self::START_ADDR,

            v_reg: [0; Self::NUM_REGS],
            i_reg: 0,

            stack: Stack::new(),

            dt: 0,
            st: 0,
        }
    }

    fn reset(&mut self) {
        self.pc = Self::START_ADDR;

        self.v_reg = [0; Self::NUM_REGS];
        self.i_reg = 0;
        self.stack.reset();

        self.dt = 0;
        self.st = 0;
    }

    // Attempting to pop an empty stack will result in underflow panic
    // Add extra handling if you want

    fn fetch<T: Bus>(&mut self, bus: &T) -> Self::Addr {
        let high = bus.fetch_memory(self.pc);
        let low = bus.fetch_memory(self.pc + 1);

        self.advance_pc();
        (high as u16) << 8 | (low as u16)
    }

    fn tick_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                // BEEP
            }
            self.st -= 1;
        }
    }

    fn pop_stack(&mut self) -> Self::Addr {
        self.stack.pop()
    }

    fn push_stack(&mut self, val: Self::Addr) {
        self.stack.push(val);
    }

    fn get_register(&self, r_num: usize) -> Self::Data {
        self.v_reg[r_num]
    }

    fn asign_register(&mut self, r_num: usize, val: Self::Data) {
        self.v_reg[r_num] = val;
    }

    fn get_pc(&self) -> Self::Addr {
        self.pc
    }

    fn set_pc(&mut self, val: Self::Addr) {
        self.pc = val;
    }

    fn advance_pc(&mut self) {
        self.set_pc(self.get_pc() + 2);
    }
}

/// A fixed-size Last-In-First-Out (LIFO) storage for return addresses.
///
/// Used primarily for handling subroutine calls (CALL) and returns (RET).
pub(crate) struct Stack {
    /// Stack Pointer: Points to the next available slot in the stack.
    sp: u16,
    /// Internal storage for return addresses.
    stack: [u16; Self::STACK_SIZE],
}

impl Stack {
    /// The standard capacity for a Chip-8 stack (16 levels of nesting).
    pub const STACK_SIZE: usize = 16;

    pub fn new() -> Self {
        Self {
            sp: 0,
            stack: [0; Self::STACK_SIZE],
        }
    }

    pub fn reset(&mut self) {
        self.sp = 0;
        self.stack = [0; Self::STACK_SIZE];
    }

    /// Pushes a return address onto the stack.
    ///
    /// # Panics
    /// Will panic if the stack is full (Stack Overflow).
    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    /// Pops the top address from the stack.
    ///
    /// # Panics
    /// Will panic if attempting to pop from an empty stack (Stack Underflow).
    fn pop(&mut self) -> u16 {
        if self.sp == 0 {
            panic!("Stack Underflow!");
        }

        self.sp -= 1;
        self.stack[self.sp as usize]
    }
}
