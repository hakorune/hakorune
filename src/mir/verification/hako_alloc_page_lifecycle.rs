use crate::mir::function::{TypedObjectFieldStorage, TypedObjectPlan};
use crate::mir::verification_types::VerificationError;
use crate::mir::MirModule;

const REPORT_BOX_NAME: &str = "HakoAllocPageLifecycleInvariantReport";
const OBSERVER_REPORT_FUNCTION: &str = "HakoAllocPageLifecycleInvariantObserver.report/17";
const OBSERVER_OBSERVE_FUNCTION: &str = "HakoAllocPageLifecycleInvariantObserver.observeHeapPage/3";
const DECOMMIT_ATTEMPT_FUNCTION: &str = "HakoAllocPurgeStateAwareDecommitGuard.attemptHeapPage/2";
const RECOMMIT_ATTEMPT_FUNCTION: &str = "HakoAllocRecommitHeapIntegration.attemptHeapPage/3";
const PAGE_ACQUIRE_FUNCTION: &str = "HakoAllocPageModel.acquire/1";
const PAGE_RELEASE_LOCAL_FUNCTION: &str = "HakoAllocPageModel.releaseLocal/1";

const REQUIRED_FUNCTIONS: &[&str] = &[
    OBSERVER_REPORT_FUNCTION,
    OBSERVER_OBSERVE_FUNCTION,
    DECOMMIT_ATTEMPT_FUNCTION,
    RECOMMIT_ATTEMPT_FUNCTION,
    PAGE_ACQUIRE_FUNCTION,
    PAGE_RELEASE_LOCAL_FUNCTION,
];

const REQUIRED_REPORT_FIELDS: &[&str] = &[
    "state",
    "active",
    "retired",
    "decommitted",
    "recommitted",
    "acquire_allowed",
    "decommit_candidate",
    "recommit_required",
    "duplicate_decommit_blocked",
    "marked_generations",
    "recommitted_generations",
];

