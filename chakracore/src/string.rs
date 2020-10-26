use crate::error::JsError;
use bitflags::_core::mem::MaybeUninit;
use chakracore_sys::{JsCopyString, JsCreateString, JsValueRef};
use std::convert::TryInto;
use std::ffi::{CStr, CString};
use std::ptr;

pub struct JsString {
    handle: JsValueRef,
}

impl JsString {
    /// Create a JsString
    pub fn new<T: Into<Vec<u8>>>(value: T) -> Result<Self, JsError> {
        let string = CString::new(value).unwrap();

        let mut handle = MaybeUninit::uninit();
        let res = unsafe {
            JsCreateString(
                string.as_ptr(),
                string.as_bytes().len() as u64,
                handle.as_mut_ptr(),
            )
        };
        JsError::assert(res)?;

        Ok(Self {
            handle: unsafe { handle.assume_init() },
        })
    }

    /// Convert to a String
    pub fn to_string(self) -> Result<String, JsError> {
        // get size of buffer
        let mut length = 0;
        let res = unsafe { JsCopyString(self.handle, ptr::null_mut(), 0, &mut length) };
        JsError::assert(res)?;

        // copy to buffer
        let total_length = length + 1;
        let mut buffer: Vec<u8> = vec![0; total_length.try_into().unwrap()];
        let res = unsafe {
            JsCopyString(
                self.handle,
                buffer.as_mut_ptr() as *mut i8,
                total_length,
                ptr::null_mut(),
            )
        };
        JsError::assert(res)?;

        let result_str = CStr::from_bytes_with_nul(buffer.as_slice()).unwrap();
        Ok(result_str.to_owned().into_string().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::JsScriptContext;
    use crate::runtime::{JsRuntime, JsRuntimeAttributes};

    #[test]
    fn create_string() {
        let mut runtime = JsRuntime::new(JsRuntimeAttributes::None).unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        context.set_current_context();

        let s = JsString::new("hello world!");
        assert_eq!(s.map(|x| x.handle.is_null()), Ok(false));
    }

    #[test]
    fn create_and_get_string() {
        let mut runtime = JsRuntime::new(JsRuntimeAttributes::None).unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        context.set_current_context();

        let js_string = JsString::new("hello world!");
        assert!(js_string.is_ok());

        let js_string = js_string.unwrap();
        assert!(!js_string.handle.is_null());
        assert_eq!(js_string.to_string(), Ok("hello world!".to_string()));
    }
}
