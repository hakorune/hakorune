use super::*;
use crate::mir::MirInstruction;
use crate::parser::NyashParser;
use std::path::Path;
use std::process::Command;

#[test]
fn normal_package_source_refuses_top_level_generic_g0() {
    let error = NyashParser::parse_from_string(
        "static function generic_g0(i: i64, j: i64): i64 {
            loop(i < 3) { loop(j < 3) { j = j + 1 } i = i + 1 }
            return j
        }
        static box Main { main() { return generic_g0(0, 0) } }",
    )
    .expect_err("file-scope G0 function must be rejected");
    assert!(
        error.to_string().contains("decl/function-outside-box"),
        "{error}"
    );
}

#[test]
fn qualified_static_helper_rejects_declared_contract_drift() {
    let source = "static box Helpers { checked_pair(i: i64, j: i64): i64 { return i + j } }
                  static box Main { main() { return Helpers.checked_pair(0, 0) } }";
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        MirCompiler::with_options(false)
            .compile_normal_with_published(published_request(source), |view, _| {
                let input = view.issue_lifecycle_physical_abi_input()?;
                for (mutation, expected) in [
                    "ordinary-parameter-count",
                    "parameter_contract_carrier_missing",
                    "parameter_contract_row_drift",
                    "return_contract_carrier_missing",
                ]
                .into_iter()
                .enumerate()
                {
                    let mut module = view.module().clone();
                    let function = module.functions.get_mut("Helpers.checked_pair/2").unwrap();
                    match mutation {
                        0 => {
                            function.metadata.declared_param_decls.clear();
                            function.metadata.parameter_entry_contracts.clear();
                        }
                        1 => function.metadata.parameter_entry_contracts.clear(),
                        2 => function.metadata.parameter_entry_contracts[0].source_parameter_name
                            = "foreign".to_owned(),
                        3 => function.metadata.return_exit_contract = None,
                        _ => unreachable!(),
                    }
                    let result = if mutation == 3 {
                        crate::mir::return_exit_backend_capability::enforce_lifecycle_return_exit_backend_supported(&module, &input)
                    } else {
                        crate::mir::parameter_entry_backend_capability::enforce_lifecycle_parameter_entry_backend_supported(&module, &input)
                    };
                    let error = result.expect_err("contract mutation must reject before encoding");
                    assert!(error.contains(expected), "mutation={mutation}: {error}");
                }
                Ok::<(), String>(())
            })
            .expect("source-backed static method metadata negatives");
    });
}

#[test]
#[ignore = "requires LLVM18, target/release/ny-llvmc, and lifecycle-kernel archive"]
fn accepted_variable_recurrence_fixtures_reach_exe_with_final_values() {
    std::thread::Builder::new()
        .name("accepted-var-exe-acceptance".to_owned())
        .stack_size(32 * 1024 * 1024)
        .spawn(|| {
            crate::test_support::with_env_vars(
                &[
                    ("NYASH_MACRO_DISABLE", Some("1")),
                    ("NYASH_NY_LLVM_COMPILER", Some("target/release/ny-llvmc")),
                    ("NYASH_DISABLE_PLUGINS", Some("1")),
                ],
                || {
                    let runtime = Path::new("target/release");
                    let required_files = [
                        runtime.join("libnyash_lifecycle_kernel.a"),
                        Path::new("target/release/ny-llvmc").to_path_buf(),
                        Path::new("target/release/libhako_llvmc_ffi.so").to_path_buf(),
                    ];
                    let llvm18_available =
                        ["llvm-config-18", "llc-18", "opt-18"]
                            .into_iter()
                            .all(|tool| {
                                Command::new(tool)
                                    .arg("--version")
                                    .output()
                                    .is_ok_and(|output| output.status.success())
                            });
                    assert!(
                        required_files.iter().all(|path| path.is_file()) && llvm18_available,
                        "explicit VAR EXE acceptance requires LLVM18, target/release/ny-llvmc, its FFI library, and the lifecycle-kernel archive"
                    );
                    crate::runtime::ring0::ensure_global_ring0_initialized();
                    let directory = tempfile::tempdir().expect("EXE acceptance temp directory");
                    let executable = directory.path().join("variable-recurrence");
                    let cases = [
                        (
                            "accepted fixture",
                            include_str!("../../../apps/tests/loop_simple_while_inline_explicit_step_min.hako"),
                            "6",
                        ),
                        (
                            "renamed bindings with bound 3 and step 2",
                            r#"static box Main {
  main() {
    local cursor = 0
    local total = 0
    loop(cursor < 3) {
      total = total + cursor
      cursor = cursor + 2
    }
    print(total * 10 + cursor)
    return 0
  }
}"#,
                            "24",
                        ),
                        (
                            "zero iterations retain both initial values",
                            r#"static box Main {
  main() {
    local cursor = 0
    local total = 7
    loop(cursor < 0) {
      total = total + cursor
      cursor = cursor + 2
    }
    print(total * 10 + cursor)
    return 0
  }
}"#,
                            "70",
                        ),
                    ];
                    for (label, source, expected_stdout) in cases {
                        let mut compiler = MirCompiler::with_options(true);
                        let mut callbacks = 0;
                        compiler
                            .compile_normal_with_published(
                                published_request(source),
                                |view, verification| {
                                    callbacks += 1;
                                    assert!(verification.is_ok(), "{label}: {verification:?}");
                                    assert!(view.module().functions.contains_key("main"));
                                    let emitted = crate::host_providers::llvm_codegen::emit_published_view_exe(
                                        view,
                                        executable.to_str().expect("UTF-8 executable path"),
                                        runtime.to_str(),
                                        None,
                                    )
                                    .map_err(|error| format!("exe={error}"))?;
                                    assert!(
                                        emitted,
                                        "{label}: the existing typed EXE route must own publication"
                                    );
                                    let output = Command::new(&executable)
                                        .env("NYASH_NYRT_SILENT_RESULT", "1")
                                        .env("HAKO_NYRT_PLUGIN_HOST", "off")
                                        .output()
                                        .map_err(|error| error.to_string())?;
                                    assert_eq!(output.status.code(), Some(0), "{label}: {output:?}");
                                    assert_eq!(
                                        String::from_utf8_lossy(&output.stdout).trim(),
                                        expected_stdout,
                                        "{label}"
                                    );
                                    Ok::<(), String>(())
                                },
                            )
                            .unwrap_or_else(|error| panic!("{label}: {error}"));
                        assert_eq!(callbacks, 1, "{label}: one publication callback");
                    }
                },
            )
        })
        .expect("spawn VAR EXE acceptance with the compiler test stack")
        .join()
        .expect("VAR EXE acceptance thread");
}

