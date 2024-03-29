#![allow(deref_nullptr)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::redundant_static_lifetimes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{c_void, CStr, CString};
    use std::os::raw::c_ushort;
    use std::ptr;

    static mut BASIC_LOGGING_DONE: bool = false;
    static mut BASIC_LOGGING_WITH_CALLBACK_DONE: bool = false;

    fn assert_no_error(error_code: _JsErrorCode) {
        assert_eq!(error_code, _JsErrorCode_JsNoError);
    }

    #[test]
    fn basic() {
        let script = "(()=>{return 'Hello World!';})()";
        let script_c = CString::new(script).unwrap().into_raw();

        unsafe {
            let mut runtime: JsRuntimeHandle = ptr::null_mut();
            let mut context: JsContextRef = ptr::null_mut();

            // Create a runtime.
            let res = JsCreateRuntime(
                _JsRuntimeAttributes_JsRuntimeAttributeNone,
                None,
                &mut runtime,
            );
            assert_no_error(res);

            // Create an execution context.
            let res = JsCreateContext(runtime, &mut context);
            assert_no_error(res);

            // Now set the current execution context.
            let res = JsSetCurrentContext(context);
            assert_no_error(res);

            let mut fname = ptr::null_mut();
            let sample = CString::new("sample").unwrap();
            let res = JsCreateString(sample.as_ptr(), 6, &mut fname);
            assert_no_error(res);

            let mut script_source = ptr::null_mut();
            let res = JsCreateExternalArrayBuffer(
                script_c as *mut _,
                script.len() as u32,
                None,
                ptr::null_mut(),
                &mut script_source,
            );
            assert_no_error(res);

            // run the script
            let mut result = ptr::null_mut();
            let res = JsRun(
                script_source as *mut _,
                0usize,
                fname as *mut _,
                _JsParseScriptAttributes_JsParseScriptAttributeNone,
                &mut result,
            );
            assert_no_error(res);

            // Convert your script result to String in JavaScript; redundant if your script returns a String
            let mut resultJSString = ptr::null_mut();
            let res = JsConvertValueToString(result as *mut _, &mut resultJSString);
            assert_no_error(res);

            // get size of buffer
            let mut length = 0;
            JsCopyString(resultJSString as *mut _, ptr::null_mut(), 0, &mut length);

            // copy to buffer
            let total_length = length + 1;
            let mut buffer: Vec<u8> = vec![0; total_length.try_into().unwrap()];
            let res = JsCopyString(
                resultJSString as *mut _,
                buffer.as_mut_ptr() as *mut i8,
                total_length,
                ptr::null_mut(),
            );
            assert_no_error(res);

            // convert string back and assert
            let result_str = CStr::from_bytes_with_nul(buffer.as_slice()).unwrap();
            assert_eq!(result_str.to_str(), Ok("Hello World!"));

            // release script_c memory
            let _ = CString::from_raw(script_c);

            // clear current context
            let res = JsSetCurrentContext(ptr::null_mut());
            assert_no_error(res);

            // dispose runtime
            let res = JsDisposeRuntime(runtime);
            assert_no_error(res);
        }
    }

    unsafe extern "C" fn log(
        _callee: JsValueRef,
        _is_construct_call: bool,
        _arguments: *mut JsValueRef,
        _argument_count: c_ushort,
        _callback_state: *mut c_void,
    ) -> JsValueRef {
        BASIC_LOGGING_DONE = true;
        ptr::null_mut()
    }

    #[test]
    fn basic_logging() {
        let script = "(() => { console.log('hello world'); })()";
        let script_c = CString::new(script).unwrap().into_raw();

        unsafe {
            let mut runtime: JsRuntimeHandle = ptr::null_mut();
            let mut context: JsContextRef = ptr::null_mut();

            // Create a runtime.
            let res = JsCreateRuntime(
                _JsRuntimeAttributes_JsRuntimeAttributeNone,
                None,
                &mut runtime,
            );
            assert_no_error(res);

            // Create an execution context.
            let res = JsCreateContext(runtime, &mut context);
            assert_no_error(res);

            // Now set the current execution context.
            let res = JsSetCurrentContext(context);
            assert_no_error(res);

            // create console object
            let mut console = ptr::null_mut();
            assert_no_error(JsCreateObject(&mut console));

            // create log function
            let mut log_func = ptr::null_mut();
            assert_no_error(JsCreateFunction(Some(log), ptr::null_mut(), &mut log_func));
            let log_string = CString::new("log").unwrap();
            let mut log_prop_id = ptr::null_mut();
            assert_no_error(JsCreatePropertyId(
                log_string.as_ptr(),
                log_string.as_bytes().len() as u64,
                &mut log_prop_id,
            ));
            assert_no_error(JsSetProperty(console, log_prop_id, log_func, true));

            // set console as property of global object
            let mut global = ptr::null_mut();
            assert_no_error(JsGetGlobalObject(&mut global));
            let console_string = CString::new("console").unwrap();
            let mut console_prop_id = ptr::null_mut();
            assert_no_error(JsCreatePropertyId(
                console_string.as_ptr(),
                console_string.as_bytes().len() as u64,
                &mut console_prop_id,
            ));
            assert_no_error(JsSetProperty(global, console_prop_id, console, true));

            let mut fname = ptr::null_mut();
            let sample = CString::new("sample").unwrap();
            let res = JsCreateString(sample.as_ptr(), 6, &mut fname);
            assert_no_error(res);

            let mut script_source = ptr::null_mut();
            let res = JsCreateExternalArrayBuffer(
                script_c as *mut _,
                script.len() as u32,
                None,
                ptr::null_mut(),
                &mut script_source,
            );
            assert_no_error(res);

            // run the script
            let mut result = ptr::null_mut();
            let res = JsRun(
                script_source as *mut _,
                0usize,
                fname as *mut _,
                _JsParseScriptAttributes_JsParseScriptAttributeNone,
                &mut result,
            );
            assert_no_error(res);

            // release script_c memory
            let _ = CString::from_raw(script_c);

            // clear current context
            let res = JsSetCurrentContext(ptr::null_mut());
            assert_no_error(res);

            // dispose runtime
            let res = JsDisposeRuntime(runtime);
            assert_no_error(res);

            assert!(BASIC_LOGGING_DONE);
        }
    }

    unsafe extern "C" fn handler(
        _callee: JsValueRef,
        _is_construct_call: bool,
        _arguments: *mut JsValueRef,
        _argument_count: c_ushort,
        callback_state: *mut c_void,
    ) -> JsValueRef {
        if !callback_state.is_null() {
            let closure: &mut Box<dyn Fn()> = std::mem::transmute(callback_state);
            closure();
        }

        ptr::null_mut()
    }

    #[test]
    fn basic_logging_with_callback() {
        let script = "(() => { console.log('hello world'); })()";
        let script_c = CString::new(script).unwrap().into_raw();

        unsafe {
            let mut runtime: JsRuntimeHandle = ptr::null_mut();
            let mut context: JsContextRef = ptr::null_mut();

            // Create a runtime.
            let res = JsCreateRuntime(
                _JsRuntimeAttributes_JsRuntimeAttributeNone,
                None,
                &mut runtime,
            );
            assert_no_error(res);

            // Create an execution context.
            let res = JsCreateContext(runtime, &mut context);
            assert_no_error(res);

            // Now set the current execution context.
            let res = JsSetCurrentContext(context);
            assert_no_error(res);

            // create console object
            let mut console = ptr::null_mut();
            assert_no_error(JsCreateObject(&mut console));

            fn custom_handler() {
                unsafe {
                    BASIC_LOGGING_WITH_CALLBACK_DONE = true;
                }
            }

            let cb: Box<Box<dyn Fn()>> = Box::new(Box::new(custom_handler));
            let cb = Box::into_raw(cb);

            // set handler
            let mut log_func = ptr::null_mut();
            assert_no_error(JsCreateFunction(Some(handler), cb as *mut _, &mut log_func));
            let log_string = CString::new("log").unwrap();
            let mut log_prop_id = ptr::null_mut();
            assert_no_error(JsCreatePropertyId(
                log_string.as_ptr(),
                log_string.as_bytes().len() as u64,
                &mut log_prop_id,
            ));
            assert_no_error(JsSetProperty(console, log_prop_id, log_func, true));

            // set console as property of global object
            let mut global = ptr::null_mut();
            assert_no_error(JsGetGlobalObject(&mut global));
            let console_string = CString::new("console").unwrap();
            let mut console_prop_id = ptr::null_mut();
            assert_no_error(JsCreatePropertyId(
                console_string.as_ptr(),
                console_string.as_bytes().len() as u64,
                &mut console_prop_id,
            ));
            assert_no_error(JsSetProperty(global, console_prop_id, console, true));

            let mut fname = ptr::null_mut();
            let sample = CString::new("sample").unwrap();
            let res = JsCreateString(sample.as_ptr(), 6, &mut fname);
            assert_no_error(res);

            let mut script_source = ptr::null_mut();
            let res = JsCreateExternalArrayBuffer(
                script_c as *mut _,
                script.len() as u32,
                None,
                ptr::null_mut(),
                &mut script_source,
            );
            assert_no_error(res);

            // run the script
            let mut result = ptr::null_mut();
            let res = JsRun(
                script_source as *mut _,
                0usize,
                fname as *mut _,
                _JsParseScriptAttributes_JsParseScriptAttributeNone,
                &mut result,
            );
            assert_no_error(res);

            // release script_c memory
            let _ = CString::from_raw(script_c);

            // release callback
            let _ = Box::from_raw(cb);

            // clear current context
            let res = JsSetCurrentContext(ptr::null_mut());
            assert_no_error(res);

            // dispose runtime
            let res = JsDisposeRuntime(runtime);
            assert_no_error(res);

            assert!(BASIC_LOGGING_WITH_CALLBACK_DONE);
        }
    }

    unsafe extern "C" fn handler_with_return(
        _callee: JsValueRef,
        _is_construct_call: bool,
        _arguments: *mut JsValueRef,
        _argument_count: c_ushort,
        _callback_state: *mut c_void,
    ) -> JsValueRef {
        let mut value = ptr::null_mut();
        assert_no_error(JsIntToNumber(42, &mut value));
        value
    }

    #[test]
    fn handler_returns_something() {
        let script = "getValue()";
        let script_c = CString::new(script).unwrap().into_raw();

        unsafe {
            let mut runtime: JsRuntimeHandle = ptr::null_mut();
            let mut context: JsContextRef = ptr::null_mut();

            // Create a runtime.
            let res = JsCreateRuntime(
                _JsRuntimeAttributes_JsRuntimeAttributeNone,
                None,
                &mut runtime,
            );
            assert_no_error(res);

            // Create an execution context.
            let res = JsCreateContext(runtime, &mut context);
            assert_no_error(res);

            // Now set the current execution context.
            let res = JsSetCurrentContext(context);
            assert_no_error(res);

            // set handler
            let mut handler = ptr::null_mut();
            assert_no_error(JsCreateFunction(
                Some(handler_with_return),
                ptr::null_mut(),
                &mut handler,
            ));
            let func_string = CString::new("getValue").unwrap();
            let mut func_name = ptr::null_mut();
            assert_no_error(JsCreatePropertyId(
                func_string.as_ptr(),
                func_string.as_bytes().len() as u64,
                &mut func_name,
            ));

            // set func as global
            let mut global = ptr::null_mut();
            assert_no_error(JsGetGlobalObject(&mut global));
            assert_no_error(JsSetProperty(global, func_name, handler, true));

            let mut fname = ptr::null_mut();
            let sample = CString::new("sample").unwrap();
            let res = JsCreateString(sample.as_ptr(), 6, &mut fname);
            assert_no_error(res);

            let mut script_source = ptr::null_mut();
            let res = JsCreateExternalArrayBuffer(
                script_c as *mut _,
                script.len() as u32,
                None,
                ptr::null_mut(),
                &mut script_source,
            );
            assert_no_error(res);

            // run the script
            let mut result = ptr::null_mut();
            let res = JsRun(
                script_source as *mut _,
                0usize,
                fname as *mut _,
                _JsParseScriptAttributes_JsParseScriptAttributeNone,
                &mut result,
            );
            assert_no_error(res);

            // convert result to int
            let mut int_result = 0;
            assert_no_error(JsNumberToInt(result, &mut int_result as *mut _));
            assert_eq!(int_result, 42);

            // release script_c memory
            let _ = CString::from_raw(script_c);

            // clear current context
            let res = JsSetCurrentContext(ptr::null_mut());
            assert_no_error(res);

            // dispose runtime
            let res = JsDisposeRuntime(runtime);
            assert_no_error(res);
        }
    }
}
