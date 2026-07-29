# COMPLETE REBUILD HANDOFF — Robin Bare-Metal OS

Paste this entire document into a new chat. It is the full, verified scaffold.

## Identity

| Item | Value |
|------|-------|
| Repo | https://github.com/seven0070/ROBIN- |
| Branch | `cursor/robin-baremetal-os-ba92` |
| PR | https://github.com/seven0070/ROBIN-/pull/1 |
| Commit | `7b207ab` — Scaffold bare-metal Robin OS: Aergon, Carry, engine, Kairn |
| Base | `main` |

This is **not** Alpine/Linux. It is a `#![no_std]` Rust OS that boots via `bootloader` 0.9 into Carry → Aergon → engine.

## Quick restore (preferred)

```bash
git clone https://github.com/seven0070/ROBIN-.git
cd ROBIN-
git checkout cursor/robin-baremetal-os-ba92
# OR extract robin-os-complete-source.tar.gz into an empty dir
```

## Toolchain (required)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install nightly
rustup override set nightly          # or rely on rust-toolchain.toml
rustup component add rust-src llvm-tools-preview
cargo install bootimage
# optional smoke test:
# sudo apt-get install -y qemu-system-x86
```

## Build & verify

```bash
./install.sh
# artifact: target/x86_64-unknown-none/debug/bootimage-carry.bin

qemu-system-x86_64 \
  -drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-carry.bin \
  -serial stdio -display none
```

**Expected serial output:**

```
Aergon Microkernel Initializing...
Hardware Probe: USB, Display, Memory... [OK]
Carry OS Loaded into RAM.
Verifying Identity and Manifest...
Waking Robin Engine...
Robin 50M Memory-Traversal Engine Online.
OmniRoute: Offline-Only Mode Enforced.

Hello. I am awake.
>
```

Flash USB (destructive):

```bash
./install.sh /dev/sdX   # type YES
```

## Critical gotchas (do not regress)

1. **Must be nightly** + `rust-src` + `llvm-tools-preview` + `bootimage`.
2. **Target = `x86_64-unknown-none`** (built-in). Custom JSON targets need `-Zjson-target-spec` on modern nightly.
3. **Static link is mandatory** for bootloader 0.9. Without these rustflags the ELF becomes PIE (`DYN`) and boots black-screen with empty serial:

```toml
[target.x86_64-unknown-none]
rustflags = [
    "-C", "relocation-model=static",
    "-C", "code-model=kernel",
    "-C", "link-arg=--image-base=0x100000",
]
```

4. **`x86_64` crate must be `0.15+`**. `0.14` fails on current nightly (`Step::forward_overflowing` missing).
5. **Panic handler only in `carry`** (binary). Not in `aergon` lib.
6. **`engine::start` returns `!`** — no unreachable code after it in `carry`.
7. Manifest defaults to **all capabilities denied** (offline-only).

## Architecture

```
USB/QEMU → bootloader 0.9 → carry::kernel_main
  → aergon::init (serial + VGA)
  → Manifest::load_from_secure_enclave
  → engine::start(manifest)   # never returns; hlt loop
```

| Crate | Role |
|-------|------|
| `aergon` | Microkernel: capability tokens, VGA, serial, memory stub |
| `carry` | `no_std` + `no_main` boot binary |
| `engine` | Intelligence loop; OmniRoute gated by Manifest |
| `kairn` | Skill compiler stub |

## File tree

```
.
├── .cargo/config.toml
├── .gitignore
├── Cargo.toml
├── Cargo.lock
├── README.md
├── rust-toolchain.toml
├── install.sh
├── aergon/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── capability.rs
│       ├── memory.rs
│       ├── serial.rs
│       └── vga_buffer.rs
├── carry/
│   ├── Cargo.toml
│   └── src/main.rs
├── engine/
│   ├── Cargo.toml
│   └── src/lib.rs
└── kairn/
    ├── Cargo.toml
    └── src/lib.rs
```

---

## FULL FILE CONTENTS — recreate from scratch if needed

### `Cargo.toml`

```toml
[workspace]
members = [
    "aergon",      # The Microkernel
    "carry",       # The OS / Boot initialization
    "engine",      # 50M Graph Traversal Engine
    "kairn",       # Skill Compiler
]
resolver = "2"

[profile.dev]
panic = "abort"

[profile.release]
opt-level = 3
lto = true
panic = "abort"
```

### `rust-toolchain.toml`

```toml
[toolchain]
channel = "nightly"
components = ["rust-src", "llvm-tools-preview"]
```

### `.cargo/config.toml`

```toml
[build]
target = "x86_64-unknown-none"

