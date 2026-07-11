use crate::mir::MirModule;

pub(crate) const BACKEND_UNSUPPORTED_TAG: &str = "[type/weak_field_contract_backend_unsupported]";

pub(crate) fn enforce_weak_field_backend_supported(
    module: &MirModule,
    backend: &str,
) -> Result<(), String> {
    let rows = module.metadata.weak_field_contract_specs.len();
    if rows == 0 || backend == "mir-interpreter" {
        return Ok(());
    }
    Err(format!(
        "{} backend={} contract_rows={} require={}",
        BACKEND_UNSUPPORTED_TAG,
        backend,
        rows,
        crate::mir::type_contracts::weak_field::WEAK_FIELD_CAPABILITY
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::UserBoxFieldDecl;

    #[test]
    fn only_reference_vm_supports_weak_field_guard() {
        let mut module = MirModule::new("weak-backend".to_string());
        module.metadata.user_box_field_decls.insert(
            "Node".to_string(),
            vec![UserBoxFieldDecl {
                name: "parent".to_string(),
                declared_type_name: None,
                is_weak: true,
            }],
        );
        assert!(
            crate::mir::backend_capability::enforce_mir_backend_supported(
                &module,
                "mir-interpreter"
            )
            .is_ok()
        );
        let error =
            crate::mir::backend_capability::enforce_mir_backend_supported(&module, "ny-llvmc-exe")
                .unwrap_err();
        assert!(error.contains(BACKEND_UNSUPPORTED_TAG), "{error}");
    }
}
