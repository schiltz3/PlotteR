#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use anyhow::{anyhow, Result};
use native_dialog::{FileDialog, MessageDialog, MessageType};
use std::ffi::OsString;
use std::fs::File;
use std::path::PathBuf;
use std::process;
use std::{env, vec};

#[derive(Debug)]
struct Column {
    header: String,
    column: Vec<f64>,
}

fn run() -> Result<()> {
    //parse_columns(path)?;
    Ok(())
}

fn parse_columns(file_path: PathBuf) -> Result<Vec<Column>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut columns: Vec<Column> = rdr
        .headers()?
        .into_iter()
        .map(|header| Column {
            header: header.into(),
            column: vec![],
        })
        .collect();

    for row in rdr.records() {
        for (column, raw_column) in columns.iter_mut().zip(row?.into_iter()) {
            column.column.push(raw_column.parse()?);
        }
    }
    Ok(columns)
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
