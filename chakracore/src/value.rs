use chakracore_sys::JsValueRef;

pub struct JsValue {
    pub(crate) handle: JsValueRef,
}
