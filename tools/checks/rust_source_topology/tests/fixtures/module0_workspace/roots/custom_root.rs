mod flat;
mod tree;

mod inline {
    mod nested;
}

#[path = "alternate.rs"]
mod renamed;

#[path = "redirected"]
mod redirected_inline {
    mod inside;
}

#[cfg_attr(target_arch = "wasm32", path = "platform_wasm.rs")]
#[cfg_attr(not(target_arch = "wasm32"), path = "platform_host.rs")]
mod platform;

#[cfg(debug_assertions)]
mod debug;

#[cfg(not(debug_assertions))]
mod release;

#[cfg(test)]
mod test_only;

#[cfg(feature = "vm-reference")]
mod feature_vm;

#[cfg(feature = "llvm-harness")]
mod feature_llvm_harness;

#[cfg(feature = "llvm")]
mod must_stay_excluded;

include!("must_remain_opaque.rs");
