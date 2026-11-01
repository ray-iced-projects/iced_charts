//! CSV table reader for chart data sources.
//!
//! Supports both *tidy* files (unique header names, e.g. OHLC) selected by name,
//! and *paired/wide* files (duplicate header names, e.g. scatter x/y pairs)
//! selected by zero-based column index.

use std::path::Path;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error raised while loading or querying a CSV data source.
#[derive(Debug, Error)]
pub enum CsvError {
    #[error("failed to read CSV file '{0}': {1}")]
    Io(String, String),
    #[error("column '{0}' not found in CSV header")]
    ColumnNotFound(String),
    #[error("column index {0} out of range (file has {1} columns)")]
    IndexOutOfRange(usize, usize),
}

/// Selects a CSV column by header name or by zero-based index.
///
/// Index selection is required for files with duplicate header names.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ColumnRef {
    /// Select by header name (first match wins).
    Name(String),
    /// Select by zero-based column index.
    Index(usize),
}

/// A parsed CSV file: header names plus string-valued rows.
#[derive(Debug, Clone)]
pub struct CsvTable {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl CsvTable {
    /// Read and parse a CSV file from disk.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, CsvError> {
        let path_ref = path.as_ref();
        let display = path_ref.display().to_string();

        let mut reader = csv::ReaderBuilder::new()
            .trim(csv::Trim::All)
            .from_path(path_ref)
            .map_err(|e| CsvError::Io(display.clone(), e.to_string()))?;

        let headers = reader
            .headers()
            .map_err(|e| CsvError::Io(display.clone(), e.to_string()))?
            .iter()
            .map(|s| s.to_string())
            .collect();

        let mut rows = Vec::new();
        for record in reader.records() {
            let record = record.map_err(|e| CsvError::Io(display.clone(), e.to_string()))?;
            rows.push(record.iter().map(|s| s.to_string()).collect());
        }

        Ok(CsvTable { headers, rows })
    }

    /// Resolve a column selector to its zero-based index.
    fn resolve_index(&self, col: &ColumnRef) -> Result<usize, CsvError> {
        match col {
            ColumnRef::Index(i) => {
                if *i < self.headers.len() {
                    Ok(*i)
                } else {
                    Err(CsvError::IndexOutOfRange(*i, self.headers.len()))
                }
            }
            ColumnRef::Name(name) => self
                .headers
                .iter()
                .position(|h| h == name)
                .ok_or_else(|| CsvError::ColumnNotFound(name.clone())),
        }
    }

    /// Extract a column as raw strings.
    pub fn column_strings(&self, col: &ColumnRef) -> Result<Vec<String>, CsvError> {
        let idx = self.resolve_index(col)?;
        Ok(self
            .rows
            .iter()
            .map(|row| row.get(idx).cloned().unwrap_or_default())
            .collect())
    }

    /// Extract a column as numeric values; unparseable or empty cells become `None`.
    pub fn column_f64(&self, col: &ColumnRef) -> Result<Vec<Option<f64>>, CsvError> {
        let idx = self.resolve_index(col)?;
        Ok(self
            .rows
            .iter()
            .map(|row| {
                row.get(idx)
                    .and_then(|cell| {
                        let t = cell.trim();
                        if t.is_empty() { None } else { t.parse::<f64>().ok() }
                    })
            })
            .collect())
    }
}
