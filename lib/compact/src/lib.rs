//! This crate makes it possible to store objects containing dynamic fields
//! either compactly in consecutive memory or using traditional heap pointers.
//!
//! Bread-and-butter datastructures are offered, they feature:
//!
//!   * transparent access semantics, independent of currently used storage
//!   * automatic spill from exhausted compact storage to heap storage
//!   * recursive re-compaction
//!
//! This is used in `Kay` for:
//!
//!   * Storing actor state compactly in one place for cache coherency and easy persistence
//!   * Sending complex, dynamically-sized messages over boundaries
//!     such as actors, threads and the network

#![warn(missing_docs)]
#![feature(plugin)]
#![plugin(clippy)]
#![feature(conservative_impl_trait)]
#![feature(specialization)]

extern crate allocators;
mod pointer_to_maybe_compact;
#[macro_use]
mod compact;
mod compact_vec;
mod compact_dict;

pub use self::compact::Compact;
pub use self::compact_vec::CompactVec as CVec;
pub use self::compact_dict::CompactDict as CDict;
