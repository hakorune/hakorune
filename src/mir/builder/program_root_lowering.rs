//! Shared Program-only root/catalog lowering.
//!
//! Selected normal/default reaches this owner through its sealed Program
//! product. No arbitrary-AST root facade participates in this lifecycle.

use std::collections::HashMap;

use crate::ast::ASTNode;
use hakorune_mir_builder::BoxCompilationContext;

use super::callable_declaration_catalog::VerifiedSameModuleCallableDeclarationCatalogV1;
use super::main_expansion::VerifiedRawRootExpansionV1;
use super::module_draft_collector::ModuleDraftCollectorV1;
use super::module_lifecycle::RootCallableCapturePortV1;
use super::module_lowering_invocation::ModuleLoweringPortV1;
use super::nonmain_static_box_method_batch::PreparedNonMainStaticBoxMethodBatchV1;
use super::normal_default_root_catalog_lifecycle::{
    NormalDefaultRootCatalogLifecycleErrorV1, PreparedNormalDefaultProgramRootV1,
};
use super::program_declaration_facts::PreparedNormalProgramDeclarationFactsV1;
use super::program_root_work_plan::{PreparedProgramRootWorkPlanV1, ProgramRootTerminalScheduleV1};
use super::recursive_child_lowering::RawInvocationChildPortV1;
use super::{MirBuilder, ValueId};

pub(super) struct ProgramDeferredStaticBoxLifecycleV1 {
    methods: PreparedNonMainStaticBoxMethodBatchV1,
}

impl ProgramDeferredStaticBoxLifecycleV1 {
    pub(super) fn new(name: String, methods: HashMap<String, ASTNode>) -> Self {
        Self {
            methods: PreparedNonMainStaticBoxMethodBatchV1::prepare(name, methods),
        }
    }

    pub(super) fn lower_with_port_v1<Port>(
        self,
        builder: &mut MirBuilder,
        callables: &mut Port,
    ) -> Result<(), String>
    where
        Port: RootCallableCapturePortV1,
    {
        builder.trace_compile(format!("lower static box {}", self.methods.owner()));
        builder.comp_ctx.compilation_context = Some(BoxCompilationContext::new());
        self.methods.lower_with_port_v1(builder, callables)?;
        builder.comp_ctx.compilation_context = None;
        Ok(())
    }
}

impl MirBuilder {
    pub(in crate::mir::builder) fn lower_normal_default_program_root_catalog_v1(
        &mut self,
        source: &PreparedNormalDefaultProgramRootV1,
        expansion: &VerifiedRawRootExpansionV1<'_>,
    ) -> Result<ValueId, NormalDefaultRootCatalogLifecycleErrorV1> {
        let lowering_statements = source.clone_lowering_statements();
        let catalog =
            VerifiedSameModuleCallableDeclarationCatalogV1::seal_root(source.source_ast())
                .map_err(|error| {
                    NormalDefaultRootCatalogLifecycleErrorV1::CatalogSeal(
                        format!("[mir/callable-catalog/seal] {error:?}").into(),
                    )
                })?;
        self.comp_ctx
            .install_callable_declaration_catalog(catalog)
            .map_err(|error| {
                NormalDefaultRootCatalogLifecycleErrorV1::CatalogInstall(error.to_string().into())
            })?;
        self.lower_program_root_after_catalog_install_v1(
            lowering_statements,
            source.source_ast(),
            expansion,
        )
        .map_err(|error| NormalDefaultRootCatalogLifecycleErrorV1::RootLower(error.into()))
    }

    fn lower_program_root_after_catalog_install_v1(
        &mut self,
        statements: Vec<ASTNode>,
        snapshot: &ASTNode,
        expansion: &VerifiedRawRootExpansionV1<'_>,
    ) -> Result<ValueId, String> {
        let mut collector = ModuleDraftCollectorV1::default();
        let result = {
            let mut module_port = ModuleLoweringPortV1::from_collector(&mut collector);
            let mut port = RawInvocationChildPortV1::new(&mut module_port);
            self.lower_program_root_with_callable_port_v1(
                statements, snapshot, expansion, &mut port,
            )
        }?;
        let target = self
            .current_module
            .as_mut()
            .ok_or_else(|| "[freeze:contract][mir/callable-collector/module-missing]".to_owned())?;
        let prepared = collector
            .prepare_normal_legacy_drain(target)
            .map_err(|rejected| {
                let error = rejected.error().to_string();
                rejected.discard();
                format!("[freeze:contract][mir/callable-collector/atomic-commit] {error}")
            })?;
        prepared.commit();
        Ok(result)
    }

    pub(in crate::mir::builder) fn lower_program_root_with_callable_port_v1<Port>(
        &mut self,
        statements: Vec<ASTNode>,
        snapshot: &ASTNode,
        expansion: &VerifiedRawRootExpansionV1<'_>,
        callables: &mut Port,
    ) -> Result<ValueId, String>
    where
        Port: RootCallableCapturePortV1,
    {
        PreparedNormalProgramDeclarationFactsV1::collect(snapshot).install_into(&mut self.comp_ctx);
        if let Some(module) = self.current_module.as_mut() {
            let specs = crate::mir::static_data_plan::collect_static_table_specs_from_ast(
                &module.name,
                snapshot,
            )?;
            let plans = crate::mir::static_data_plan::static_data_plans_from_specs(&specs);
            module.metadata.static_table_contract_specs = specs;
            module.metadata.static_data_plans = plans;
        }

        let is_app_mode = expansion.is_app_mode();
        self.root_is_app_mode = Some(is_app_mode);
        let work = PreparedProgramRootWorkPlanV1::prepare(statements, is_app_mode);
        self.lower_program_root_work_plan_with_callable_port_v1(work, expansion, callables)
    }

    fn lower_program_root_work_plan_with_callable_port_v1<Port>(
        &mut self,
        work: PreparedProgramRootWorkPlanV1,
        expansion: &VerifiedRawRootExpansionV1<'_>,
        callables: &mut Port,
    ) -> Result<ValueId, String>
    where
        Port: RootCallableCapturePortV1,
    {
        let work = work.into_parts();
        for immediate in work.immediate {
            immediate.lower_with_port_v1(self, callables)?;
        }
        for deferred in work.deferred_static {
            let (name, methods) = deferred.into_parts();
            ProgramDeferredStaticBoxLifecycleV1::new(name, methods)
                .lower_with_port_v1(self, callables)?;
        }

        match (work.terminal, expansion) {
            (ProgramRootTerminalScheduleV1::ScriptRuntime, VerifiedRawRootExpansionV1::Script) => {
                callables.lower_body(self, work.runtime_statements)
            }
            (
                ProgramRootTerminalScheduleV1::VerifiedAppMain,
                VerifiedRawRootExpansionV1::App(main),
            ) => self
                .build_verified_static_main_box_with_port_v1(callables, main)
                .map_err(|error| error.to_string()),
            _ => Err("[freeze:contract][mir/program-root-work-plan/terminal-drift]".to_owned()),
        }
    }
}
