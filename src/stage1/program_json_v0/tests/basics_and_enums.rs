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

#[test]
fn source_to_program_json_v0_emits_record_decls_separate_from_user_boxes() {
    let source = r#"
record Meta<T> {
  ptr: i64
  payload: T
}

box Ordinary {
  x: i64
}

static box Main {
  main() {
return 0
  }
}
"#;

    let json = source_to_program_json_v0_strict(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let record_decls = value["record_decls"].as_array().expect("record decls");
    assert_eq!(record_decls.len(), 1);
    assert_eq!(record_decls[0]["name"], "Meta");
    assert_eq!(record_decls[0]["type_parameters"], serde_json::json!(["T"]));
    assert_eq!(record_decls[0]["field_decls"][0]["name"], "ptr");
    assert_eq!(record_decls[0]["field_decls"][0]["declared_type"], "i64");
    assert_eq!(record_decls[0]["field_decls"][0]["field_index"], 0);

    let user_box_decls = value["user_box_decls"].as_array().expect("user box decls");
    assert!(user_box_decls
        .iter()
        .all(|decl| decl.get("name").and_then(serde_json::Value::as_str) != Some("Meta")));
    assert!(user_box_decls
        .iter()
        .any(|decl| decl.get("name").and_then(serde_json::Value::as_str) == Some("Ordinary")));
}

#[test]
fn source_to_program_json_v0_emits_type_alias_decls_metadata_only() {
    let source = r#"
type Bytes = usize

static box Main {
  main() {
return 0
  }
}
"#;

    let json = source_to_program_json_v0_strict(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let aliases = value["type_alias_decls"]
        .as_array()
        .expect("type alias decls");
    assert_eq!(aliases.len(), 1);
    assert_eq!(aliases[0]["name"], "Bytes");
    assert_eq!(aliases[0]["target_type"], "usize");
}

#[test]
fn source_to_program_json_v0_rewrites_if_some_sugar_to_local_plus_if() {
    let source = r#"
enum Option<T> {
  None
  Some(T)
}

static box Main {
  main() {
local value = some 7
if some v = value {
  return v
} else {
  return 0
}
  }
}
"#;

    let json = source_to_program_json_v0_strict(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let body = value["body"].as_array().expect("body");

    assert_eq!(body[1]["type"], "Local");
    let temp_name = body[1]["name"].as_str().expect("temp local name");
    assert!(
        temp_name.starts_with("__ny_option_some_subject_"),
        "expected hidden temp local, got {temp_name}"
    );
    assert_eq!(body[2]["type"], "If");
    assert_eq!(body[2]["cond"]["type"], "EnumMatch");
    assert_eq!(body[2]["cond"]["enum"], "Option");
    assert_eq!(body[2]["cond"]["arms"][0]["variant"], "Some");
    assert_eq!(body[2]["cond"]["arms"][0]["expr"]["value"], true);
    assert_eq!(body[2]["cond"]["arms"][1]["variant"], "None");
    assert_eq!(body[2]["cond"]["arms"][1]["expr"]["value"], false);
    assert_eq!(body[2]["then"][0]["type"], "Local");
    assert_eq!(body[2]["then"][0]["name"], "v");
    assert_eq!(body[2]["then"][0]["expr"]["type"], "EnumMatch");
    assert_eq!(body[2]["then"][0]["expr"]["scrutinee"]["name"], temp_name);
    assert_eq!(body[2]["then"][0]["expr"]["arms"][0]["bind"], "v");
    assert_eq!(body[2]["then"][1]["type"], "Return");
    assert_eq!(body[2]["else"][0]["type"], "Return");
}

#[test]
fn source_to_program_json_v0_rejects_option_some_null_payload() {
    let source = r#"
enum Option<T> {
  None
  Some(T)
}

static box Main {
  main() {
local x = Option::Some(null)
return 0
  }
}
"#;

    let error = source_to_program_json_v0_strict(source)
        .expect_err("Option::Some(null) should fail fast on the shared enum lane");
    assert!(error.contains("[freeze:contract][option/some_nullish]"));
    assert!(error.contains("Option::Some payload must not be null or void"));
}

#[test]
fn source_to_program_json_v0_rejects_option_some_void_payload() {
    let source = r#"
enum Option<T> {
  None
  Some(T)
}

static box Main {
  main() {
local x = Option::Some(void)
return 0
  }
}
"#;

    let error = source_to_program_json_v0_strict(source)
        .expect_err("Option::Some(void) should fail fast on the shared enum lane");
    assert!(error.contains("[freeze:contract][option/some_nullish]"));
    assert!(error.contains("Option::Some payload must not be null or void"));
}

#[test]
fn source_to_program_json_v0_rejects_non_exhaustive_enum_match() {
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
  _ => 0
}
  }
}
"#;

    let error = source_to_program_json_v0_strict(source)
        .expect_err("non-exhaustive enum match should fail");
    assert!(error.contains("non-exhaustive enum match"));
    assert!(error.contains("None"));
}

