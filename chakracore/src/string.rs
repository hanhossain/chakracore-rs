use crate::error::JsError;
use crate::handle::IntoHandle;
use crate::value::JsValue;
use chakracore_sys::{JsConvertValueToString, JsCopyString, JsCreateString, JsValueRef};
use std::convert::{TryFrom, TryInto};
use std::ffi::{CStr, CString};
use std::fmt::{Debug, Formatter};
use std::ptr;

pub struct JsString {
    pub(crate) handle: JsValueRef,
}

impl JsString {
    /// Create a `JsString`
    pub fn new<T: Into<Vec<u8>>>(value: T) -> Result<Self, JsError> {
        let string = CString::new(value).unwrap();

        let mut handle = ptr::null_mut();
        let res =
            unsafe { JsCreateString(string.as_ptr(), string.as_bytes().len() as u64, &mut handle) };
        JsError::assert(res)?;

        Ok(Self { handle })
    }

    /// Convert to a String
    pub fn to_string(&self) -> Result<String, JsError> {
        // get size of buffer
        let mut length = 0;
        JsError::assert(unsafe { JsCopyString(self.handle, ptr::null_mut(), 0, &mut length) })?;

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

impl TryFrom<JsValue> for JsString {
    type Error = JsError;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let mut result = ptr::null_mut();
        let res = unsafe { JsConvertValueToString(value.handle, &mut result) };
        JsError::assert(res)?;

        Ok(JsString { handle: result })
    }
}

impl IntoHandle for JsString {
    fn into_handle(self) -> JsValueRef {
        self.handle
    }
}

impl Debug for JsString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = self.to_string();
        f.debug_struct("JsBoolean").field("value", &result).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::JsScriptContext;
    use crate::runtime::JsRuntime;

    #[test]
    fn create_string() {
        let mut runtime = JsRuntime::new().unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        context.set_current_context().unwrap();

        let s = JsString::new("hello world!");
        println!("hi: {:?}", s);
        assert_eq!(s.map(|x| x.handle.is_null()), Ok(false));
    }

    #[test]
    fn create_and_get_string() {
        let mut runtime = JsRuntime::new().unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        context.set_current_context().unwrap();

        let js_string = JsString::new("hello world!");
        assert!(js_string.is_ok());

        let js_string = js_string.unwrap();
        assert!(!js_string.handle.is_null());
        assert_eq!(js_string.to_string(), Ok("hello world!".to_string()));
    }
}
