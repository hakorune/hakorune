#![cfg(feature = "wasm-backend")]

#[path = "wasm_demo_min_fixture/common.rs"]
mod common;
#[path = "wasm_demo_min_fixture/compile.rs"]
mod compile;
#[path = "wasm_demo_min_fixture/default_lane.rs"]
mod default_lane;
#[path = "wasm_demo_min_fixture/parity.rs"]
mod parity;
#[path = "wasm_demo_min_fixture/route_trace.rs"]
mod route_trace;
#[path = "common/wasm.rs"]
mod wasm_common;
