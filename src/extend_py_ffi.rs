use pyo3::ffi::*;

extern "C" {
    pub fn PyFrame_GetBack(f: *mut PyFrameObject) -> *mut PyFrameObject;
}