pub(super) fn check_hako_alloc_page_lifecycle_invariants(
    module: &MirModule,
) -> Result<(), Vec<VerificationError>> {
    if !has_lifecycle_surface(module) {
        return Ok(());
    }

    let mut errors = Vec::new();
    check_required_functions(module, &mut errors);
    check_report_plan(module, &mut errors);

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn has_lifecycle_surface(module: &MirModule) -> bool {
    module
        .metadata
        .typed_object_plans
        .iter()
        .any(|plan| plan.box_name == REPORT_BOX_NAME)
        || module.functions.contains_key(OBSERVER_REPORT_FUNCTION)
        || module.functions.contains_key(OBSERVER_OBSERVE_FUNCTION)
}

fn check_required_functions(module: &MirModule, errors: &mut Vec<VerificationError>) {
    for function_name in REQUIRED_FUNCTIONS {
        if !module.functions.contains_key(*function_name) {
            push_error(
                errors,
                function_name,
                &format!("missing required lifecycle function `{}`", function_name),
            );
        }
    }
}

fn check_report_plan(module: &MirModule, errors: &mut Vec<VerificationError>) {
    let Some(plan) = module
        .metadata
        .typed_object_plans
        .iter()
        .find(|plan| plan.box_name == REPORT_BOX_NAME)
    else {
        push_error(
            errors,
            REPORT_BOX_NAME,
            "missing lifecycle typed object plan",
        );
        return;
    };

    for field_name in REQUIRED_REPORT_FIELDS {
        check_report_field(plan, field_name, errors);
    }
}

fn check_report_field(
    plan: &TypedObjectPlan,
    field_name: &str,
    errors: &mut Vec<VerificationError>,
) {
    let Some(field) = plan.fields.iter().find(|field| field.name == field_name) else {
        push_error(
            errors,
            &plan.box_name,
            &format!("missing lifecycle report field `{}`", field_name),
        );
        return;
    };

    if field.declared_type_name.as_deref() != Some("i64") {
        push_error(
            errors,
            &plan.box_name,
            &format!("lifecycle report field `{}` must declare `i64`", field_name),
        );
    }
    if field.storage != TypedObjectFieldStorage::I64 {
        push_error(
            errors,
            &plan.box_name,
            &format!(
                "lifecycle report field `{}` must use i64 storage",
                field_name
            ),
        );
    }
    if field.is_weak {
        push_error(
            errors,
            &plan.box_name,
            &format!("lifecycle report field `{}` must stay strong", field_name),
        );
    }
}

fn push_error(errors: &mut Vec<VerificationError>, owner: &str, reason: &str) {
    errors.push(
        VerificationError::HakoAllocPageLifecycleInvariantViolation {
            owner: owner.to_string(),
            reason: reason.to_string(),
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::function::{FunctionSignature, TypedObjectFieldPlan};
    use crate::mir::{BasicBlockId, EffectMask, MirFunction, MirType};

    fn report_field(name: &str, slot: u32) -> TypedObjectFieldPlan {
        TypedObjectFieldPlan {
            name: name.to_string(),
            slot,
            declared_type_name: Some("i64".to_string()),
            storage: TypedObjectFieldStorage::I64,
            is_weak: false,
        }
    }

    fn report_plan() -> TypedObjectPlan {
        TypedObjectPlan {
            box_name: REPORT_BOX_NAME.to_string(),
            type_id: 1,
            layout_kind: "runtime_slot_object_v0".to_string(),
            field_count: REQUIRED_REPORT_FIELDS.len() as u32,
            fields: REQUIRED_REPORT_FIELDS
                .iter()
                .enumerate()
                .map(|(slot, name)| report_field(name, slot as u32))
                .collect(),
        }
    }

    fn add_function(module: &mut MirModule, name: &str, arity: usize) {
        module.add_function(MirFunction::new(
            FunctionSignature {
                name: name.to_string(),
                params: vec![MirType::Integer; arity],
                return_type: MirType::Integer,
                effects: EffectMask::new(),
            },
            BasicBlockId::new(0),
        ));
    }

    fn valid_module() -> MirModule {
        let mut module = MirModule::new("hako-alloc-page-lifecycle-test".to_string());
        module.metadata.typed_object_plans.push(report_plan());
        add_function(&mut module, OBSERVER_REPORT_FUNCTION, 17);
        add_function(&mut module, OBSERVER_OBSERVE_FUNCTION, 3);
        add_function(&mut module, DECOMMIT_ATTEMPT_FUNCTION, 2);
        add_function(&mut module, RECOMMIT_ATTEMPT_FUNCTION, 3);
        add_function(&mut module, PAGE_ACQUIRE_FUNCTION, 1);
        add_function(&mut module, PAGE_RELEASE_LOCAL_FUNCTION, 1);
        module
    }

    fn first_reason(module: &MirModule) -> String {
        let errors = check_hako_alloc_page_lifecycle_invariants(module).unwrap_err();
        match &errors[0] {
            VerificationError::HakoAllocPageLifecycleInvariantViolation { reason, .. } => {
                reason.clone()
            }
            other => panic!("unexpected verifier error: {:?}", other),
        }
    }

    #[test]
    fn verifier_ignores_modules_without_page_lifecycle_surface() {
        let module = MirModule::new("unrelated".to_string());
        assert!(check_hako_alloc_page_lifecycle_invariants(&module).is_ok());
    }

    #[test]
    fn verifier_accepts_valid_page_lifecycle_surface() {
        let module = valid_module();
        assert!(check_hako_alloc_page_lifecycle_invariants(&module).is_ok());
    }

    #[test]
    fn verifier_rejects_missing_required_function() {
        let mut module = valid_module();
        module.functions.remove(OBSERVER_OBSERVE_FUNCTION);

        let reason = first_reason(&module);
        assert!(reason.contains("missing required lifecycle function"));
    }

    #[test]
    fn verifier_rejects_missing_report_plan() {
        let mut module = valid_module();
        module.metadata.typed_object_plans.clear();

        let reason = first_reason(&module);
        assert!(reason.contains("missing lifecycle typed object plan"));
    }

    #[test]
    fn verifier_rejects_bad_report_field_shape() {
        let mut module = valid_module();
        let report = module
            .metadata
            .typed_object_plans
            .iter_mut()
            .find(|plan| plan.box_name == REPORT_BOX_NAME)
            .expect("report plan");
        let field = report
            .fields
            .iter_mut()
            .find(|field| field.name == "recommit_required")
            .expect("recommit_required field");
        field.declared_type_name = Some("IntegerBox".to_string());

        let reason = first_reason(&module);
        assert!(reason.contains("recommit_required"));
        assert!(reason.contains("must declare `i64`"));
    }
}
