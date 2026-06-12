use super::*;

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
fn source_to_program_json_v0_rewrites_guard_let_enum_variant_to_local_if_binding() {
    let source = r#"
static box Main {
  main() {
local result: Result<i64, String> = Result::Ok(7)
guard let Result::Ok(value) = result else {
  return 0
}
return value
  }
}
"#;

    let json = source_to_program_json_v0_strict(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let body = value["body"].as_array().expect("body");

    assert_eq!(body[1]["type"], "Local");
    let temp_name = body[1]["name"].as_str().expect("temp local name");
    assert!(
        temp_name.starts_with("__ny_guard_let_subject_"),
        "expected hidden guard-let temp local, got {temp_name}"
    );
    assert_eq!(body[2]["type"], "If");
    assert_eq!(body[2]["cond"]["type"], "EnumMatch");
    assert_eq!(body[2]["cond"]["enum"], "Result");
    assert_eq!(body[2]["cond"]["arms"][0]["variant"], "Ok");
    assert_eq!(body[2]["cond"]["arms"][0]["expr"]["value"], false);
    assert_eq!(body[2]["cond"]["arms"][1]["variant"], "Err");
    assert_eq!(body[2]["cond"]["arms"][1]["expr"]["value"], true);
    assert_eq!(body[2]["then"][0]["type"], "Return");
    assert_eq!(body[3]["type"], "Local");
    assert_eq!(body[3]["name"], "value");
    assert_eq!(body[3]["expr"]["type"], "EnumMatch");
    assert_eq!(body[3]["expr"]["scrutinee"]["name"], temp_name);
    assert_eq!(body[3]["expr"]["arms"][0]["bind"], "value");
    assert_eq!(body[4]["type"], "Return");
    assert_eq!(body[4]["expr"]["name"], "value");
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
fn source_to_program_json_v0_rejects_prelude_option_missing_arm_diagnostic() {
    let source = r#"
static box Main {
  main() {
local value: Option<i64> = Option::Some(1)
return match value {
  Some(v) => v
}
  }
}
"#;

    let error = source_to_program_json_v0_strict(source)
        .expect_err("prelude Option missing arm should fail with explicit diagnostic");
    assert!(error.contains("[enum/missing-arm][prelude]"), "{error}");
    assert!(error.contains("Option"), "{error}");
    assert!(error.contains("Option::None"), "{error}");
}

#[test]
fn source_to_program_json_v0_rejects_prelude_result_missing_arm_diagnostic() {
    let source = r#"
static box Main {
  main() {
local value: Result<i64, String> = Result::Ok(1)
return match value {
  Ok(v) => v
  _ => 0
}
  }
}
"#;

    let error = source_to_program_json_v0_strict(source)
        .expect_err("prelude Result missing arm should fail with explicit diagnostic");
    assert!(error.contains("[enum/missing-arm][prelude]"), "{error}");
    assert!(error.contains("Result::Err"), "{error}");
    assert!(
        error.contains("`_` does not satisfy known-enum exhaustiveness"),
        "{error}"
    );
    assert!(error.contains("explicit prelude variant arm"), "{error}");
}

#[test]
fn source_to_program_json_v0_rejects_known_enum_underscore_exhaustiveness_rule() {
    let source = r#"
enum PageState {
  Active
  Retired
}

static box Main {
  main() {
local state = PageState::Active
return match state {
  Active => 1
  _ => 0
}
  }
}
"#;

    let error = source_to_program_json_v0_strict(source)
        .expect_err("known enum default arm must not satisfy exhaustiveness");
    assert!(
        error.contains("[enum/exhaustiveness][underscore]"),
        "{error}"
    );
    assert!(error.contains("PageState"), "{error}");
    assert!(error.contains("Retired"), "{error}");
    assert!(
        error.contains("`_` does not satisfy known-enum exhaustiveness"),
        "{error}"
    );
}

#[test]
fn source_to_program_json_v0_rejects_prelude_result_constructor_without_expected_type() {
    let source = r#"
static box Main {
  main() {
local value = Result::Err("bad")
return 0
  }
}
"#;

    let error = source_to_program_json_v0_strict(source)
        .expect_err("prelude generic enum constructor needs expected type");
    assert!(error.contains("[enum/expected-type][prelude]"), "{error}");
    assert!(error.contains("Result::Err"), "{error}");
    assert!(
        error.contains("local value: Result<T,E> = Result::Err(...)"),
        "{error}"
    );
}

#[test]
fn source_to_program_json_v0_rejects_prelude_option_unit_constructor_without_expected_type() {
    let source = r#"
static box Main {
  main() {
local value = Option::None
return 0
  }
}
"#;

    let error = source_to_program_json_v0_strict(source)
        .expect_err("prelude generic unit enum constructor needs expected type");
    assert!(error.contains("[enum/expected-type][prelude]"), "{error}");
    assert!(error.contains("Option::None"), "{error}");
    assert!(
        error.contains("local value: Option<T> = Option::None"),
        "{error}"
    );
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
