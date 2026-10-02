//! Existing source-state construction; ownership and entry selection stay unchanged.
use super::*;

impl CallableSemanticLoweringState {
    pub(in crate::mir::builder) fn from_exact_source(
        input: ResolvedFunctionLoweringInputV1<'_>,
    ) -> Result<Self, String> {
        Self::from_exact_source_with_dynamic_source(input, None)
    }

    pub(in crate::mir::builder) fn from_exact_source_with_dynamic_source(
        input: ResolvedFunctionLoweringInputV1<'_>,
        dynamic_source: Option<
            Rc<super::super::normal_callable_dynamic_source::VerifiedSourceBackedDynamicCallableV1>,
        >,
    ) -> Result<Self, String> {
        Self::from_exact_source_with_dynamic_source_and_core_methods(
            input,
            dynamic_source,
            BTreeMap::new(),
        )
    }

    pub(in crate::mir::builder) fn from_exact_source_with_dynamic_source_and_core_methods(
        input: ResolvedFunctionLoweringInputV1<'_>,
        dynamic_source: Option<
            Rc<super::super::normal_callable_dynamic_source::VerifiedSourceBackedDynamicCallableV1>,
        >,
        source_core_method_calls: BTreeMap<
            crate::mir::resolved_semantics::SourceExprSiteV1,
            SelectedSourceCoreMethodCallV1,
        >,
    ) -> Result<Self, String> {
        Self::from_exact_source_with_dynamic_source_and_core_methods_and_loop_break_source(
            input,
            dynamic_source,
            source_core_method_calls,
            None,
        )
    }

