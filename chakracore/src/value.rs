use crate::error::JsError;
use chakracore_sys::{JsGetValueType, JsValueRef};

#[derive(Debug, Eq, PartialEq)]
pub enum JsType {
    JsUndefined,
    JsNull,
    JsNumber,
    JsString,
    JsBoolean,
    JsObject,
    JsFunction,
    JsError,
    JsArray,
    JsSymbol,
    JsArrayBuffer,
    JsTypedArray,
    JsDataView,
}

pub struct JsValue {
    pub(crate) handle: JsValueRef,
}

impl JsValue {
    pub fn get_type(&self) -> Result<JsType, JsError> {
        let mut result = 0u32;
        let res = unsafe { JsGetValueType(self.handle, &mut result) };
        JsError::assert(res)?;

        Ok(match result {
            0 => JsType::JsUndefined,
            1 => JsType::JsNull,
            2 => JsType::JsNumber,
            3 => JsType::JsString,
            4 => JsType::JsBoolean,
            5 => JsType::JsObject,
            6 => JsType::JsFunction,
            7 => JsType::JsError,
            8 => JsType::JsArray,
            9 => JsType::JsSymbol,
            10 => JsType::JsArrayBuffer,
            11 => JsType::JsTypedArray,
            12 => JsType::JsDataView,
            _ => unreachable!(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boolean::JsBoolean;
    use crate::context::JsScriptContext;
    use crate::number::JsNumber;
    use crate::runtime::{JsRuntime, JsRuntimeAttributes};
    use std::convert::TryFrom;

    #[test]
    fn get_type_number() {
        let number = JsNumber::try_from(42).unwrap();
        let value: JsValue = number.into();
        assert_eq!(value.get_type(), Ok(JsType::JsNumber));
    }

    #[test]
    fn get_type_boolean() {
        let mut runtime = JsRuntime::new(JsRuntimeAttributes::None).unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        context.set_current_context().unwrap();

        let bool = JsBoolean::try_from(true).unwrap();
        let value: JsValue = bool.into();
        assert_eq!(value.get_type(), Ok(JsType::JsBoolean));
    }
}
