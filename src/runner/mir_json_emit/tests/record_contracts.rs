use super::super::build_mir_json_root;
use crate::mir::MirCompiler;
use crate::parser::NyashParser;

#[test]
fn mir_json_exports_fresh_record_carrier_and_operations() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = r#"
record Point {
  x: i64
  y: i64 = 2
}

static box Main {
  main() {
    local point = Point { x: 1 }
    return point.x
  }
}
"#;
    let ast = NyashParser::parse_from_string(source).expect("parse record JSON fixture");
    let mut compiler = MirCompiler::with_options(false);
    let module = compiler
        .compile(ast)
        .expect("compile record JSON fixture")
        .module;
    let root = build_mir_json_root(&module).expect("record MIR JSON");
    let function = root["functions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|function| function["name"].as_str() == Some("main"))
        .expect("main function");
    let contracts = function["metadata"]["record_value_contracts"]
        .as_array()
        .expect("record carrier array");
    assert_eq!(contracts.len(), 1);
    assert_eq!(contracts[0]["boundary"], "construct");
    assert_eq!(contracts[0]["fields"].as_array().unwrap().len(), 2);
    let ops = function["blocks"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|block| block["instructions"].as_array().unwrap())
        .filter_map(|instruction| instruction["op"].as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        ops.iter()
            .filter(|op| **op == "record_field_contract_check")
            .count(),
        2
    );
    assert!(ops.contains(&"record_value_publish"));
    assert_eq!(
        root["record_decls"][0]["field_decls"][1]["has_default"],
        true
    );
}
