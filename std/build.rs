use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

fn main() {
    let dest = Path::new(&env::var("OUT_DIR").unwrap()).join("modules.rs");
    let mut dest = File::create(dest).unwrap();

    write!(
        &mut dest,
        r###"// Generated by std/build.rs
use std::collections::HashMap;

pub fn modules() -> HashMap<String, String> {{
    let mut modules = HashMap::new();
"###
    )
    .unwrap();

    let root = Path::new(".").canonicalize().unwrap();
    walkdir::WalkDir::new(&root)
        .into_iter()
        .filter_entry(|e| {
            // Skip tests/ directory
            !e.file_type().is_dir() || e.file_name().to_str().map_or(true, |name| name != "tests")
        })
        .filter_map(|e| e.ok())
        .filter(|e| {
            // Collect .llrl
            e.file_type().is_file()
                && e.path()
                    .extension()
                    .map_or(false, |ext| ext.to_str().map_or(false, |ext| ext == "llrl"))
        })
        .for_each(|e| {
            let module_name = e
                .path()
                .canonicalize()
                .unwrap()
                .strip_prefix(&root)
                .unwrap()
                .with_extension("")
                .display()
                .to_string()
                .replace('\\', "/");
            let mut module_contents = String::new();
            File::open(e.path())
                .unwrap()
                .read_to_string(&mut module_contents)
                .unwrap();
            writeln!(
                &mut dest,
                r####"    modules.insert("{}".to_string(), r###"{}"###.to_string());"####,
                module_name, module_contents,
            )
            .unwrap();
        });

    writeln!(
        &mut dest,
        r###"
    modules
}}
"###
    )
    .unwrap();
}
