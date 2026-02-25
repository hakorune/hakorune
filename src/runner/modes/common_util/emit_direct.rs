use std::{fmt::Display, path::Path, process};

use nyash_rust::mir::VerificationError;

use super::verifier_gate;

pub(crate) fn maybe_emit_mir_json_and_exit<E, F>(
    emit_path: Option<&str>,
    verification_result: &Result<(), Vec<VerificationError>>,
    route: &str,
    quiet: bool,
    emit_fn: F,
) where
    E: Display,
    F: FnOnce(&Path) -> Result<(), E>,
{
    let Some(path) = emit_path else {
        return;
    };
    verifier_gate::enforce_direct_emit_verify_or_exit(
        verification_result,
        route,
        verifier_gate::VM_DIRECT_EMIT_MIR_VERIFY_TAG,
    );
    let out_path = Path::new(path);
    if let Err(err) = emit_fn(out_path) {
        eprintln!("❌ MIR JSON emit error: {}", err);
        process::exit(1);
    }
    if !quiet {
        println!("MIR JSON written: {}", out_path.display());
    }
    process::exit(0);
}

pub(crate) fn maybe_emit_exe_and_exit<E, F>(
    emit_path: Option<&str>,
    verification_result: &Result<(), Vec<VerificationError>>,
    route: &str,
    quiet: bool,
    emit_fn: F,
) where
    E: Display,
    F: FnOnce(&str) -> Result<(), E>,
{
    let Some(exe_out) = emit_path else {
        return;
    };
    verifier_gate::enforce_direct_emit_verify_or_exit(
        verification_result,
        route,
        verifier_gate::VM_DIRECT_EMIT_EXE_VERIFY_TAG,
    );
    if let Err(err) = emit_fn(exe_out) {
        eprintln!("❌ {}", err);
        process::exit(1);
    }
    if !quiet {
        println!("EXE written: {}", exe_out);
    }
    process::exit(0);
}
