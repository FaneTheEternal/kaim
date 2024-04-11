use std::ffi::{c_int, CStr};
use std::time::SystemTime;

use bitflags::bitflags;
use pyo3::{Py, PyAny, Python};
use pyo3::ffi::*;

use crate::extend_py_ffi::*;
use crate::profiler::Profiler;
use kaim_types::Entry;

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

unsafe fn get_frame_info(frame: *mut PyFrameObject) -> String {
    if frame.is_null() {
        return "<null>".into();
    }
    let code = PyFrame_GetCode(frame);
    let code = if code.is_null() {
        return "<code is null>".into();
    } else { *code };
    format!(
        "{fname}:{name}:{line}",
        fname = py_pointer_to_str(code.co_filename),
        name = py_pointer_to_str(code.co_qualname),
        line = PyFrame_GetLineNumber(frame),
    )
}


pub unsafe extern "C" fn callback(
    ctx: *mut PyObject,
    frame: *mut PyFrameObject,
    what: c_int,
    arg: *mut PyObject,
) -> c_int {
    if ctx.is_null() | frame.is_null() {
        return 0;
    }
    Python::with_gil(|py| {
        let py_ctx = Py::<PyAny>::from_borrowed_ptr(py, ctx);
        let profiler = match py_ctx.downcast_bound::<Profiler>(py) {
            Ok(o) => o,
            Err(err) => {
                println!("Non `Profile` context var: {err:?}");
                return;
            }
        };
        let mut profiler = profiler.borrow_mut();

        let event: TraceEvent = match what.try_into() {
            Ok(event) => event,
            _ => return,
        };

        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("`SystemTime::now()` return time before unix epoch")
            .as_secs_f64();

        let frame_id = frame as usize;
        match event {
            TraceEvent::Call => {
                profiler.stack.push(Entry::call(
                    frame_id,
                    get_frame_info(PyFrame_GetBack(frame)),
                    get_frame_info(frame),
                    now,
                ));
            }
            TraceEvent::CCall => {
                profiler.stack.push(Entry::ccall(
                    frame_id,
                    get_frame_info(frame),
                    py_pointer_to_str(arg),
                    now,
                ))
            }
            TraceEvent::Return | TraceEvent::CReturn => {
                let matched = {
                    if let Some(last) = profiler.stack.last() {
                        last.id == frame_id && (
                            !matches!(event, TraceEvent::CReturn) || last.info.eq(&py_pointer_to_str(arg))
                        )
                    } else {
                        false
                    }
                };
                if matched {
                    let mut last = profiler.stack.pop().unwrap();
                    last.fin(now);
                    profiler.entries.push(last);
                }
            }
            _ => return,
        }
    });

    0
}

#[allow(dead_code)]
#[derive(Debug)]
#[non_exhaustive]
#[repr(i32)]
pub enum TraceEvent {
    Call,
    Exception,
    Line,
    Return,
    CCall,
    CException,
    CReturn,
    Opcode,
}

const LARGER_THAN_ANY_TRACE_EVENT: i32 = TraceEvent::Opcode as i32 + 1;

impl TryFrom<c_int> for TraceEvent {
    type Error = &'static str;
    /// Cast i32 event (raw from Python) to Rust enum.
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= LARGER_THAN_ANY_TRACE_EVENT || value < 0 {
            return Err("Not valid enum value");
        }
        Ok(unsafe { std::mem::transmute(value) })
    }
}


bitflags! {
    #[derive(Debug)]
    pub struct CodeFlags: c_int {
        const CO_OPTIMIZED = CO_OPTIMIZED;
        const CO_NEWLOCALS = CO_NEWLOCALS;
        const CO_VARARGS = CO_VARARGS;
        const CO_VARKEYWORDS = CO_VARKEYWORDS;
        const CO_NESTED = CO_NESTED;
        const CO_GENERATOR = CO_GENERATOR;
        const CO_NOFREE = CO_NOFREE;
        const CO_COROUTINE = CO_COROUTINE;
        const CO_ITERABLE_COROUTINE = CO_ITERABLE_COROUTINE;
        const CO_ASYNC_GENERATOR = CO_ASYNC_GENERATOR;
        const CO_FUTURE_DIVISION = CO_FUTURE_DIVISION;
        const CO_FUTURE_ABSOLUTE_IMPORT = CO_FUTURE_ABSOLUTE_IMPORT;
        const CO_FUTURE_WITH_STATEMENT = CO_FUTURE_WITH_STATEMENT;
        const CO_FUTURE_PRINT_FUNCTION = CO_FUTURE_PRINT_FUNCTION;
        const CO_FUTURE_UNICODE_LITERALS = CO_FUTURE_UNICODE_LITERALS;
        const CO_FUTURE_BARRY_AS_BDFL = CO_FUTURE_BARRY_AS_BDFL;
        const CO_FUTURE_GENERATOR_STOP = CO_FUTURE_GENERATOR_STOP;
    }
}