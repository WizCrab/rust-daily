//! # Ownership System
//!
//! `Ownership System` is one of the most important concepts in Rust.
//!
//! This system originates from the `RAII` pattern in C++ language, which stands for `Resource Acquisition Is Initialization`.
//!
//! In short this means that every resource allocation is tied to the object lifetime.
//! Because of `Ownership System`, we don't have to allocate and free resources ourselves, which leads to the greater memory safety:
//!
//! ```rust
//! let s = String::from(":D"); // allocates memory for string on the heap, which is tied to the variable `s` lifetime
//! ```
//!
//! ---------------------------------------------------------------------------------------------------
//!
//! TODO: deeper explanation
