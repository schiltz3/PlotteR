extern crate csv;

use std::ffi::OsString;
use std::fs::File;
use std::process;
use std::{env, vec};

use anyhow::{anyhow, Result};

#[derive(Debug)]
struct Column {
    header: String,
    column: Vec<f64>,
}
fn run() -> Result<()> {
    parse_columns()?;
    Ok(())
}


fn parse_columns() -> Result<Vec<Column>> {
    let file_path = get_first_arg()?;
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

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
