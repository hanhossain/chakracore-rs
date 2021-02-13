use crate::error::JsError;
use crate::value::JsValue;
use chakracore_sys::{
    JsConvertValueToNumber, JsDoubleToNumber, JsIntToNumber, JsNumberToDouble, JsNumberToInt,
    JsValueRef,
};
use std::convert::{TryFrom, TryInto};
use std::ptr;

#[derive(Debug)]
pub struct JsNumber {
    pub(crate) handle: JsValueRef,
}

impl TryFrom<i32> for JsNumber {
    type Error = JsError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let mut result = ptr::null_mut();

        let res = unsafe { JsIntToNumber(value, &mut result) };
        JsError::assert(res)?;

        Ok(Self { handle: result })
    }
}

impl TryFrom<f64> for JsNumber {
    type Error = JsError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        let mut result = ptr::null_mut();

        let res = unsafe { JsDoubleToNumber(value, &mut result) };
        JsError::assert(res)?;

        Ok(Self { handle: result })
    }
}

impl TryFrom<JsValue> for JsNumber {
    type Error = JsError;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let mut result = ptr::null_mut();
        let res = unsafe { JsConvertValueToNumber(value.handle, &mut result) };
        JsError::assert(res)?;

        Ok(JsNumber { handle: result })
    }
}

impl TryInto<i32> for JsNumber {
    type Error = JsError;

    fn try_into(self) -> Result<i32, Self::Error> {
        let mut result = 0;
        let res = unsafe { JsNumberToInt(self.handle, &mut result as *mut _) };
        JsError::assert(res)?;

        Ok(result)
    }
}

impl TryInto<f64> for JsNumber {
    type Error = JsError;

    fn try_into(self) -> Result<f64, Self::Error> {
        let mut result = 0_f64;
        let res = unsafe { JsNumberToDouble(self.handle, &mut result as *mut _) };
        JsError::assert(res)?;

        Ok(result)
    }
}

impl Into<JsValue> for JsNumber {
    fn into(self) -> JsValue {
        JsValue {
            handle: self.handle,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_from_int() {
        let number = JsNumber::try_from(42).unwrap();
        assert!(!number.handle.is_null());
    }

    #[test]
    fn convert_to_int() {
        let number = JsNumber::try_from(42).unwrap();
        assert_eq!(number.try_into(), Ok(42));
    }

    #[test]
    fn convert_from_double() {
        let number = JsNumber::try_from(3.14).unwrap();
        assert!(!number.handle.is_null());
    }

    #[test]
    fn convert_to_double() {
        let number = JsNumber::try_from(3.14).unwrap();
        assert_eq!(number.try_into(), Ok(3.14));
    }
}
