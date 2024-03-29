use crate::error::JsError;
use crate::value::JsValue;
use chakracore_sys::{
    JsConvertValueToNumber, JsDoubleToNumber, JsIntToNumber, JsNumberToDouble, JsNumberToInt,
    JsValueRef,
};
use std::fmt::{Debug, Formatter};
use std::ptr;

pub struct JsNumber {
    pub(crate) handle: JsValueRef,
}

impl From<i32> for JsNumber {
    fn from(value: i32) -> Self {
        let mut result = ptr::null_mut();
        let res = unsafe { JsIntToNumber(value, &mut result) };
        JsError::assert(res).unwrap();
        Self { handle: result }
    }
}

impl From<f64> for JsNumber {
    fn from(value: f64) -> Self {
        let mut result = ptr::null_mut();

        let res = unsafe { JsDoubleToNumber(value, &mut result) };
        JsError::assert(res).unwrap();

        Self { handle: result }
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

impl TryFrom<JsNumber> for i32 {
    type Error = JsError;

    fn try_from(value: JsNumber) -> Result<i32, Self::Error> {
        let mut result = 0;
        let res = unsafe { JsNumberToInt(value.handle, &mut result as *mut _) };
        JsError::assert(res)?;

        Ok(result)
    }
}

impl TryFrom<JsNumber> for f64 {
    type Error = JsError;

    fn try_from(value: JsNumber) -> Result<f64, Self::Error> {
        let mut result = 0_f64;
        let res = unsafe { JsNumberToDouble(value.handle, &mut result as *mut _) };
        JsError::assert(res)?;

        Ok(result)
    }
}

impl From<JsNumber> for JsValue {
    fn from(number: JsNumber) -> JsValue {
        JsValue {
            handle: number.handle,
        }
    }
}

impl Debug for JsNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = 0_f64;
        let res = unsafe { JsNumberToDouble(self.handle, &mut result as *mut _) };
        let error = JsError::assert(res);

        f.debug_struct("JsNumber")
            .field("value", &error.and(Ok(result)))
            .finish()
    }
}

impl From<i32> for JsValue {
    fn from(value: i32) -> JsValue {
        let number = JsNumber::from(value);
        JsValue {
            handle: number.handle,
        }
    }
}

impl From<f64> for JsValue {
    fn from(value: f64) -> JsValue {
        let number = JsNumber::from(value);
        JsValue {
            handle: number.handle,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_from_int() {
        let number = JsNumber::from(42);
        assert!(!number.handle.is_null());
    }

    #[test]
    fn convert_to_int() {
        let number = JsNumber::from(42);
        assert_eq!(number.try_into(), Ok(42));
    }

    #[test]
    fn convert_from_double() {
        let number = JsNumber::from(std::f64::consts::PI);
        assert!(!number.handle.is_null());
    }

    #[test]
    fn convert_to_double() {
        let number = JsNumber::from(std::f64::consts::PI);
        assert_eq!(number.try_into(), Ok(std::f64::consts::PI));
    }
}
