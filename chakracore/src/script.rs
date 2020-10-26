use crate::error::JsError;
use chakracore_sys::{JsCreateExternalArrayBuffer, JsValueRef};
use std::ffi::CString;
use std::mem::MaybeUninit;
use std::ptr;

#[derive(Debug)]
pub struct JsScript {
    handle: JsValueRef,
    size: usize,
    raw: *mut i8,
}

impl JsScript {
    pub fn new<T: Into<Vec<u8>>>(script: T) -> Result<Self, JsError> {
        let script = CString::new(script).unwrap();
        let size = script.as_bytes().len();
        let script = script.into_raw();
        let mut source = MaybeUninit::uninit();

        let res = unsafe {
            JsCreateExternalArrayBuffer(
                script as *mut _,
                size as u32,
                None,
                ptr::null_mut(),
                source.as_mut_ptr(),
            )
        };
        JsError::assert(res)?;
        let source = unsafe { source.assume_init() };

        Ok(Self {
            handle: source,
            size,
            raw: script,
        })
    }
}

impl Drop for JsScript {
    fn drop(&mut self) {
        // release script from memory
        unsafe {
            let _ = CString::from_raw(self.raw);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::JsScriptContext;
    use crate::runtime::{JsRuntime, JsRuntimeAttributes};

    #[test]
    fn create_script() {
        let mut runtime = JsRuntime::new(JsRuntimeAttributes::None).unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        context.set_current_context().unwrap();

        let script = JsScript::new("(() => { return 'Hello world'; })()");
        assert!(script.is_ok());
    }
}