#[test]
fn file_ingress_rejects_bare_static_loop_prefix() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for bound in [0, 1, 3] {
            let source = format!("static box Main {{ main() {{ return 0 }} int_to_str(n: i64): i64 {{ local value = to_i64(n) local i = 0 loop(i < {bound}) {{ i = i + 1 }} return value }} to_i64(value: i64): i64 {{ return value }} }}");
            let mut compiler = MirCompiler::with_options(false);
            let missing = compiler
                .compile_normal(published_request(&source))
                .expect_err("file source must reject a bare static prefix");
            assert!(missing.contains("[call/target-missing]"), "{missing}");
        }
    });
}

#[test]
fn optimized_pair_root_with_callable_loop_child_reaches_physical_abi() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let source = "box Pair { left: i64 right: i64 birth(left, right) { me.left = left me.right = right } } static box Main { main() { local pair = new Pair(10, 20) return pair.left + pair.right } int_to_str(n: i64): i64 { local value = n local i = 0 loop(i < 3) { i = i + 1 } return value } }";
        let mut compiler = MirCompiler::new();
        compiler
            .compile_normal_with_published(published_request(source), |view, verification| {
                assert!(verification.is_ok(), "{verification:?}");
                let helper = view
                    .module()
                    .functions
                    .get("Main.int_to_str/1")
                    .expect("selected loop child");
                assert!(helper
                    .blocks
                    .values()
                    .flat_map(|block| block.all_instructions())
                    .any(|instruction| matches!(
                        instruction,
                        crate::mir::MirInstruction::Phi { .. }
                    )));
                let input = view.issue_lifecycle_physical_abi_input()?;
                assert_eq!(
                    input.entry().root_result(),
                    published_backend_view::CompiledEntryRootResultV1::I64
                );
                Ok::<(), String>(())
            })
            .expect("optimized Pair-plus-Loop module must reach physical ABI");
    });
}

/// Resolves `value` through `Copy` chains and reports whether it reaches the
/// destination of any `Phi` instruction in `function`.  The callable-source
/// loop lanes publish carrier phis into the ledger the source reads resolve,
/// so carrier reads must reach a phi dst — never replay the pre-loop value.
fn resolves_to_phi_dst(
    function: &crate::mir::MirFunction,
    value: crate::mir::ValueId,
) -> bool {
    let phi_dsts: std::collections::BTreeSet<crate::mir::ValueId> = function
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .filter_map(|instruction| match instruction {
            crate::mir::MirInstruction::Phi { dst, .. } => Some(*dst),
            _ => None,
        })
        .collect();
    let copy_sources: std::collections::BTreeMap<crate::mir::ValueId, crate::mir::ValueId> = function
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .filter_map(|instruction| match instruction {
            crate::mir::MirInstruction::Copy { dst, src } => Some((*dst, *src)),
            _ => None,
        })
        .collect();
    let mut current = value;
    for _ in 0..8 {
        if phi_dsts.contains(&current) {
            return true;
        }
        let Some(src) = copy_sources.get(&current) else {
            return false;
        };
        current = *src;
    }
    false
}

