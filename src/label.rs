use crate::{_Label, color::Color};
use pyo3::{exceptions::PyValueError, prelude::*};
use std::{hash::Hash, ops::Range, sync::Arc};

#[pyclass(frozen, eq, hash)]
#[derive(Clone, Debug, PartialEq, Hash)]
pub struct Label {
    pub(crate) span: Range<usize>,
    pub(crate) target: Option<Arc<str>>,
    pub(crate) message: Option<String>,
    pub(crate) color: Option<Color>,
    pub(crate) order: Option<i32>,
    pub(crate) priority: Option<i32>,
}

impl Label {
    pub fn new(span: Range<usize>) -> Self {
        Label {
            span,
            target: None,
            message: None,
            color: None,
            order: None,
            priority: None,
        }
    }

    pub fn to_ariadne(&self) -> _Label {
        let target = self.target.clone().unwrap_or("<string>".into());
        let mut label = ariadne::Label::new((target, self.span.clone()));
        if let Some(message) = &self.message {
            label = label.with_message(message);
        }
        if let Some(color) = &self.color {
            label = label.with_color(color.inner);
        }
        if let Some(order) = self.order {
            label = label.with_order(order);
        }
        if let Some(priority) = self.priority {
            label = label.with_priority(priority);
        }
        label
    }

    pub fn to_ariadne_with_default(&self, path: Arc<str>) -> _Label {
        if self.target.is_none() {
            self.clone().replace_target(path).to_ariadne()
        } else {
            self.to_ariadne()
        }
    }

    fn replace_target(mut self, target: Arc<str>) -> Self {
        self.target = Some(target.clone());
        self
    }

    fn set_params(
        mut self,
        target: Option<&str>,
        message: Option<&str>,
        color: Option<Color>,
        order: Option<i32>,
        priority: Option<i32>,
    ) -> Self {
        if let Some(target) = target {
            self.target = Some(target.into());
        }
        if let Some(msg) = message {
            self.message = Some(msg.into());
        }
        if let Some(color) = color {
            self.color = Some(color);
        }
        if let Some(order) = order {
            self.order = Some(order);
        }
        if let Some(priority) = priority {
            self.priority = Some(priority);
        }
        self
    }
}

#[pymethods]
impl Label {
    #[new]
    #[pyo3(signature=(start, end, *, path=None, message=None, color=None, order=None, priority=None))]
    pub(crate) fn py_new(
        start: usize,
        end: usize,
        path: Option<&str>,
        message: Option<&str>,
        color: Option<Color>,
        order: Option<i32>,
        priority: Option<i32>,
    ) -> PyResult<Self> {
        if start > end {
            let msg = "Start index must be less than or equal to end index";
            return Err(PyValueError::new_err(msg));
        }
        Ok(Label::new(start..end).set_params(path, message, color, order, priority))
    }
    #[pyo3(signature=(*, message=None, color=None, order=None, priority=None))]
    fn copy(
        &self,
        message: Option<&str>,
        color: Option<Color>,
        order: Option<i32>,
        priority: Option<i32>,
    ) -> Self {
        self.clone()
            .set_params(None, message, color, order, priority)
    }

    fn __repr__(&self) -> String {
        let mut args = vec![self.span.start.to_string(), self.span.end.to_string()];
        if let Some(target) = &self.target {
            args.push(format!("path={target:?}"));
        }
        if let Some(message) = &self.message {
            args.push(format!("message={message:?}"));
        }
        if let Some(color) = &self.color {
            args.push(format!("color={color:?}"));
        }
        if let Some(order) = self.order {
            args.push(format!("order={order}"));
        }
        if let Some(priority) = self.priority {
            args.push(format!("priority={priority}"));
        }
        let args = args.join(", ");
        format!("Label({args})")
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }
}