#[test]
fn source_to_program_json_v0_emits_record_enum_payload_box_contract() {
    let source = r#"
enum Token {
  Ident { name: String }
  Eof
}

static box Main {
  main() {
local tok = Token::Ident { name: "hello" }
return match tok {
  Ident { name } => name
  Eof => "eof"
}
  }
}
"#;

    let json = source_to_program_json_v0_strict(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let payload_box = "__NyVariantPayload_Token_Ident";

    let enum_decls = value["enum_decls"].as_array().expect("enum decls");
    assert_eq!(enum_decls[0]["variants"][0]["payload_type"], payload_box);
    assert_eq!(
        enum_decls[0]["variants"][0]["record_fields"][0]["name"],
        "name"
    );

    let user_box_decls = value["user_box_decls"].as_array().expect("user box decls");
    let payload_decl = user_box_decls
        .iter()
        .find(|decl| decl["name"] == payload_box)
        .expect("hidden payload box decl");
    assert_eq!(payload_decl["field_decls"][0]["name"], "name");
    assert_eq!(payload_decl["field_decls"][0]["declared_type"], "String");

    let body = value["body"].as_array().expect("body");
    assert_eq!(body[0]["expr"]["type"], "EnumCtor");
    assert_eq!(body[0]["expr"]["payload_type"], payload_box);
    assert_eq!(body[0]["expr"]["args"][0]["type"], "New");
    assert_eq!(body[0]["expr"]["args"][0]["class"], payload_box);

    assert_eq!(body[1]["expr"]["type"], "EnumMatch");
    assert_eq!(body[1]["expr"]["arms"][0]["payload_type"], payload_box);
    assert_eq!(body[1]["expr"]["arms"][0]["expr"]["type"], "BlockExpr");
    assert_eq!(
        body[1]["expr"]["arms"][0]["expr"]["prelude"][0]["expr"]["type"],
        "Field"
    );
    assert_eq!(
        body[1]["expr"]["arms"][0]["expr"]["prelude"][0]["expr"]["field"],
        "name"
    );
}

#[test]
fn source_to_program_json_v0_emits_tuple_enum_payload_box_contract() {
    let source = r#"
enum Pair {
  Both(Integer, Integer)
  None
}

static box Main {
  main() {
local pair = Pair::Both(1, 2)
return match pair {
  Both(left, right) => left + right
  None => 0
}
  }
}
"#;

    let json = source_to_program_json_v0_strict(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let payload_box = "__NyVariantPayload_Pair_Both";

    let enum_decls = value["enum_decls"].as_array().expect("enum decls");
    assert_eq!(enum_decls[0]["variants"][0]["payload_type"], payload_box);
    assert!(
        enum_decls[0]["variants"][0]["record_fields"]
            .as_array()
            .expect("record fields")
            .is_empty(),
        "tuple surface stays tuple-shaped in enum inventory"
    );

    let user_box_decls = value["user_box_decls"].as_array().expect("user box decls");
    let payload_decl = user_box_decls
        .iter()
        .find(|decl| decl["name"] == payload_box)
        .expect("hidden payload box decl");
    assert_eq!(payload_decl["field_decls"][0]["name"], "_0");
    assert_eq!(payload_decl["field_decls"][0]["declared_type"], "Integer");
    assert_eq!(payload_decl["field_decls"][1]["name"], "_1");
    assert_eq!(payload_decl["field_decls"][1]["declared_type"], "Integer");

    let body = value["body"].as_array().expect("body");
    assert_eq!(body[0]["expr"]["type"], "EnumCtor");
    assert_eq!(body[0]["expr"]["payload_type"], payload_box);
    assert_eq!(body[0]["expr"]["args"][0]["type"], "New");
    assert_eq!(body[0]["expr"]["args"][0]["class"], payload_box);
    assert_eq!(body[0]["expr"]["args"][0]["args"][0]["value"], 1);
    assert_eq!(body[0]["expr"]["args"][0]["args"][1]["value"], 2);

    assert_eq!(body[1]["expr"]["type"], "EnumMatch");
    assert_eq!(body[1]["expr"]["arms"][0]["payload_type"], payload_box);
    assert_eq!(body[1]["expr"]["arms"][0]["expr"]["type"], "BlockExpr");
    assert_eq!(
        body[1]["expr"]["arms"][0]["expr"]["prelude"][0]["expr"]["field"],
        "_0"
    );
    assert_eq!(
        body[1]["expr"]["arms"][0]["expr"]["prelude"][1]["expr"]["field"],
        "_1"
    );
}
