use crate::error::JsError;
use chakracore_sys::{JsCreateObject, JsValueRef};
use std::ptr;

#[derive(Debug)]
pub struct JsObject {
    handle: JsValueRef,
}

impl JsObject {
    pub fn new() -> Result<Self, JsError> {
        let mut result = ptr::null_mut();
        let res = unsafe { JsCreateObject(&mut result) };
        JsError::assert(res)?;

        Ok(Self { handle: result })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::JsScriptContext;
    use crate::runtime::{JsRuntime, JsRuntimeAttributes};

    #[test]
    fn create_object() {
        let mut runtime = JsRuntime::new(JsRuntimeAttributes::None).unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        context.set_current_context().unwrap();

        let object = JsObject::new().unwrap();
        assert!(!object.handle.is_null());
    }
}
