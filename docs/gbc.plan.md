# Roadmap - Game Boy Color Emulator

## Overview

| Phase | Goal | Milestone |
|--------|--------|--------|
| 1 | ROM Loader | Load ROMs and read headers |
| 2 | Memory Bus | Working memory system |
| 3 | Basic CPU | Execute simple instructions |
| 4 | CPU Tests | Pass the basic tests |
| 5 | Flow Control | Loops, calls, and returns |
| 6 | Full CPU | Pass `cpu_instrs.gb` |
| 7 | Timers | Working timer |
| 8 | Interrupts | Interrupt system |
| 9 | Initial Rendering | Display tiles |
| 10 | Background | Render backgrounds |
| 11 | Boot Sequence | Show the Nintendo logo |
| 12 | Sprites | Show characters |
| 13 | MBC1 | Larger games boot |
| 14 | First Playable Game | Fully playable game |
| 15 | GBC Features | GBC-exclusive games |
| 16 | - | Advanced compatibility |
| 17 | Audio | Sounds and music |
| 18 | Optimization | Compatibility and performance |

---

# Phase 1 - ROM Loader

## Goal

Load and interpret ROMs.

## Implement

- [x] Read `.gb` files
- [x] Read `.gbc` files
- [x] Header parser
- [x] Display ROM information

## Initial Structure

```go
type Cartridge struct {
    ROM []byte
}
```

## Milestone

- [x] Load any ROM
- [x] Display cartridge information

---

# Phase 2 - Memory Bus

## Goal

Centralize memory access.

## Implement

```go
Read(addr uint16) byte
Write(addr uint16, value byte)
```

## Components

- [ ] ROM
- [ ] VRAM
- [ ] WRAM
- [ ] HRAM
- [ ] I/O registers

## Milestone

- [ ] CPU accessing all memory through the bus

---

# Phase 3 - Basic CPU

## Goal

Execute the first instructions.

## Registers

- [ ] AF
- [ ] BC
- [ ] DE
- [ ] HL
- [ ] SP
- [ ] PC

## First Opcodes

- [ ] NOP
- [ ] LD
- [ ] INC
- [ ] DEC

## Milestone

- [ ] Execute simple programs without crashes

---

# Phase 4 - CPU Tests

## Goal

Validate arithmetic operations.

## Implement

- [ ] ADD
- [ ] ADC
- [ ] SUB
- [ ] SBC
- [ ] AND
- [ ] OR
- [ ] XOR
- [ ] CP

## Tests

- [ ] Community test ROMs

## Milestone

- [ ] Pass the basic ALU tests

---

# Phase 5 - Flow Control

## Goal

Execute real programs.

## Implement

- [ ] JP
- [ ] JR
- [ ] CALL
- [ ] RET
- [ ] RST

## Concepts

- [ ] Stack
- [ ] Conditional jumps
- [ ] Addressing

## Milestone

- [ ] Programs with working loops

---

# Phase 6 - Full CPU

## Goal

Implement all instructions.

## Implement

- [ ] 256 main opcodes
- [ ] 256 CB-prefixed opcodes

## Tests

- [ ] cpu_instrs.gb

## Milestone

- [ ] Fully pass `cpu_instrs.gb`

---

# Phase 7 - Timers

## Goal

Implement internal timing.

## Registers

- [ ] DIV
- [ ] TIMA
- [ ] TMA
- [ ] TAC

## Milestone

- [ ] Timer tests passing

---

# Phase 8 - Interrupts

## Goal

Implement system events.

## Types

- [ ] VBlank
- [ ] LCD
- [ ] Timer
- [ ] Serial
- [ ] Joypad

## Registers

- [ ] IME
- [ ] IE
- [ ] IF

## Milestone

- [ ] Boot ROM running correctly

---

# Phase 9 - Initial Rendering

## Goal

Draw the first pixels.

## Implement

- [ ] SDL2
- [ ] Framebuffer
- [ ] Tile rendering

## Resolution

```text
160x144
```

## Milestone

- [ ] Display tiles from VRAM

---

# Phase 10 - Background

## Goal

Render backgrounds.

## Implement

- [ ] Tile Maps
- [ ] Scroll
- [ ] Palettes

## Milestone

- [ ] Full screen rendered

---

# Phase 11 - Boot Sequence

## Goal

Show the boot sequence.

## Synchronization

- [ ] CPU
- [ ] PPU
- [ ] Timers

## Milestone

- [ ] Nintendo logo appears correctly

---

# Phase 12 - Sprites

## Goal

Render movable objects.

## Implement

- [ ] OAM
- [ ] Priority
- [ ] Horizontal flip
- [ ] Vertical flip

## Milestone

- [ ] Sprites appear correctly

---

# Phase 13 - MBC1

## Goal

Support larger ROMs.

## Implement

- [ ] ROM banking
- [ ] RAM banking

## Milestone

- [ ] Commercial games boot

---

# Phase 14 - First Playable Game

## Goal

Run a full game.

## Milestone

- [ ] Complete a game session without major issues

---

# Phase 15 - Game Boy Color Features

## Goal

Add GBC-specific support.

## Implement

- [ ] VRAM Bank 0
- [ ] VRAM Bank 1
- [ ] WRAM banking
- [ ] Color Palettes
- [ ] HDMA
- [ ] Double Speed Mode

## Milestone

- [ ] GBC-exclusive games boot

---

# Phase 16 - Advanced Compatibility

## Goal

Validate advanced compatibility.

## Fix

- [ ] Graphics bugs
- [ ] Timing bugs
- [ ] Interrupt bugs

## Milestone

- [ ] Play normally
- [ ] Save progress
- [ ] Load save

---

# Phase 17 - Audio

## Goal

Implement the APU.

## Channels

- [ ] Square 1
- [ ] Square 2
- [ ] Wave
- [ ] Noise

## Milestone

- [ ] Music and effects working

---

# Phase 18 - Optimization

## Goal

Increase compatibility and performance.

## Improvements

- [ ] Profiling
- [ ] Automated tests
- [ ] Save States
- [ ] Debugger
- [ ] Benchmarks

## Milestone

- [ ] Compatibility above 90%
