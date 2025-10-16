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
//! # Ownership rules
//!
//! According to the Rust Book, there is 3 main rules:
//! - Each value in Rust has an owner
//! - There can only be one owner at a time
//! - When the owner goes out of scope, the value will be dropped
//!
//! ---------------------------------------------------------------------------------------------------
//!
//! # Ownership System In Action
//!
//! ```rust
//! let s = String::from(":D"); // `s` owns string data stored at the heap
//! // `move` operation:
//! let s2 = s; // rust performs shallow copy of `s` into `s2`, and invalidates `s`, so `s2` is the new and only owner
//! // println!("{s}"); // Error, `s` was moved
//! // at the end `s` goes out of scope, so it's memory is freed
//! ```
//!
//! ---------------------------------------------------------------------------------------------------
//!
//! # Ownership Related Traits
//!
//! Rust has a special [`Copy`] trait, which tells us, if a type is a simple stack-based type.
//! Why is this important? If a type implements [`Copy`] trait, this means that:
//! - it's value is stored at the stack
//! - this type can't implement [`Drop`] trait, so no specail actions needed to free the memory
//! - variables that use this type do not move, but rather are trivially copied
//!
//! ```rust
//! fn takes_ownership(s: String) { // `s` is the new and only owner
//!     println!("{s}");
//! } // `s` goes out of scope, so it's memory is freed
//!
//! fn makes_copy(i: i32) { // `i` is a copy of the `x` variable
//!     println!("{i}");
//! }
//!
//! let s = String::from(":D"); // `s` owns string data stored at the `heap`
//! takes_ownership(s); // `s` moved into the function's first argument
//!
//! let x = 5; // `x` owns 5, but `i32` implements `Copy` trait
//! makes_copy(x); // `x` is copied into the `i`
//! ```
//!
//! If the type does not impplement [`Copy`] trait, the variable is `moved`,
//! but if the type does implement this trait, then the variable is trivially `copied`
//!
//! ---------------------------------------------------------------------------------------------------
//!
//! # Non-Obvious Ownership Interactions
//!
//! When using `for` instruction for example to iterate over every element in a collection,
//! rust implicitly calles `into_iter` method on that collection, and this method takes the ownership:
//!
//! ```rust
//! let v = vec![1, 2, 3, 4, 5];
//! let mut sum = 0;
//! for i in v { // here rust calles `v.into_iter()`, and `v` moves out
//!     sum += i;
//! }
//! // println!("sum:{}, elements:{:?}", sum, v) // Error: borrow of moved `v`
//! ```
//!
//! `+` operatior functionality provides [`std::ops::Add`] trait and it's method `add` takes ownership of `self`,
//! so when concatenating Strings using `+` operatior, first part is moved:
//!
//! ```rust
//! let s1 = String::from(":");
//! let s2 = s1 + "D"; // s1 has been moved here and can no longer be used
//! // println!("{s1}") // Error: borrow of moved `s1`
//! ```
