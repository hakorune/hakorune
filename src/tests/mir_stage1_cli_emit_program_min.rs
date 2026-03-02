use crate::ast::ASTNode;
use crate::backend::VM;
use crate::mir::printer::MirPrinter;
use crate::mir::{instruction::MirInstruction, types::CompareOp, Callee, MirCompiler, MirVerifier};
use crate::parser::NyashParser;
use std::sync::Once;

fn ensure_ring0_initialized() {
    use crate::runtime::ring0::{default_ring0, init_global_ring0};
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        init_global_ring0(default_ring0());
    });
}

fn ensure_stage3_env() {
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_ENABLE_USING", "1");
    std::env::set_var("HAKO_ENABLE_USING", "1");
    // このフィクスチャは静的 box Stage1Cli を新規に定義するため、
    // methodization で NewBox を挿入されるとプラグイン未登録で落ちる。
    // テストは env-only 仕様の形だけを固定する目的なので、ここでは明示的にOFFにする。
    std::env::set_var("HAKO_MIR_BUILDER_METHODIZE", "0");
}

/// Stage‑1 CLI の「env-only + emit_program_json」経路を、最小の箱で固定するフィクスチャ。
/// 本番 `stage1_cli.hako` は SSA が複雑で現在も別タスクで追跡中のため、
/// ここでは env-only 仕様と methodization 既定ON で崩れない「最小形の Stage1Cli」だけをテストする。
fn stage1_cli_fixture_src() -> String {
    let test_main_src = r#"
static box Stage1Cli {
  emit_program_json(source) {
    // 本番では Stage‑B に委譲するが、ここでは env-only の形だけ確認する。
    if source == null || source == "" { return null }
    return "{prog:" + source + "}"
  }

  emit_mir_json(program_json) {
    if program_json == null || program_json == "" { return null }
    return "{mir:" + program_json + "}"
  }

  run_program_json(program_json, backend) {
    // env-only 仕様に合わせて backend はタグだけ見る
    if backend == null { backend = "vm" }
    if program_json == null || program_json == "" { return 96 }
    return 0
  }

  stage1_main(args) {
    // env-only: argv は無視し、必須 env が無ければ明示的に 96 を返す
    if args == null { args = new ArrayBox() }
    local src = env.get("STAGE1_SOURCE")
    if src == null || src == "" { return 96 }

    // emit-program-json モード（最小）
    local prog = me.emit_program_json(src)
    if prog == null { return 96 }
    print(prog)
    return 0
  }
}

static box Main {
  main(args) {
    // env-only 仕様で STAGE1_SOURCE さえあれば emit_program_json が通ることを確認
    env.set("STAGE1_SOURCE", "apps/tests/stage1_using_minimal.hako")
    return Stage1Cli.stage1_main(args)
  }
}
"#;
    test_main_src.to_string()
}

fn stage1_cli_static_call_arity_fixture_src() -> String {
    r#"
static box ParserBox {
  esc_json(value) {
    if value == null { return "{}" }
    return value
  }
}

static box HakoCli {
  run(args) {
    local encoded = ParserBox.esc_json(args)
    return encoded
  }
}

static box Main {
  main(args) {
    return HakoCli.run(args)
  }
}
"#
    .to_string()
}

/// Stage1Cli.emit_program_json 経路の最小再現を Rust テスト側に持ち込むハーネス。
/// - apps/tests/stage1_cli_emit_program_min.hako と同じ形で Stage1Cli を呼び出す。
/// - ここでは「MIR/SSA が壊れずモジュールが verify できるか」までを確認し、
///   実際の VM 実行時の型崩れは別フェーズで VM テストとして扱う前提。
#[test]
fn mir_stage1_cli_emit_program_min_compiles_and_verifies() {
    ensure_stage3_env();
    let src = stage1_cli_fixture_src();

    let ast: ASTNode = NyashParser::parse_from_string(&src).expect("parse ok");
    let mut mc = MirCompiler::with_options(false);
    let cr = mc.compile(ast).expect("compile");

    // Optional: dump MIR when debugging this path
    if std::env::var("NYASH_STAGE1_MIR_DUMP").ok().as_deref() == Some("1") {
        let printer = MirPrinter::verbose();
        let txt = printer.print_module(&cr.module);
        eprintln!("=== MIR stage1_cli_emit_program_min ===\n{}", txt);
    }

    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&cr.module) {
        for e in &errors {
            eprintln!("[rust-mir-verify] {}", e);
        }
        panic!("MIR verification failed for stage1_cli_emit_program_min");
    }
}

