pub mod compilation_options;
pub mod handle;
pub mod js;
pub mod rooted;
pub mod runtime;
pub mod utf8_source;

pub use spidermonkey_wasm_sys::jsffi::{
    JSClass, JSContext, JSObject, OnNewGlobalHookOption, RealmOptions,
};

pub use spidermonkey_wasm_sys::jsrealm::JSAutoRealm;

// Re-export low-level Rooted types for macro convenience
pub use spidermonkey_wasm_sys::jsgc::Rooted as RawRooted;
