# Chip8-Core

A headless, platform-agnostic Chip-8 emulation engine written in Rust.

## 🏛️ Architecture & Design
Unlike monolithic emulators, **Chip8-Core** is designed as a modular library. It provides the "brain" and "memory" of a Chip-8 system while remaining entirely decoupled from rendering or input APIs.

### The "Bus" Pattern
The core of this library utilizes a **Bus-mediated architecture**. This ensures that the CPU never has direct, unsafe access to hardware components. Instead, it communicates through a `Bus` trait, which coordinates data movement between:
* **RAM**: 4KB of addressable memory.
* **Display**: A 64x32 monochrome buffer.
* **Keypad**: A 16-key hex-input system.



### Memory Safety & Abstraction
This library makes heavy use of Rust's advanced features to ensure safety:
* **Trait-Based Hardware**: External frontends (SDL2, WebAssembly, etc.) implement the `Screen` and `Keys` traits to integrate with the core.
* **Restricted Views**: The library uses `FrameView` and `InputHandle` wrappers to provide restricted access to internal states, preventing common emulation bugs like race conditions or illegal memory writes.
* **Associated Types**: The `Cpu` trait utilizes associated types to remain flexible across different data and address widths.

## 🛠️ Project Status (Work in Progress)
This library is currently under active development:
- [x] **Memory**: Full 4KB RAM implementation with font support.
- [x] **CPU**: Core register set, stack logic, and program counter.
- [x] **Architecture**: Fully implemented Bus and Trait-based hardware abstraction.
- [ ] **Opcodes**: Approximately 50% of the instruction set is decoded and functional.
- [ ] **Timers**: Delay and Sound timer logic integration.
- [ ] **Frontend**: Example implementation using SDL2.

## 📚 References
The development of this library was guided by the following technical resources:
* **[Building a Chip-8 Emulator](https://github.com/aquova/chip8-book)** by Austin Morlan (Aquova) - Primary technical reference.
* **Cowgod's Chip-8 Technical Reference** - Instruction set specifications.

## 📜 License
This project is open-source. Feel free to use it as a reference for your own emulation journeys.
