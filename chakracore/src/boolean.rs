use crate::error::JsError;
use chakracore_sys::{JsBoolToBoolean, JsBooleanToBool, JsValueRef};
use std::ptr;

pub struct JsBoolean {
    pub(crate) handle: JsValueRef,
}

impl JsBoolean {
    /// Create JsBoolean from bool
    pub fn from_bool(val: bool) -> Result<Self, JsError> {
        let mut result = ptr::null_mut();
        let res = unsafe { JsBoolToBoolean(val, &mut result) };
        JsError::assert(res)?;

        Ok(Self { handle: result })
    }

    /// Convert JsBoolean to bool
    pub fn to_bool(&self) -> Result<bool, JsError> {
        let mut result = false;
        let res = unsafe { JsBooleanToBool(self.handle, &mut result as *mut _) };
        JsError::assert(res)?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::context::JsScriptContext;
    use crate::runtime::{JsRuntime, JsRuntimeAttributes};

    use super::*;

    #[test]
    fn convert_from_bool() {
        let mut runtime = JsRuntime::new(JsRuntimeAttributes::None).unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        context.set_current_context().unwrap();

        let boolean = JsBoolean::from_bool(true).unwrap();
        assert!(!boolean.handle.is_null());
    }

    #[test]
    fn convert_to_bool() {
        let mut runtime = JsRuntime::new(JsRuntimeAttributes::None).unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        context.set_current_context().unwrap();

        let boolean = JsBoolean::from_bool(true).unwrap();
        assert_eq!(boolean.to_bool(), Ok(true));
    }
}
