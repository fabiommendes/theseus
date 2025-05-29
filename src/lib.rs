use pyo3::prelude::*;
use std::{ops::Range, sync::Arc};

mod color;
use color::Color;
mod color_generator;
use color_generator::ColorGenerator;
mod report;
use report::Report;
mod label;
use label::Label;
mod config;
use config::Config;

// Rust type definitions
pub(crate) type _Span = (Arc<str>, Range<usize>);
pub(crate) type _Label = ariadne::Label<_Span>;
pub(crate) type _Report<'a> = ariadne::Report<'a, _Span>;

#[pymodule]
fn theseus(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Color>()?;
    m.add_class::<ColorGenerator>()?;
    m.add_class::<Report>()?;
    m.add_class::<Label>()?;
    m.add_class::<Config>()?;
    Ok(())
}
