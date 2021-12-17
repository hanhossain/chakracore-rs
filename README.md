# chakracore-rs
Targets release `v1.11.24`.

See [JSRT Reference](https://github.com/chakra-core/ChakraCore/wiki/JavaScript-Runtime-%28JSRT%29-Reference) for the raw API.

## Important Todos:
- [x] A nice way to add functions to an object (like `console.log('hello world')`)
- [ ] A way to add functions that can return a value to an object (to allow something like `var a = sum(1, 2)`)
  - [x] returns ()
  - [x] returns i32
  - [ ] returns i64
  - [ ] returns isize
  - [ ] returns u32
  - [ ] returns u64
  - [ ] returns usize
  - [ ] returns f32
  - [ ] returns f64
  - [ ] other types?
- [ ] Remove /usr/local/lib/libChakraCore.dylib
- [ ] Pull in a static chakracore lib through a git submodule
- [ ] A way to write a strongly typed handler

## JSRT Typedef References:

- [ ] FetchImportedModuleCallback
- [ ] FetchImportedModuleFromScriptCallback
- [ ] NotifyModuleReadyCallback
- [ ] JsBackgroundWorkItemCallback
- [ ] JsBeforeCollectCallback
- [ ] JsContextRef
- [ ] JsFinalizeCallback
- [ ] JsHostPromiseRejectionTrackerCallback
- [ ] JsMemoryAllocationCallback
- [ ] JsModuleRecord
- [ ] JsNativeFunction
- [ ] JsObjectBeforeCollectCallback
- [ ] JsPromiseContinuationCallback
- [ ] JsPropertyIdRef
- [ ] JsRef
- [ ] JsRuntimeHandle
- [ ] JsSerializedLoadScriptCallBack
- [ ] JsSerializedScriptLoadSourceCallback
- [ ] JsSerializedScriptUnloadCallback
- [ ] JsSourceContext
- [ ] JsThreadServiceCallback
- [ ] JsValueRef
- [ ] JsWeakRef

## JSRT Const References:

- [ ] JS_INVALID_REFERENCE
- [ ] JS_INVALID_RUNTIME_HANDLE
- [ ] JS_SOURCE_CONTEXT_NONE


## JSRT Enum References:

- [x] JsErrorCode
- [ ] JsMemoryEventType
- [ ] JsModuleHostInfoKind
- [ ] JsParseModuleSourceFlags
- [ ] JsParseScriptAttributes
- [ ] JsPromiseState
- [ ] JsPropertyIdType
- [x] JsRuntimeAttributes
- [ ] JsTypedArrayType
- [ ] JsValueType


## JSRT API References:

- [ ] JsAddRef
- [x] JsBoolToBoolean
- [x] JsBooleanToBool
- [ ] JsCallFunction
- [ ] JsCollectGarbage
- [ ] JsConstructObject
- [x] JsConvertValueToBoolean
- [x] JsConvertValueToNumber
- [ ] JsConvertValueToObject
- [x] JsConvertValueToString
- [x] JsCopyString
- [ ] JsCopyStringOneByte
- [ ] JsCopyStringUtf16
- [ ] JsCopyPropertyId
- [ ] JsCreateArray
- [ ] JsCreateArrayBuffer
- [x] JsCreateContext
- [ ] JsCreateDataView
- [ ] JsCreateEnhancedFunction
- [ ] JsCreateError
- [x] JsCreateExternalArrayBuffer
- [ ] JsCreateExternalObject
- [ ] JsCreateExternalObjectWithPrototype
- [ ] JsCreateFunction
- [ ] JsCreateNamedFunction
- [x] JsCreateObject
- [ ] JsCreatePromise
- [ ] JsCreatePropertyId
- [ ] JsCreateRangeError
- [ ] JsCreateReferenceError
- [x] JsCreateRuntime
- [ ] JsCreateSharedArrayBufferWithSharedContent
- [x] JsCreateString
- [ ] JsCreateStringUtf16
- [ ] JsCreateSymbol
- [ ] JsCreateSyntaxError
- [ ] JsCreateTypeError
- [ ] JsCreateTypedArray
- [ ] JsCreateURIError
- [ ] JsCreateWeakReference
- [ ] JsDefineProperty
- [ ] JsDeleteIndexedProperty
- [ ] JsDeleteProperty
- [ ] JsDisableRuntimeExecution
- [x] JsDisposeRuntime
- [x] JsDoubleToNumber
- [ ] JsEnableRuntimeExecution
- [ ] JsEquals
- [ ] JsGetAndClearException
- [ ] JsGetAndClearExceptionWithMetadata
- [ ] JsGetArrayBufferStorage
- [ ] JsGetContextData
- [ ] JsGetContextOfObject
- [ ] JsGetCurrentContext
- [ ] JsGetDataViewInfo
- [ ] JsGetDataViewStorage
- [ ] JsGetExtensionAllowed
- [ ] JsGetExternalData
- [ ] JsGetFalseValue
- [x] JsGetGlobalObject
- [ ] JsGetIndexedPropertiesExternalData
- [ ] JsGetIndexedProperty
- [ ] JsGetModuleHostInfo
- [ ] JsGetModuleNamespace
- [ ] JsGetNullValue
- [ ] JsGetOwnPropertyDescriptor
- [ ] JsGetOwnPropertyNames
- [ ] JsGetOwnPropertySymbols
- [ ] JsGetPromiseResult
- [ ] JsGetPromiseState
- [ ] JsGetProperty
- [ ] JsGetPropertyIdFromName
- [ ] JsGetPropertyIdFromSymbol
- [ ] JsGetPropertyIdType
- [ ] JsGetPropertyNameFromId
- [ ] JsGetPrototype
- [ ] JsGetProxyProperties
- [ ] JsGetRuntime
- [ ] JsGetRuntimeMemoryLimit
- [ ] JsGetRuntimeMemoryUsage
- [ ] JsGetSharedArrayBufferContent
- [ ] JsGetStringLength
- [ ] JsGetSymbolFromPropertyId
- [ ] JsGetTrueValue
- [ ] JsGetTypedArrayInfo
- [ ] JsGetTypedArrayStorage
- [ ] JsGetUndefinedValue
- [x] JsGetValueType
- [ ] JsGetWeakReferenceValue
- [ ] JsHasException
- [ ] JsHasExternalData
- [ ] JsHasIndexedPropertiesExternalData
- [ ] JsHasIndexedProperty
- [ ] JsHasOwnProperty
- [ ] JsHasProperty
- [ ] JsIdle
- [ ] JsInitializeModuleRecord
- [ ] JsInstanceOf
- [x] JsIntToNumber
- [ ] JsIsRuntimeExecutionDisabled
- [ ] JsLessThan
- [ ] JsLessThanOrEqual
- [ ] JsModuleEvaluation
- [ ] JsObjectDefineProperty
- [x] JsObjectDeleteProperty
- [ ] JsObjectGetOwnPropertyDescriptor
- [x] JsObjectGetProperty
- [ ] JsObjectHasOwnProperty
- [x] JsObjectHasProperty
- [x] JsObjectSetProperty
- [x] JsNumberToDouble
- [x] JsNumberToInt
- [ ] JsParse
- [ ] JsParseModuleSource
- [ ] JsParseSerialized
- [ ] JsParseScript
- [ ] JsParseScriptWithAttributes
- [ ] JsParseSerializedScript
- [ ] JsParseSerializedScriptWithCallback
- [ ] JsPointerToString
- [ ] JsPreventExtension
- [ ] JsRelease
- [ ] JsReleaseSharedArrayBufferContentHandle
- [x] JsRun
- [ ] JsRunScript
- [ ] JsRunScriptWithParserState
- [ ] JsRunSerialized
- [ ] JsRunSerializedScript
- [ ] JsRunSerializedScriptWithCallback
- [ ] JsSerialize
- [ ] JsSerializeParserState
- [ ] JsSerializeScript
- [ ] JsSetContextData
- [x] JsSetCurrentContext
- [ ] JsSetException
- [ ] JsSetExternalData
- [ ] JsSetHostPromiseRejectionTracker
- [ ] JsSetIndexedPropertiesToExternalData
- [ ] JsSetIndexedProperty
- [ ] JsSetModuleHostInfo
- [ ] JsSetObjectBeforeCollectCallback
- [ ] JsSetPromiseContinuationCallback
- [ ] JsSetProperty
- [ ] JsSetPrototype
- [ ] JsSetRuntimeBeforeCollectCallback
- [ ] JsSetRuntimeMemoryAllocationCallback
- [ ] JsSetRuntimeMemoryLimit
- [ ] JsStrictEquals
- [ ] JsStringToPointer
