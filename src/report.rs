use std::io::Write;
use std::ops::Range;
use std::sync::Arc;

use crate::_Report;
use crate::color::Color;
use crate::config::Config;
use crate::label::Label;
use pyo3::exceptions::{PyFileNotFoundError, PyTypeError, PyUnicodeDecodeError, PyValueError};
use pyo3::types::{PyDict, PyIterator, PyList, PyString};
use pyo3::{prelude::*, IntoPyObjectExt};

#[pyclass]
pub struct Report {
    source: Source,
    span: Range<usize>,
    config: Config,
    code: Option<String>,
    message: Option<String>,
    kind: ReportKind,
    labels: Vec<Label>,
    notes: Vec<String>,
    helps: Vec<String>,
    files: Vec<Source>,
    colors: ariadne::ColorGenerator,
}

impl Report {
    pub fn new(source: Source, span: Range<usize>, config: Config) -> Self {
        Report {
            source,
            span,
            config,
            code: None,
            message: None,
            kind: ReportKind::Error,
            labels: Vec::new(),
            notes: Vec::new(),
            helps: Vec::new(),
            files: Vec::new(),
            colors: ariadne::ColorGenerator::new(),
        }
    }

    pub fn build_ariadne_report(&self) -> _Report<'_> {
        let path = self.source.path.clone();
        let mut builder =
            ariadne::Report::build(self.kind.to_ariadne(), (path.clone(), self.span.clone()));

        builder = builder.with_config(self.config.inner);
        if let Some(code) = self.code.as_ref() {
            builder = builder.with_code(code);
        }
        if let Some(msg) = self.message.as_ref() {
            builder = builder.with_message(msg);
        }

        builder.with_notes(self.notes.clone());
        builder.with_helps(self.helps.clone());
        for label in &self.labels {
            builder = builder.with_label(label.to_ariadne_with_default(path.clone()));
        }

        builder.finish()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_params(
        mut self,
        code: Option<String>,
        message: Option<String>,
        kind: ReportKind,
        labels: Vec<Label>,
        notes: Vec<String>,
        helps: Vec<String>,
        files: Vec<Source>,
    ) -> Self {
        self.code = code;
        self.message = message;
        self.kind = kind;
        self.labels = labels;
        self.notes = notes;
        self.helps = helps;
        self.files = files;
        self
    }
    pub fn prepare_files(&self) -> Vec<(Arc<str>, Arc<str>)> {
        let target = self.source.pair();
        let mut files = vec![target];
        for source in &self.files {
            files.push(source.pair());
        }
        files
    }
}

#[pymethods]
impl Report {
    #[new]
    #[pyo3(signature=(source, start, end, code=None, message=None, kind=None, color=None, labels=vec![], notes=vec![], helps=vec![], config=Config::new(ariadne::Config::default()), files=not_given()))]
    #[allow(clippy::too_many_arguments)]
    fn py_new(
        source: &Bound<'_, PyAny>,
        start: usize,
        end: usize,
        code: Option<String>,
        message: Option<String>,
        kind: Option<&str>,
        color: Option<Color>,
        labels: Vec<Label>,
        notes: Vec<String>,
        helps: Vec<String>,
        config: Config,
        files: PyObject,
    ) -> PyResult<Self> {
        let span = start..end;
        let source = Source::from_python(source)?;
        let kind = ReportKind::from_params(kind, color)?;
        let files = parse_files(files)?;

        let mut report = Report::new(source, span, config);
        report = report.set_params(code, message, kind, labels, notes, helps, files);
        Ok(report)
    }

    #[pyo3(signature=(stderr=false))]
    fn print(&self, stderr: bool) -> PyResult<()> {
        let report = self.build_ariadne_report();
        let files = ariadne::sources(self.prepare_files());
        if stderr {
            let writer = PyWriter::stderr()?;
            report.write(files, writer)?;
        } else {
            let writer = PyWriter::stdout()?;
            report.write_for_stdout(files, writer)?;
        }
        Ok(())
    }

