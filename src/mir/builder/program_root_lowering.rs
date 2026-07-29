//! Shared Program-only root/catalog lowering.
//!
//! Selected normal/default reaches this owner through its sealed Program
//! product. No arbitrary-AST root facade participates in this lifecycle.

use std::collections::HashMap;

use crate::ast::ASTNode;
use hakorune_mir_builder::BoxCompilationContext;

use super::callable_declaration_catalog::VerifiedSameModuleCallableDeclarationCatalogV1;
use super::declaration_order::sorted_method_entries;
use super::instance_box_constructor_batch::PreparedInstanceBoxConstructorBatchV1;
use super::main_expansion::VerifiedRawRootExpansionV1;
use super::module_draft_collector::ModuleDraftCollectorV1;
use super::module_lifecycle::RootCallableCapturePortV1;
use super::module_lowering_invocation::ModuleLoweringPortV1;
use super::nonmain_static_box_method_batch::PreparedNonMainStaticBoxMethodBatchV1;
use super::normal_default_root_catalog_lifecycle::{
    NormalDefaultRootCatalogLifecycleErrorV1, PreparedNormalDefaultProgramRootV1,
};
use super::recursive_child_lowering::RawInvocationChildPortV1;
use super::{declaration_indexer, MirBuilder, SameModuleCallableNamespaceV1, ValueId};

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
        let drafts = collector.into_draft_functions();
        self.current_module
            .as_mut()
            .ok_or_else(|| "[freeze:contract][mir/callable-collector/module-missing]".to_owned())?
            .try_add_functions_atomic(drafts)
            .map_err(|error| {
                format!("[freeze:contract][mir/callable-collector/atomic-commit] {error}")
            })?;
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
        declaration_indexer::index_declarations(self, snapshot);
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
        self.lower_program_statements_with_callable_port_v1(statements, expansion, callables)
    }

    fn lower_program_statements_with_callable_port_v1<Port>(
        &mut self,
        statements: Vec<ASTNode>,
        expansion: &VerifiedRawRootExpansionV1<'_>,
        callables: &mut Port,
    ) -> Result<ValueId, String>
    where
        Port: RootCallableCapturePortV1,
    {
        use crate::ast::ASTNode as N;

        let is_app_mode = expansion.is_app_mode();
        let mut deferred_static_boxes: Vec<(String, HashMap<String, ASTNode>)> = Vec::new();
        for statement in &statements {
            if let N::BoxDeclaration {
                name,
                methods,
                is_static,
                fields,
                field_decls,
                constructors,
                init_fields,
                weak_fields,
                ..
            } = statement
            {
                if *is_static {
                    if name != "Main" && is_app_mode {
                        deferred_static_boxes.push((name.clone(), methods.clone()));
                    }
                } else {
                    self.comp_ctx.register_user_box_declared_fields(
                        name.clone(),
                        fields,
                        field_decls,
                        init_fields,
                        weak_fields,
                    );
                    self.build_box_declaration(
                        name.clone(),
                        methods.clone(),
                        fields.clone(),
                        weak_fields.clone(),
                    )?;
                    PreparedInstanceBoxConstructorBatchV1::prepare(name, constructors)
                        .lower_with_port_v1(self, callables)?;
                    for (method_name, method_ast) in sorted_method_entries(methods) {
                        if let N::FunctionDeclaration {
                            params,
                            param_decls,
                            return_type_name,
                            body,
                            is_static,
                            uses,
                            attrs,
                            ..
                        } = method_ast
                        {
                            if !*is_static {
                                let canonical_key = self
                                    .comp_ctx
                                    .callable_declaration_catalog()
                                    .map_err(|error| error.to_string())?
                                    .declaration_for(
                                        SameModuleCallableNamespaceV1::InstanceBoxMethod,
                                        name,
                                        method_name,
                                        params.len(),
                                    )
                                    .ok_or_else(|| {
                                        format!(
                                            "[freeze:contract][mir/instance-capture/catalog] \
                                             missing exact declaration for {name}.{method_name}/{}",
                                            params.len()
                                        )
                                    })?
                                    .key()
                                    .clone();
                                let function_name =
                                    format!("{}.{}/{}", name, method_name, params.len());
                                callables.lower_root_instance_method(
                                    self,
                                    canonical_key,
                                    name.clone(),
                                    method_name.to_owned(),
                                    function_name,
                                    params.clone(),
                                    param_decls.clone(),
                                    return_type_name.clone(),
                                    body.clone(),
                                    uses.clone(),
                                    attrs.clone(),
                                )?;
                            }
                        }
                    }
                }
            } else if let N::FunctionDeclaration {
                name,
                params,
                param_decls,
                return_type_name,
                body,
                uses,
                attrs,
                ..
            } = statement
            {
                callables.lower_static_box_method(
                    self,
                    format!("{}/{}", name, params.len()),
                    params.clone(),
                    param_decls.clone(),
                    return_type_name.clone(),
                    body.clone(),
                    uses.clone(),
                    attrs.clone(),
                )?;
            }
        }

        let runtime_statements: Vec<N> = statements
            .into_iter()
            .filter(|statement| !matches!(statement, N::FunctionDeclaration { .. }))
            .collect();
        for (name, methods) in deferred_static_boxes {
            ProgramDeferredStaticBoxLifecycleV1::new(name, methods)
                .lower_with_port_v1(self, callables)?;
        }

        match expansion {
            VerifiedRawRootExpansionV1::Script => callables.lower_body(self, runtime_statements),
            VerifiedRawRootExpansionV1::App(main) => self
                .build_verified_static_main_box_with_port_v1(callables, main)
                .map_err(|error| error.to_string()),
        }
    }
}
