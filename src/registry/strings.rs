//! # Strings In Rust
//!
//! Strings in Rust are not that complicated, as it may seem at first!
//!
//! Rust's [`std`] library provides a lot of functionality built off the [`String`] and [`str`] types:
//! - [`str`] - `Unsized` data type, representing `slice` ("view") of UTF-8 String, stored somewhere else
//! - [`String`] - `Sized` data type, representing a UTF-8 growable `String`, stored at the `Heap`
//!
//! ```rust
//! let string = String::from(":D"); // new String, stored at the heap, owned type
//! let slice = &string[..]; // reference to the `string`, aka string slice
//! ```
//!
//! ---------------------------------------------------------------------------------------------------
//!
//! # String Internals
//!
//! Rust [`String`]s are UTF-8 encoded, and based on the [`Vec<u8>`].
//! Here lies the main complexity of [`String`]s: Every single [`char`] can be represented as a single `byte`, or as a sequence of `bytes`.
//! This means that this is unsafe to directly slice a string by indexes: we can specify an index, that will be in the middle of the UTF-8 char bytes sequence:
//!
//! ```rust
//! let s = String::from("🦀"); // this string has single char, but 4 bytes
//! let bytes: Vec<u8> = s.into_bytes(); // [240, 159, 166, 128]
//! ```
//!
//! ---------------------------------------------------------------------------------------------------
//!
//! # String Slices
//!
//! We can reference a part of a [`String`] using [`str`] string slices, which is a `Unsized` type.
//! But it is impartant to remember that a single UTF-8 [`char`] can be represented as a `sequence` of several `bytes`:
//!
//! ```rust
//! let s = String::from("Hello, Strings!");
//! assert_eq!(&s[0..5], "Hello"); // works fine
//!
//! let s = String::from("Crab: 🦀");
//! // println!("{}", &s[6..=6]); // Error: not a char boundary
//! assert_eq!(&s[6..10], "🦀"); // crab emoji is 4 bytes long
//! ```
//!
//! ---------------------------------------------------------------------------------------------------