    pub(in crate::mir::builder) fn from_exact_source_with_dynamic_source_and_core_methods_and_loop_break_source(
        input: ResolvedFunctionLoweringInputV1<'_>,
        dynamic_source: Option<
            Rc<super::super::normal_callable_dynamic_source::VerifiedSourceBackedDynamicCallableV1>,
        >,
        source_core_method_calls: BTreeMap<
            crate::mir::resolved_semantics::SourceExprSiteV1,
            SelectedSourceCoreMethodCallV1,
        >,
        loop_break_source: Option<LoopBreakSourcePackageLoanV1>,
    ) -> Result<Self, String> {
        let source_loop_bridge = source_loop_bridge::CallableLoopSourceBridgeV1::from_input(input)?;
        let dynamic_origins = match dynamic_source {
            Some(source) => CallableDynamicOriginLoweringStateV1::from_shared_source(source),
            None => {
                let source = SourceBackedDynamicCallableIssuerV1::issue_from_resolved_input(input)
                    .map_err(|error| {
                        format!("[freeze:contract][callable-dynamic-source] {error:?}")
                    })?;
                CallableDynamicOriginLoweringStateV1::from_source(source)
            }
        }
        .map_err(|error| error.to_string())?;
        let forest = input.forest();
        let [root] = forest.roots() else {
            return Err(freeze("root-cardinality"));
        };
        let owner = forest.owner(*root).ok_or_else(|| freeze("root-owner"))?;
        let owner_id = owner.owner();
        let brand_constructors = super::super::brand_constructor_lowering_projection::BrandConstructorLoweringProjectionV1::from_verified_owner(
            owner_id,
            input.function().expression_sites(),
            input.function().brand_call_relations(),
        )
        .map_err(|error| format!("[freeze:contract][callable-brand-projection] {error:?}"))?;
        if dynamic_origins.owner() != owner_id {
            return Err(freeze("dynamic-origin-owner"));
        }
        let mut receiver = None;
        let mut parameters = BTreeMap::new();
        let mut locals = BTreeMap::<_, BTreeMap<_, _>>::new();
        let mut declared = BTreeSet::new();
        let mut binding_names = BTreeMap::new();

        for site in owner.declaration_sites() {
            let Some(binding) = owner.declaration_binding(site) else {
                return Err(freeze("missing-declaration-binding"));
            };
            let name = owner
                .binding(binding)
                .ok_or_else(|| freeze("missing-binding-record"))?
                .diagnostic_name();
            binding_names.insert(binding, Box::<str>::from(name));
            if binding.owner() != owner_id || !declared.insert(binding) {
                return Err(freeze("foreign-or-duplicate-declaration"));
            }
            match site {
                SourceBindingSiteV1::Receiver => {
                    if receiver.replace(binding).is_some() {
                        return Err(freeze("duplicate-receiver"));
                    }
                }
                SourceBindingSiteV1::Parameter { index } => {
                    if parameters.insert(*index, binding).is_some() {
                        return Err(freeze("duplicate-parameter"));
                    }
                }
                SourceBindingSiteV1::Local { statement, ordinal } => {
                    if locals
                        .entry(statement.node().clone())
                        .or_default()
                        .insert(*ordinal, binding)
                        .is_some()
                    {
                        return Err(freeze("duplicate-local"));
                    }
                }
                _ => {}
            }
        }

        let parameters = ordered_bindings(parameters, "parameter-ordinal-gap")?;
        let locals = locals
            .into_iter()
            .map(|(site, bindings)| {
                ordered_bindings(bindings, "local-ordinal-gap").map(|bindings| (site, bindings))
            })
            .collect::<Result<_, _>>()?;

        let mut variables = BTreeMap::new();
        for (site, reference) in owner.variable_refs() {
            let ResolvedLexicalRefV1::Local(binding) = reference else {
                continue;
            };
            if binding.owner() != owner_id {
                return Err(freeze("foreign-variable-binding"));
            }
            if variables.insert(site.node().clone(), *binding).is_some() {
                return Err(freeze("duplicate-variable-site"));
            }
        }

        let mut assignments = BTreeMap::new();
        let explicit_extern_calls = owner
            .explicit_extern_calls()
            .map(|(site, call)| (site.node().clone(), call.symbol().into()))
            .collect();
        for (site, target) in owner.assignment_targets() {
            let ResolvedAssignmentTargetV1::BindingRebind(binding) = target else {
                continue;
            };
            if binding.owner() != owner_id {
                return Err(freeze("foreign-assignment-binding"));
            }
            if assignments.insert(site.node().clone(), *binding).is_some() {
                return Err(freeze("duplicate-assignment-site"));
            }
        }

        let mut direct_lambda_captures = BTreeMap::new();
        for (child, _) in forest.owners() {
            let Some(edge) = forest.parent(child) else {
                continue;
            };
            if edge.parent_owner() != owner_id {
                continue;
            }
            let captures = forest
                .ordered_capture_demands(child)
                .iter()
                .map(|demand| {
                    let binding = demand.source_binding();
                    if binding.owner() != owner_id {
                        return Err(freeze("direct-lambda-foreign-capture"));
                    }
                    let name = owner
                        .binding(binding)
                        .ok_or_else(|| freeze("direct-lambda-missing-binding"))?
                        .diagnostic_name();
                    Ok((Box::<str>::from(name), binding))
                })
                .collect::<Result<Vec<_>, String>>()?
                .into_boxed_slice();
            if direct_lambda_captures
                .insert(edge.definition_site().site().node().clone(), captures)
                .is_some()
            {
                return Err(freeze("duplicate-direct-lambda-site"));
            }
        }

        let initializers = map_local::retain_initializers(input, &locals)?;
        Ok(Self {
            initializers,
            owner: owner_id,
            function_origin: input.function().function_origin(),
            source_kind: input.function().source_kind(),
            receiver,
            parameters,
            locals,
            variables,
            assignments,
            binding_names,
            explicit_extern_calls,
            brand_constructors,
            direct_lambda_captures,
            values: MaterializedValuesV1::default(),
            dynamic_origins,
            construction: construction::ConstructionState::NotConstruction,
            fault_frame: Some(
                crate::mir::builder::function_fault_frame::FunctionFaultFrameV1::borrowed(),
            ),
            entry_installed: false,
            borrowed_entry_formals: None,
            materialized_locals: BTreeSet::new(),
            consumed_variables: BTreeSet::new(),
            consumed_assignments: BTreeSet::new(),
            consumed_direct_lambdas: BTreeSet::new(),
            consumed_brand_constructors: BTreeSet::new(),
            consumed_source_core_method_calls: BTreeSet::new(),
            source_loop_bridge,
            loop_break_source,
            source_core_method_calls,
            named_array_writes: Vec::new(),
            named_array_field_providers: BTreeMap::new(),
            named_array_field_allocations: BTreeSet::new(),
            source_static_result_publications: BTreeMap::new(),
            consumed_source_static_result_publications: BTreeSet::new(),
            ordinary_new_claim_ledger: None,
        })
    }
}
