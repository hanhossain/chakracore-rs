use chakracore_sys::JsValueRef;

pub trait IntoHandle {
    fn into_handle(self) -> JsValueRef;
}
