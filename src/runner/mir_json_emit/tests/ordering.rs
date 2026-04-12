use super::super::ordered_harness_functions;
use super::make_function;
use crate::mir::MirModule;

#[test]
fn ordered_harness_functions_puts_entry_main_first() {
    let mut module = MirModule::new("test".to_string());
    module.functions.insert(
        "Main.equals/1".to_string(),
        make_function("Main.equals/1", false),
    );
    module.functions.insert(
        "condition_fn".to_string(),
        make_function("condition_fn", false),
    );
    module
        .functions
        .insert("main".to_string(), make_function("main", true));

    let ordered: Vec<_> = ordered_harness_functions(&module)
        .into_iter()
        .map(|(name, _)| name.as_str())
        .collect();

    assert_eq!(ordered[0], "main");
    assert_eq!(ordered[1], "Main.equals/1");
    assert_eq!(ordered[2], "condition_fn");
}
