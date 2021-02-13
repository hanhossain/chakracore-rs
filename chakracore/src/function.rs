use crate::error::JsError;
use crate::value::JsValue;
use chakracore_sys::{JsCreateFunction, JsValueRef};
use std::ffi::c_void;
use std::os::raw::c_ushort;
use std::ptr;

unsafe extern "C" fn handler(
    _callee: JsValueRef, // TODO: what should we do with the callee?
    is_construct_call: bool,
    arguments: *mut JsValueRef,
    argument_count: c_ushort,
    callback_state: *mut c_void,
) -> JsValueRef {
    let mut args = Vec::new();
    for i in 0..argument_count as usize {
        args.push(JsValue {
            handle: arguments.add(i).read() as *mut _,
        });
    }

    let context = JsFunctionContext {
        argument_count,
        arguments: args,
        is_construct_call,
    };
    let closure = &mut *(callback_state as *mut Box<dyn FnMut(JsFunctionContext)>);
    closure(context);

    ptr::null_mut()
}

pub struct JsFunctionContext {
    pub argument_count: u16,
    pub arguments: Vec<JsValue>,
    pub is_construct_call: bool,
}

#[derive(Debug)]
pub struct JsFunction {
    handle: JsValueRef,
}

impl JsFunction {
    pub fn new<'a>(callback: Box<dyn FnMut(JsFunctionContext) + 'a>) -> Result<Self, JsError> {
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
    use crate::runtime::JsRuntime;
    use crate::script::JsScript;
    use crate::string::JsString;
    use crate::value::JsType;

    #[test]
    fn create_function() {
        let mut succeeded = false;

        let mut runtime = JsRuntime::new().unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        context.set_current_context().unwrap();

        let custom_handler = |_| succeeded = true;

        let hello_world = JsFunction::new(Box::new(custom_handler)).unwrap();
        let key = JsString::new("helloWorld").unwrap();

        let mut global = JsObject::global().unwrap();
        global.set_property(&key, hello_world).unwrap();

        let script = JsScript::new("test", "helloWorld()").unwrap();
        runtime.run_script(&script).unwrap();

        assert!(succeeded);
    }

    #[test]
    fn create_function_with_parameters() {
        let mut runtime = JsRuntime::new().unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        context.set_current_context().unwrap();

        let mut argument_count = 0;
        let mut argument_types = Vec::new();
        let mut is_construct_call = true;

        // TODO: need to assert the arguments are correct
        // TODO: will require messing with the Js models though
        let custom_handler = |c: JsFunctionContext| {
            argument_count = c.argument_count;
            argument_types = c.arguments.iter().map(|x| x.get_type()).collect();
            is_construct_call = c.is_construct_call;
        };

        let hello_world = JsFunction::new(Box::new(custom_handler)).unwrap();
        let key = JsString::new("helloWorld").unwrap();

        let mut global = JsObject::global().unwrap();
        global.set_property(&key, hello_world).unwrap();

        let script = JsScript::new("test", "helloWorld(1, 2, 3, 4)").unwrap();
        runtime.run_script(&script).unwrap();

        assert_eq!(argument_count, 5);
        assert_eq!(
            argument_types,
            vec![
                Ok(JsType::Undefined),
                Ok(JsType::Number),
                Ok(JsType::Number),
                Ok(JsType::Number),
                Ok(JsType::Number),
            ]
        );
        assert!(!is_construct_call);
    }
}
