#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;
    use std::ffi::{CStr, CString};
    use std::ptr;

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
}
