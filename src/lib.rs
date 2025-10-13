use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;
use std::vec::IntoIter;

mod registry;

const TABLET_UNREADABLE_MSG: &str = "The tablet is expected to be readable!";
const TABLET_BROKEN_NAME_MSG: &str = "The tablet is expected to have a valid name!";

pub struct Transcriptor;

impl Transcriptor {
    const SEPARATOR: &str = "-----";

    fn line_fmt(line: &str) -> String {
        let mut formatted = String::new();
        formatted.push_str(
            line.replace("//!", "")
                .replace("```should_panic", "```rust")
                .trim(),
        );
        formatted.push('\n');
        formatted
    }

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tablet(&'static str, (usize, usize));
pub type Shard = Tablet;

impl Tablet {
    pub fn path_str(&self) -> &'static str {
        self.0
    }

    pub fn start(&self) -> usize {
        self.1.0
    }

    pub fn end(&self) -> usize {
        self.1.1
    }

    pub fn path(&self) -> &'static Path {
        Path::new(self.path_str())
    }

    pub fn length(&self) -> usize {
        self.end() - self.start() + 1
    }

    pub fn name(&self) -> &'static str {
        self.path()
            .file_stem()
            .expect(TABLET_BROKEN_NAME_MSG)
            .to_str()
            .expect(TABLET_BROKEN_NAME_MSG)
    }

    pub fn shards(&self) -> Shards {
        self.into()
    }
}

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

pub struct Registry;

impl Registry {
    fn tablet(path: &'static str) -> Tablet {
        let data = File::open(Path::new(path)).expect(TABLET_UNREADABLE_MSG);
        let length = BufReader::new(data).lines().count();
        Tablet(path, (0, length - 1))
    }

    pub fn catalog() -> Vec<Tablet> {
        registry::TABLETS
            .iter()
            .map(|&path| Self::tablet(path))
            .collect()
    }

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
