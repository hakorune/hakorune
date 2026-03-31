use super::super::super::*;
use super::with_joinir_strict_without_planner_required;

#[test]
fn compile_v0_emits_mir_call_open_with_two_args() {
    let runner = NyashRunner::new(crate::cli::CliConfig::default());
    let source = r#"
static box Main {
  main() {
    local f = new FileBox()
    return f.open("/tmp/phase29y_missing_input.txt", "r")
  }
}
"#;
    let mir_json = compile_source_to_mir_json_v0(&runner, "<inline>", source)
        .expect("compile_source_to_mir_json_v0 should succeed");
    let root: serde_json::Value = serde_json::from_str(&mir_json).expect("valid mir json");
    let inst = root["functions"]
        .as_array()
        .and_then(|funcs| funcs.iter().find(|f| f["name"].as_str() == Some("main")))
        .and_then(|main| main["blocks"].as_array())
        .and_then(|blocks| {
            blocks.iter().find_map(|b| {
                b["instructions"].as_array().and_then(|insts| {
                    insts.iter().find(|inst| {
                        inst["op"].as_str() == Some("mir_call")
                            && inst["mir_call"]["callee"]["type"].as_str() == Some("Method")
                            && inst["mir_call"]["callee"]["name"].as_str() == Some("open")
                    })
                })
            })
        })
        .cloned()
        .expect("main mir_call(open) must exist");
    let args_len = inst["mir_call"]["args"]
        .as_array()
        .map(|a| a.len())
        .unwrap_or(0);
    assert!(args_len >= 2, "unexpected open args shape: {}", inst);
    assert!(
        inst["mir_call"]["callee"]["receiver"].is_number(),
        "open mir_call must carry receiver: {}",
        inst
    );
}

#[test]
fn compile_v0_emits_mir_call_compile_obj_with_one_arg() {
    let runner = NyashRunner::new(crate::cli::CliConfig::default());
    let source = r#"
static box Main {
  main() {
    return LlvmBackendBox.compile_obj("/tmp/phase29y_probe_input.mir.json")
  }
}
"#;
    let mir_json = compile_source_to_mir_json_v0(&runner, "<inline>", source)
        .expect("compile_source_to_mir_json_v0 should succeed");
    let root: serde_json::Value = serde_json::from_str(&mir_json).expect("valid mir json");
    let inst = root["functions"]
        .as_array()
        .and_then(|funcs| funcs.iter().find(|f| f["name"].as_str() == Some("main")))
        .and_then(|main| main["blocks"].as_array())
        .and_then(|blocks| {
            blocks.iter().find_map(|b| {
                b["instructions"].as_array().and_then(|insts| {
                    insts.iter().find(|inst| {
                        inst["op"].as_str() == Some("mir_call")
                            && inst["mir_call"]["callee"]["type"].as_str() == Some("Method")
                            && inst["mir_call"]["callee"]["name"].as_str() == Some("compile_obj")
                    })
                })
            })
        })
        .cloned()
        .expect("main mir_call(compile_obj) must exist");
    let args_len = inst["mir_call"]["args"]
        .as_array()
        .map(|a| a.len())
        .unwrap_or(0);
    assert!(args_len >= 1, "unexpected compile_obj args shape: {}", inst);
    assert!(
        inst["mir_call"]["callee"]["box_name"].as_str() == Some("LlvmBackendBox"),
        "compile_obj mir_call must target LlvmBackendBox: {}",
        inst
    );
}

