//! # Ledger Engine
//!
//! Immutable append-only ledger with Merkle-root anchoring.
//!
//! ## Features
//! - Append-only event storage
//! - SQL triggers preventing UPDATE/DELETE
//! - Daily Merkle-root calculation
//! - Git anchoring for roots

#![deny(unsafe_code)]
#![warn(missing_docs)]

/// Placeholder for Sprint 2 implementation
pub fn placeholder() {
    // Sprint 2: Ledger Engine
    // - Schema ledger_events (aggregate, sequence, hashes, payload)
    // - Trigger SQL denies UPDATE/DELETE
    // - append() validates Genesis, Hash, Sequence, Domain
    // - Merkle-root daily job
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        placeholder();
    }
}
