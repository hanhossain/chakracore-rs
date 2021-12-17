use crate::error::JsError;
use crate::handle::IntoHandle;
use crate::number::JsNumber;
use crate::value::JsValue;
use chakracore_sys::{JsCreateFunction, JsValueRef};
use std::ffi::c_void;
use std::marker::PhantomData;
use std::os::raw::c_ushort;
use std::ptr;

unsafe extern "C" fn handler_unit(
    _callee: JsValueRef, // TODO: what should we do with the callee?
    is_construct_call: bool,
    arguments: *mut JsValueRef,
    argument_count: c_ushort,
    callback_state: *mut c_void,
) -> JsValueRef {
    let context = JsFunctionContext::new(argument_count, arguments, is_construct_call);
    let closure = &mut *(callback_state as *mut Box<dyn FnMut(JsFunctionContext)>);
    closure(context);
    ptr::null_mut()
}

unsafe extern "C" fn handler_i32(
    _callee: JsValueRef, // TODO: what should we do with the callee?
    is_construct_call: bool,
    arguments: *mut JsValueRef,
    argument_count: c_ushort,
    callback_state: *mut c_void,
) -> JsValueRef {
    let context = JsFunctionContext::new(argument_count, arguments, is_construct_call);
    let closure = &mut *(callback_state as *mut Box<dyn FnMut(JsFunctionContext) -> i32>);
    let result: i32 = closure(context);
    JsNumber::try_from(result).unwrap().handle
}

pub struct JsFunctionContext {
    pub argument_count: u16,
    pub arguments: Vec<JsValue>,
    pub is_construct_call: bool,
}

impl JsFunctionContext {
    fn new(argument_count: u16, arguments: *mut JsValueRef, is_construct_call: bool) -> Self {
        let mut args = Vec::new();
        for i in 0..argument_count as usize {
            args.push(JsValue {
                handle: unsafe { arguments.add(i).read() } as *mut _,
            });
        }

        JsFunctionContext {
            argument_count,
            arguments: args,
            is_construct_call,
        }
    }
}

#[derive(Debug)]
pub struct JsFunction<T> {
    handle: JsValueRef,
    _marker: PhantomData<T>,
}

impl JsFunction<()> {
    pub fn new<'a>(callback: Box<dyn FnMut(JsFunctionContext) + 'a>) -> Result<Self, JsError> {
        let callback = Box::new(callback);

        // TODO: don't forget to drop this later
        let callback = Box::into_raw(callback);

        let mut func = ptr::null_mut();
        let res = unsafe { JsCreateFunction(Some(handler_unit), callback as *mut _, &mut func) };
        JsError::assert(res)?;

        Ok(Self {
            handle: func,
            _marker: PhantomData,
        })
    }
}

impl JsFunction<i32> {
    pub fn new<'a>(
        callback: Box<dyn FnMut(JsFunctionContext) -> i32 + 'a>,
    ) -> Result<Self, JsError> {
        let callback = Box::new(callback);

        // TODO: don't forget to drop this later
        let callback = Box::into_raw(callback);

        let mut func = ptr::null_mut();
        let res = unsafe { JsCreateFunction(Some(handler_i32), callback as *mut _, &mut func) };
        JsError::assert(res)?;

        Ok(Self {
            handle: func,
            _marker: PhantomData,
        })
    }
}

impl<T> IntoHandle for JsFunction<T> {
    fn into_handle(self) -> JsValueRef {
        self.handle
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::JsScriptContext;
    use crate::number::JsNumber;
    use crate::object::JsObject;
    use crate::runtime::JsRuntime;
    use crate::script::JsScript;
    use crate::string::JsString;
    use crate::value::JsType;
    use std::convert::{TryFrom, TryInto};

    #[test]
    fn create_function() {
        let mut succeeded = false;

        let mut runtime = JsRuntime::new().unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        context.set_current_context().unwrap();

        let custom_handler = |_| succeeded = true;

        let hello_world = JsFunction::<()>::new(Box::new(custom_handler)).unwrap();
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
        let mut a: Vec<i32> = Vec::new();

        let custom_handler = |c: JsFunctionContext| {
            argument_count = c.argument_count;
            argument_types = c.arguments.iter().map(|x| x.get_type()).collect();
            is_construct_call = c.is_construct_call;
            a = c
                .arguments
                .into_iter()
                .skip(1)
                .map(|x| JsNumber { handle: x.handle })
                .map(|x| x.try_into().unwrap())
                .collect();
        };

        let hello_world = JsFunction::<()>::new(Box::new(custom_handler)).unwrap();
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

        assert_eq!(a, vec![1, 2, 3, 4]);
    }

    #[test]
    fn create_function_returns_i32() {
        let mut runtime = JsRuntime::new().unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        context.set_current_context().unwrap();

        let custom_handler = |_| 42;

        let hello_world = JsFunction::<i32>::new(Box::new(custom_handler)).unwrap();
        let key = JsString::new("helloWorld").unwrap();

        let mut global = JsObject::global().unwrap();
        global.set_property(&key, hello_world).unwrap();

        let script = JsScript::new("test", "helloWorld()").unwrap();
        let result: JsNumber = runtime.run_script(&script).unwrap().try_into().unwrap();
        assert_eq!(Ok(42), result.try_into());
    }

    #[test]
    fn create_function_with_parameters_returns_i32() {
        let mut runtime = JsRuntime::new().unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        context.set_current_context().unwrap();

        let custom_handler = |c: JsFunctionContext| {
            let arguments: Vec<i32> = c
                .arguments
                .into_iter()
                .skip(1)
                .map(|x| JsNumber { handle: x.handle })
                .map(|x| x.try_into().unwrap())
                .collect();
            arguments[0] + arguments[1]
        };

        let hello_world = JsFunction::<i32>::new(Box::new(custom_handler)).unwrap();
        let key = JsString::new("helloWorld").unwrap();

        let mut global = JsObject::global().unwrap();
        global.set_property(&key, hello_world).unwrap();

        let script = JsScript::new("test", "helloWorld(1, 2)").unwrap();
        let result: JsNumber = runtime.run_script(&script).unwrap().try_into().unwrap();
        assert_eq!(Ok(3), result.try_into());
    }

    fn hello_world_handle(context: JsFunctionContext) {
        if let Some(value) = context.arguments.into_iter().nth(1) {
            let value = JsString::try_from(value).unwrap();
            assert_eq!(&value.to_string().unwrap(), "hello world");
        }
    }

    #[test]
    fn create_hello_world_function() {
        let mut runtime = JsRuntime::new().unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        context.set_current_context().unwrap();

        let log = JsFunction::<()>::new(Box::new(hello_world_handle)).unwrap();
        let key = JsString::new("log").unwrap();
        let mut global = JsObject::global().unwrap();
        global.set_property(&key, log).unwrap();

        let script = JsScript::new("test", "log('hello world')").unwrap();
        runtime.run_script(&script).unwrap();
    }
}
