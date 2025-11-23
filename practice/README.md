# Embedded Rust Workshop - Practice

Welcome to the hands-on portion of the workshop! This directory contains everything you need to build a complete embedded Rust application from scratch.

## Structure

```
practice/
├── workshop/          # 👈 Your working directory
│   ├── src/
│   │   └── main.rs   # Start here!
│   ├── Cargo.toml
│   └── README.md     # Session-by-session instructions
│
├── snippets/         # Reference code for each session
│   ├── session2/
│   ├── session3/
│   └── ...          # Helpful examples when you get stuck
│
├── final/           # The end goal
│   └── firmware/    # Complete reference implementation
│
├── session2/        # Legacy standalone session (kept for reference)
└── session4/        # Legacy standalone session (kept for reference)
```

## The Journey

You'll build a **single application** that evolves through the workshop sessions:

### Sessions 2-4: Foundation
- **Session 2**: Read LIS3DH accelerometer using PAC (direct register access)
- **Session 3**: Create embedded-hal driver for portability
- **Session 4**: Migrate to embassy-stm32 HAL for simplicity

### Sessions 5-8: Async & Interrupts
- **Session 5**: Add interrupt handling (EXTI)
- **Session 6**: Understand async by building an executor
- **Session 7**: Migrate to embassy-executor
- **Session 8**: Use async interrupt handling (ExtiInput)

### Sessions 9-11: Advanced Patterns
- **Session 9**: Add Channel-based data streaming
- **Session 10**: Use multiple executors for priorities
- **Session 11**: Add custom panic handler

### Sessions 12-13: Integration
- **Session 12**: Integrate C library (optional)
- **Session 13**: Send data over Ethernet with embassy-net

### Final Application
A complete sensor node that:
- ✅ Reads LIS3DH accelerometer with interrupts
- ✅ Uses async/await for efficient multitasking
- ✅ Streams data through channels
- ✅ Sends readings over Ethernet
- ✅ Handles errors gracefully

## Getting Started

### 1. Navigate to Workshop Directory
```bash
cd practice/workshop
```

### 2. Read Current Session Instructions
```bash
cat README.md
```

The README updates for each session with:
- Current goals
- Tasks to complete
- Which snippets to reference
- Testing instructions

### 3. Build and Run
```bash
cargo build
cargo run
```

### 4. When Stuck
Check `../snippets/sessionX/` for reference code!

## Hardware Required

- **Board**: STM32H563ZI Nucleo
- **Sensor**: LIS3DH accelerometer (I2C)
- **Connections**:
  - SCL → PB8
  - SDA → PB9
  - INT1 → PC1 (or check your board)
  - VCC → 3.3V
  - GND → GND

## Snippets vs Workshop

**snippets/**
- Reference implementations
- Copy-pasteable examples
- Use when stuck or for inspiration

**workshop/**
- Your code
- Builds progressively
- Where you do the actual work

## Tips

1. **Try first, snippet second** - Attempt implementation before looking at snippets
2. **Understand, don't just copy** - Know why the code works
3. **Build often** - Catch errors early
4. **Use defmt logs** - `info!()`, `warn!()`, `error!()` are your friends
5. **Check final/** - See where you're heading

## Session-by-Session Workflow

Each session:
1. Read workshop/README.md for current session
2. Implement the required features
3. Reference snippets when needed
4. Test on hardware
5. Move to next session

The README in workshop/ will guide you!

## Questions?

- Check `workshop/README.md` for current session details
- Look in `snippets/sessionX/` for examples
- Review `final/firmware/` for the complete implementation
- Ask the instructor!

## Archive

The `archive/` directory contains old standalone session folders that are no longer part of the workshop. They're kept for reference only.

---

**Let's build something cool! 🚀**
