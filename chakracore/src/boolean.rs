use crate::error::JsError;
use crate::value::JsValue;
use chakracore_sys::{JsBoolToBoolean, JsBooleanToBool, JsConvertValueToBoolean, JsValueRef};
use std::fmt::{Debug, Formatter};
use std::ptr;

pub struct JsBoolean {
    pub(crate) handle: JsValueRef,
}

impl TryFrom<bool> for JsBoolean {
    type Error = JsError;

    fn try_from(value: bool) -> Result<Self, Self::Error> {
        let mut result = ptr::null_mut();
        let res = unsafe { JsBoolToBoolean(value, &mut result) };
        JsError::assert(res)?;

        Ok(Self { handle: result })
    }
}

impl TryInto<bool> for JsBoolean {
    type Error = JsError;

    fn try_into(self) -> Result<bool, Self::Error> {
        let mut result = false;
        let res = unsafe { JsBooleanToBool(self.handle, &mut result as *mut _) };
        JsError::assert(res)?;

        Ok(result)
    }
}

impl TryFrom<JsValue> for JsBoolean {
    type Error = JsError;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let mut result = ptr::null_mut();
        let res = unsafe { JsConvertValueToBoolean(value.handle, &mut result) };
        JsError::assert(res)?;

        Ok(JsBoolean { handle: result })
    }
}

impl From<JsBoolean> for JsValue {
    fn from(boolean: JsBoolean) -> JsValue {
        JsValue {
            handle: boolean.handle,
        }
    }
}

impl Debug for JsBoolean {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = false;
        let res = unsafe { JsBooleanToBool(self.handle, &mut result as *mut _) };
        let error = JsError::assert(res);

        f.debug_struct("JsBoolean")
            .field("value", &error.and(Ok(result)))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::context::JsScriptContext;
    use crate::runtime::JsRuntime;

    use super::*;

    #[test]
    fn convert_from_bool() {
        let mut runtime = JsRuntime::new().unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        context.set_current_context().unwrap();

        let boolean = JsBoolean::try_from(true).unwrap();
        assert!(!boolean.handle.is_null());
    }

    #[test]
    fn convert_to_bool() {
        let mut runtime = JsRuntime::new().unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        context.set_current_context().unwrap();

        let boolean = JsBoolean::try_from(true).unwrap();
        assert_eq!(boolean.try_into(), Ok(true));
    }
}
