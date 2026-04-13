use super::super::super::*;

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
fn compile_v0_emits_mir_call_extern_hako_osvm_reserve_bytes_i64() {
    let runner = NyashRunner::new(crate::cli::CliConfig::default());
    let source = r#"
static box Main {
  main() {
    local p = externcall "hako_osvm_reserve_bytes_i64"(4096)
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
                            && inst["mir_call"]["callee"]["name"].as_str()
                                == Some("hako_osvm_reserve_bytes_i64")
                    })
                })
            })
        })
        .cloned()
        .expect("main mir_call(Extern:hako_osvm_reserve_bytes_i64) must exist");
    assert_eq!(
        inst["mir_call"]["args"].as_array().map(|a| a.len()),
        Some(1),
        "extern hako_osvm_reserve_bytes_i64 must receive one runtime arg: {}",
        inst
    );
    assert!(
        inst["dst"].is_number(),
        "extern hako_osvm_reserve_bytes_i64 mir_call must carry dst: {}",
        inst
    );
}

#[test]
fn compile_v0_emits_mir_call_extern_hako_osvm_commit_bytes_i64() {
    let runner = NyashRunner::new(crate::cli::CliConfig::default());
    let source = r#"
static box Main {
  main() {
    local base = 1024
    local rc = externcall "hako_osvm_commit_bytes_i64"(base, 4096)
    return rc
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
                            && inst["mir_call"]["callee"]["name"].as_str()
                                == Some("hako_osvm_commit_bytes_i64")
                    })
                })
            })
        })
        .cloned()
        .expect("main mir_call(Extern:hako_osvm_commit_bytes_i64) must exist");
    assert_eq!(
        inst["mir_call"]["args"].as_array().map(|a| a.len()),
        Some(2),
        "extern hako_osvm_commit_bytes_i64 must receive two runtime args: {}",
        inst
    );
    assert!(
        inst["dst"].is_number(),
        "extern hako_osvm_commit_bytes_i64 mir_call must carry dst: {}",
        inst
    );
}

#[test]
fn compile_v0_emits_mir_call_extern_hako_osvm_decommit_bytes_i64() {
    let runner = NyashRunner::new(crate::cli::CliConfig::default());
    let source = r#"
static box Main {
  main() {
    local base = 1024
    local rc = externcall "hako_osvm_decommit_bytes_i64"(base, 4096)
    return rc
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
                            && inst["mir_call"]["callee"]["name"].as_str()
                                == Some("hako_osvm_decommit_bytes_i64")
                    })
                })
            })
        })
        .cloned()
        .expect("main mir_call(Extern:hako_osvm_decommit_bytes_i64) must exist");
    assert_eq!(
        inst["mir_call"]["args"].as_array().map(|a| a.len()),
        Some(2),
        "extern hako_osvm_decommit_bytes_i64 must receive two runtime args: {}",
        inst
    );
    assert!(
        inst["dst"].is_number(),
        "extern hako_osvm_decommit_bytes_i64 mir_call must carry dst: {}",
        inst
    );
}
