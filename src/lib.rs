//! Public Registry API
//!
//! Implements read only access for all internal `Tablet`s.
//!
//! [`Tablet`] - Single title
//! [`Shard`] - Single note from `Tablet`
//! [`Registry`] - Collection of all `Tablet`s and `Shard`s available
//! [`Transcriptor`] - Special tool for reading `Tablet`s and `Shard`s in `markdown` format
//!
//! # Examples
//!
//! ```
//! use rust_daily::{Registry, Transcriptor, Tablet};
//!
//! let first_tablet = Registry::catalog().into_iter().next().unwrap();
//! let tablet_shard = first_tablet.shards().next().unwrap();
//!
//! let first_shard = Registry::heap().into_iter().next().unwrap();
//! assert_eq!(tablet_shard, first_shard);
//!
//! let shard_markdown_string = Transcriptor::read(&tablet_shard);
//! ```

use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;
use std::vec::IntoIter;

mod registry;

const TABLET_UNREADABLE_MSG: &str = "The tablet is expected to be readable!";
const TABLET_BROKEN_NAME_MSG: &str = "The tablet is expected to have a valid name!";

/// `Tablet` represents a single title. Contains only path to the title file, start and end lines, and methods representing common info
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tablet(&'static str, (usize, usize));
/// `Shard` is logically different from the [`Tablet`], but actually is just an alias. Represents one note from the [`Tablet`]
pub type Shard = Tablet;

impl Tablet {
    /// returns path to the note as `&str`
    pub fn path_str(&self) -> &'static str {
        self.0
    }

    /// which line to start reading from
    pub fn start(&self) -> usize {
        self.1.0
    }

    /// which line to stop reading on
    pub fn end(&self) -> usize {
        self.1.1
    }

    /// returns path to the note as &[`Path`]
    pub fn path(&self) -> &'static Path {
        Path::new(self.path_str())
    }

    /// count of lines in the note
    pub fn length(&self) -> usize {
        self.end() - self.start() + 1
    }

    /// name of the note. Originates from the filename
    pub fn name(&self) -> &'static str {
        self.path()
            .file_stem()
            .expect(TABLET_BROKEN_NAME_MSG)
            .to_str()
            .expect(TABLET_BROKEN_NAME_MSG)
    }

    /// returns [`Shards`] iterator over every [`Shard`] in this title
    pub fn shards(&self) -> Shards {
        self.into()
    }
}

/// `Transcriptor` represents a special tool for reading [`Tablet`]s and [`Shard`]s in the `markdown` format
pub struct Transcriptor;

impl Transcriptor {
    const SEPARATOR: &str = "-----";

    // formats one line to match `markdown` format
    fn line_fmt(line: &str) -> String {
        let mut formatted = String::new();
        formatted.push_str(
            line.replace("//!", "")
                .replace("```should_panic", "```rust")
                .replace("```no_run", "```rust")
                .trim(),
        );
        formatted.push('\n');
        formatted
    }

    // finds all separators in the `Tablet`
    fn segmentation(tablet: &Tablet) -> Result<Vec<(usize, usize)>> {
        let mut segments: Vec<(usize, usize)> = Vec::new();
        let mut ptr: usize = tablet.start();
        let data = File::open(tablet.path())?;
        for (num, line) in BufReader::new(data)
            .lines()
            .skip(tablet.start())
            .take(tablet.length())
            .enumerate()
        {
            if line?.contains(Self::SEPARATOR) {
                segments.push((ptr, num - 1));
                ptr = num + 1;
            }
        }
        segments.push((ptr, tablet.end()));
        Ok(segments)
    }

    /// reads the contents of [`Tablet`] or [`Shard`], formats it to match the `markdown` format, and returns as [`String`]
    pub fn read(tablet: &Tablet) -> Result<String> {
        let mut contents = String::new();
        let data = File::open(tablet.path())?;
        for line in BufReader::new(data)
            .lines()
            .skip(tablet.start())
            .take(tablet.length())
        {
            let line = Self::line_fmt(line?.as_str());
            contents.push_str(line.as_str());
        }
        Ok(contents.trim().to_string())
    }
}

/// `Shards` is an iterator over every [`Shard`] from the [`Tablet`]
#[derive(Debug, Clone)]
pub struct Shards {
    origin: Tablet,
    segments: IntoIter<(usize, usize)>,
}

impl Iterator for Shards {
    type Item = Shard;
    fn next(&mut self) -> Option<Self::Item> {
        Some(Tablet(self.origin.path_str(), self.segments.next()?))
    }
}

impl From<Tablet> for Shards {
    fn from(tablet: Tablet) -> Self {
        Shards {
            origin: tablet,
            segments: Transcriptor::segmentation(&tablet)
                .expect(TABLET_UNREADABLE_MSG)
                .into_iter(),
        }
    }
}

impl From<&Tablet> for Shards {
    fn from(tablet: &Tablet) -> Self {
        Self::from(*tablet)
    }
}

/// `Registry` represents a collection of all [`Tablet`]s and [`Shard`]s available
pub struct Registry;

impl Registry {
    fn tablet(path: &'static str) -> Tablet {
        let data = File::open(Path::new(path)).expect(TABLET_UNREADABLE_MSG);
        let length = BufReader::new(data).lines().count();
        Tablet(path, (0, length - 1))
    }

    /// returns all available [`Tablet`]s in the form of [`Vec`]. Use [`Transcriptor`] to read from the [`Tablet`]
    pub fn catalog() -> Vec<Tablet> {
        registry::TABLETS
            .iter()
            .map(|&path| Self::tablet(path))
            .collect()
    }

    /// returns all available [`Shard`]s in the form of [`Vec`]. Use [`Transcriptor`] to read from the [`Shard`]
    pub fn heap() -> Vec<Shard> {
        Self::catalog()
            .iter()
            .flat_map(|tablet| tablet.shards())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Prints every single Tablet
    #[test]
    fn print_catalog() {
        let cat = Registry::catalog();
        print!("{}", "\n".repeat(7));
        println!("{}", "==============================".repeat(4));
        println!("\nVec Catalog: {:#?} Len: {}\n", cat, cat.len());
        cat.iter().for_each(|tablet| {
            println!(
                "\n################# TABLET {name} #################\n[[{contents}]]",
                name = tablet.name().to_ascii_uppercase(),
                contents = Transcriptor::read(tablet).expect(TABLET_UNREADABLE_MSG)
            )
        });
        println!("\n{}", "==============================".repeat(4));
        print!("{}", "\n".repeat(7));
    }

    // Prints every single Shard
    #[test]
    fn print_heap() {
        let heap = Registry::heap();
        print!("{}", "\n".repeat(7));
        println!("{}", "==============================".repeat(4));
        println!("\nVec Heap: {:#?} Len: {}\n", heap, heap.len());
        heap.iter().for_each(|shard| {
            println!(
                "\n################# shard {name}-{line} #################\n[[{contents}]]",
                name = shard.name(),
                line = shard.start(),
                contents = Transcriptor::read(shard).expect(TABLET_UNREADABLE_MSG)
            )
        });
        println!("\n{}", "==============================".repeat(4));
        print!("{}", "\n".repeat(7));
    }
}
