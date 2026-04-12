use super::super::build_mir_json_root;
use super::make_function;
use crate::ast::RuneAttr;
use crate::mir::MirModule;

#[test]
fn build_mir_json_root_emits_function_runes_as_attrs() {
    let mut module = MirModule::new("test".to_string());
    let mut function = make_function("main", true);
    function.metadata.runes = vec![
        RuneAttr {
            name: "Symbol".to_string(),
            args: vec!["main_sym".to_string()],
        },
        RuneAttr {
            name: "CallConv".to_string(),
            args: vec!["c".to_string()],
        },
    ];
    module.functions.insert("main".to_string(), function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let runes = root["functions"][0]["attrs"]["runes"]
        .as_array()
        .expect("attrs.runes array");
    assert_eq!(runes.len(), 2);
    assert_eq!(runes[0]["name"], "Symbol");
    assert_eq!(runes[0]["args"], serde_json::json!(["main_sym"]));
    assert_eq!(runes[1]["name"], "CallConv");
    assert_eq!(runes[1]["args"], serde_json::json!(["c"]));
}
