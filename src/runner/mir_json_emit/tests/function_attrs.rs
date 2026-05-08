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

#[test]
fn build_mir_json_root_emits_inline_plans_from_hint_runes() {
    let mut module = MirModule::new("test".to_string());
    let mut function = make_function("Main.align_up/2", false);
    function.metadata.runes = vec![RuneAttr {
        name: "Hint".to_string(),
        args: vec!["inline".to_string()],
    }];
    crate::mir::rune_plan_refresh::refresh_function_rune_plans(&mut function);
    module
        .functions
        .insert("Main.align_up/2".to_string(), function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let plans = root["functions"][0]["metadata"]["inline_plans"]
        .as_array()
        .expect("metadata.inline_plans array");
    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0]["function"], "Main.align_up/2");
    assert_eq!(plans[0]["request"], "prefer");
    assert_eq!(plans[0]["hotness"], serde_json::Value::Null);
    assert_eq!(plans[0]["max_ir"], serde_json::Value::Null);
    assert_eq!(plans[0]["requires"], serde_json::json!([]));
    assert_eq!(plans[0]["verified"], false);
    assert_eq!(plans[0]["fallback"], "keep_call");
    assert_eq!(plans[0]["source"], "rune_hint");
}

#[test]
fn build_mir_json_root_emits_effect_and_capability_plans() {
    let mut module = MirModule::new("test".to_string());
    let mut function = make_function("Main.fast/0", false);
    function.metadata.runes = vec![
        RuneAttr {
            name: "Contract".to_string(),
            args: vec!["no_alloc".to_string()],
        },
        RuneAttr {
            name: "Contract".to_string(),
            args: vec!["no_safepoint".to_string()],
        },
    ];
    crate::mir::rune_plan_refresh::refresh_function_rune_plans(&mut function);
    module.functions.insert("Main.fast/0".to_string(), function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let metadata = &root["functions"][0]["metadata"];
    let effect_plans = metadata["effect_plans"]
        .as_array()
        .expect("metadata.effect_plans array");
    assert_eq!(effect_plans.len(), 1);
    assert_eq!(effect_plans[0]["function"], "Main.fast/0");
    assert_eq!(
        effect_plans[0]["requires"],
        serde_json::json!(["no_alloc", "no_safepoint"])
    );
    assert_eq!(effect_plans[0]["verified"], false);
    assert_eq!(effect_plans[0]["source"], "rune_contract");

    assert_eq!(metadata["capability_plans"], serde_json::json!([]));
}
