mod ffi;
mod types;
mod profiler;

use pyo3::ffi::{Py_tracefunc, PyEval_SetProfile};
use ffi::*;
use types::*;

use pyo3::prelude::*;


#[pyfunction]
fn start(ctx: &Bound<'_, PyAny>) {
    let cb: Py_tracefunc = callback;
    unsafe {
        PyEval_SetProfile(cb.into(), ctx.as_ptr());
    }
}


#[pymodule]
fn kaim(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(start, m)?)?;
    m.add_class::<profiler::Profiler>()?;
    Ok(())
}
