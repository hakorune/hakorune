use super::wasm_common;
use nyash_rust::backend::wasm::WasmBackend;
use nyash_rust::mir::MirCompiler;
use nyash_rust::parser::NyashParser;
use std::fs;

pub(crate) fn compile_fixture_to_wat_direct(rel: &str) -> String {
    let _ = nyash_rust::runtime::ring0::ensure_global_ring0_initialized();
    let mir_module = compile_fixture_to_mir_module(rel);
    let mut wasm_backend = WasmBackend::new();
    wasm_backend
        .compile_to_wat(mir_module)
        .expect("fixture should compile to WAT")
}

pub(crate) fn compile_fixture_to_mir_module(rel: &str) -> nyash_rust::mir::MirModule {
    let _ = nyash_rust::runtime::ring0::ensure_global_ring0_initialized();
    let fixture = wasm_common::fixture_path(rel);
    let source = fs::read_to_string(&fixture).expect("fixture should be readable");
    let ast = NyashParser::parse_from_string(&source).expect("fixture should parse");
    let mut compiler = MirCompiler::new();
    compiler
        .compile(ast)
        .expect("fixture should lower to MIR")
        .module
}

pub(crate) fn compile_fixture_to_wasm_direct(rel: &str) -> Vec<u8> {
    let mir_module = compile_fixture_to_mir_module(rel);
    let mut wasm_backend = WasmBackend::new();
    wasm_backend
        .compile_module(mir_module)
        .expect("fixture should compile to WASM")
}
