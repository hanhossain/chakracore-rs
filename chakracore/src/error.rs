use std::os::raw::c_uint;
use thiserror::Error;

// TODO: may want to deny dead_code
#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum JsError {
    /// Category of errors that relates to incorrect usage of the API itself.
    #[error("Category of errors that relates to incorrect usage of the API itself.")]
    CategoryUsage,

    /// An argument to a hosting API was invalid.
    #[error("An argument to a hosting API was invalid.")]
    InvalidArgument,

    /// An argument to a hosting API was null in a context where null is not allowed.
    #[error("An argument to a hosting API was null in a context where null is not allowed.")]
    NullArgument,

    /// The hosting API requires that a context be current, but there is no current context.
    #[error(
        "The hosting API requires that a context be current, but there is no current context."
    )]
    NoCurrentContext,

    /// The engine is in an exception state and no APIs can be called until the exception is
    /// cleared.
    #[error(
        "The engine is in an exception state and no APIs can be called until the exception is \
        cleared."
    )]
    InExceptionState,

    /// A hosting API is not yet implemented.
    #[error("A hosting API is not yet implemented.")]
    NotImplemented,

    /// A hosting API was called on the wrong thread.
    #[error("A hosting API was called on the wrong thread.")]
    WrongThread,

    /// A runtime that is still in use cannot be disposed.
    #[error("A runtime that is still in use cannot be disposed.")]
    RuntimeInUse,

    /// A bad serialized script was used, or the serialized script was serialized by a different
    /// version of the Chakra engine.
    #[error(
        "A bad serialized script was used, or the serialized script was serialized by a different \
        version of the Chakra engine."
    )]
    BadSerializedScript,

    /// The runtime is in a disabled state.
    #[error("The runtime is in a disabled state.")]
    InDisabledState,

    /// Runtime does not support reliable script interruption.
    #[error("Runtime does not support reliable script interruption.")]
    CannotDisableExecution,

    /// A heap enumeration is currently underway in the script context.
    #[error("A heap enumeration is currently underway in the script context.")]
    HeapEnumInProgress,

    /// A hosting API that operates on object values was called with a non-object value.
    #[error("A hosting API that operates on object values was called with a non-object value.")]
    ArgumentNotObject,

    /// A script context is in the middle of a profile callback.
    #[error("A script context is in the middle of a profile callback.")]
    InProfileCallback,

    /// A thread service callback is currently underway.
    #[error("A thread service callback is currently underway.")]
    InThreadServiceCallback,

    /// Scripts cannot be serialized in debug contexts.
    #[error("Scripts cannot be serialized in debug contexts.")]
    CannotSerializeDebugScript,

    /// The context cannot be put into a debug state because it is already in a debug state.
    #[error(
        "The context cannot be put into a debug state because it is already in a debug state."
    )]
    AlreadyDebuggingContext,

    /// The context cannot start profiling because it is already profiling.
    #[error("The context cannot start profiling because it is already profiling.")]
    AlreadyProfilingContext,

    /// Idle notification given when the host did not enable idle processing.
    #[error("Idle notification given when the host did not enable idle processing.")]
    IdleNotEnabled,

    /// The context did not accept the enqueue callback.
    #[error("The context did not accept the enqueue callback.")]
    CannotSetProjectionEnqueueCallback,

    /// Failed to start projection.
    #[error("Failed to start projection.")]
    CannotStartProjection,

    /// The operation is not supported in an object before collect callback.
    #[error("The operation is not supported in an object before collect callback.")]
    InObjectBeforeCollectCallback,

    /// Object cannot be unwrapped to IInspectable pointer.
    #[error("Object cannot be unwrapped to IInspectable pointer.")]
    ObjectNotInspectable,

    /// "A hosting API that operates on symbol property ids but was called with a non-symbol
    /// property id. The error code is returned by JsGetSymbolFromPropertyId if the function is
    /// called with non-symbol property id."
    #[error(
        "A hosting API that operates on symbol property ids but was called with a non-symbol \
        property id. The error code is returned by JsGetSymbolFromPropertyId if the function is \
        called with non-symbol property id."
    )]
    PropertyNotSymbol,

    /// A hosting API that operates on string property ids but was called with a non-string property
    /// id. The error code is returned by existing JsGetPropertyNamefromId if the function is called
    /// with non-string property id.
    #[error(
        "A hosting API that operates on string property ids but was called with a non-string \
        property id. The error code is returned by existing JsGetPropertyNamefromId if the \
        function is called with non-string property id."
    )]
    PropertyNotString,

    /// Module evaluation is called in wrong context.
    #[error("Module evaluation is called in wrong context.")]
    InvalidContext,

    /// Module evaluation is called in wrong context.
    #[error("Module evaluation is called in wrong context.")]
    InvalidModuleHostInfoKind,

    /// Module was parsed already when JsParseModuleSource is called.
    #[error("Module was parsed already when JsParseModuleSource is called.")]
    ModuleParsed,

    /// Argument passed to JsCreateWeakReference is a primitive that is not managed by the GC. No
    /// weak reference is required, the value will never be collected.
    #[error(
        "Argument passed to JsCreateWeakReference is a primitive that is not managed by the GC. \
        No weak reference is required, the value will never be collected."
    )]
    NoWeakRefRequired,

    /// The Promise object is still in the pending state.
    #[error("The Promise object is still in the pending state.")]
    PromisePending,

    /// Module was not yet evaluated when JsGetModuleNamespace was called.
    #[error("Module was not yet evaluated when JsGetModuleNamespace was called.")]
    ModuleNotEvaluated,

    /// Category of errors that relates to errors occurring within the engine itself.
    #[error("Category of errors that relates to errors occurring within the engine itself.")]
    CategoryEngine,

    /// The Chakra engine has run out of memory.
    #[error("The Chakra engine has run out of memory.")]
    OutOfMemory,

    /// The Chakra engine failed to set the Floating Point Unit state.
    #[error("The Chakra engine failed to set the Floating Point Unit state.")]
    BadFPUState,

    /// Category of errors that relates to errors in a script.
    #[error("Category of errors that relates to errors in a script.")]
    CategoryScript,

    /// A JavaScript exception occurred while running a script.
    #[error("A JavaScript exception occurred while running a script.")]
    ScriptException,

    /// JavaScript failed to compile.
    #[error("JavaScript failed to compile.")]
    ScriptCompile,

    /// A script was terminated due to a request to suspend a runtime.
    #[error("A script was terminated due to a request to suspend a runtime.")]
    ScriptTerminated,

    /// A script was terminated because it tried to use eval or function and eval was disabled.
    #[error(
        "A script was terminated because it tried to use eval or function and eval was disabled."
    )]
    ScriptEvalDisabled,

    /// Category of errors that are fatal and signify failure of the engine.
    #[error("Category of errors that are fatal and signify failure of the engine.")]
    CategoryFatal,

    /// A fatal error in the engine has occurred.
    #[error("A fatal error in the engine has occurred.")]
    Fatal,

    /// A hosting API was called with object created on different javascript runtime.
    #[error("A hosting API was called with object created on different javascript runtime.")]
    WrongRuntime,

    /// Category of errors that are related to failures during diagnostic operations.
    #[error("Category of errors that are related to failures during diagnostic operations.")]
    CategoryDiagError,

    /// The object for which the debugging API was called was not found.
    #[error("The object for which the debugging API was called was not found.")]
    DiagAlreadyInDebugMode,

    /// The debugging API can only be called when VM is in debug mode.
    #[error("The debugging API can only be called when VM is in debug mode.")]
    DiagNotInDebugMode,

    /// The debugging API can only be called when VM is at a break.
    #[error("The debugging API can only be called when VM is at a break.")]
    DiagNotAtBreak,

    /// Debugging API was called with an invalid handle.
    #[error("Debugging API was called with an invalid handle.")]
    DiagInvalidHandle,

    /// The object for which the debugging API was called was not found.
    #[error("The object for which the debugging API was called was not found.")]
    DiagObjectNotFound,

    /// VM was unable to perform the request action.
    #[error("VM was unable to perform the request action.")]
    DiagUnableToPerformAction,
}

