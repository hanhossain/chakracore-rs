use crate::error::JsError;
use crate::string::JsString;
use chakracore_sys::{JsCreateExternalArrayBuffer, JsValueRef};
use std::ffi::CString;
use std::ptr;

#[derive(Debug)]
pub struct JsScript {
    pub(crate) handle: JsValueRef,
    pub(crate) source_url: JsString,
    size: usize,
    raw: *mut i8,
}

impl JsScript {
    #[allow(clippy::cast_possible_truncation)]
    /// Convert a string into a script
    pub fn new<TUrl: Into<Vec<u8>>, TScript: Into<Vec<u8>>>(
        url: TUrl,
        script: TScript,
    ) -> Result<Self, JsError> {
        let script = CString::new(script).unwrap();
        let size = script.as_bytes().len();
        let script = script.into_raw();
        let mut source = ptr::null_mut();

        let res = unsafe {
            JsCreateExternalArrayBuffer(
                script as *mut _,
                size as u32,
                None,
                ptr::null_mut(),
                &mut source,
            )
        };

        JsError::assert(res)?;

        Ok(Self {
            handle: source,
            source_url: JsString::new(url)?,
            size,
            raw: script,
        })
    }
}

impl Drop for JsScript {
    fn drop(&mut self) {
        // release script from memory
        unsafe {
            std::mem::drop(CString::from_raw(self.raw));
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

        let script = JsScript::new("hello", "(() => { return 'Hello world'; })()");
        assert!(script.is_ok());
    }
}