#[test]
fn compile_v0_emits_mir_call_extern_hako_mem_alloc() {
    let runner = NyashRunner::new(crate::cli::CliConfig::default());
    let source = r#"
static box Main {
  main() {
    local p = externcall "hako_mem_alloc"(8)
    return p
  }
}
"#;
    let mir_json = compile_source_to_mir_json_v0(&runner, "<inline>", source)
        .expect("compile_source_to_mir_json_v0 should succeed");
    let root: serde_json::Value = serde_json::from_str(&mir_json).expect("valid mir json");
    let inst = root["functions"]
        .as_array()
        .and_then(|funcs| funcs.iter().find(|f| f["name"].as_str() == Some("main")))
        .and_then(|main| main["blocks"].as_array())
        .and_then(|blocks| {
            blocks.iter().find_map(|b| {
                b["instructions"].as_array().and_then(|insts| {
                    insts.iter().find(|inst| {
                        inst["op"].as_str() == Some("mir_call")
                            && inst["mir_call"]["callee"]["type"].as_str() == Some("Extern")
                            && inst["mir_call"]["callee"]["name"].as_str() == Some("hako_mem_alloc")
                    })
                })
            })
        })
        .cloned()
        .expect("main mir_call(Extern:hako_mem_alloc) must exist");
    assert_eq!(
        inst["mir_call"]["args"].as_array().map(|a| a.len()),
        Some(1),
        "extern hako_mem_alloc must receive one runtime arg: {}",
        inst
    );
    assert!(
        inst["dst"].is_number(),
        "extern hako_mem_alloc mir_call must carry dst: {}",
        inst
    );
}

#[test]
fn merge_prelude_text_with_imports_resolves_nested_static_box_aliases() {
    let runner = NyashRunner::new(crate::cli::CliConfig::default());
    let source = r#"
using "hako.mir.builder.internal.lower_return_method_array_map" as LowerBox
static box Main {
  main(args) {
    return 0
  }
}
"#;

    let (_merged, imports) =
        crate::runner::modes::common_util::resolve::merge_prelude_text_with_imports(
            &runner, source, "<inline>",
        )
        .expect("merge with imports");

    assert_eq!(
        imports.get("LowerBox").map(String::as_str),
        Some("LowerReturnMethodArrayMapBox")
    );
    assert_eq!(
        imports.get("JsonFragBox").map(String::as_str),
        Some("JsonFragBox")
    );
    assert_eq!(
        imports.get("PatternUtilBox").map(String::as_str),
        Some("PatternUtilBox")
    );
    assert_eq!(
        imports.get("MethodAliasPolicy").map(String::as_str),
        Some("MethodAliasPolicy")
    );
}

#[test]
fn compile_v0_uses_imported_static_box_alias_without_newbox_aliases() {
    with_joinir_strict_without_planner_required(|| {
        let runner = NyashRunner::new(crate::cli::CliConfig::default());
        let source = r#"
using "hako.mir.builder.internal.lower_return_method_array_map" as LowerBox
static box Main { method main(args){
  local j = env.get("PROG_JSON")
  if j == null { return 1 }
  local out = LowerBox.try_lower(j)
  if out == null { return 2 }
  return 0
}}
"#;

        let mir_json = compile_source_to_mir_json_v0(&runner, "<inline>", source)
            .expect("compile should work in strict without planner_required");
        let root: serde_json::Value = serde_json::from_str(&mir_json).expect("valid mir json");

        fn has_newbox_alias(root: &serde_json::Value, alias: &str) -> bool {
            root["functions"]
                .as_array()
                .into_iter()
                .flatten()
                .flat_map(|func| func["blocks"].as_array().into_iter().flatten())
                .flat_map(|block| block["instructions"].as_array().into_iter().flatten())
                .any(|inst| {
                    inst["op"].as_str() == Some("newbox") && inst["type"].as_str() == Some(alias)
                })
        }

        fn find_direct_try_lower_call<'a>(
            root: &'a serde_json::Value,
        ) -> Option<&'a serde_json::Value> {
            root["functions"].as_array().and_then(|funcs| {
                funcs.iter().find_map(|func| {
                    func["blocks"].as_array().and_then(|blocks| {
                        blocks.iter().find_map(|block| {
                            block["instructions"].as_array().and_then(|insts| {
                                insts.iter().find(|inst| {
                                    inst["op"].as_str() == Some("mir_call")
                                        && inst["mir_call"]["callee"]["name"].as_str()
                                            == Some("LowerReturnMethodArrayMapBox.try_lower/1")
                                })
                            })
                        })
                    })
                })
            })
        }

        for alias in [
            "LowerBox",
            "JsonFragBox",
            "PatternUtilBox",
            "MethodAliasPolicy",
        ] {
            assert!(
                !has_newbox_alias(&root, alias),
                "import alias {} should not be materialized as newbox: {}",
                alias,
                mir_json
            );
        }

        let try_lower_call = find_direct_try_lower_call(&root).unwrap_or_else(|| {
            panic!(
                "direct-lower concrete global call must exist in emitted MIR JSON: {}",
                mir_json
            )
        });
        assert!(
            try_lower_call["mir_call"]["callee"]["type"].as_str() == Some("Global"),
            "selected static call must stay Global, not receiver-less Method: {}",
            try_lower_call
        );
        assert!(
            try_lower_call["mir_call"].get("func").is_none(),
            "direct static call should carry concrete callee and no legacy func indirection: {}",
            try_lower_call
        );
        assert!(
            !mir_json.contains("\"receiver\":null"),
            "direct-lower static alias must not regress to Method{{receiver:null}}: {}",
            mir_json
        );
    });
}

