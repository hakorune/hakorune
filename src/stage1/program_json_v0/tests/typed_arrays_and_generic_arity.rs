use super::*;

#[test]
fn source_to_program_json_v0_lowers_typed_array_literal_context() {
    let source = r#"
brand PageId: i64

static box Main {
  main() {
local ids: Array<PageId> = []
return 0
  }
}
"#;

    let json = source_to_program_json_v0_strict(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let body = value["body"].as_array().expect("body");

    assert_eq!(body[0]["type"], "Local");
    assert_eq!(body[0]["name"], "ids");
    assert_eq!(body[0]["declared_type"], "Array<PageId>");
    assert_eq!(body[0]["expr"]["type"], "ArrayLiteral");
    assert_eq!(body[0]["expr"]["declared_type"], "Array<PageId>");
    assert_eq!(body[0]["expr"]["element_type"], "PageId");
    assert_eq!(body[0]["expr"]["elements"], serde_json::json!([]));
}

#[test]
fn source_to_program_json_v0_accepts_typed_array_method_contract() {
    let source = r#"
static box Main {
  main() {
local ids: Array<i64> = []
ids.push(1)
local first = ids.get(0)
ids.set(0, 2)
local n = ids.length()
return n
  }
}
"#;

    let json = source_to_program_json_v0_strict(source).expect("typed array methods");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let body = value["body"].as_array().expect("body");

    assert_eq!(body[1]["expr"]["type"], "Method");
    assert_eq!(body[1]["expr"]["method"], "push");
    assert_eq!(body[2]["expr"]["method"], "get");
    assert_eq!(body[3]["expr"]["method"], "set");
    assert_eq!(body[4]["expr"]["method"], "length");
}

#[test]
fn source_to_program_json_v0_rejects_typed_array_method_contract_noncanonical_method() {
    let source = r#"
static box Main {
  main() {
local ids: Array<i64> = []
ids.len()
return 0
  }
}
"#;

    let error = source_to_program_json_v0_strict(source)
        .expect_err("typed array non-canonical method should fail");
    assert!(error.contains("[array/method-contract]"), "{error}");
    assert!(error.contains("push/get/set/length"), "{error}");
    assert!(error.contains("len"), "{error}");
}

#[test]
fn source_to_program_json_v0_rejects_typed_array_method_contract_arity() {
    let source = r#"
static box Main {
  main() {
local ids: Array<i64> = []
ids.set(0)
return 0
  }
}
"#;

    let error =
        source_to_program_json_v0_strict(source).expect_err("typed array method arity should fail");
    assert!(error.contains("[array/method-contract]"), "{error}");
    assert!(error.contains("set"), "{error}");
    assert!(error.contains("expects 2 arg(s), got 1"), "{error}");
}

#[test]
fn source_to_program_json_v0_accepts_typed_array_element_checks_for_brands() {
    let source = r#"
brand PageId: i64

static box Main {
  main() {
local ids: Array<PageId> = [PageId(1)]
ids.push(PageId(2))
ids.set(0, PageId(3))
return 0
  }
}
"#;

    source_to_program_json_v0_strict(source).expect("brand element values should match");
}

#[test]
fn source_to_program_json_v0_rejects_typed_array_element_check_literal_mismatch() {
    let source = r#"
brand PageId: i64

static box Main {
  main() {
local ids: Array<PageId> = [1]
return 0
  }
}
"#;

    let error = source_to_program_json_v0_strict(source)
        .expect_err("raw integer must not satisfy PageId element type");
    assert!(error.contains("[array/element-type]"), "{error}");
    assert!(error.contains("array literal element"), "{error}");
    assert!(error.contains("PageId"), "{error}");
    assert!(error.contains("i64"), "{error}");
}

#[test]
fn source_to_program_json_v0_rejects_typed_array_element_check_push_mismatch() {
    let source = r#"
brand PageId: i64

static box Main {
  main() {
local ids: Array<PageId> = []
ids.push(1)
return 0
  }
}
"#;

    let error = source_to_program_json_v0_strict(source)
        .expect_err("push value must match PageId element type");
    assert!(error.contains("[array/element-type]"), "{error}");
    assert!(error.contains("push value"), "{error}");
    assert!(error.contains("PageId"), "{error}");
}

#[test]
fn source_to_program_json_v0_rejects_typed_array_element_check_set_mismatch() {
    let source = r#"
brand PageId: i64

static box Main {
  main() {
local ids: Array<PageId> = []
ids.set(0, 1)
return 0
  }
}
"#;

    let error = source_to_program_json_v0_strict(source)
        .expect_err("set value must match PageId element type");
    assert!(error.contains("[array/element-type]"), "{error}");
    assert!(error.contains("set value"), "{error}");
    assert!(error.contains("PageId"), "{error}");
}

#[test]
fn source_to_program_json_v0_rejects_typed_array_inference_unresolved_element() {
    let source = r#"
static box Main {
  main() {
local ids: Array<T> = []
return 0
  }
}
"#;

    let error = source_to_program_json_v0_strict(source)
        .expect_err("unresolved Array<T> element should fail fast");
    assert!(error.contains("[array/inference]"), "{error}");
    assert!(error.contains("Array<T>"), "{error}");
    assert!(
        error.contains("unresolved Array element type `T`"),
        "{error}"
    );
}

#[test]
fn source_to_program_json_v0_rejects_typed_array_inference_mixed_literals() {
    let source = r#"
static box Main {
  main() {
local ids: Array<i64> = [1, "bad"]
return 0
  }
}
"#;

    let error = source_to_program_json_v0_strict(source)
        .expect_err("mixed direct literal elements should fail fast");
    assert!(error.contains("[array/element-type]"), "{error}");
    assert!(error.contains("array literal element"), "{error}");
    assert!(error.contains("i64"), "{error}");
    assert!(error.contains("String"), "{error}");
}

#[test]
fn source_to_program_json_v0_rejects_untyped_empty_array_literal() {
    let source = r#"
static box Main {
  main() {
local ids = []
return 0
  }
}
"#;

    let error =
        source_to_program_json_v0_strict(source).expect_err("untyped array literal should fail");
    assert!(error.contains("[array/literal-context]"));
}

#[test]
fn source_to_program_json_v0_rejects_packed_array_literal_without_backend_fallback() {
    let source = r#"
record Meta {
  ptr: i64
}

static box Main {
  main() {
local metas: PackedArray<Meta> = []
return 0
  }
}
"#;

    let error = source_to_program_json_v0_strict(source)
        .expect_err("PackedArray literal must not fallback to ArrayBox");
    assert!(error.contains("[array/literal-context]"));
    assert!(error.contains("PackedArray"));
}

#[test]
fn source_to_program_json_v0_accepts_matching_generic_arities() {
    let source = r#"
record Meta<T> {
  value: T
}

box Store {
  metas: Array<Meta<PageId>>
}

static box Main {
  main(items: Array<PageId>): Result<PageId, Error> {
return 0
  }
}
"#;

    let json = source_to_program_json_v0_strict(source).expect("matching arities");
    assert!(json.contains("\"Array<Meta<PageId>>\""));
}

#[test]
fn source_to_program_json_v0_accepts_packed_array_integer_record_eligibility() {
    let source = r#"
brand PageId: i64
type Bytes = usize

record Meta {
  page: PageId
  size: Bytes
}

box Store {
  metas: PackedArray<Meta>
}

static box Main {
  main() {
return 0
  }
}
"#;

    let json = source_to_program_json_v0_strict(source)
        .expect("integer-lane record PackedArray should be eligible");
    assert!(json.contains("\"PackedArray<Meta>\""));
}

#[test]
fn source_to_program_json_v0_rejects_packed_array_ordinary_box_element() {
    let source = r#"
box Item {
  value: i64
}

box Store {
  items: PackedArray<Item>
}

static box Main {
  main() {
return 0
  }
}
"#;

    let error = source_to_program_json_v0_strict(source)
        .expect_err("ordinary box element must fail PackedArray eligibility");
    assert!(error.contains("[packed/eligibility]"), "{error}");
    assert!(error.contains("reason=ordinary-box-element"), "{error}");
    assert!(error.contains("type=PackedArray<Item>"), "{error}");
}

#[test]
fn source_to_program_json_v0_rejects_packed_array_handle_field() {
    let source = r#"
record Meta {
  label: String
}

box Store {
  metas: PackedArray<Meta>
}

static box Main {
  main() {
return 0
  }
}
"#;

    let error = source_to_program_json_v0_strict(source)
        .expect_err("handle field must fail PackedArray eligibility");
    assert!(error.contains("[packed/eligibility]"), "{error}");
    assert!(
        error.contains("reason=unsupported-field-storage"),
        "{error}"
    );
    assert!(error.contains("field=label"), "{error}");
}

#[test]
fn source_to_program_json_v0_rejects_packed_array_generic_record_instantiation() {
    let source = r#"
record Meta<T> {
  value: T
}

box Store {
  metas: PackedArray<Meta<PageId>>
}

static box Main {
  main() {
return 0
  }
}
"#;

    let error = source_to_program_json_v0_strict(source)
        .expect_err("generic record instantiation must fail PackedArray eligibility");
    assert!(error.contains("[packed/eligibility]"), "{error}");
    assert!(error.contains("reason=generic-element"), "{error}");
    assert!(error.contains("type=PackedArray<Meta<PageId>>"), "{error}");
}

#[test]
fn source_to_program_json_v0_rejects_builtin_generic_arity_mismatch() {
    let source = r#"
box Store {
  ids: Array<PageId, BlockId>
}

static box Main {
  main() {
return 0
  }
}
"#;

    let error = source_to_program_json_v0_strict(source)
        .expect_err("builtin generic arity mismatch must fail-fast");
    assert!(error.contains("[generic/arity]"), "{error}");
    assert!(error.contains("type=Array"), "{error}");
    assert!(error.contains("expected=1"), "{error}");
    assert!(error.contains("actual=2"), "{error}");
}

#[test]
fn source_to_program_json_v0_rejects_declared_generic_arity_mismatch() {
    let source = r#"
record Meta<T> {
  value: T
}

box Store {
  metas: PackedArray<Meta<PageId, BlockId>>
}

static box Main {
  main() {
return 0
  }
}
"#;

    let error = source_to_program_json_v0_strict(source)
        .expect_err("declared generic arity mismatch must fail-fast");
    assert!(error.contains("[generic/arity]"), "{error}");
    assert!(error.contains("type=Meta"), "{error}");
    assert!(error.contains("expected=1"), "{error}");
    assert!(error.contains("actual=2"), "{error}");
}

#[test]
fn source_to_program_json_v0_rejects_bare_declared_generic_type() {
    let source = r#"
record Meta<T> {
  value: T
}

box Store {
  metas: PackedArray<Meta>
}

static box Main {
  main() {
return 0
  }
}
"#;

    let error = source_to_program_json_v0_strict(source)
        .expect_err("bare declared generic type must fail-fast");
    assert!(error.contains("[generic/arity]"), "{error}");
    assert!(error.contains("type=Meta"), "{error}");
    assert!(error.contains("expected=1"), "{error}");
    assert!(error.contains("actual=0"), "{error}");
}
