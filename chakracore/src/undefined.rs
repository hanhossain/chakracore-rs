use crate::handle::IntoHandle;
use chakracore_sys::JsValueRef;
use std::ptr;

impl IntoHandle for () {
    fn into_handle(self) -> JsValueRef {
        ptr::null_mut()
    }
}
