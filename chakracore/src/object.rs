use crate::error::JsError;
use crate::string::JsString;
use chakracore_sys::{JsCreateObject, JsGetGlobalObject, JsObjectHasProperty, JsValueRef};
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

    pub fn global() -> Result<Self, JsError> {
        let mut result = ptr::null_mut();
        let res = unsafe { JsGetGlobalObject(&mut result) };
        JsError::assert(res)?;

        Ok(Self { handle: result })
    }

    pub fn has_property(&self, key: &JsString) -> Result<bool, JsError> {
        let mut result = false;
        let res = unsafe { JsObjectHasProperty(self.handle, key.handle, &mut result) };
        JsError::assert(res)?;

        Ok(result)
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

    #[test]
    fn get_global_object() {
        let mut runtime = JsRuntime::new(JsRuntimeAttributes::None).unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        context.set_current_context().unwrap();

        let object = JsObject::global().unwrap();
        assert!(!object.handle.is_null());
    }

    #[test]
    fn has_property_string() {
        let mut runtime = JsRuntime::new(JsRuntimeAttributes::None).unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        context.set_current_context().unwrap();

        let object = JsObject::global().unwrap();
        assert!(!object
            .has_property(&JsString::new("hello").unwrap())
            .unwrap());
    }
}
