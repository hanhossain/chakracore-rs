use crate::error::JsError;
use crate::script::JsScript;
use bitflags::bitflags;
use chakracore_sys::{
    JsCreateRuntime, JsDisposeRuntime, JsRun, JsRuntimeHandle,
    _JsParseScriptAttributes_JsParseScriptAttributeNone,
};
use std::mem::MaybeUninit;
use std::ptr;

bitflags! {
    pub struct JsRuntimeAttributes: u32 {
        /// No special attributes.
        const None = 0;

        /// The runtime will not do any work (such as garbage collection) on background threads.
        const DisableBackgroundWork = 1;

        /// The runtime should support reliable script interruption. This increases the number of
        /// places where the runtime will check for a script interrupt request at the cost of a
        /// small amount of runtime performance.
        const AllowScriptInterrupt = 2;

        /// Host will call JsIdle, so enable idle processing. Otherwise, the runtime will manage
        /// memory slightly more aggressively.
        const EnableIdleProcessing = 4;

        /// Runtime will not generate native code.
        const DisableNativeCodeGeneration = 8;

        /// Using eval or function constructor will throw an exception.
        const DisableEval = 16;

        /// Runtime will enable all experimental features.
        const EnableExperimentalFeatures = 32;

        /// Calling JsSetException will also dispatch the exception to the script debugger (if any)
        /// giving the debugger a chance to break on the exception.
        const DispatchSetExceptionsToDebugger = 64;

        /// Disable Failfast fatal error on OOM
        const DisableFatalOnOOM = 128;

        const DisableExecutablePageAllocation = 256;
    }
}

pub struct JsRuntime {
    pub(crate) handle: JsRuntimeHandle,
}

impl JsRuntime {
    /// Create a new JsRuntime.
    pub fn new(attributes: JsRuntimeAttributes) -> Result<Self, JsError> {
        let mut runtime: MaybeUninit<JsRuntimeHandle> = MaybeUninit::uninit();
        let res = unsafe { JsCreateRuntime(attributes.bits, None, runtime.as_mut_ptr()) };
        JsError::assert(res)?;

        Ok(Self {
            handle: unsafe { runtime.assume_init() },
        })
    }

    pub fn run_script(&mut self, script: &JsScript) -> Result<(), JsError> {
        let res = unsafe {
            JsRun(
                script.handle,
                0usize,
                script.source_url.handle,
                _JsParseScriptAttributes_JsParseScriptAttributeNone,
                ptr::null_mut(),
            )
        };
        JsError::assert(res)?;

        Ok(())
    }
}

impl Drop for JsRuntime {
    fn drop(&mut self) {
        unsafe {
            let res = JsDisposeRuntime(self.handle);
            JsError::assert(res).expect("Failed to dispose runtime.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::JsScriptContext;

    #[test]
    fn create_runtime() {
        let runtime = JsRuntime::new(JsRuntimeAttributes::None);
        assert_eq!(runtime.map(|x| x.handle.is_null()), Ok(false));
    }

    #[test]
    fn run_script() {
        let mut runtime = JsRuntime::new(JsRuntimeAttributes::None).unwrap();
        let mut context = JsScriptContext::new(&mut runtime).unwrap();
        context.set_current_context().unwrap();

        let script = JsScript::new("test", "(() => { var a = 1 + 1; })()").unwrap();
        runtime.run_script(&script).unwrap();
    }
}
