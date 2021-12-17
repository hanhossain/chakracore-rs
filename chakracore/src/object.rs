use crate::boolean::JsBoolean;
use crate::error::JsError;
use crate::handle::IntoHandle;
use crate::string::JsString;
use crate::value::JsValue;
use chakracore_sys::{
    JsCreateObject, JsGetGlobalObject, JsObjectDeleteProperty, JsObjectGetProperty,
    JsObjectHasProperty, JsObjectSetProperty, JsValueRef,
};
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

    pub fn set_property<T: Into<JsValue>>(
        &mut self,
        key: &JsString,
        value: T,
    ) -> Result<(), JsError> {
        let value = value.into();
        let res = unsafe { JsObjectSetProperty(self.handle, key.handle, value.handle, true) };
        JsError::assert(res)
    }

    pub fn get_property(&self, key: &JsString) -> Result<JsValue, JsError> {
        let mut handle = ptr::null_mut();
        let res = unsafe { JsObjectGetProperty(self.handle, key.handle, &mut handle) };
        JsError::assert(res)?;

        Ok(JsValue { handle })
    }

    pub fn delete_property(&self, key: &JsString) -> Result<bool, JsError> {
        let mut handle = ptr::null_mut();
        let res = unsafe { JsObjectDeleteProperty(self.handle, key.handle, true, &mut handle) };
        JsError::assert(res)?;
        JsBoolean::try_from(JsValue { handle })?.try_into()
    }
}

impl IntoHandle for JsObject {
    fn into_handle(self) -> JsValueRef {
        self.handle
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::JsScriptContext;
    use crate::number::JsNumber;
    use crate::runtime::JsRuntime;

    #[test]
    fn create_object() {
        let mut runtime = JsRuntime::new().unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        context.set_current_context().unwrap();

        let object = JsObject::new().unwrap();
        assert!(!object.handle.is_null());
    }

    #[test]
    fn get_global_object() {
        let mut runtime = JsRuntime::new().unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        context.set_current_context().unwrap();

        let object = JsObject::global().unwrap();
        assert!(!object.handle.is_null());
    }

    #[test]
    fn has_property_string() {
        let mut runtime = JsRuntime::new().unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        context.set_current_context().unwrap();

        let object = JsObject::global().unwrap();
        assert!(!object
            .has_property(&JsString::new("hello").unwrap())
            .unwrap());
    }

    #[test]
    fn set_property_object() {
        let mut runtime = JsRuntime::new().unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        context.set_current_context().unwrap();

        let mut global = JsObject::global().unwrap();
        let console = JsObject::new().unwrap();
        let console_key = JsString::new("console").unwrap();
        global.set_property(&console_key, console).unwrap();

        assert!(global.has_property(&console_key).unwrap());
    }

    #[test]
    fn get_property_number() {
        let mut runtime = JsRuntime::new().unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        context.set_current_context().unwrap();

        let mut global = JsObject::global().unwrap();
        let pi = JsNumber::try_from(std::f64::consts::PI).unwrap();
        let pi_key = JsString::new("pi").unwrap();
        global.set_property(&pi_key, pi).unwrap();

        assert!(global.has_property(&pi_key).unwrap());

        let property: JsNumber = global.get_property(&pi_key).unwrap().try_into().unwrap();
        assert_eq!(property.try_into(), Ok(std::f64::consts::PI));
    }

    #[test]
    fn delete_property_string() {
        let mut runtime = JsRuntime::new().unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        context.set_current_context().unwrap();

        let mut global = JsObject::global().unwrap();
        let console = JsObject::new().unwrap();
        let console_key = JsString::new("console").unwrap();
        global.set_property(&console_key, console).unwrap();

        assert!(global.has_property(&console_key).unwrap());

        let result = global.delete_property(&console_key);
        assert_eq!(result, Ok(true));
        assert!(!global.has_property(&console_key).unwrap());
    }
}
