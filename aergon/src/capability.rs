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
