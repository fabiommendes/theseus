use ariadne::Color::*;
use pyo3::{exceptions::PyTypeError, prelude::*, types::PyString};
use std::hash::Hash;

#[pyclass(frozen, eq, hash)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub(crate) inner: ariadne::Color,
}

impl Color {
    pub fn new(inner: ariadne::Color) -> Self {
        Color { inner }
    }

    fn new_str(name: &str) -> PyResult<Self> {
        match name {
            "primary" => Ok(Self::new(Primary)),
            "black" => Ok(Self::new(Black)),
            "white" => Ok(Self::new(White)),
            "red" => Ok(Self::new(Red)),
            "green" => Ok(Self::new(Green)),
            "blue" => Ok(Self::new(Blue)),
            "yellow" => Ok(Self::new(Yellow)),
            "cyan" => Ok(Self::new(Cyan)),
            "magenta" => Ok(Self::new(Magenta)),
            "bright-black" => Ok(Self::new(BrightBlack)),
            "bright-white" => Ok(Self::new(BrightWhite)),
            "bright-red" => Ok(Self::new(BrightRed)),
            "bright-green" => Ok(Self::new(BrightGreen)),
            "bright-blue" => Ok(Self::new(BrightBlue)),
            "bright-yellow" => Ok(Self::new(BrightYellow)),
            "bright-cyan" => Ok(Self::new(BrightCyan)),
            "bright-magenta" => Ok(Self::new(BrightMagenta)),
            _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Unknown color: {}",
                name
            ))),
        }
    }

    fn new_fixed(id: u8) -> Self {
        Self::new(Fixed(id))
    }
}

#[pymethods]
impl Color {
    #[staticmethod]
    fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(Rgb(r, g, b))
    }

    #[new]
    fn py_new(arg: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(name) = arg.downcast::<PyString>() {
            let str = name.to_str()?;
            Self::new_str(str)
        } else if let Ok(id) = arg.extract::<u8>() {
            Ok(Self::new_fixed(id))
        } else {
            let msg = "Expected a string, tuple of three u8s, or a single u8";
            Err(PyTypeError::new_err(msg))
        }
    }

    pub fn __repr__(&self) -> String {
        self.__str__()
    }

    pub fn __str__(&self) -> String {
        match self.inner {
            Fixed(u8) => format!("Color({})", u8),
            Rgb(r, g, b) => format!("Color.rgb({r}, {g}, {b})"),
            Primary => "Color('primary')".into(),
            Black => "Color('black')".into(),
            Red => "Color('red')".into(),
            Green => "Color('green')".into(),
            Yellow => "Color('yellow')".into(),
            Blue => "Color('blue')".into(),
            Magenta => "Color('magenta')".into(),
            Cyan => "Color('cyan')".into(),
            White => "Color('white')".into(),
            BrightBlack => "Color('bright-black')".into(),
            BrightRed => "Color('bright-red')".into(),
            BrightGreen => "Color('bright-green')".into(),
            BrightYellow => "Color('bright-yellow')".into(),
            BrightBlue => "Color('bright-blue')".into(),
            BrightMagenta => "Color('bright-magenta')".into(),
            BrightCyan => "Color('bright-cyan')".into(),
            BrightWhite => "Color('bright-white')".into(),
        }
    }
}

impl Hash for Color {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self.inner {
            Fixed(id) => id.hash(state),
            Rgb(r, g, b) => (r, g, b).hash(state),
            Primary => "primary".hash(state),
            Black => "black".hash(state),
            Red => "red".hash(state),
            Green => "green".hash(state),
            Yellow => "yellow".hash(state),
            Blue => "blue".hash(state),
            Magenta => "magenta".hash(state),
            Cyan => "cyan".hash(state),
            White => "white".hash(state),
            BrightBlack => "bright-black".hash(state),
            BrightRed => "bright-red".hash(state),
            BrightGreen => "bright-green".hash(state),
            BrightYellow => "bright-yellow".hash(state),
            BrightBlue => "bright-blue".hash(state),
            BrightMagenta => "bright-magenta".hash(state),
            BrightCyan => "bright-cyan".hash(state),
            BrightWhite => "bright-white".hash(state),
        }
    }
}
