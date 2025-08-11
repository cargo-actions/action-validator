#![allow(unused)]
// build.rs

use std::path::Path;
use std::process::Command;

fn main() {
    let schemas: Vec<&str> = vec![
        "https://www.schemastore.org/github-action.json",
        "https://www.schemastore.org/github-workflow.json",
    ];

    #[cfg(debug_assertions)]
    for schema in schemas {
        let filename = Path::new(schema).file_name().unwrap().to_str().unwrap();
        Command::new("curl")
            .arg("-Lo")
            .arg(format!("schemas/{filename}"))
            .arg(schema)
            .output()
            .unwrap();
        println!("cargo:rerun-if-changed=schemas/{schema}");
    }
}
