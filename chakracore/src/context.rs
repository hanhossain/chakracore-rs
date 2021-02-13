use crate::error::JsError;
use crate::runtime::JsRuntime;
use chakracore_sys::{JsContextRef, JsCreateContext, JsSetCurrentContext};
use std::ptr;

pub struct JsScriptContext {
    context: JsContextRef,
    is_current_context: bool,
}

impl JsScriptContext {
    /// Create a script context
    pub fn new(runtime: &mut JsRuntime) -> Result<Self, JsError> {
        let mut context: JsContextRef = ptr::null_mut();
        let res = unsafe { JsCreateContext(runtime.handle, &mut context) };
        JsError::assert(res)?;

        Ok(Self {
            context,
            is_current_context: false,
        })
    }

    /// Sets the current script context on the thread.
    pub fn set_current_context(&mut self) -> Result<(), JsError> {
        let res = unsafe { JsSetCurrentContext(self.context) };
        JsError::assert(res)?;
        self.is_current_context = true;
        Ok(())
    }

    /// Clears the current script context on the thread.
    ///
    /// This does not need to be explicitly called - it will automatically be called when the
    /// context is dropped if it was set as the current context.
    pub fn clear_current_context(&mut self) -> Result<(), JsError> {
        if self.is_current_context {
            let res = unsafe { JsSetCurrentContext(std::ptr::null_mut()) };
            JsError::assert(res)?;
            self.is_current_context = false;
        }

        Ok(())
    }
}

impl Drop for JsScriptContext {
    fn drop(&mut self) {
        self.clear_current_context()
            .expect("Failed to clear current context.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_context() {
        let mut runtime = JsRuntime::new().unwrap();
        let context = JsScriptContext::new(&mut runtime);

        assert_eq!(context.map(|x| x.context.is_null()), Ok(false));
    }

    #[test]
    fn set_context() {
        let mut runtime = JsRuntime::new().unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        let res = context.set_current_context();

        assert!(res.is_ok());
    }
}
