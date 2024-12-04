// tests/unsafe_check.rs

#![forbid(unsafe_code)]

use std::fs;
use glob::glob;

#[test]
fn check_for_forbid_unsafe_code() {
    // Define the root directory of your project
    let root_directory = "."; // Current directory as the project root

    // Track files missing the `#![forbid(unsafe_code)]` attribute
    let mut files_missing_attribute = Vec::new();

    // Recursively search for all `.rs` files starting from the root directory
    for path in glob(&format!("{}/**/*.rs", root_directory))
        .expect("Failed to read glob pattern")
        .flatten()
    {
        // Read the file contents
        let contents = fs::read_to_string(&path).expect("Could not read file");

        // Skip files in `target` and `node_modules` directories, if any
        if path.to_string_lossy().contains("target") || path.to_string_lossy().contains("node_modules") {
            continue;
        }

        // Check if the file contains the `#![forbid(unsafe_code)]` attribute
        if !contents.contains("#![forbid(unsafe_code)]") {
            files_missing_attribute.push(path);
        }
    }

    // Fail the test if any files are missing the attribute
    if !files_missing_attribute.is_empty() {
        panic!(
            "The following files are missing #![forbid(unsafe_code)]:\n{}",
            files_missing_attribute
                .iter()
                .map(|path| path.to_string_lossy().to_string())
                .collect::<Vec<String>>()
                .join("\n")
        );
    }
}
