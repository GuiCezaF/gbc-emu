# Game Boy Color Emulator Roadmap

This roadmap is written as an implementation backlog. Each phase should be completed in order, but the checklist inside each phase is intentionally concrete so the next coding step is obvious.

Current baseline:
- ROM loading and header parsing exist.
- A memory bus skeleton exists with region routing.
- A minimal CPU can fetch bytes through the bus.

---

## Phase 1 - ROM Loader

Goal: load cartridge data and expose header metadata.

- [x] Read `.gb` and `.gbc` files into memory
- [x] Parse title, ROM size, and RAM size from the header
- [x] Print cartridge metadata at startup
- [x] Validate header fields more strictly
- [x] Add cartridge type parsing
- [x] Reject obviously malformed ROMs early

Done when:
- A valid ROM is loaded from disk and the emulator can report the cartridge metadata without crashing.

---

## Phase 2 - Memory Bus

Goal: centralize all CPU memory access behind a single address decoder.

- [x] Define address ranges for ROM, VRAM, ERAM, WRAM, OAM, I/O, HRAM, and IE
- [x] Route reads and writes to the current backing storage
- [ ] Handle unmapped regions with explicit open-bus behavior
- [ ] Split I/O registers from raw byte storage
- [ ] Add ROM banking hooks for future MBC support
- [ ] Add access helpers for mirrored or restricted regions

Done when:
- The CPU only touches memory through `Bus::read` and `Bus::write`, and every address in the map has a defined behavior.

---

## Phase 3 - Basic CPU

Goal: fetch, decode, and execute a minimal instruction subset.

- [x] Implement CPU state with `PC`
- [x] Fetch bytes through the bus
- [ ] Add the remaining 16-bit registers: `AF`, `BC`, `DE`, `HL`, `SP`
- [ ] Implement `NOP`
- [ ] Implement `LD r, n` and `LD rr, nn`
- [ ] Implement `INC` and `DEC`
- [ ] Add flag updates for the implemented instructions

Done when:
- The CPU can step through a ROM, advance `PC`, and execute basic load/increment/decrement instructions without corrupting state.

---

## Phase 4 - CPU Tests

Goal: verify arithmetic and flag semantics.

- [ ] Implement `ADD`
- [ ] Implement `ADC`
- [ ] Implement `SUB`
- [ ] Implement `SBC`
- [ ] Implement `AND`, `OR`, `XOR`, and `CP`
- [ ] Validate `Z`, `N`, `H`, and `C` flag behavior
- [ ] Run ALU-focused community test ROMs

Done when:
- Arithmetic results and flags match the test ROM expectations for the supported instructions.

---

## Phase 5 - Flow Control

Goal: support branching, calls, returns, and stack usage.

- [ ] Implement `JP`, `JR`, `CALL`, `RET`, and `RST`
- [ ] Implement conditional variants
- [ ] Add stack push/pop helpers
- [ ] Verify call depth and return address handling

Done when:
- Loops, subroutines, and conditional branches behave correctly in small programs.

---

## Phase 6 - Full CPU

Goal: complete the instruction set needed by standard CPU test ROMs.

- [ ] Implement all 256 main opcodes
- [ ] Implement all 256 CB-prefixed opcodes
- [ ] Audit each opcode for correct flags, cycles, and operand width
- [ ] Run `cpu_instrs.gb`

Done when:
- The official CPU instruction test ROM passes cleanly.

---

## Phase 7 - Timers

Goal: emulate divider and timer behavior.

- [ ] Implement `DIV`
- [ ] Implement `TIMA`, `TMA`, and `TAC`
- [ ] Apply timer increments based on the selected clock source
- [ ] Handle `TIMA` overflow and reload timing
- [ ] Reset `DIV` correctly on write

Done when:
- Timer test ROMs pass and timer-driven games stay in sync.

---

## Phase 8 - Interrupts

Goal: request and service hardware interrupts correctly.

- [ ] Implement `IME`, `IE`, and `IF`
- [ ] Add interrupt request bits for VBlank, LCD, Timer, Serial, and Joypad
- [ ] Prioritize interrupts in hardware order
- [ ] Push `PC` and jump to the correct vector
- [ ] Support delayed enable/disable behavior

Done when:
- Interrupts are requested, masked, acknowledged, and serviced like hardware.

---

## Phase 9 - Initial Rendering

Goal: show tiles on screen.

- [ ] Create a window and frame buffer
- [ ] Decode tile data from VRAM
- [ ] Convert tile bitplanes into pixels
- [ ] Present a stable frame at 160x144

Done when:
- The emulator can display tile graphics from VRAM.

---

## Phase 10 - Background

Goal: render the background layer.

- [ ] Read the correct tile map
- [ ] Apply `SCX` and `SCY`
- [ ] Apply background palette mapping
- [ ] Render the full 160x144 view

Done when:
- Background scrolling and palette selection visibly affect the output.

---

## Phase 11 - Boot Sequence

Goal: reproduce the boot ROM visible behavior.

- [ ] Match CPU, PPU, and timer timing during boot
- [ ] Render the logo correctly
- [ ] Unmap boot behavior at the right point
- [ ] Continue into cartridge execution after boot

Done when:
- The boot sequence reaches the expected post-logo state and hands off to the cartridge.

---

## Phase 12 - Sprites

Goal: render OAM sprites correctly.

- [ ] Read sprite attributes from OAM
- [ ] Support priority rules
- [ ] Support horizontal and vertical flip
- [ ] Respect sprite size configuration

Done when:
- Sprites appear at the correct position, orientation, and priority.

---

## Phase 13 - MBC1

Goal: support banked cartridges.

- [ ] Implement ROM bank selection
- [ ] Implement RAM bank selection
- [ ] Handle RAM enable writes
- [ ] Route banked reads and writes through the bus

Done when:
- Larger commercial ROMs boot and can access their save RAM.

---

## Phase 14 - First Playable Game

Goal: get one real game into a playable state.

- [ ] Boot to the title screen
- [ ] Accept controller input
- [ ] Render core gameplay without blocking bugs
- [ ] Preserve save data if the game requires it

Done when:
- At least one commercial game can be started and played for a meaningful session.

---

## Phase 15 - Game Boy Color Features

Goal: support GBC-specific hardware.

- [ ] Add VRAM bank selection
- [ ] Add WRAM banking
- [ ] Add color palette registers
- [ ] Add HDMA
- [ ] Add double-speed mode

Done when:
- GBC-only games boot and render color graphics correctly.

---

## Phase 16 - Advanced Compatibility

Goal: reduce game-specific edge cases.

- [ ] Fix graphics timing bugs
- [ ] Fix interrupt edge cases
- [ ] Fix timer corner cases
- [ ] Fix save/load edge cases

Done when:
- Common commercial games run without obvious hardware-accuracy regressions.

---

## Phase 17 - Audio

Goal: generate Game Boy sound.

- [ ] Implement square channel 1
- [ ] Implement square channel 2
- [ ] Implement wave channel
- [ ] Implement noise channel
- [ ] Mix channels into host audio output

Done when:
- Games produce recognizable music and sound effects in sync with emulation.

---

## Phase 18 - Optimization

Goal: improve speed and maintainability without breaking accuracy.

- [ ] Add profiling
- [ ] Add regression tests for hardware behavior
- [ ] Add save states
- [ ] Add debugger views for CPU, memory, and PPU state
- [ ] Add benchmarks for representative workloads

Done when:
- Performance is measured, regressions are caught automatically, and the emulator remains accurate under optimization work.
