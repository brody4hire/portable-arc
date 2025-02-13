#![no_std]

extern crate alloc;

pub type Arc<T> = alloc::sync::Arc<T>;
