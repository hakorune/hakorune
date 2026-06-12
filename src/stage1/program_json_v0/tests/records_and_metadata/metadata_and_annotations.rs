use super::*;

#[test]
fn source_to_program_json_v0_transports_contract_metadata() {
    let source = r#"
static box Main {
  main() {
return me.releaseLocal(1)
  }

  method releaseLocal(block: i64): i64
    requires block >= 0
    ensures block >= 0
  {
return block
  }
}
"#;

    let json = source_to_program_json_v0_strict(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let defs = value["defs"].as_array().expect("helper defs");
    let release = defs
        .iter()
        .find(|def| def["name"] == "releaseLocal")
        .expect("releaseLocal def");
    let contracts = release["contracts"].as_array().expect("contracts metadata");

    assert_eq!(contracts.len(), 2);
    assert_eq!(contracts[0]["kind"], "requires");
    assert_eq!(contracts[1]["kind"], "ensures");
}

#[test]
fn source_to_program_json_v0_transports_invariant_metadata() {
    let source = r#"
box Page {
  used: i64
  invariant used >= 0
}

record Meta {
  ptr: i64
  invariant ptr >= 0
}

static box Main {
  main() {
return 0
  }
}
"#;

    let json = source_to_program_json_v0_strict(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let user_box_decls = value["user_box_decls"].as_array().expect("user box decls");
    let page = user_box_decls
        .iter()
        .find(|decl| decl["name"] == "Page")
        .expect("Page decl");
    let record_decls = value["record_decls"].as_array().expect("record decls");
    let meta = record_decls
        .iter()
        .find(|decl| decl["name"] == "Meta")
        .expect("Meta decl");

    assert_eq!(
        page["invariants"]
            .as_array()
            .expect("Page invariants")
            .len(),
        1
    );
    assert_eq!(
        meta["invariants"]
            .as_array()
            .expect("Meta invariants")
            .len(),
        1
    );
}

#[test]
fn source_to_program_json_v0_transports_transition_metadata() {
    let source = r#"
enum PageState {
  Active
  Retired
}

box Page {
  state: PageState
  transition PageState::Active -> PageState::Retired by retire
}

static box Main {
  main() {
return 0
  }
}
"#;

    let json = source_to_program_json_v0_strict(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let user_box_decls = value["user_box_decls"].as_array().expect("user box decls");
    let page = user_box_decls
        .iter()
        .find(|decl| decl["name"] == "Page")
        .expect("Page decl");
    let transitions = page["transitions"]
        .as_array()
        .expect("transitions metadata");

    assert_eq!(transitions.len(), 1);
    assert_eq!(transitions[0]["from"], "PageState::Active");
    assert_eq!(transitions[0]["to"], "PageState::Retired");
    assert_eq!(transitions[0]["method"], "retire");
}

#[test]
fn source_to_program_json_v0_normalizes_legacy_dot_transition_refs() {
    let source = r#"
enum PageState {
  Active
  Retired
}

box Page {
  state: PageState
  transition PageState.Active -> PageState.Retired by retire
}

static box Main {
  main() {
return 0
  }
}
"#;

    let json = source_to_program_json_v0_strict(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let user_box_decls = value["user_box_decls"].as_array().expect("user box decls");
    let page = user_box_decls
        .iter()
        .find(|decl| decl["name"] == "Page")
        .expect("Page decl");
    let transitions = page["transitions"]
        .as_array()
        .expect("transitions metadata");

    assert_eq!(transitions[0]["from"], "PageState::Active");
    assert_eq!(transitions[0]["to"], "PageState::Retired");
}

#[test]
fn source_to_program_json_v0_transports_uses_metadata() {
    let source = r#"
static box Main {
  main() {
return me.reserve(1)
  }

  method reserve(size: i64): i64
    uses osvm, rawbuf
  {
return size
  }
}
"#;

    let json = source_to_program_json_v0_strict(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let defs = value["defs"].as_array().expect("helper defs");
    let reserve = defs
        .iter()
        .find(|def| def["name"] == "reserve")
        .expect("reserve def");
    let uses = reserve["uses"].as_array().expect("uses metadata");

    assert_eq!(
        uses,
        &vec![serde_json::json!("osvm"), serde_json::json!("rawbuf")]
    );
}

#[test]
fn source_to_program_json_v0_transports_generic_type_metadata() {
    let source = r#"
record Meta<T> {
  value: T
}

box Store {
  metas: Array<Meta<PageId>>
}

static box Main {
  main() {
return me.process(0)
  }

  method process(items: Array<PageId>): Result<PageId, Error> {
return 0
  }
}
"#;

    let json = source_to_program_json_v0_strict(source).expect("program json");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let record_decls = value["record_decls"].as_array().expect("record decls");
    assert_eq!(record_decls[0]["type_parameters"], serde_json::json!(["T"]));
    assert_eq!(record_decls[0]["field_decls"][0]["declared_type"], "T");

    let user_box_decls = value["user_box_decls"].as_array().expect("user box decls");
    let store = user_box_decls
        .iter()
        .find(|decl| decl["name"] == "Store")
        .expect("Store decl");
    assert_eq!(
        store["field_decls"][0]["declared_type"],
        "Array<Meta<PageId>>"
    );

    let defs = value["defs"].as_array().expect("helper defs");
    let process = defs
        .iter()
        .find(|def| def["name"] == "process")
        .expect("process def");
    assert_eq!(process["param_decls"][0]["declared_type"], "Array<PageId>");
    assert_eq!(process["return_type"], "Result<PageId,Error>");
}

#[test]
fn source_to_program_json_v0_transports_local_type_annotation_metadata() {
    let source = r#"
static box Main {
  main() {
local ids: Array<PageId> = null
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
}

#[test]
fn source_to_program_json_v0_rejects_multi_local_after_type_annotation() {
    let source = r#"
static box Main {
  main() {
local ids: Array<PageId>, other
return 0
  }
}
"#;

    let error = source_to_program_json_v0_strict(source)
        .expect_err("typed local with comma should fail fast");
    assert!(error.contains("single local binding after a type annotation"));
}
