//! # Borrow Checker Rules
//!
//! `Borrowing` is a concept which allows us to temporarily borrow data from the owner, and `reference` it following some rules
//!
//! Rust's `Borrow Checker` ensures we follow this `brrowing rules`:
//! - At any given time, you can have either one `mutable reference` or any number of `immutable references`
//! - References must always be valid
//!
//! ```rust
//! let s = String::from(":D"); // `s` ownes string data located on the heap
//! let r = &s; // `r` immutably borrows string from `s`
//! ```
//!
//! ---------------------------------------------------------------------------------------------------
//!
//! # References Scope
//!
//! All references have a specific scope, which starts from where it is introduced
//! and continues through the last time that reference is used:
//!
//! ```rust
//! let mut s = String::from(":D");
//! let r1 = &s; // new immutable reference
//! println!("{r1}"); // here `r1` goes out of scope
//! let r2 = &mut s; // new mutable reference
//! println!("{r2}"); // here `r2` goes out of scope
//! ```
//!
//! ---------------------------------------------------------------------------------------------------
//!
//! # Dangling References
//!
//! Rust's `Borrow Checker` ensures that all references are valid,
//! but in the next example we are trying to return a reference to the deallocated [`String`]:
//!
//! ```no_run
//! //fn dangle() -> &String {
//! //    let s = String::from(":D");
//! //    &s
//! //}
//! ```
//!
//! This code won't even compile, because we need to specify a `lifetime` of the returned &[`String`],
//! and this [`lifetimes`] concept prevents dangling references from existence
//!
//! ---------------------------------------------------------------------------------------------------
//!
//! # References Ownership
//!
//! References are totally valid variables, so they also obey to the `Ownership System`:
//! - `immutable references` implement [`Copy`] trait, but the `mutable references` do not
//!
//! ```rust
//! let s = String::from(":D");
//! let r1 = &s; // immutable reference to the `s`
//! let r2 = r1; // `r1` is copied into the `r2`
//!
//! let mut s = String::from(":D");
//! let r1 = &mut s; // mutable reference to the `s`
//! //let r2 = r1; // PROBLEM
//! ```
