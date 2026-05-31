use super::*;

#[test]
fn source_to_program_json_v0_minimal_main() {
    let source = r#"
static box Main {
  main() {
print(42)
return 0
  }
}
"#;
    let json = source_to_program_json_v0_strict(source).expect("program json");
    assert!(json.contains("\"kind\":\"Program\""));
    assert!(json.contains("\"version\":0"));
    assert!(json.contains("\"env.console.log\""));
}

#[test]
fn source_to_program_json_v0_emits_statement_family_shapes() {
    with_features(Some("stage3"), || {
        let source = r#"
static box Main {
  main() {
local i = 0
loop i < 3 {
print(i)
i = i + 1
if i == 2 {
continue
}
if i == 3 {
break
}
}
loop j in 0..2 {
print(j)
}
return i
  }
}
"#;
        let json = source_to_program_json_v0_strict(source).expect("program json");
        let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
        let body = value["body"].as_array().expect("body");
        assert_eq!(body[0]["type"], "Local");
        assert_eq!(body[1]["type"], "Loop");
        assert_eq!(body[1]["body"][0]["type"], "Expr");
        assert_eq!(body[1]["body"][1]["type"], "Local");
        assert_eq!(body[1]["body"][2]["type"], "If");
        assert_eq!(body[1]["body"][2]["then"][0]["type"], "Continue");
        assert_eq!(body[1]["body"][3]["then"][0]["type"], "Break");
        assert_eq!(body[2]["type"], "LoopRange");
        assert_eq!(body[2]["body"][0]["type"], "Expr");
        assert_eq!(body[3]["type"], "Return");
    });
}

