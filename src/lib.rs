//! This crate exports `Arc` type alias which is either of:
//!
//! - `Arc` from `portable-atomic-util` - enabled if the following requirements are met:
//!   - use RUSTFLAGS with `--cfg portable_atomic_arc`
//!   - enable `critical-section` crate feature
//! - otherwise: `alloc::sync::alloc::sync::Arc`

// NOTE that the following options are not documented and should be considered unstable:
// - internal_select_portable_atomic_util feature - use Arc from portable-atomic-util
//   (should use `critical-section` feature to enable this)
// - unstable_maybe_portable_atomic_arc cfg option - use `critical-section` feature
//   (or internal_select_portable_atomic_util feature) to configure whether to
//   to use Arc from portable-atomic-util or from alloc::sync
//   NOTE the following:
//   - This unstable cfg option is added to make it easier to test portable-rustls with multiple combinations of
//     optional features without having to adjust the Rust cfg option flags for different cases.
//   - It is generally bad API practice for the API to depend only on cargo feature configuration,
//     as any component in a larger application or library could suddenly enable a cargo feature.
//     Considering that Arc is most commonly used from `alloc::sync` (aka `std::sync`),
//     using Arc from a different crate should be considered an API change.
//     It is for this reason that it is encouraged to use --cfg portable_atomic_arc together with
//     the more commonly-used `critical-section` feature name to explicitly enable
//     this API change.
//
// TODO: Update crate documentation to document these unstable options, with info from these notes.

#![no_std]

#[cfg(all(portable_atomic_arc, not(feature = "internal_select_portable_atomic_util")))]
compile_error!("portable_atomic_arc cfg flag requires critical-section (or internal_select_portable_atomic_util) feature to be enabled");

// TODO: enforce this with exception for documentation builds for docs.rs (etc.)
// #[cfg(all(feature = "critical-section", not(portable_atomic_arc)))]
// compile_error!("critical-section feature requires RUSTFLAGS with `--cfg portable_atomic_arc`");

extern crate alloc;

#[cfg(all(
    // (should use `critical-section` feature to enable this)
    feature = "internal_select_portable_atomic_util",
    any(portable_atomic_arc, unstable_maybe_portable_atomic_arc),
))]
/// `Arc` type alias - see crate documentation for more information
pub type Arc<T> = portable_atomic_util::Arc<T>;
#[cfg(not(feature = "internal_select_portable_atomic_util"))]
/// `Arc` type alias - see crate documentation for more information
pub type Arc<T> = alloc::sync::Arc<T>;
