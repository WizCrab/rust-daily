//! # Sized Trait
//!
//! Every Programming Language has to somehow approach the problem of data types whose size is unknown at compile time,
//! known as dynamically sized types, DSTs, or Unsized.
//!
//! Rust has [`Sized`] trait, which is a part of the solution to this problem.
//! Understanding how the [`Sized`] trait and consequential `trait bounds` work is essential for dealing with
//! other complex Rust concepts.
//!
//! [`Sized`] is a trait that marks all types with a constant size known at compile time.
//!
//! All generic type parameters have an implicit trait bound: `T: Sized`,
//! but we can override this using `T: ?Sized` explicit bound:
//!
//! ```rust
//! struct Foo<T>(T);
//! struct Bar<T: ?Sized>(T);
//! // struct FooUse(Foo<[i32]>); // error: Sized is not implemented for [i32]
//! struct BarUse(Bar<[i32]>); // OK
//! ```
//!
//! ---------------------------------------------------------------------------------------------------
//!
//! # Common Unsized Types
//!
//! To understand [`Sized`] trait better, we can take a look at common Unsized data types at Rust:
//!
//! ```no_run
//! // str - string slice
//! // [T] - slice of type T
//! // dyn Trait - trait object
//! ```
//!
//! When we use `?Sized` data types, we have to refernece them,
//! because references are `Sized` and can be used as regular data types:
//!
//! ```rust
//! use std::fmt::Display;
//!
//! fn show<T: Display + ?Sized>(d: &T) { // we have to reference T
//!     println!("{}", d);
//! }
//!
//! show("?Sized");
//! ```
//!
//! ---------------------------------------------------------------------------------------------------
//!
//! # Referencing Unsized Types
//!
//! When referencing `Unsized` data types, we can use regular references like `&`,
//! or `smart pointers` like [`Box`], [`std::rc::Rc`], and others, or even raw pointers like `*const` and `*mut`:
//!
//! ```rust
//! let ref_arr: &[i32] = &[1, 2, 3];
//! let boxed_arr: Box<[i32]> = Box::new([1, 2, 3]);
//! ```
//!
//! But it is important to remember about `lifetimes` rules when working with `Unsized` types.
//!
//! Those references are stored in the stack, and have fixed known sizes, but the actual data may be located on the stack (like simple arrays),
//! or in the heap (like Box or Vec or String), or even in the code (text) section of the memory (like string literals).
//!
//! ---------------------------------------------------------------------------------------------------
//!
//! # Less Common Unsized Examples
//!
//! Here is an example of another `Unsized` data type [`std::path::Path`]:
//!
//! ```rust
//! use std::path::Path;
//!
//! fn assert_file_name(path: &Path, name: &str) { // Path and str are `Unsized` so we have to reference them
//!     assert_eq!(path.file_name().unwrap(), name);
//! }
//!
//! let path: &Path = Path::new("foo/bar.rs");
//! assert_file_name(path, "bar.rs");
//! ```
//!
//! ---------------------------------------------------------------------------------------------------
//!
//! # Self + Sized
//!
//! TODO: explain special case with `Self`, `dyn Trait` and [`Sized`]
