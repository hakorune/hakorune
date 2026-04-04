use super::super::NyashRunner;
#[cfg(test)]
use serde_json::Value;
#[cfg(test)]
use std::collections::HashMap;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

mod compile_bridge;
mod driver_source;
mod driver_spawn;
mod payload_normalize;
mod shape_contract;
mod subset_check;

const VM_HAKO_PHASE: &str = "s5l";
const DYNAMIC_METHOD_FUNC_ID: u64 = u32::MAX as u64;
const DYNAMIC_METHOD_BRIDGE_FUNC_ID: u64 = DYNAMIC_METHOD_FUNC_ID - 1;

type VmHakoErr = (&'static str, String);

impl NyashRunner {
    /// Phase29z-S5l:
    /// - Compile source to MIR(JSON v0)
    /// - Accept subset (S0 + S1a + S1b + S1c + S1d method-call(id 1-int) bridge + S1e direct boxcall(id 1-int) contract + S1f call-bridge optimize-on contract + S2a externcall(print 1-int) + S2b compare-origin bool(0/1) print + S2c externcall-only vm execution + S2d externcall(env.get/1) + S2e full legacy mir_call(print) withdrawal + S2f compare(!=) + S3a safepoint + S3b keepalive + S3c release_strong + S4a debug + S4b select + S4c barrier + S5a load + S5b store + S5c phi + S5d typeop + S5e weakref + S5f ref_new + S5g future_new + S5h future_set + S5i await + S5k array_get + S5l array_set)
    /// - Execute accepted MIR payload via .hako MiniVmEntry bridge
    ///
    /// This stays as the explicit reference/conformance lane. It is not a
    /// day-to-day mainline owner and should only move when the reference bucket
    /// itself changes.
    pub(crate) fn execute_vm_hako_mode(&self, filename: &str) {
        let code = match std::fs::read_to_string(filename) {
            Ok(content) => content,
            Err(e) => {
                eprintln!(
                    "[vm-hako/read-error] phase={} file={} message={}",
                    VM_HAKO_PHASE, filename, e
                );
                process::exit(1);
            }
        };

        let mir_json = match compile_source_to_mir_json_v0(self, filename, &code) {
            Ok(json) => json,
            Err((kind, message)) => {
                eprintln!(
                    "[vm-hako/{}] phase={} file={} message={}",
                    kind, VM_HAKO_PHASE, filename, message
                );
                process::exit(1);
            }
        };

        if let Err((func, bb, op)) = check_vm_hako_subset_json(&mir_json) {
            eprintln!(
                "[vm-hako/unimplemented] phase={} route=subset-check file={} func={} bb={} op={}",
                VM_HAKO_PHASE, filename, func, bb, op
            );
            process::exit(1);
        }

        let payload_json = match extract_main_payload_json(&mir_json) {
            Ok(v) => v,
            Err(e) => {
                eprintln!(
                    "[vm-hako/contract-error] phase={} file={} message={}",
                    VM_HAKO_PHASE, filename, e
                );
                process::exit(1);
            }
        };

        let exit_code = match run_vm_hako_driver(filename, &payload_json) {
            Ok(code) => code,
            Err((kind, message)) => {
                eprintln!(
                    "[vm-hako/{}] phase={} file={} message={}",
                    kind, VM_HAKO_PHASE, filename, message
                );
                process::exit(1);
            }
        };

        process::exit(exit_code);
    }
}

fn compile_source_to_mir_json_v0(
    runner: &NyashRunner,
    filename: &str,
    code: &str,
) -> Result<String, VmHakoErr> {
    compile_bridge::compile_source_to_mir_json_v0(runner, filename, code)
}

fn run_vm_hako_driver(filename: &str, payload_json: &str) -> Result<i32, VmHakoErr> {
    driver_spawn::run_vm_hako_driver(filename, payload_json)
}

fn extract_main_payload_json(json_text: &str) -> Result<String, String> {
    payload_normalize::extract_main_payload_json(json_text)
}

#[cfg(test)]
fn parse_print_arg_from_instruction(
    inst: &Value,
    handle_by_reg: &HashMap<u64, String>,
) -> Result<Option<u64>, &'static str> {
    shape_contract::parse_print_arg_from_instruction(inst, handle_by_reg)
}

fn check_vm_hako_subset_json(json_text: &str) -> Result<(), (String, u32, String)> {
    subset_check::check_vm_hako_subset_json(json_text)
}

fn temp_seed() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{}_{}", std::process::id(), now)
}

#[cfg(test)]
mod tests;
