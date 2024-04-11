use pyo3::prelude::*;

mod ffi;
mod profiler;
mod extend_py_ffi;

#[pymodule]
fn kaim(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<profiler::Profiler>()?;
    Ok(())
}
