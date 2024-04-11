use std::ffi::c_int;

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
