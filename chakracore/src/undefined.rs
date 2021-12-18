use crate::value::JsValue;
use std::ptr;

impl From<()> for JsValue {
    fn from(_: ()) -> JsValue {
        JsValue {
            handle: ptr::null_mut(),
        }
    }
}
