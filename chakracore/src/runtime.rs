use bitflags::bitflags;
use chakracore_sys::JsRuntimeHandle;

bitflags! {
    struct JsRuntimeAttributes: u32 {
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

struct JsRuntime {
    handle: JsRuntimeHandle,
}

impl JsRuntime {
    fn new() -> Self {
        todo!()
    }
}
