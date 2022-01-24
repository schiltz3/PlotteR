use anyhow::{Result};
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct Column {
    pub header: String,
    pub column: Vec<f64>,
}

pub(crate) fn parse_columns(file_path: PathBuf) -> Result<Vec<Column>> {
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
pub struct PivotIter<Iter>(pub Vec<Iter>);

impl<I, O> Iterator for PivotIter<I>
where
    I: Iterator<Item = O>,
{
    type Item = Vec<O>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter_mut().map(|it| it.next()).collect()
    }
}
