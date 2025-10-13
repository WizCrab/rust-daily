use std::path::Path;
fn main() {
    let p = Path::new("/foo/bar.txt");
    println!("name: {}", p.file_name().unwrap().to_str().unwrap());
    println!("prefix: {}", p.file_stem().unwrap().to_str().unwrap());
}
