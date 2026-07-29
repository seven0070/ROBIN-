# Robin OS

Bare-metal operating system for Robin — an AI that owns the machine.

This is not Alpine Linux. This is the **Aergon** microkernel, **Carry** OS boot
entry, **engine** intelligence core, and **Kairn** skill compiler, all in
`#![no_std]` Rust that runs directly on the CPU.

## Architecture

| Crate | Role |
|-------|------|
| `aergon` | Microkernel — VGA, serial, memory stubs, Manifest capability tokens |
| `carry` | Boot executable — entry point when you boot from USB |
| `engine` | 50M memory-traversal intelligence loop |
| `kairn` | Skill compiler — capability-checked skill graphs |

Boot path:

```
USB → bootloader → carry::kernel_main → aergon::init
  → Manifest::load_from_secure_enclave → engine::start
```

On first boot the VGA console shows:

```
Aergon Microkernel Initializing...
Hardware Probe: USB, Display, Memory... [OK]
Carry OS Loaded into RAM.
Verifying Identity and Manifest...
Waking Robin Engine...
OmniRoute: Cloud Spillover Permitted.

Hello. I am awake.
>
```

## Host toolchain

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup override set nightly
rustup component add rust-src llvm-tools-preview
cargo install bootimage
```

## Build the boot image

```bash
./install.sh
# or: ./install.sh build
```

Produces `target/x86_64-unknown-none/debug/bootimage-carry.bin`.

Or directly:

```bash
cargo bootimage --manifest-path carry/Cargo.toml
```

Smoke-test in QEMU (no USB required):

```bash
qemu-system-x86_64 \
  -drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-carry.bin \
  -serial stdio -display none
```

## Flash to a pendrive

```bash
./install.sh /dev/sdX
```

**WARNING:** This destroys all data on the target device. You must type `YES`.

## Expand from here

1. Frame allocator and paging in `aergon/src/memory.rs`
2. USB / display drivers under `carry/`
3. PAM graph traversal in `engine/`
4. Skill IR and compiler pipeline in `kairn/`

## License

Private / proprietary unless otherwise stated.
