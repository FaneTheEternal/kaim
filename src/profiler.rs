use pyo3::prelude::*;

#[pyclass]
pub struct Profiler {
    entries: Vec<ProfileEntry>,
}

#[pymethods]
impl Profiler {
    #[new]
    fn new() -> Self {
        Profiler {
            entries: vec![],
        }
    }

    fn start(mut slf: PyRefMut<'_, Self>) -> PyResult<()> {
        crate::ffi::set_profiler(slf.as_ptr());
        Ok(())
    }
}

struct ProfileEntry {
    id: usize,
    name: String,
    duration: usize,
}
