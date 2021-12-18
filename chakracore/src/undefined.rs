use crate::value::JsValue;
use std::ptr;

impl Into<JsValue> for () {
    fn into(self) -> JsValue {
        JsValue {
            handle: ptr::null_mut(),
        }
    }
}
