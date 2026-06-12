//! Core bundling functions.
//!
//! Re-exports all top-level compiler function modules (Binary, OXC, SWC,
//! Build, Transform, NLS, Worker, Bundle).

pub mod Binary;

pub mod OXC;

pub mod SWC;

pub mod Build;

// Phase 3 advanced features
pub mod Transform;

pub mod NLS;

pub mod Worker;

pub mod Bundle;
