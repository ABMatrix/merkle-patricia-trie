
// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate core;
#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
mod std {
    pub use alloc::vec;
    pub use alloc::vec::Vec;
    pub use alloc::rc::Rc;
    pub use alloc::string::String;
    pub use alloc::format;
    pub use alloc::borrow::ToOwned;

    pub use core::fmt;
    pub use core::cell::RefCell;
}

#[cfg(feature = "std")]
mod std {
    pub use std::rc::Rc;
    pub use std::cell::RefCell;
    pub use std::fmt;
}

mod nibbles;
mod node;
mod trie;
mod error;
mod db;
mod tests;

pub use db::{MemoryDB};
pub use error::{TrieError};
pub use trie::{MerklePatriciaTrie, Trie};