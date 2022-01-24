#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use anyhow::{anyhow, Result};

use std::ffi::OsString;


use std::process;
use std::{env};

fn run() -> Result<()> {
    //parse_columns(path)?;
    Ok(())
}

fn get_first_arg() -> Result<OsString> {
    match env::args_os().nth(1) {
        None => Err(anyhow!("expected 1 argument, but got none:")),
        Some(file_path) => Ok(file_path),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
    let app = plottr::TemplateApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