#[test]
fn source_to_program_json_v0_emits_task_scope_shape() {
    let source = r#"
static box Main {
  main() {
co {
local value = 1
}
return 0
  }
}
"#;
    let json = source_to_program_json_v0_strict(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let body = value["body"].as_array().expect("body");
    assert_eq!(body[0]["type"], "TaskScope");
    assert_eq!(body[0]["spelling"], "co");
    assert_eq!(body[0]["body"][0]["type"], "Local");
    assert_eq!(body[1]["type"], "Return");
}

#[test]
fn source_to_program_json_v0_rejects_sync_box_until_runtime_rows() {
    let source = r#"
sync box Counter {
  value: i64
}

static box Main {
  main() {
return 0
  }
}
"#;
    let error = source_to_program_json_v0_strict(source)
        .expect_err("sync box must not silently lower as ordinary box");
    assert!(
        error.contains("[program_json_v0/sync_box_not_supported]"),
        "{error}"
    );
}

#[test]
fn source_to_program_json_v0_rejects_context_scope_until_propagation_row() {
    let source = r#"
static box Main {
  main() {
local rid = 1
context request_id = rid {
local value = 1
}
return 0
  }
}
"#;
    let error = source_to_program_json_v0_strict(source)
        .expect_err("context scope must not silently lower as lexical block");
    assert!(
        error.contains("[program_json_v0/context_scope_not_supported]"),
        "{error}"
    );
}

#[test]
fn source_to_program_json_v0_supports_static_method_call() {
    let source = r#"
static box Driver {
  main(args) {
return 0
  }
}
static box Main {
  main(args) {
return Driver.main(args)
  }
}
"#;
    let json = source_to_program_json_v0_strict(source).expect("program json");
    assert!(json.contains("\"kind\":\"Program\""));
    assert!(json.contains("\"type\":\"Call\""));
    assert!(json.contains("\"Driver.main\""));
}

#[test]
fn source_to_program_json_v0_emits_enum_inventory_and_ctor() {
    let source = r#"
enum Option<T> {
  None
  Some(T)
}

static box Main {
  main() {
local x = Option::Some("hello")
return 0
  }
}
"#;

    let json = source_to_program_json_v0_strict(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let enum_decls = value["enum_decls"].as_array().expect("enum decls");
    assert_eq!(enum_decls.len(), 1);
    assert_eq!(enum_decls[0]["name"], "Option");
    assert_eq!(enum_decls[0]["type_parameters"], serde_json::json!(["T"]));
    assert_eq!(enum_decls[0]["variants"][1]["name"], "Some");
    assert_eq!(enum_decls[0]["variants"][1]["payload_type"], "T");

    let body = value["body"].as_array().expect("body");
    assert_eq!(body[0]["type"], "Local");
    assert_eq!(body[0]["expr"]["type"], "EnumCtor");
    assert_eq!(body[0]["expr"]["enum"], "Option");
    assert_eq!(body[0]["expr"]["variant"], "Some");
}

#[test]
fn source_to_program_json_v0_emits_brand_inventory_constructor_and_unwrap() {
    let source = r#"
brand PageId: i64

static box Main {
  main() {
local page = PageId(7)
return PageId.unwrap(page)
  }
}
"#;

    let json = source_to_program_json_v0_strict(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let brand_decls = value["brand_decls"].as_array().expect("brand decls");
    assert_eq!(brand_decls.len(), 1);
    assert_eq!(brand_decls[0]["name"], "PageId");
    assert_eq!(brand_decls[0]["underlying_type"], "i64");

    let body = value["body"].as_array().expect("body");
    assert_eq!(body[0]["type"], "Local");
    assert_eq!(body[0]["expr"]["type"], "BrandConstruct");
    assert_eq!(body[0]["expr"]["brand"], "PageId");
    assert_eq!(body[0]["expr"]["underlying_type"], "i64");
    assert_eq!(body[0]["expr"]["value"]["value"], 7);

    assert_eq!(body[1]["type"], "Return");
    assert_eq!(body[1]["expr"]["type"], "BrandUnwrap");
    assert_eq!(body[1]["expr"]["brand"], "PageId");
    assert_eq!(body[1]["expr"]["underlying_type"], "i64");
    assert_eq!(body[1]["expr"]["value"]["name"], "page");
}

#[test]
fn source_to_program_json_v0_rejects_brand_constructor_arity() {
    let source = r#"
brand PageId: i64

static box Main {
  main() {
local page = PageId()
return 0
  }
}
"#;

    let error = source_to_program_json_v0_strict(source)
        .expect_err("brand constructor arity must fail-fast");
    assert!(error.contains("[brand/constructor-arity]"), "{error}");
}

#[test]
fn source_to_program_json_v0_rejects_brand_unsupported_static_method() {
    let source = r#"
brand PageId: i64

static box Main {
  main() {
return PageId.cast(7)
  }
}
"#;

    let error = source_to_program_json_v0_strict(source)
        .expect_err("unsupported brand static method must fail-fast");
    assert!(
        error.contains("[brand/unsupported-static-method]"),
        "{error}"
    );
}

#[test]
fn source_to_program_json_v0_accepts_matching_brand_method_arg() {
    let source = r#"
brand BlockId: i64

static box Main {
  main() {
local block = BlockId(7)
return me.releaseLocal(block)
  }

  method releaseLocal(block: BlockId): i64 {
return 1
  }
}
"#;

    let json = source_to_program_json_v0_strict(source).expect("matching brand arg");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    assert_eq!(value["kind"], "Program");
}

#[test]
fn source_to_program_json_v0_rejects_mismatched_brand_method_arg() {
    let source = r#"
brand PageId: i64
brand BlockId: i64

static box Main {
  main() {
local page = PageId(7)
return me.releaseLocal(page)
  }

  method releaseLocal(block: BlockId): i64 {
return 1
  }
}
"#;

    let error =
        source_to_program_json_v0_strict(source).expect_err("mismatched brand arg must fail-fast");
    assert!(error.contains("[brand/mismatch]"), "{error}");
    assert!(error.contains("expected BlockId, got PageId"), "{error}");
}

#[test]
fn source_to_program_json_v0_rejects_unbranded_value_for_brand_arg() {
    let source = r#"
brand BlockId: i64

static box Main {
  main() {
return me.releaseLocal(7)
  }

  method releaseLocal(block: BlockId): i64 {
return 1
  }
}
"#;

    let error = source_to_program_json_v0_strict(source).expect_err("unbranded arg must fail-fast");
    assert!(error.contains("[brand/mismatch]"), "{error}");
    assert!(error.contains("expected BlockId, got unbranded"), "{error}");
}

#[test]
fn source_to_program_json_v0_emits_known_enum_match() {
    let source = r#"
enum Option<T> {
  None
  Some(T)
}

static box Main {
  main() {
local value = Option::Some(1)
return match value {
  Some(v) => v
  None => 0
}
  }
}
"#;

    let json = source_to_program_json_v0_strict(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let body = value["body"].as_array().expect("body");
    assert_eq!(body[1]["type"], "Return");
    assert_eq!(body[1]["expr"]["type"], "EnumMatch");
    assert_eq!(body[1]["expr"]["enum"], "Option");
    assert_eq!(body[1]["expr"]["arms"][0]["variant"], "Some");
    assert_eq!(body[1]["expr"]["arms"][0]["bind"], "v");
    assert_eq!(body[1]["expr"]["arms"][1]["variant"], "None");
    assert!(body[1]["expr"]["else"].is_null());
}

#[test]
fn source_to_program_json_v0_emits_unit_enum_ctor() {
    let source = r#"
enum Option<T> {
  None
  Some(T)
}

static box Main {
  main() {
local x = Option::None
return 0
  }
}
"#;

    let json = source_to_program_json_v0_strict(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let body = value["body"].as_array().expect("body");
    assert_eq!(body[0]["type"], "Local");
    assert_eq!(body[0]["expr"]["type"], "EnumCtor");
    assert_eq!(body[0]["expr"]["enum"], "Option");
    assert_eq!(body[0]["expr"]["variant"], "None");
    assert!(body[0]["expr"]["args"]
        .as_array()
        .expect("args array")
        .is_empty());
}

#[test]
fn source_to_program_json_v0_uses_result_option_prelude() {
    let source = r#"
static box Main {
  main() {
local empty: Option<i64> = Option::None
local ok: Result<i64, String> = Result::Ok(7)
local err: Result<i64, String> = Result::Err("bad")
return 0
  }
}
"#;

    let json = source_to_program_json_v0_strict(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let body = value["body"].as_array().expect("body");

    assert_eq!(body[0]["expr"]["type"], "EnumCtor");
    assert_eq!(body[0]["expr"]["enum"], "Option");
    assert_eq!(body[0]["expr"]["variant"], "None");
    assert_eq!(body[1]["expr"]["type"], "EnumCtor");
    assert_eq!(body[1]["expr"]["enum"], "Result");
    assert_eq!(body[1]["expr"]["variant"], "Ok");
    assert_eq!(body[2]["expr"]["type"], "EnumCtor");
    assert_eq!(body[2]["expr"]["enum"], "Result");
    assert_eq!(body[2]["expr"]["variant"], "Err");
}

#[test]
fn source_to_program_json_v0_rejects_dot_enum_variant_surface() {
    let source = r#"
static box Main {
  main() {
local bad = Result.Ok(1)
return 0
  }
}
"#;

    let error =
        source_to_program_json_v0_strict(source).expect_err("dot enum variant should fail-fast");
    assert!(error.contains("[enum/variant-surface]"), "{error}");
    assert!(error.contains("Result::Ok"), "{error}");
}

#[test]
fn source_to_program_json_v0_rejects_prelude_option_some_null_payload() {
    let source = r#"
static box Main {
  main() {
local x = Option::Some(null)
return 0
  }
}
"#;

    let error = source_to_program_json_v0_strict(source)
        .expect_err("prelude Option::Some(null) should fail fast");
    assert!(error.contains("[freeze:contract][option/some_nullish]"));
}

#[test]
fn source_to_program_json_v0_rejects_prelude_enum_payload_some_missing_arg() {
    let source = r#"
static box Main {
  main() {
local x = Option::Some()
return 0
  }
}
"#;

    let error = source_to_program_json_v0_strict(source)
        .expect_err("Option::Some missing payload should fail fast");
    assert!(error.contains("[enum/payload][prelude]"), "{error}");
    assert!(
        error.contains("Option::Some expects 1 payload arg(s), got 0"),
        "{error}"
    );
}

#[test]
fn source_to_program_json_v0_rejects_prelude_enum_payload_none_extra_arg() {
    let source = r#"
static box Main {
  main() {
local x = Option::None(1)
return 0
  }
}
"#;

    let error = source_to_program_json_v0_strict(source)
        .expect_err("Option::None extra payload should fail fast");
    assert!(error.contains("[enum/payload][prelude]"), "{error}");
    assert!(
        error.contains("Option::None expects 0 payload arg(s), got 1"),
        "{error}"
    );
}

#[test]
fn source_to_program_json_v0_rejects_prelude_enum_payload_result_err_missing_arg() {
    let source = r#"
static box Main {
  main() {
local x = Result::Err()
return 0
  }
}
"#;

    let error = source_to_program_json_v0_strict(source)
        .expect_err("Result::Err missing payload should fail fast");
    assert!(error.contains("[enum/payload][prelude]"), "{error}");
    assert!(
        error.contains("Result::Err expects 1 payload arg(s), got 0"),
        "{error}"
    );
}

#[test]
fn source_to_program_json_v0_emits_option_sugar_some_and_none() {
    let source = r#"
enum Option<T> {
  None
  Some(T)
}

static box Main {
  main() {
local empty = none
local full = some 7
return 0
  }
}
"#;

    let json = source_to_program_json_v0_strict(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let body = value["body"].as_array().expect("body");

    assert_eq!(body[0]["expr"]["type"], "EnumCtor");
    assert_eq!(body[0]["expr"]["enum"], "Option");
    assert_eq!(body[0]["expr"]["variant"], "None");
    assert_eq!(body[1]["expr"]["type"], "EnumCtor");
    assert_eq!(body[1]["expr"]["enum"], "Option");
    assert_eq!(body[1]["expr"]["variant"], "Some");
    assert_eq!(body[1]["expr"]["args"][0]["value"], 7);
}
