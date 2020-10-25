use crate::error::JsError;
use crate::runtime::JsRuntime;
use chakracore_sys::{JsContextRef, JsCreateContext, JsSetCurrentContext};
use std::mem::MaybeUninit;

pub struct JsScriptContext {
    context: JsContextRef,
    is_current_context: bool,
}

impl JsScriptContext {
    /// Create a script context
    pub fn new(runtime: &mut JsRuntime) -> Result<Self, JsError> {
        let mut context: MaybeUninit<JsContextRef> = MaybeUninit::uninit();
        let res = unsafe { JsCreateContext(runtime.handle, context.as_mut_ptr()) };
        JsError::assert(res)?;

        Ok(Self {
            context: unsafe { context.assume_init() },
            is_current_context: false,
        })
    }

    pub fn set_current_context(&mut self) -> Result<(), JsError> {
        let res = unsafe { JsSetCurrentContext(self.context) };
        JsError::assert(res)?;
        self.is_current_context = true;
        Ok(())
    }

    fn clear_current_context() -> Result<(), JsError> {
        let res = unsafe { JsSetCurrentContext(std::ptr::null_mut()) };
        JsError::assert(res)
    }
}

impl Drop for JsScriptContext {
    fn drop(&mut self) {
        if self.is_current_context {
            JsScriptContext::clear_current_context().expect("Failed to clear current context.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::JsRuntimeAttributes;

    #[test]
    fn create_context() {
        let mut runtime = JsRuntime::new(JsRuntimeAttributes::None).unwrap();
        let context = JsScriptContext::new(&mut runtime);

        assert!(context.is_ok());
        assert!(!context.unwrap().context.is_null());
    }

    #[test]
    fn set_context() {
        let mut runtime = JsRuntime::new(JsRuntimeAttributes::None).unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        let res = context.set_current_context();

        assert!(res.is_ok());
    }
}
