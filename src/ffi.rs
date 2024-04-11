use std::ffi::{c_int, CStr};
use std::ptr::{null, null_mut};

use pyo3::{Py, PyAny, Python};
use pyo3::ffi::*;

use crate::profiler::Profiler;
use crate::TraceEvent;

pub fn set_profiler(ctx: *mut PyObject) {
    unsafe {
        PyEval_SetProfile(Some(callback), ctx);
    }
}

unsafe fn py_pointer_to_str(ptr: *mut PyObject) -> String {
    if ptr.is_null() {
        "<null>".into()
    } else {
        CStr::from_ptr(
            PyUnicode_AsUTF8(
                PyObject_Str(ptr)
            )
        ).to_string_lossy().to_string()
    }
}


pub unsafe extern "C" fn callback(
    ctx: *mut PyObject,
    frame: *mut PyFrameObject,
    what: c_int,
    arg: *mut PyObject,
) -> c_int {
    if ctx.is_null() {
        return 0;
    }
    Python::with_gil(|py| {
        let py_ctx = Py::<PyAny>::from_owned_ptr(py, ctx);
        let profiler = match py_ctx.downcast_bound::<Profiler>(py) {
            Ok(o) => o,
            Err(err) => {
                dbg!(err);
                return;
            }
        };
        let profiler = profiler.borrow_mut();
    });

    let event: TraceEvent = match what.try_into() {
        Ok(event) => event,
        _ => return 0,
    };

    if frame.is_null() {
        return 0;
    }

    let f_code = PyFrame_GetCode(frame);
    if f_code.is_null() {
        return 0;
    }

    let s_frame = {
        let f_code = *f_code;
        let file_name = py_pointer_to_str(f_code.co_filename);
        let name = py_pointer_to_str(f_code.co_name);
        let lineno = f_code.co_firstlineno;
        format!("{file_name}.{name}.{lineno}")
    };
    println!(
        "[{}] Frame: {}; Event: {:?}, Arg: {}",
        py_pointer_to_str(ctx),
        s_frame,
        event,
        py_pointer_to_str(arg),
    );

    0
}
