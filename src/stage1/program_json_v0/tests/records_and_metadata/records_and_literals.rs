use super::*;

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
fn source_to_program_json_v0_emits_record_literal_shape_metadata() {
    let source = r#"
record Meta {
  ptr: i64
  size: usize
}

static box Main {
  main() {
local meta = Meta { ptr: 1, size: 2 }
return 0
  }
}
"#;

    let json = source_to_program_json_v0_strict(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let body = value["body"].as_array().expect("body");
    assert_eq!(body[0]["type"], "Local");
    assert_eq!(body[0]["expr"]["type"], "RecordLiteral");
    assert_eq!(body[0]["expr"]["record"], "Meta");
    assert_eq!(body[0]["expr"]["fields"][0]["name"], "ptr");
    assert_eq!(body[0]["expr"]["fields"][0]["value"]["value"], 1);
    assert_eq!(body[0]["expr"]["fields"][1]["name"], "size");
    assert_eq!(body[0]["expr"]["fields"][1]["value"]["value"], 2);
}

#[test]
fn source_to_program_json_v0_rejects_record_literal_missing_field() {
    let source = r#"
record Meta {
  ptr: i64
  size: usize
}

static box Main {
  main() {
local meta = Meta { ptr: 1 }
return 0
  }
}
"#;

    let error = source_to_program_json_v0_strict(source).expect_err("missing field must reject");
    assert!(error.contains("[record/literal-shape] Meta missing field `size`"));
}

#[test]
fn source_to_program_json_v0_fills_record_literal_defaults_when_omitted() {
    let source = r#"
record Meta {
  ptr: i64
  size: usize = 64
}

static box Main {
  main() {
local meta = Meta { ptr: 1 }
return 0
  }
}
"#;

    let json = source_to_program_json_v0_strict(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let body = value["body"].as_array().expect("body");
    assert_eq!(body[0]["type"], "Local");
    assert_eq!(body[0]["expr"]["type"], "RecordLiteral");
    assert_eq!(body[0]["expr"]["record"], "Meta");
    assert_eq!(body[0]["expr"]["fields"][0]["name"], "ptr");
    assert_eq!(body[0]["expr"]["fields"][0]["value"]["value"], 1);
    assert_eq!(body[0]["expr"]["fields"][1]["name"], "size");
    assert_eq!(body[0]["expr"]["fields"][1]["value"]["value"], 64);
}

#[test]
fn source_to_program_json_v0_record_literal_keeps_type_namespace_with_same_value_name() {
    let source = r#"
record Meta {
  Meta: i64 = 0
  flag: i64 = 1
}

static box Main {
  main() {
local Meta = 9
local rec = Meta { Meta }
return rec.Meta
  }
}
"#;

    let json = source_to_program_json_v0_strict(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let body = value["body"].as_array().expect("body");
    assert_eq!(body[1]["type"], "Local");
    assert_eq!(body[1]["expr"]["type"], "RecordLiteral");
    assert_eq!(body[1]["expr"]["record"], "Meta");
    assert_eq!(body[1]["expr"]["fields"][0]["name"], "Meta");
    assert_eq!(body[1]["expr"]["fields"][0]["value"]["type"], "Var");
    assert_eq!(body[1]["expr"]["fields"][0]["value"]["name"], "Meta");
    assert_eq!(body[1]["expr"]["fields"][1]["name"], "flag");
    assert_eq!(body[1]["expr"]["fields"][1]["value"]["value"], 1);
    assert_eq!(body[2]["expr"]["type"], "RecordField");
    assert_eq!(body[2]["expr"]["record"], "Meta");
    assert_eq!(body[2]["expr"]["field"], "Meta");
}

#[test]
fn source_to_program_json_v0_rejects_record_literal_extra_field() {
    let source = r#"
record Meta {
  ptr: i64
}

static box Main {
  main() {
local meta = Meta { ptr: 1, size: 2 }
return 0
  }
}
"#;

    let error = source_to_program_json_v0_strict(source).expect_err("extra field must reject");
    assert!(error.contains("[record/literal-shape] Meta extra field `size`"));
}

#[test]
fn source_to_program_json_v0_lowers_record_field_read() {
    let source = r#"
record Meta {
  ptr: i64
  size: usize
}

static box Main {
  main() {
local meta = Meta { ptr: 1, size: 2 }
return meta.ptr
  }
}
"#;

    let json = source_to_program_json_v0_strict(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let body = value["body"].as_array().expect("body");
    assert_eq!(body[1]["type"], "Return");
    assert_eq!(body[1]["expr"]["type"], "RecordField");
    assert_eq!(body[1]["expr"]["record"], "Meta");
    assert_eq!(body[1]["expr"]["field"], "ptr");
    assert_eq!(body[1]["expr"]["field_index"], 0);
    assert_eq!(body[1]["expr"]["declared_type"], "i64");
    assert_eq!(body[1]["expr"]["recv"]["type"], "Var");
    assert_eq!(body[1]["expr"]["recv"]["name"], "meta");
}

#[test]
fn source_to_program_json_v0_lowers_record_with_update() {
    let source = r#"
record Meta {
  ptr: i64
  size: usize
}

static box Main {
  main() {
local meta = Meta { ptr: 1, size: 2 }
local next = meta with { size: 3 }
return next.size
  }
}
"#;

    let json = source_to_program_json_v0_strict(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let body = value["body"].as_array().expect("body");
    assert_eq!(body[1]["type"], "Local");
    assert_eq!(body[1]["expr"]["type"], "RecordUpdate");
    assert_eq!(body[1]["expr"]["record"], "Meta");
    assert_eq!(body[1]["expr"]["updates"][0]["name"], "size");
    assert_eq!(body[1]["expr"]["updates"][0]["field_index"], 1);
    assert_eq!(body[2]["expr"]["type"], "RecordField");
    assert_eq!(body[2]["expr"]["record"], "Meta");
    assert_eq!(body[2]["expr"]["field"], "size");
}

#[test]
fn source_to_program_json_v0_rejects_record_with_update_unknown_field() {
    let source = r#"
record Meta {
  ptr: i64
}

static box Main {
  main() {
local meta = Meta { ptr: 1 }
local next = meta with { size: 3 }
return 0
  }
}
"#;

    let error =
        source_to_program_json_v0_strict(source).expect_err("unknown update field must reject");
    assert!(error.contains("[record/field-read] Meta has no field `size`"));
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
