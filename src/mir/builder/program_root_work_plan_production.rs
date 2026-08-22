//! Production work-plan transport for the selected normal root.
//!
//! The source window is issued before this module.  This module only carries
//! that existing admission into the work plan and performs the old root
//! statement classification; it does not issue Script semantic rows.

use super::super::normal_script_program_item_admission::classify_normal_script_program_item_v1;
use super::super::normal_script_root_demand_window::PreparedScriptRootAdmissionV1;
use super::{
    classify_statement, collect_constructor_demand_expectations,
    issue_manifest_for_disposition,
    PreparedProgramRootImmediateWorkV1, PreparedProgramRootRuntimeStatementV1,
    PreparedProgramRootRuntimeWorkV1, PreparedProgramRootWorkPlanSealV1,
    PreparedProgramRootWorkPlanV1, ProgramRootStatementDispositionV1,
    ProgramRootTerminalScheduleV1, ProgramRootWorkPlanAdmissionV1,
};
use super::super::callable_declaration_catalog::VerifiedSelectedNormalCallableSourceInventoryV1;
use super::super::normal_instance_constructor_admission::{
    InstanceConstructorDemandManifestBuilderV1, VerifiedInstanceConstructorPhysicalSourceCohortV1,
};
use crate::ast::ASTNode;

impl PreparedProgramRootWorkPlanV1 {
    pub(in crate::mir::builder) fn prepare_raw_compatibility(
        statements: Vec<ASTNode>,
        is_app_mode: bool,
    ) -> Self {
        Self::prepare_with_script_root_admission_and_constructor_sources(
            statements,
            is_app_mode,
            ProgramRootWorkPlanAdmissionV1::RawCompatibility,
            None,
            None,
            None,
        )
        .expect("raw compatibility work plan")
    }

    pub(in crate::mir::builder) fn prepare_with_script_root_admission_and_constructor_sources(
        statements: Vec<ASTNode>,
        is_app_mode: bool,
        work_plan_admission: ProgramRootWorkPlanAdmissionV1,
        selected_callable_sources: Option<&VerifiedSelectedNormalCallableSourceInventoryV1>,
        constructor_source_cohort: Option<&VerifiedInstanceConstructorPhysicalSourceCohortV1>,
        script_root_admission: Option<PreparedScriptRootAdmissionV1>,
    ) -> Result<Self, String> {
        assert_eq!(
            selected_callable_sources.is_some(),
            work_plan_admission == ProgramRootWorkPlanAdmissionV1::SelectedNormal,
            "selected callable inventory must match work-plan admission",
        );
        match (
            work_plan_admission,
            is_app_mode,
            script_root_admission.is_some(),
        ) {
            (ProgramRootWorkPlanAdmissionV1::RawCompatibility, _, false)
            | (ProgramRootWorkPlanAdmissionV1::SelectedNormal, true, false)
            | (ProgramRootWorkPlanAdmissionV1::SelectedNormal, false, true) => {}
            _ => {
                return Err(
                    "[freeze:contract][mir/script-neutral-window/work-plan-edge]".to_owned(),
                )
            }
        }
        if work_plan_admission == ProgramRootWorkPlanAdmissionV1::SelectedNormal {
            let cohort = constructor_source_cohort.ok_or_else(|| {
                "[freeze:contract][mir/instance-constructor-source/cohort-missing]".to_owned()
            })?;
            cohort.validate_program(&statements)?;
        }
        let mut immediate = Vec::new();
        let mut deferred_static = Vec::new();
        let mut runtime_statements = Vec::new();
        let mut demand_manifest = InstanceConstructorDemandManifestBuilderV1::default();
        for (statement_index, statement) in statements.into_iter().enumerate() {
            let normal_script_kind = (work_plan_admission
                == ProgramRootWorkPlanAdmissionV1::SelectedNormal)
                .then(|| classify_normal_script_program_item_v1(&statement));
            let disposition = classify_statement(
                statement,
                is_app_mode,
                statement_index,
                work_plan_admission,
                normal_script_kind,
                selected_callable_sources,
                constructor_source_cohort,
            );
            if work_plan_admission == ProgramRootWorkPlanAdmissionV1::SelectedNormal {
                issue_manifest_for_disposition(&mut demand_manifest, &disposition)?;
            }
            match disposition {
                ProgramRootStatementDispositionV1::ImmediateAndRuntime { work, runtime } => {
                    immediate.push(work);
                    runtime_statements.push(runtime);
                }
                ProgramRootStatementDispositionV1::ImmediateOnly(work) => {
                    immediate.push(work);
                }
                ProgramRootStatementDispositionV1::DeferredAndRuntime { work, runtime } => {
                    runtime_statements.push(runtime);
                    deferred_static.push(work);
                }
                ProgramRootStatementDispositionV1::RuntimeOnly(statement) => {
                    runtime_statements.push(statement)
                }
            }
        }
        let runtime = PreparedProgramRootRuntimeWorkV1::prepare(runtime_statements, work_plan_admission);
        let constructor_demand_manifest = match work_plan_admission {
            ProgramRootWorkPlanAdmissionV1::RawCompatibility => None,
            ProgramRootWorkPlanAdmissionV1::SelectedNormal => Some(demand_manifest.finish()),
        };
        let actual_tickets = collect_constructor_demand_expectations(&immediate, &runtime);
        if let Some(manifest) = constructor_demand_manifest.as_ref() {
            manifest.validate_exact(&actual_tickets)?;
        }
        Ok(PreparedProgramRootWorkPlanV1 {
            immediate: immediate.into_boxed_slice(),
            deferred_static: deferred_static.into_boxed_slice(),
            runtime,
            terminal: if is_app_mode {
                ProgramRootTerminalScheduleV1::VerifiedAppMain
            } else {
                ProgramRootTerminalScheduleV1::ScriptRuntime
            },
            script_root_admission,
            constructor_demand_manifest,
            _seal: PreparedProgramRootWorkPlanSealV1,
        })
    }
}
