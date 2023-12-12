use camino::Utf8Path;
use std::env::var;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Pass -nostartfiles to the linker.
    println!("cargo:rustc-link-arg=-nostartfiles");

    let out_dir =
        PathBuf::from(var("OUT_DIR").expect("The OUT_DIR environment variable must be set"));
    let output = out_dir.join("c-tests-list.rs");
    let mut output = File::create(output).unwrap();

    // Find all the tests in c-tests/src.
    let src = Utf8Path::new("c-tests/src");
    for entry in src.read_dir_utf8().unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        if file_name.starts_with('.') {
            continue;
        }

        let test_name = file_name.to_owned();
        let test_name = test_name.replace(".", "_");
        let test_name = test_name.replace("-", "_");

        writeln!(&mut output, "#[test]").unwrap();
        writeln!(&mut output, "fn test_{}() {{", test_name).unwrap();
        writeln!(&mut output, "    test(\"{}\");", file_name).unwrap();
        writeln!(&mut output, "}}").unwrap();
        writeln!(&mut output).unwrap();

        println!("cargo:rerun-if-changed={}", src.join(file_name));
    }
    println!("cargo:rerun-if-changed={}", src);

    let inc = Utf8Path::new("c-tests/include");
    for entry in src.read_dir_utf8().unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        if file_name.starts_with('.') {
            continue;
        }

        println!("cargo:rerun-if-changed={}", inc.join(file_name));
    }
    println!("cargo:rerun-if-changed={}", inc);
}
