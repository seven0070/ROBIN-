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
