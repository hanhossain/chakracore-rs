use crate::error::JsError;
use chakracore_sys::{JsBoolToBoolean, JsBooleanToBool, JsValueRef};
use std::mem::MaybeUninit;

pub struct JsBoolean {
    pub(crate) handle: JsValueRef,
}

impl JsBoolean {
    /// Create JsBoolean from bool
    pub fn from_bool(val: bool) -> Result<Self, JsError> {
        let mut result = MaybeUninit::uninit();
        let res = unsafe { JsBoolToBoolean(val, result.as_mut_ptr()) };
        JsError::assert(res)?;

        Ok(Self {
            handle: unsafe { result.assume_init() },
        })
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