/// Source-bound lowering recurses deeply enough in debug builds to exceed
/// the default 8 MiB test-thread stack; pins that drive it run on a widened
/// thread the same way the variable-accum family does.
fn run_on_test_thread(name: &str, body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name(name.to_owned())
        .stack_size(32 * 1024 * 1024)
        .spawn(body)
        .expect("spawn test thread")
        .join()
        .expect("test thread panicked");
}

#[test]
fn callable_source_return_in_body_loop_reads_bind_header_phi_dst() {
    run_on_test_thread(
        "callable-source-return-in-body-loop",
        callable_source_return_in_body_loop_reads_bind_header_phi_dst_inner,
    );
}

fn callable_source_return_in_body_loop_reads_bind_header_phi_dst_inner() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        // find/3 shape: a carrier read inside the loop body must resolve the
        // header phi dst — the baseline replayed the pre-loop entry value.
        // The checked source loop keeps the body call and its early return.
        let source = "static box Main { main() { return 0 } find(text, n: i64): i64 { local pos = 0 loop(pos < n) { local sub = text.substring(pos, pos + 1) if sub == \"x\" { return pos } pos = pos + 1 } return -1 } }";
        let mut compiler = MirCompiler::with_options(false);
        let result = compiler
            .compile_normal(published_request(source))
            .expect("callable source find-shape compile");
        assert!(
            result.verification_result.is_ok(),
            "{:?}",
            result.verification_result
        );
        let helper = result
            .module
            .functions
            .get("Main.find/2")
            .expect("Main.find definition");
        let header_phi_dsts: Vec<crate::mir::ValueId> = helper
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .filter_map(|instruction| match instruction {
                crate::mir::MirInstruction::Phi { dst, .. } => Some(*dst),
                _ => None,
            })
            .collect();
        assert!(
            !header_phi_dsts.is_empty(),
            "carrier publication must keep the header phi spine"
        );
        // The loop comparison and early return must reach a phi dst.
        let const_dsts: std::collections::BTreeSet<crate::mir::ValueId> = helper
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .filter_map(|instruction| match instruction {
                crate::mir::MirInstruction::Const { dst, .. } => Some(*dst),
                _ => None,
            })
            .collect();
        let mut compare_operands = Vec::new();
        let mut non_const_returns = Vec::new();
        for block in helper.blocks.values() {
            for instruction in block.all_instructions() {
                match instruction {
                    crate::mir::MirInstruction::Compare { lhs, rhs, .. } => {
                        compare_operands.push(*lhs);
                        compare_operands.push(*rhs);
                    }
                    crate::mir::MirInstruction::Return { value: Some(value) }
                        if !const_dsts.contains(value) =>
                    {
                        non_const_returns.push(*value);
                    }
                    _ => {}
                }
            }
        }
        assert!(
            compare_operands
                .iter()
                .any(|value| resolves_to_phi_dst(helper, *value)),
            "the in-body carrier compare must resolve a phi dst: {compare_operands:?}"
        );
        assert!(
            non_const_returns
                .iter()
                .any(|value| resolves_to_phi_dst(helper, *value)),
            "the in-body `return i` must resolve a phi dst, not replay the \
             entry value: {non_const_returns:?}"
        );
    });
}

#[test]
fn callable_source_loop_exit_read_binds_after_phi_dst() {
    run_on_test_thread(
        "callable-source-loop-exit-read",
        callable_source_loop_exit_read_binds_after_phi_dst_inner,
    );
}

fn callable_source_loop_exit_read_binds_after_phi_dst_inner() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        // Post-loop carrier read: `return matched` after the loop must bind
        // the after-merge phi dst — not the body's last writer, not the
        // init.  index_of shape: an if/else inside the body rebinds the
        // carrier, so the join publication must reach the ledger too.
        let source = "static box Main { main() { return 0 } count_at(text, n: i64): i64 { local i = 0 local matched = 0 loop(i < n) { local c = text.substring(i, i + 1) if c == \"x\" { matched = matched + 1 } else { matched = 0 } i = i + 1 } return matched } }";
        let mut compiler = MirCompiler::with_options(false);
        let result = compiler
            .compile_normal(published_request(source))
            .expect("callable source count-shape compile");
        assert!(
            result.verification_result.is_ok(),
            "{:?}",
            result.verification_result
        );
        let helper = result
            .module
            .functions
            .get("Main.count_at/2")
            .expect("Main.count_at definition");
        let mut returned = None;
        for block in helper.blocks.values() {
            for instruction in block.all_instructions() {
                if let crate::mir::MirInstruction::Return { value: Some(value) } = instruction {
                    returned = Some(*value);
                }
            }
        }
        let returned = returned.expect("count_at must return");
        assert!(
            resolves_to_phi_dst(helper, returned),
            "post-loop carrier read must bind the after-phi dst, got %{:?}",
            returned
        );
    });
}
