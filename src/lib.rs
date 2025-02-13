//! This crate exports `Arc` type alias which is either of:
//!
//! - `Arc` from `portable-atomic-util` - enabled if the following requirements are met:
//!   - use RUSTFLAGS with `--cfg portable_atomic_arc`
//!   - enable `critical-section` crate feature
//! - otherwise: `alloc::sync::alloc::sync::Arc`

#![no_std]

#[cfg(all(portable_atomic_arc, not(feature = "critical-section")))]
compile_error!("portable_atomic_arc cfg flag requires critical-section feature to be enabled");

// TODO: enforce this with exception for documentation builds for docs.rs (etc.)
// #[cfg(all(feature = "critical-section", not(portable_atomic_arc)))]
// compile_error!("critical-section feature requires RUSTFLAGS with `--cfg portable_atomic_arc`");

extern crate alloc;

#[cfg(portable_atomic_arc)]
/// `Arc` type alias - see crate documentation for more information
pub type Arc<T> = portable_atomic_util::Arc<T>;

#[cfg(not(portable_atomic_arc))]
/// `Arc` type alias - see crate documentation for more information
pub type Arc<T> = alloc::sync::Arc<T>;
