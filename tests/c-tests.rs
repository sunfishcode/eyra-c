#![feature(cfg_target_abi)]

use pretty_assertions::assert_eq;
use std::env::var;
use std::process::Command;
use std::sync::Once;

include!(concat!(env!("OUT_DIR"), "/c-tests-list.rs"));

fn test(name: &str) {
    let out_dir = var("OUT_DIR").unwrap();

    // It appears "cargo test" doesn't build our staticlib target. So build it.
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let mut cargo = Command::new("cargo");
        cargo.arg("build");
        #[cfg(not(debug_assertions))]
        cargo.arg("--release");
        assert!(cargo.status().unwrap().success());
    });

    // We're not running in build.rs here, so we need to configure some things
    // in `cc` manually.
    let mut build = cc::Build::new();
    build
        .target(&target_lexicon::HOST.to_string())
        .opt_level(0)
        .host(&target_lexicon::HOST.to_string());

    let source = format!("c-tests/src/{}", name);
    build
        .clone()
        .file(source)
        .include("c-tests/include")
        .compile(name);

    // Link the archive into an executable to produce the reference
    // executable.
    let mut compiler = build.clone().get_compiler().to_command();
    let exe = format!("{}/{}.reference", out_dir, name);
    compiler.arg(&format!("{}/lib{}.a", out_dir, name));
    compiler.arg("-o");
    compiler.arg(&exe);
    let output = compiler.output().unwrap();
    if !output.status.success() {
        eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        eprintln!("status: {}", output.status);
    }
    assert!(output.status.success());

    // Run the reference executable.
    eprintln!();
    eprintln!("Running reference executable {}:", exe);
    let mut command = Command::new(&exe);
    let reference_output = command.output().unwrap();
    if !reference_output.status.success() {
        eprintln!(
            "stdout: {}",
            String::from_utf8_lossy(&reference_output.stdout)
        );
        eprintln!(
            "stderr: {}",
            String::from_utf8_lossy(&reference_output.stderr)
        );
        eprintln!("status: {}", reference_output.status);
    }
    assert!(reference_output.status.success());

    // Link the archive into an executable to produce the Eyra executable.
    let mut compiler = build.clone().get_compiler().to_command();
    let exe = format!("{}/{}.eyra", out_dir, name);
    compiler.arg(&format!("{}/lib{}.a", out_dir, name));
    compiler.arg("-nostdlib");

    #[cfg(debug_assertions)]
    compiler.arg("target/debug/libc.a");
    #[cfg(not(debug_assertions))]
    compiler.arg("target/release/libc.a");

    compiler.arg("-Wl,--require-defined=main");
    compiler.arg("-o");
    compiler.arg(&exe);
    for lib_dir in var("LD_LIBRARY_PATH").unwrap().split(':') {
        compiler.arg("-L");
        compiler.arg(lib_dir);
    }

    let output = compiler.output().unwrap();
    if !output.status.success() {
        eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        eprintln!("status: {}", output.status);
    }
    assert!(output.status.success());

    // Run the Eyra executable.
    eprintln!();
    eprintln!("Running Eyra executable {}:", exe);
    let mut command = Command::new(&exe);
    let eyra_output = command.output().unwrap();
    if !eyra_output.status.success() {
        eprintln!("stdout: {}", String::from_utf8_lossy(&eyra_output.stdout));
        eprintln!("stderr: {}", String::from_utf8_lossy(&eyra_output.stderr));
        eprintln!("status: {}", eyra_output.status);
    }
    assert!(eyra_output.status.success());

    // Compare the output.
    eprintln!();
    assert_eq!(
        String::from_utf8(reference_output.stderr).unwrap(),
        String::from_utf8(eyra_output.stderr).unwrap()
    );
    assert_eq!(
        String::from_utf8(reference_output.stdout).unwrap(),
        String::from_utf8(eyra_output.stdout).unwrap()
    );
}