    fn color(&mut self) -> Color {
        Color::new(self.colors.next())
    }

    fn add_label(&mut self, label: Label) {
        self.labels.push(label);
    }

    #[pyo3(signature=(start, end, *, path=None, message=None, color=None, order=None, priority=None))]
    #[allow(clippy::too_many_arguments)]
    fn label(
        &mut self,
        start: usize,
        end: usize,
        path: Option<&str>,
        message: Option<&str>,
        color: Option<Color>,
        order: Option<i32>,
        priority: Option<i32>,
    ) -> PyResult<Label> {
        let color = color.or_else(|| Some(Color::new(self.colors.next())));
        let label = Label::py_new(start, end, path, message, color, order, priority)?;
        self.labels.push(label.clone());
        Ok(label)
    }

    fn add_note(&mut self, note: String) {
        self.notes.push(note);
    }

    fn add_help(&mut self, help: String) {
        self.helps.push(help);
    }
}

#[derive(Clone)]
pub enum ReportKind {
    Error,
    Warning,
    Advice,
    // We must own the custom kind string to avoid lifetime issues
    Custom(Arc<str>, Color),
}

impl ReportKind {
    fn from_params(name: Option<&str>, color: Option<Color>) -> PyResult<Self> {
        match (name, color) {
            (None, None) | (Some("error"), None) => Ok(ReportKind::Error),
            (Some("warning"), None) | (Some("warn"), None) => Ok(ReportKind::Warning),
            (Some("advice"), None) => Ok(ReportKind::Advice),
            (None, Some(_)) => {
                let msg = "Color specified without a name for custom report kind";
                Err(PyValueError::new_err(msg))
            }
            (Some(name), Some(color)) => {
                if name.is_empty() {
                    let msg = "Custom report kind name cannot be empty";
                    return Err(PyValueError::new_err(msg));
                } else if name == "error" || name == "warning" || name == "advice" || name == "warn"
                {
                    let msg = format!("Cannot set custom color for {}", name);
                    return Err(PyValueError::new_err(msg));
                }
                Ok(ReportKind::Custom(name.into(), color))
            }
            (Some(_), None) => {
                let msg = "Must specify a color for custom report kind";
                Err(PyValueError::new_err(msg))
            }
        }
    }

    fn to_ariadne(&self) -> ariadne::ReportKind<'_> {
        match self {
            ReportKind::Error => ariadne::ReportKind::Error,
            ReportKind::Warning => ariadne::ReportKind::Warning,
            ReportKind::Advice => ariadne::ReportKind::Advice,
            ReportKind::Custom(name, color) => ariadne::ReportKind::Custom(name, color.inner),
        }
    }
}

pub struct Source {
    path: Arc<str>,
    source: Arc<str>,
}

impl Source {
    fn new(path: Arc<str>, source: Arc<str>) -> Self {
        Source { path, source }
    }

    fn from_python(source: &Bound<'_, PyAny>) -> PyResult<Self> {
        // Strings
        if let Ok(source) = source.downcast::<PyString>() {
            let source = source.to_str()?.into();
            Ok(Source::new("<string>".into(), source))
        // Paths
        } else if let Ok(path) = from_path(source) {
            let source = std::fs::read_to_string(path.to_string()).map_err(|e| {
                let msg = format!("Failed to read file '{}': {}", path, e);
                PyFileNotFoundError::new_err(msg)
            })?;
            Ok(Source::new(path, source.into()))
        // File-like
        } else if let Ok(source) = from_file_like(source) {
            Ok(source)
        } else {
            let msg = "Expected a string, path or file-like object";
            Err(PyErr::new::<PyTypeError, _>(msg))
        }
    }

    fn pair(&self) -> (Arc<str>, Arc<str>) {
        (self.path.clone(), self.source.clone())
    }
}

