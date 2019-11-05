
// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate core;

#[cfg(feature = "std")]
extern crate alloc;

mod nibbles;
mod node;
mod trie;
mod error;
mod db;
mod tests;

pub use db::{MemoryDB};
pub use error::{TrieError};
pub use trie::{MerklePatriciaTrie, Trie};