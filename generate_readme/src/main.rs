//! Internal use only

use cargo_readme::generate_readme;
use std::path::Path;
use std::fs::File;
use regex::Regex;

fn main() {
    let source_path   = Path::new("src/lib.rs");
    let template_path = Path::new("README.tpl");
    let mut source    = File::open(&source_path).unwrap();
    let mut template  = File::open(&template_path).unwrap();

    let mut readme = generate_readme(
        &Path::new("."),
        &mut source,
        Some(&mut template),
        true, // title
        true, // badges
        true, // licence
        true, // indent headings
    ).unwrap();

    // Fix broken links in README by changing curved brackets to square brackets.
    // E.g.:
    //     [`.only()`](Ansi::only)
    //     [`.only()`][Ansi::only]
    readme = Regex::new(r"\[(`[^\]]+`)\]\(([^\)]+)\)").unwrap().replace_all(&readme, "[$1][$2]").to_string();

    println!("{}", readme)
}
