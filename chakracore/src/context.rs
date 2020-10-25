use crate::error::JsError;
use crate::runtime::JsRuntime;
use chakracore_sys::{JsContextRef, JsCreateContext};
use std::mem::MaybeUninit;

pub struct JsScriptContext {
    context: JsContextRef,
}

impl JsScriptContext {
    /// Create a script context
    pub fn new(runtime: &mut JsRuntime) -> Result<Self, JsError> {
        let mut context: MaybeUninit<JsContextRef> = MaybeUninit::uninit();
        let res = unsafe { JsCreateContext(runtime.handle, context.as_mut_ptr()) };
        JsError::assert(res)?;

        Ok(Self {
            context: unsafe { context.assume_init() },
        })
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
}
