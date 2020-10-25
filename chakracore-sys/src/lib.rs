#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;
    use std::ffi::{CStr, CString};
    use std::mem::MaybeUninit;
    use std::ptr;

    fn assert_no_error(error_code: _JsErrorCode) {
        assert_eq!(error_code, _JsErrorCode_JsNoError);
    }

    #[test]
    fn basic() {
        let script = "(()=>{return 'Hello World!';})()";
        let script_c = CString::new(script).unwrap().into_raw();

        unsafe {
            let mut runtime: MaybeUninit<JsRuntimeHandle> = MaybeUninit::uninit();
            let mut context: MaybeUninit<JsContextRef> = MaybeUninit::uninit();

            // Create a runtime.
            let res = JsCreateRuntime(
                _JsRuntimeAttributes_JsRuntimeAttributeNone,
                None,
                runtime.as_mut_ptr(),
            );
            assert_no_error(res);
            let runtime = runtime.assume_init();

            // Create an execution context.
            let res = JsCreateContext(runtime, context.as_mut_ptr());
            assert_no_error(res);
            let context = context.assume_init();

            // Now set the current execution context.
            let res = JsSetCurrentContext(context);
            assert_no_error(res);

            let mut fname = MaybeUninit::uninit();
            let sample = CString::new("sample").unwrap();
            let res = JsCreateString(sample.as_ptr(), 6, fname.as_mut_ptr());
            assert_no_error(res);
            let fname = fname.assume_init();

            let mut script_source = MaybeUninit::uninit();
            let res = JsCreateExternalArrayBuffer(
                script_c as *mut _,
                script.len() as u32,
                None,
                ptr::null_mut(),
                script_source.as_mut_ptr(),
            );
            assert_no_error(res);
            let script_source = script_source.assume_init();

            // run the script
            let mut result = MaybeUninit::uninit();
            let res = JsRun(
                script_source as *mut _,
                0usize,
                fname as *mut _,
                _JsParseScriptAttributes_JsParseScriptAttributeNone,
                result.as_mut_ptr(),
            );
            assert_no_error(res);
            let result = result.assume_init();

            // Convert your script result to String in JavaScript; redundant if your script returns a String
            let mut resultJSString = MaybeUninit::uninit();
            let res = JsConvertValueToString(result as *mut _, resultJSString.as_mut_ptr());
            assert_no_error(res);
            let resultJSString = resultJSString.assume_init();

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
