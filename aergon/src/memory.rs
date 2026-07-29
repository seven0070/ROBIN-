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