impl JsError {
    fn from(error_code: c_uint) -> Option<Self> {
        match error_code {
            0 => None,
            65536 => Some(JsError::CategoryUsage),
            65537 => Some(JsError::InvalidArgument),
            65538 => Some(JsError::NullArgument),
            65539 => Some(JsError::NoCurrentContext),
            65540 => Some(JsError::InExceptionState),
            65541 => Some(JsError::NotImplemented),
            65542 => Some(JsError::WrongThread),
            65543 => Some(JsError::RuntimeInUse),
            65544 => Some(JsError::BadSerializedScript),
            65545 => Some(JsError::InDisabledState),
            65546 => Some(JsError::CannotDisableExecution),
            65547 => Some(JsError::HeapEnumInProgress),
            65548 => Some(JsError::ArgumentNotObject),
            65549 => Some(JsError::InProfileCallback),
            65550 => Some(JsError::InThreadServiceCallback),
            65551 => Some(JsError::CannotSerializeDebugScript),
            65552 => Some(JsError::AlreadyDebuggingContext),
            65553 => Some(JsError::AlreadyProfilingContext),
            65554 => Some(JsError::IdleNotEnabled),
            65555 => Some(JsError::CannotSetProjectionEnqueueCallback),
            65556 => Some(JsError::CannotStartProjection),
            65557 => Some(JsError::InObjectBeforeCollectCallback),
            65558 => Some(JsError::ObjectNotInspectable),
            65559 => Some(JsError::PropertyNotSymbol),
            65560 => Some(JsError::PropertyNotString),
            65561 => Some(JsError::InvalidContext),
            65562 => Some(JsError::InvalidModuleHostInfoKind),
            65563 => Some(JsError::ModuleParsed),
            65564 => Some(JsError::NoWeakRefRequired),
            65565 => Some(JsError::PromisePending),
            65566 => Some(JsError::ModuleNotEvaluated),
            131072 => Some(JsError::CategoryEngine),
            131073 => Some(JsError::OutOfMemory),
            131074 => Some(JsError::BadFPUState),
            196608 => Some(JsError::CategoryScript),
            196609 => Some(JsError::ScriptException),
            196610 => Some(JsError::ScriptCompile),
            196611 => Some(JsError::ScriptTerminated),
            196612 => Some(JsError::ScriptEvalDisabled),
            262144 => Some(JsError::CategoryFatal),
            262145 => Some(JsError::Fatal),
            262146 => Some(JsError::WrongRuntime),
            327680 => Some(JsError::CategoryDiagError),
            327681 => Some(JsError::DiagAlreadyInDebugMode),
            327682 => Some(JsError::DiagNotInDebugMode),
            327683 => Some(JsError::DiagNotAtBreak),
            327684 => Some(JsError::DiagInvalidHandle),
            327685 => Some(JsError::DiagObjectNotFound),
            327686 => Some(JsError::DiagUnableToPerformAction),
            _ => unreachable!(),
        }
    }
}
