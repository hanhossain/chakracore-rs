use crate::error::JsError;
use crate::value::JsValue;
use chakracore_sys::{
    JsDoubleToNumber, JsIntToNumber, JsNumberToDouble, JsNumberToInt, JsValueRef,
};
use std::ptr;

#[derive(Debug)]
pub struct JsNumber {
    pub(crate) handle: JsValueRef,
}

impl JsNumber {
    /// Create a JsNumber from an i32
    pub fn from_i32(val: i32) -> Result<Self, JsError> {
        let mut result = ptr::null_mut();

        let res = unsafe { JsIntToNumber(val, &mut result) };
        JsError::assert(res)?;

        Ok(Self { handle: result })
    }

    /// Convert a JsNumber to an i32
    pub fn to_i32(&self) -> Result<i32, JsError> {
        let mut result = 0;
        let res = unsafe { JsNumberToInt(self.handle, &mut result as *mut _) };
        JsError::assert(res)?;

        Ok(result)
    }

    /// Create a JsNumber from a f64
    pub fn from_f64(val: f64) -> Result<Self, JsError> {
        let mut result = ptr::null_mut();

        let res = unsafe { JsDoubleToNumber(val, &mut result) };
        JsError::assert(res)?;

        Ok(Self { handle: result })
    }

    /// Convert a JsNumber to an f64
    pub fn to_f64(&self) -> Result<f64, JsError> {
        let mut result = 0f64;
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

impl Into<JsValue> for &JsNumber {
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
        let number = JsNumber::from_i32(42).unwrap();
        assert!(!number.handle.is_null());
    }

    #[test]
    fn convert_to_int() {
        let number = JsNumber::from_i32(42).unwrap();
        assert_eq!(number.to_i32(), Ok(42));
    }

    #[test]
    fn convert_from_double() {
        let number = JsNumber::from_f64(3.14).unwrap();
        assert!(!number.handle.is_null());
    }

    #[test]
    fn convert_to_double() {
        let number = JsNumber::from_f64(3.14).unwrap();
        assert_eq!(number.to_f64(), Ok(3.14));
    }
}
