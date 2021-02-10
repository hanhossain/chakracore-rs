use crate::error::JsError;
use crate::value::JsValue;
use chakracore_sys::{JsCreateFunction, JsValueRef};
use std::ffi::c_void;
use std::os::raw::c_ushort;
use std::ptr;

unsafe extern "C" fn handler(
    _callee: JsValueRef,
    _is_construct_call: bool,
    _arguments: *mut JsValueRef,
    _argument_count: c_ushort,
    callback_state: *mut c_void,
) -> JsValueRef {
    // todo: null check
    let closure: &mut Box<dyn Fn()> = std::mem::transmute(callback_state);
    closure();

    ptr::null_mut()
}

#[derive(Debug)]
pub struct JsFunction {
    handle: JsValueRef,
}

impl JsFunction {
    pub fn new(callback: Box<dyn Fn()>) -> Result<Self, JsError> {
        let callback = Box::new(callback);

        // TODO: don't forget to drop this later
        let callback = Box::into_raw(callback);

        let mut func = ptr::null_mut();
        let res = unsafe { JsCreateFunction(Some(handler), callback as *mut _, &mut func) };
        JsError::assert(res)?;

        Ok(Self { handle: func })
    }
}

impl Into<JsValue> for JsFunction {
    fn into(self) -> JsValue {
        JsValue {
            handle: self.handle,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::JsScriptContext;
    use crate::object::JsObject;
    use crate::runtime::{JsRuntime, JsRuntimeAttributes};
    use crate::script::JsScript;
    use crate::string::JsString;

    static mut BASIC_FUNCTION_DONE: bool = false;

    #[test]
    fn create_basic_function() {
        let mut runtime = JsRuntime::new(JsRuntimeAttributes::None).unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        context.set_current_context().unwrap();

        fn custom_handler() {
            unsafe {
                BASIC_FUNCTION_DONE = true;
            }
        }

        let hello_world = JsFunction::new(Box::new(custom_handler)).unwrap();
        let key = JsString::new("helloWorld").unwrap();

        let mut global = JsObject::global().unwrap();
        global.set_property(&key, hello_world).unwrap();

        let script = JsScript::new("test", "helloWorld()").unwrap();
        runtime.run_script(&script).unwrap();

        unsafe {
            assert!(BASIC_FUNCTION_DONE);
        }
    }
}
