#![feature(proc_macro_internals)]
#![feature(proc_macro_span)]
#![feature(proc_macro_diagnostic)]

extern crate proc_macro;
extern crate goblin;
extern crate syn;
extern crate tempfile;

use proc_macro::bridge::server::SameThread;
use std::path::PathBuf;
use goblin::Object;
use std::fs::{File, create_dir, canonicalize};
use std::io::{Read, Error, ErrorKind};
use std::{io, fs};
use proc_macro::bridge::client::ProcMacro;
use std::process::Command;
use std::io::Write;
use tempfile::TempDir;

mod rustc_server;
mod dynamic_lib;

use dynamic_lib::DynamicLibrary;

static NEW_REGISTRAR_SYMBOL: &str = "__rustc_proc_macro_decls_";
const EXEC_STRATEGY: SameThread = SameThread;

fn read_bytes(file: &PathBuf) -> Option<Vec<u8>> {
    let mut fd = File::open(file).ok()?;
    let mut buffer = Vec::new();
    fd.read_to_end(&mut buffer).ok()?;

    Some(buffer)
}

fn get_symbols_from_lib(file: &PathBuf) -> Option<Vec<String>> {
    let buffer = read_bytes(file)?;
    let object = Object::parse(&buffer).ok()?;

    return match object {
        Object::Elf(elf) => {
            let symbols = elf.dynstrtab.to_vec().ok()?;
            let names = symbols.iter().map(|s| s.to_string()).collect();

            Some(names)
        }

        _ => None
    };
}

fn is_derive_registrar_symbol(symbol: &str) -> bool {
    symbol.contains(NEW_REGISTRAR_SYMBOL)
}

fn find_registrar_symbol(file: &PathBuf) -> Option<String> {
    let symbols = get_symbols_from_lib(file)?;

    symbols
        .iter()
        .find(|s| is_derive_registrar_symbol(s))
        .map(|s| s.clone())
}

fn parse_string(code: &str) -> Option<proc_macro2::TokenStream> {
    syn::parse_str(code).ok()
}

fn setup_temp_proc_macro_project(root_dir: &PathBuf) -> io::Result<()> {
    let mut cargo_toml = File::create(root_dir.join("Cargo.toml"))?;
    write!(
        &mut cargo_toml,
        "{}",
        r#"
[package]
name = "test_proc_macro"
version = "0.1.0"

[lib]
proc-macro = true

[dependencies]
    "#
    )?;

    create_dir(root_dir.join("src"))?;
    let mut lib_file = File::create(root_dir.join("src").join("lib.rs"))?;
    write!(
        &mut lib_file,
        "{}",
        r#"
extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn make_answer_macro(input: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
    "#
    )?;

    Ok(())
}

fn compile_proc_macro(dir: &PathBuf) -> io::Result<PathBuf> {
    Command::new("cargo")
        .current_dir(dir)
        .arg("+nightly")
        .arg("build")
        .status()?;

    // FIXME change for windows

    let buf = dir
        .join("target")
        .join("debug")
        .join("libtest_proc_macro.so");
    if buf.is_file() {
        Ok(buf)
    } else {
        Err(io::Error::from(ErrorKind::NotFound))
    }
}

fn find_test_proc_macro() -> io::Result<PathBuf> {
    let mut test_exe = std::env::current_exe()?;
    test_exe.pop();

    for entry in fs::read_dir(&test_exe)? {
        let entry = entry?;
        let name = entry.file_name().to_str().unwrap().to_string();
        if entry.path().is_file()
            && name.starts_with("libtest_proc_macro")
            && name.ends_with(".so") {
            return Ok(entry.path());
        }
    }

    Err(io::Error::from(ErrorKind::NotFound))
}

#[test]
fn test_getset_expansion() -> io::Result<()> {
//    let tmp_dir = TempDir::new()?;
//    setup_temp_proc_macro_project(&tmp_dir.path().to_path_buf())?;
//    let proc_macro_lib = canonicalize(compile_proc_macro(&tmp_dir.path().to_path_buf())?)?;
    let proc_macro_lib = find_test_proc_macro()?;

    let symbol_name = find_registrar_symbol(&proc_macro_lib).expect(
        &format!("Cannot find registrar symbol in file {:?}", &proc_macro_lib)
    );

    let lib = DynamicLibrary::open(Some(&proc_macro_lib)).expect("Cannot open dynamic library!");

    let proc_macros = unsafe {
        let symbol = lib.symbol(&symbol_name).expect("No such symbol found!");
        std::mem::transmute::<*mut u8, &&[ProcMacro]>(symbol)
    };

    for proc_macro in *proc_macros {
        match proc_macro {
            ProcMacro::Bang { client, .. } => {
                let result = client.run(
                    &EXEC_STRATEGY,
                    rustc_server::Rustc::default(),
                    parse_string("struct S{}").expect("Cannot parse code"),
                );

                assert!(result.is_err(), "rustc_server should panic with unimplemented")
            }

            _ => { panic!("Not expected proc macro!") }
        }
    }

    Ok(())
}

fn main() {
    println!("Hello, world!");
}
