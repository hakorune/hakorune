use crate::mir::MirModule;
use crate::runner;
use std::collections::BTreeMap;

pub(super) fn lower_program_json_to_module(
    program_json: &str,
    imports: BTreeMap<String, String>,
) -> Result<MirModule, String> {
    match runner::json_v0_bridge::parse_json_v0_to_module_with_imports(program_json, imports) {
        Ok(module) => Ok(module),
        Err(error) => Err(super::super::failfast_error(error)),
    }
}
