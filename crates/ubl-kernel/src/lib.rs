//! # UBL Kernel
//!
//! Core cryptographic kernel providing deterministic hashing and signing.
//!
//! ## Features
//! - Canonical JSON serialization (Json✯Atomic)
//! - BLAKE3 domain-separated hashing
//! - Ed25519 signing and verification
//! - Event domain enumeration

#![deny(unsafe_code)]
#![warn(missing_docs)]

/// Placeholder for Sprint 1 implementation
pub fn placeholder() {
    // Sprint 1: Deterministic Kernel
    // - canonical_json (Json✯Atomic)
    // - blake3_hash(domain, bytes)
    // - ed25519_sign / verify
    // - Enum EventDomain
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        placeholder();
    }
}