pub struct PyWriter {
    fd: PyObject,
}

impl PyWriter {
    fn new_sys_file(name: &str) -> PyResult<Self> {
        Python::with_gil(|py| {
            let sys = PyModule::import(py, "sys")?;
            let fd = sys.getattr(name)?;
            Ok(PyWriter { fd: fd.into() })
        })
    }

    pub fn stdout() -> PyResult<Self> {
        Self::new_sys_file("stdout")
    }

    pub fn stderr() -> PyResult<Self> {
        Self::new_sys_file("stderr")
    }
}

impl Write for PyWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Python::with_gil(|py| {
            let fd = self.fd.bind(py);
            let data = std::str::from_utf8(buf).map_err(|_| {
                let msg = "Invalid UTF-8 input";
                PyUnicodeDecodeError::new_err(msg)
            })?;
            fd.call_method1("write", (data,))?;
            Ok(buf.len())
        })
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Python::with_gil(|py| {
            let fd = self.fd.bind(py);
            fd.call_method0("flush")?;
            Ok(())
        })
    }
}

fn not_given() -> PyObject {
    Python::with_gil(|py| {
        let bound = PyList::empty(py).into_any();
        bound.into_py_any(py).unwrap()
    })
}

fn parse_files(files: PyObject) -> PyResult<Vec<Source>> {
    Python::with_gil(|py| parse_files_bounded(files.bind(py)))
}

fn parse_files_bounded(files: &Bound<'_, PyAny>) -> PyResult<Vec<Source>> {
    if files.is_none() {
        return Ok(Vec::new());
    }

    // Handle lists
    if let Ok(list) = files.downcast::<PyList>() {
        let mut sources = Vec::new();
        for item in list.iter() {
            sources.push(Source::from_python(&item)?);
        }
        return Ok(sources);
    }

    // Handle dicts
    if let Ok(list) = files.downcast::<PyDict>() {
        let mut sources = Vec::new();
        for item in list.iter() {
            let path = from_path_or_str(&item.0)?;
            let source = item.1.downcast::<PyString>()?.to_str()?;
            sources.push(Source::new(path, source.into()));
        }
        return Ok(sources);
    }

    // Handle iterables
    if let Ok(iter) = files.downcast::<PyIterator>() {
        let mut sources = Vec::new();
        for item in iter {
            sources.push(Source::from_python(&item?)?);
        }
        return Ok(sources);
    }

    let msg = "Expected a list of files or dictionary mapping paths to sources";
    Err(PyTypeError::new_err(msg))
}

fn from_path_or_str(obj: &Bound<'_, PyAny>) -> PyResult<Arc<str>> {
    // Strings
    if let Ok(path) = obj.downcast::<PyString>() {
        Ok(path.to_str()?.into())

    // Path objects
    } else if let Ok(path) = from_path(obj) {
        Ok(path)
    } else {
        let msg = "Expected a string or Path object";
        Err(PyTypeError::new_err(msg))
    }
}

fn from_path(obj: &Bound<'_, PyAny>) -> PyResult<Arc<str>> {
    Python::with_gil(|py| {
        let path_type = PyModule::import(py, "pathlib")?.getattr("Path")?;
        if obj.is_instance(&path_type)? {
            Ok(obj.to_string().into())
        } else {
            Err(PyTypeError::new_err("Expected a Path object or a string"))
        }
    })
}

fn from_file_like(obj: &Bound<'_, PyAny>) -> PyResult<Source> {
    let source = obj.call_method0("read")?;
    let source = source.downcast::<PyString>()?.to_str()?;
    let path: Arc<str> = match obj.getattr("name") {
        Ok(value) => {
            let str = value.downcast::<PyString>()?.to_str()?;
            str.into()
        }
        Err(_) => "<string>".into(),
    };
    Ok(Source::new(path, source.into()))
}
