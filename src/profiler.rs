use derive_more::Deref;
use kaim_types::Entry;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

#[pyclass(subclass)]
pub struct Profiler {
    pub entries: Vec<Entry>,
    pub stack: Vec<Entry>,
}

#[pymethods]
impl Profiler {
    #[new]
    fn new() -> Self {
        Profiler {
            entries: vec![],
            stack: vec![],
        }
    }

    fn start(slf: PyRef<'_, Self>) -> PyResult<()> {
        crate::ffi::set_profiler(slf.as_ptr());
        Ok(())
    }

    fn get_entries(&self) -> Vec<PyEntry> {
        self.entries.iter()
            .map(|raw| PyEntry(raw.clone()))
            .collect()
    }

    fn dump(&self) -> PyResult<String> {
        ron::to_string(&self.entries)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
}


#[derive(Deref)]
#[pyclass]
pub struct PyEntry(Entry);

#[pymethods]
impl PyEntry {
    #[getter]
    fn id(&self) -> usize { self.id }

    #[getter]
    fn kind(&self) -> String { format!("{:?}", self.kind) }

    #[getter]
    fn called(&self) -> String { self.called.clone() }

    #[getter]
    fn info(&self) -> String { self.info.clone() }

    #[getter]
    fn time(&self) -> (f64, f64) { self.time }
}
