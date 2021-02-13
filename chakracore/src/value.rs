use crate::error::JsError;
use chakracore_sys::{JsGetValueType, JsValueRef};

#[derive(Debug, Eq, PartialEq)]
pub enum JsType {
    Undefined,
    Null,
    Number,
    String,
    Boolean,
    Object,
    Function,
    Error,
    Array,
    Symbol,
    ArrayBuffer,
    TypedArray,
    DataView,
}

pub struct JsValue {
    pub(crate) handle: JsValueRef,
}

impl JsValue {
    pub fn get_type(&self) -> Result<JsType, JsError> {
        let mut result = 0_u32;
        let res = unsafe { JsGetValueType(self.handle, &mut result) };
        JsError::assert(res)?;

        Ok(match result {
            0 => JsType::Undefined,
            1 => JsType::Null,
            2 => JsType::Number,
            3 => JsType::String,
            4 => JsType::Boolean,
            5 => JsType::Object,
            6 => JsType::Function,
            7 => JsType::Error,
            8 => JsType::Array,
            9 => JsType::Symbol,
            10 => JsType::ArrayBuffer,
            11 => JsType::TypedArray,
            12 => JsType::DataView,
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
    use crate::runtime::JsRuntime;
    use std::convert::TryFrom;

    #[test]
    fn get_type_number() {
        let number = JsNumber::try_from(42).unwrap();
        let value: JsValue = number.into();
        assert_eq!(value.get_type(), Ok(JsType::Number));
    }

    #[test]
    fn get_type_boolean() {
        let mut runtime = JsRuntime::new().unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        context.set_current_context().unwrap();

        let bool = JsBoolean::try_from(true).unwrap();
        let value: JsValue = bool.into();
        assert_eq!(value.get_type(), Ok(JsType::Boolean));
    }
}