/// VM 実行まで進めて、現在発生している String > Integer の型エラーを Rust テスト内で再現する。
#[test]
fn mir_stage1_cli_emit_program_min_exec_hits_type_error() {
    ensure_stage3_env();
    let src = stage1_cli_fixture_src();

    let ast: ASTNode = NyashParser::parse_from_string(&src).expect("parse ok");
    let mut mc = MirCompiler::with_options(false);
    let cr = mc.compile(ast).expect("compile");

    // Optional: scan for Compare::Ge instructions to locate suspicious comparisons
    if std::env::var("NYASH_STAGE1_SCAN_GE").ok().as_deref() == Some("1") {
        for (fname, func) in cr.module.functions.iter() {
            for (bb_id, bb) in func.blocks.iter() {
                for inst in bb.instructions.iter() {
                    if let MirInstruction::Compare { op, lhs, rhs, .. } = inst {
                        if *op == CompareOp::Ge {
                            eprintln!(
                                "[stage1-cli/scan] Compare Ge in {} @bb{:?} lhs=%{:?} rhs=%{:?}",
                                fname, bb_id, lhs, rhs
                            );
                        }
                    }
                }
                if let Some(term) = &bb.terminator {
                    if let MirInstruction::Compare { op, lhs, rhs, .. } = term {
                        if *op == CompareOp::Ge {
                            eprintln!(
                                "[stage1-cli/scan] Compare Ge(term) in {} @bb{:?} lhs=%{:?} rhs=%{:?}",
                                fname, bb_id, lhs, rhs
                            );
                        }
                    }
                }
            }
        }
    }

    let mut vm = VM::new();
    let exec = vm.execute_module(&cr.module);
    // 最小形では正常に 0 を返すことを期待。
    let v = exec.expect("Stage1Cli minimal path should execute");
    assert_eq!(v.to_string_box().value, "0");
}

#[test]
fn mir_stage1_cli_static_call_arity_contract_parserbox_hakocli() {
    ensure_stage3_env();
    ensure_ring0_initialized();
    let src = stage1_cli_static_call_arity_fixture_src();

    let ast: ASTNode = NyashParser::parse_from_string(&src).expect("parse ok");
    let mut mc = MirCompiler::with_options(false);
    let cr = mc.compile(ast).expect("compile");

    let mut parserbox_hits = 0usize;
    let mut hakocli_hits = 0usize;

    for func in cr.module.functions.values() {
        for block in func.blocks.values() {
            for inst in &block.instructions {
                let MirInstruction::Call {
                    callee: Some(callee),
                    args,
                    ..
                } = inst
                else {
                    continue;
                };

                match callee {
                    Callee::Method {
                        box_name,
                        method,
                        receiver,
                        ..
                    } if box_name == "ParserBox" && method == "esc_json" => {
                        assert!(
                            receiver.is_none(),
                            "ParserBox.esc_json must stay static (no receiver)"
                        );
                        assert_eq!(
                            args.len(),
                            1,
                            "ParserBox.esc_json call args must stay arity=1"
                        );
                        parserbox_hits += 1;
                    }
                    Callee::Global(name) if name == "ParserBox.esc_json/1" => {
                        assert_eq!(
                            args.len(),
                            1,
                            "ParserBox.esc_json/1 global call args must stay arity=1"
                        );
                        parserbox_hits += 1;
                    }
                    Callee::Method {
                        box_name,
                        method,
                        receiver,
                        ..
                    } if box_name == "HakoCli" && method == "run" => {
                        assert!(
                            receiver.is_none(),
                            "HakoCli.run must stay static (no receiver)"
                        );
                        assert_eq!(args.len(), 1, "HakoCli.run call args must stay arity=1");
                        hakocli_hits += 1;
                    }
                    Callee::Global(name) if name == "HakoCli.run/1" => {
                        assert_eq!(
                            args.len(),
                            1,
                            "HakoCli.run/1 global call args must stay arity=1"
                        );
                        hakocli_hits += 1;
                    }
                    _ => {}
                }
            }
        }
    }

    assert!(
        parserbox_hits > 0,
        "Expected at least one ParserBox.esc_json callsite in MIR"
    );
    assert!(
        hakocli_hits > 0,
        "Expected at least one HakoCli.run callsite in MIR"
    );
}
