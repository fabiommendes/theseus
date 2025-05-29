use crate::color::Color;
use ariadne as rs;
use pyo3::prelude::*;

#[pyclass]
pub struct ColorGenerator {
    pub(crate) inner: rs::ColorGenerator,
}

#[pymethods]
impl ColorGenerator {
    #[new]
    fn new() -> Self {
        ColorGenerator {
            inner: rs::ColorGenerator::new(),
        }
    }

    fn __next__(&mut self) -> Color {
        Color::new(self.inner.next())
    }

    fn __iter__(this: PyRef<Self>) -> PyRef<Self> {
        this
    }

    fn next(&mut self) -> Color {
        self.__next__()
    }
}
