use crate::mir::MirModule;

pub(crate) const STATIC_TABLE_BACKEND_UNSUPPORTED_TAG: &str =
    "[type/static_table_contract_backend_unsupported]";

pub(crate) fn enforce_static_table_backend_supported(
    module: &MirModule,
    backend: &str,
) -> Result<(), String> {
    let rows = module.metadata.verified_static_table_contracts.len();
    if rows == 0 || backend == "mir-interpreter" {
        return Ok(());
    }
    Err(format!(
        "{} backend={} contract_rows={} require=static_table_u16_readonly_v1",
        STATIC_TABLE_BACKEND_UNSUPPORTED_TAG, backend, rows
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::function::{StaticElementType, StaticTableContractSpec, StaticTableId};

    fn module_with_table() -> MirModule {
        let mut module = MirModule::new("static-table-backend".to_string());
        let spec = StaticTableContractSpec {
            table_id: StaticTableId {
                module_name: module.name.clone(),
                declaration_name: "DATA".to_string(),
            },
            diagnostic_name: "DATA".to_string(),
            element: StaticElementType::U16,
            values: vec![1, 2],
        };
        module.metadata.static_data_plans =
            crate::mir::static_data_plan::static_data_plans_from_specs(std::slice::from_ref(&spec));
        module.metadata.static_table_contract_specs.push(spec);
        module
    }

    #[test]
    fn only_reference_interpreter_supports_static_table_contract() {
        let module = module_with_table();
        assert!(
            crate::mir::backend_capability::enforce_mir_backend_supported(
                &module,
                "mir-interpreter"
            )
            .is_ok()
        );
        let error = crate::mir::backend_capability::enforce_mir_backend_supported(&module, "wasm")
            .unwrap_err();
        assert!(error.contains(STATIC_TABLE_BACKEND_UNSUPPORTED_TAG));
    }
}