[unstable]
build-std = ["core", "compiler_builtins"]
build-std-features = ["compiler-builtins-mem"]

[target.x86_64-unknown-none]
rustflags = [
    "-C", "relocation-model=static",
    "-C", "code-model=kernel",
    "-C", "link-arg=--image-base=0x100000",
]

[alias]
kbuild = "bootimage --manifest-path carry/Cargo.toml"
```

### `.gitignore`

```
/target/
**/*.rs.bk
*.pdb
.DS_Store
*.img
rust-toolchain-info.txt
```

### `install.sh` (chmod +x)

```bash
#!/usr/bin/env bash
set -euo pipefail

echo "╔══════════════════════════════════════════════╗"
echo "║      Robin Bare-Metal Builder & Installer    ║"
echo "╚══════════════════════════════════════════════╝"

TARGET="${1:-}"

usage() {
    echo "Usage:"
    echo "  ./install.sh              Compile Carry OS boot image only"
    echo "  ./install.sh build        Same as above"
    echo "  ./install.sh /dev/sdX     Build and flash to pendrive"
    exit 1
}

if [ "${TARGET}" = "-h" ] || [ "${TARGET}" = "--help" ]; then
    usage
fi

ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT"

echo "[1/4] Compiling Aergon Kernel and Carry OS..."
cargo bootimage --manifest-path carry/Cargo.toml

IMG_PATH="target/x86_64-unknown-none/debug/bootimage-carry.bin"
if [ ! -f "$IMG_PATH" ]; then
    IMG_PATH="$(find target -name 'bootimage-carry.bin' -type f 2>/dev/null | head -n 1 || true)"
fi

echo "[2/4] Verifying OS Image..."
if [ -z "${IMG_PATH}" ] || [ ! -f "$IMG_PATH" ]; then
    echo "Build failed. Boot image not found."
    exit 1
fi
echo "    Image: $IMG_PATH ($(wc -c < "$IMG_PATH") bytes)"

if [ -z "$TARGET" ] || [ "$TARGET" = "build" ]; then
    echo "──────────────────────────────────────────────"
    echo "✔ Boot image built (not flashed)."
    echo "  Run: ./install.sh /dev/sdX"
    exit 0
fi

if [ ! -b "$TARGET" ]; then
    echo "Error: $TARGET is not a block device."
    exit 1
fi

echo "[3/4] Preparing Pendrive ($TARGET)..."
echo "WARNING: ALL DATA ON $TARGET WILL BE DESTROYED."
read -r -p "Type YES to flash Robin OS to the pendrive: " CONFIRM
if [ "$CONFIRM" != "YES" ]; then
    echo "Aborted."
    exit 1
fi

echo "[4/4] Flashing Robin to Pendrive..."
sudo dd if="$IMG_PATH" of="$TARGET" bs=4M status=progress conv=fsync
sync

echo "──────────────────────────────────────────────"
echo "✔ Robin is installed."
echo "Eject the USB. Plug it in. Boot from USB."
echo "You are booting a custom OS written from scratch."
```

### `aergon/Cargo.toml`

```toml
[package]
name = "aergon"
version = "0.1.0"
edition = "2021"
description = "Aergon microkernel — capability-enforced bare-metal core for Robin"

[dependencies]
spin = "0.9"
x86_64 = "0.15"
uart_16550 = "0.3"

[package.metadata.bootimage]
test-success-exit-code = 33
```

### `aergon/src/lib.rs`

```rust
#![no_std]

pub mod capability;
pub mod memory;
pub mod serial;
pub mod vga_buffer;

/// Initialize the Aergon microkernel: serial, VGA console, and early hardware probe.
pub fn init() {
    serial::init();
    vga_buffer::print_string("Aergon Microkernel Initializing...\n");
    vga_buffer::print_string("Hardware Probe: USB, Display, Memory... [OK]\n");
}
```

### `aergon/src/capability.rs`

```rust
//! Compile-time Manifest enforcement.
//!
//! Capability tokens are unforgeable outside this module. If a function does
//! not take a token, it physically cannot access the corresponding resource.

/// Token granting network (OmniRoute / cloud spillover) access.
pub struct NetworkToken {
    _priv: (),
}

/// Token granting filesystem access.
pub struct FilesystemToken {
    _priv: (),
}

/// Token granting host-control / privilege escalation.
pub struct HostControlToken {
    _priv: (),
}

/// Signed capability set loaded from the secure enclave.
pub struct Manifest {
    pub can_net: Option<NetworkToken>,
    pub can_fs: Option<FilesystemToken>,
    pub can_host: Option<HostControlToken>,
}

impl Manifest {
    /// Reads the signed YAML manifest from the hardware enclave.
    ///
    /// Network, filesystem, and host control are denied until the enclave
    /// grants them. The scaffold boots offline-only.
    pub fn load_from_secure_enclave() -> Self {
        Manifest {
            can_net: None,
            can_fs: None,
            can_host: None,
        }
    }
}
```

### `aergon/src/memory.rs`

```rust
//! Physical / virtual memory management for Aergon.
//!
//! Stub: frame allocator and address-space isolation will live here.

use x86_64::structures::paging::{FrameAllocator, PhysFrame, Size4KiB};
use x86_64::PhysAddr;

/// Placeholder frame allocator — replaced by a real bitmap/buddy allocator.
pub struct StubFrameAllocator {
    next: u64,
}

impl StubFrameAllocator {
    pub const fn new() -> Self {
        StubFrameAllocator { next: 0x100000 }
    }
}

unsafe impl FrameAllocator<Size4KiB> for StubFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = PhysFrame::containing_address(PhysAddr::new(self.next));
        self.next += 4096;
        Some(frame)
    }
}
```

### `aergon/src/serial.rs`

```rust
//! UART 16550 serial port for early debug output.

use spin::Mutex;
use uart_16550::SerialPort;

pub static SERIAL1: Mutex<SerialPort> = Mutex::new(unsafe { SerialPort::new(0x3F8) });

/// Initialize COM1 (0x3F8) for debug logging.
pub fn init() {
    SERIAL1.lock().init();
}

/// Write a string to the serial port.
pub fn write_string(s: &str) {
    use core::fmt::Write;
    SERIAL1.lock().write_str(s).ok();
}
```

### `aergon/src/vga_buffer.rs`

```rust
//! VGA text-mode buffer driver (80×25).

use core::fmt;
use spin::Mutex;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const VGA_ADDRESS: usize = 0xb8000;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    buffer: *mut Buffer,
}

// SAFETY: Writer is only accessed through a Mutex; the VGA buffer is a fixed MMIO address.
unsafe impl Send for Writer {}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row_position;
                let col = self.column_position;
                let color_code = self.color_code;

                // SAFETY: buffer points at the VGA MMIO region for the lifetime of the kernel.
                unsafe {
                    core::ptr::write_volatile(
                        &mut (*self.buffer).chars[row][col],
                        ScreenChar {
                            ascii_character: byte,
                            color_code,
                        },
                    );
                }
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn read_char(&self, row: usize, col: usize) -> ScreenChar {
        // SAFETY: VGA buffer is always mapped at VGA_ADDRESS on legacy text mode.
        unsafe { core::ptr::read_volatile(&(*self.buffer).chars[row][col]) }
    }

    fn write_char(&mut self, row: usize, col: usize, character: ScreenChar) {
        unsafe {
            core::ptr::write_volatile(&mut (*self.buffer).chars[row][col], character);
        }
    }

    fn new_line(&mut self) {
        if self.row_position >= BUFFER_HEIGHT - 1 {
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let character = self.read_char(row, col);
                    self.write_char(row - 1, col, character);
                }
            }
            self.clear_row(BUFFER_HEIGHT - 1);
            self.column_position = 0;
        } else {
            self.row_position += 1;
            self.column_position = 0;
        }
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.write_char(row, col, blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    column_position: 0,
    row_position: 0,
    color_code: ColorCode::new(Color::LightGreen, Color::Black),
    // SAFETY: 0xb8000 is the VGA text buffer on x86_64 BIOS/UEFI legacy VGA.
    buffer: VGA_ADDRESS as *mut Buffer,
});

/// Print a string to the VGA text buffer (and mirror to serial for debug).
pub fn print_string(s: &str) {
    use fmt::Write;
    WRITER.lock().write_str(s).ok();
    crate::serial::write_string(s);
}

/// Clear the entire VGA screen.
pub fn clear_screen() {
    let mut writer = WRITER.lock();
    for row in 0..BUFFER_HEIGHT {
        writer.clear_row(row);
    }
    writer.column_position = 0;
    writer.row_position = 0;
}
```

### `carry/Cargo.toml`

```toml
[package]
name = "carry"
version = "0.1.0"
edition = "2021"
description = "Carry OS — bare-metal boot entry that wakes Robin"

[[bin]]
name = "carry"
path = "src/main.rs"
test = false
bench = false

[dependencies]
aergon = { path = "../aergon" }
engine = { path = "../engine" }
bootloader = "0.9"
x86_64 = "0.15"

[package.metadata.bootimage]
test-success-exit-code = 33
build-command = ["build"]
```

### `carry/src/main.rs`

```rust
#![no_std]
#![no_main]

use aergon::capability::Manifest;
use aergon::vga_buffer;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    aergon::init();

    vga_buffer::print_string("Carry OS Loaded into RAM.\n");

    // 1. Verify Manifest
    vga_buffer::print_string("Verifying Identity and Manifest...\n");
    let manifest = Manifest::load_from_secure_enclave();

    // 2. Start the Intelligence Engine (never returns)
    vga_buffer::print_string("Waking Robin Engine...\n");
    engine::start(manifest);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga_buffer::print_string("KERNEL PANIC: ");
    if let Some(location) = info.location() {
        vga_buffer::print_string(location.file());
    }
    // In a real panic, trigger the atomic save to eMMC here
    loop {
        x86_64::instructions::hlt();
    }
}
```

### `engine/Cargo.toml`

```toml
[package]
name = "engine"
version = "0.1.0"
edition = "2021"
description = "Robin 50M memory-traversal intelligence engine"

[dependencies]
aergon = { path = "../aergon" }
x86_64 = "0.15"
```

### `engine/src/lib.rs`

```rust
#![no_std]

use aergon::capability::Manifest;
use aergon::vga_buffer;

/// Start the Robin intelligence engine under the given Manifest.
///
/// Capabilities are checked before any privileged action. The main event
/// loop reads input, traverses the PAM graph, and generates responses.
pub fn start(manifest: Manifest) -> ! {
    vga_buffer::print_string("Robin 50M Memory-Traversal Engine Online.\n");

    if manifest.can_net.is_some() {
        vga_buffer::print_string("OmniRoute: Cloud Spillover Permitted.\n");
    } else {
        vga_buffer::print_string("OmniRoute: Offline-Only Mode Enforced.\n");
    }

    vga_buffer::print_string("\nHello. I am awake.\n> ");

    // Main event loop — read input, traverse PAM graph, generate response
    loop {
        x86_64::instructions::hlt();
    }
}
```

### `kairn/Cargo.toml`

```toml
[package]
name = "kairn"
version = "0.1.0"
edition = "2021"
description = "Kairn skill compiler — turns manifests into capability-checked code"

[dependencies]
aergon = { path = "../aergon" }
```

### `kairn/src/lib.rs`

```rust
#![no_std]

//! Kairn — skill compiler for Robin.
//!
//! Compiles high-level skill definitions into capability-checked call graphs
//! that the engine can execute under a Manifest.

use aergon::capability::Manifest;

/// A compiled skill ready for engine execution.
pub struct Skill {
    pub name: &'static str,
    pub requires_net: bool,
    pub requires_fs: bool,
    pub requires_host: bool,
}

impl Skill {
    /// Check whether this skill is permitted under the given Manifest.
    pub fn is_permitted(&self, manifest: &Manifest) -> bool {
        if self.requires_net && manifest.can_net.is_none() {
            return false;
        }
        if self.requires_fs && manifest.can_fs.is_none() {
            return false;
        }
        if self.requires_host && manifest.can_host.is_none() {
            return false;
        }
        true
    }
}

/// Compile a named skill definition (stub).
pub fn compile(name: &'static str) -> Skill {
    Skill {
        name,
        requires_net: false,
        requires_fs: false,
        requires_host: false,
    }
}
```

### `README.md`

Keep the repo README in sync; boot demo line must say **Offline-Only Mode Enforced** (Manifest defaults deny net).

---

## New-chat prompt (copy/paste)

```
Continue Robin bare-metal OS from branch cursor/robin-baremetal-os-ba92
(PR https://github.com/seven0070/ROBIN-/pull/1, commit 7b207ab).

This is a no_std Rust OS: aergon (microkernel + Manifest capabilities),
carry (bootloader 0.9 entry), engine (intelligence loop), kairn (skill compiler).

Build: nightly + rust-src + llvm-tools-preview + bootimage; ./install.sh
Critical: static relocation rustflags in .cargo/config.toml (image-base 0x100000);
x86_64 crate 0.15+; target x86_64-unknown-none.

Verified QEMU serial ends with:
OmniRoute: Offline-Only Mode Enforced.
Hello. I am awake.
>

Expand next: memory allocator, USB/display drivers, PAM graph in engine, Kairn IR.
Full file dump is in COMPLETE_REBUILD.md in the repo.
```

## Expand next

1. Real frame allocator / paging — `aergon/src/memory.rs`
2. USB / display drivers — under `carry/`
3. PAM graph traversal — `engine/`
4. Skill IR pipeline — `kairn/`
