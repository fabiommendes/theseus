use ariadne::{CharSet, LabelAttach, IndexType};
use pyo3::{exceptions::PyValueError, prelude::*};

#[pyclass(eq)]
#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub(crate) inner: ariadne::Config,
}

impl Config {
    pub fn new(inner: ariadne::Config) -> Self {
        Config { inner }
    }
}

#[pymethods]
impl Config {
    #[new]
    #[pyo3(
        signature=(
            *, 
            cross_gap=false, 
            compact=false, 
            underlines=true, 
            multiline_arrows=true, 
            color=true,
            tab_width=4,
            ascii=false,
            byte_indexed=false,
            label_attach="middle")
    )]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn py_new(
        cross_gap: bool,
        compact: bool,
        underlines: bool,
        multiline_arrows: bool,
        color: bool,
        tab_width: usize,
        ascii: bool,
        byte_indexed: bool,
        label_attach: &str,
    ) -> PyResult<Self> {
        let inner = ariadne::Config::default()
            .with_cross_gap(cross_gap)
            .with_compact(compact)
            .with_underlines(underlines)
            .with_multiline_arrows(multiline_arrows)
            .with_color(color)
            .with_tab_width(tab_width)
            .with_char_set(
                if ascii {
                    CharSet::Ascii
                } else {
                    CharSet::Unicode
                }
            )
            .with_index_type(
                if byte_indexed {
                    IndexType::Byte 
                } else { 
                    IndexType::Char 
                }
            )
            .with_label_attach(parse_label_attach(label_attach)?);
        Ok(Config::new(inner))
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.inner)
    }
}

fn parse_label_attach(label_attach: &str) -> PyResult<ariadne::LabelAttach> {
    match label_attach {
        "start" => Ok(LabelAttach::Start),
        "middle" => Ok(LabelAttach::End),
        "right" => Ok(LabelAttach::Middle),
        _ => {
            let msg = "label_attach must be one of 'start', 'middle', 'end' or None";
            Err(PyValueError::new_err(msg))
        }
    }
}
