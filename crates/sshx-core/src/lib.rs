//! The core crate for shared code used in the sshx application.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::fmt::Display;
use std::sync::atomic::{AtomicU32, Ordering};

use serde::{Deserialize, Serialize};

/// Protocol buffer and gRPC definitions, automatically generated by Tonic.
#[allow(missing_docs, non_snake_case)]
#[allow(clippy::derive_partial_eq_without_eq)]
pub mod proto {
    tonic::include_proto!("sshx");

    /// File descriptor set used for gRPC reflection.
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("sshx");
}

/// Generate a cryptographically-secure, random alphanumeric value.
pub fn rand_alphanumeric(len: usize) -> String {
    use rand::{distributions::Alphanumeric, thread_rng, Rng};
    thread_rng()
        .sample_iter(Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

/// Unique identifier for a shell within the session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Sid(pub u32);

impl Display for Sid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for a user within the session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Uid(pub u32);

impl Display for Uid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A counter for generating unique identifiers.
#[derive(Debug)]
pub struct IdCounter {
    next_sid: AtomicU32,
    next_uid: AtomicU32,
}

impl Default for IdCounter {
    fn default() -> Self {
        Self {
            next_sid: AtomicU32::new(1),
            next_uid: AtomicU32::new(1),
        }
    }
}

impl IdCounter {
    /// Returns the next unique shell ID.
    pub fn next_sid(&self) -> Sid {
        Sid(self.next_sid.fetch_add(1, Ordering::Relaxed))
    }

    /// Returns the next unique user ID.
    pub fn next_uid(&self) -> Uid {
        Uid(self.next_uid.fetch_add(1, Ordering::Relaxed))
    }
}