#[test]
fn direct_source_route_keeps_imported_static_box_alias_as_concrete_global_call() {
    with_joinir_strict_without_planner_required(|| {
        let runner = NyashRunner::new(crate::cli::CliConfig::default());
        let source = r#"
using "hako.mir.builder.internal.lower_return_method_array_map" as LowerBox
static box Main { method main(args){
  local j = env.get("PROG_JSON")
  if j == null { return 1 }
  local out = LowerBox.try_lower(j)
  if out == null { return 2 }
  return 0
}}
"#;

        let prepared = crate::runner::modes::common_util::source_hint::prepare_source_with_imports(
            &runner,
            "<inline>.hako",
            source,
        )
        .expect("direct source prep should resolve using imports");
        let ast = crate::parser::NyashParser::parse_from_string(&prepared.code)
            .expect("prepared direct source should parse");
        let ast = crate::r#macro::maybe_expand_and_dump(&ast, false);
        let mut compiler = crate::mir::MirCompiler::with_options(true);
        let compile =
            crate::runner::modes::common_util::source_hint::compile_with_source_hint_and_imports(
                &mut compiler,
                ast,
                Some("<inline>.hako"),
                prepared.imports,
            )
            .expect("direct source compile should succeed");
        let mir_json =
            crate::runner::mir_json_emit::emit_mir_json_string_for_harness_bin(&compile.module)
                .expect("emit mir json");
        let root: serde_json::Value = serde_json::from_str(&mir_json).expect("valid mir json");

        let direct_try_lower_call = root["functions"]
            .as_array()
            .and_then(|funcs| {
                funcs
                    .iter()
                    .find(|func| func["name"].as_str() == Some("main"))
            })
            .and_then(|func| func["blocks"].as_array())
            .and_then(|blocks| {
                blocks.iter().find_map(|block| {
                    block["instructions"].as_array().and_then(|insts| {
                        insts.iter().find(|inst| {
                            inst["op"].as_str() == Some("mir_call")
                                && inst["mir_call"]["callee"]["name"].as_str()
                                    == Some("LowerReturnMethodArrayMapBox.try_lower/1")
                        })
                    })
                })
            })
            .unwrap_or_else(|| {
                panic!(
                    "direct source route must emit concrete Global try_lower call in main: {}",
                    mir_json
                )
            });

        assert_eq!(
            direct_try_lower_call["mir_call"]["callee"]["type"].as_str(),
            Some("Global"),
            "direct source route must keep imported static box alias as Global: {}",
            direct_try_lower_call
        );
        assert!(
            !mir_json.contains("\"box_name\":\"LowerBox\""),
            "direct source route must not leave alias box_name in emitted MIR: {}",
            mir_json
        );
        assert!(
            !mir_json.contains("\"receiver\":null"),
            "direct source route must not regress to Method{{receiver:null}}: {}",
            mir_json
        );
    });
}
