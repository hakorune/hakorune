//! Retain the existing Direct emission and its completed argument descent.
//! Physical dependencies corroborate that descent; they issue no source class.
use super::*;
use crate::mir::resolved_semantics::SourceExprSiteV1;
use crate::mir::normal_callable_semantic_package::DirectCallDispositionRowV1;
use crate::mir::resolved_semantics::home_new_prefix::{LocalCallObservationV1, LocalCallResultClassV1};
use crate::mir::instruction::InvokeCallResultKind;
use crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1;
#[derive(Debug)]
enum ScalarCallSourceV1 {
    Direct(DirectCallDispositionRowV1),
    Static { original: Rc<StaticIncomingSourceV1>, observations: Vec<(u32, SourceExprSiteV1, ValueId)>,
        projections: Vec<LexicalCallArgumentProjectionV1>, transport: Vec<Rc<crate::mir::builder::LocalSsaPhysicalRelationV1>> },
}
type Binding = (BasicBlockId, MirInstruction);
type Boundary = super::super::super::physical_boundary::FinishedBindings;

#[derive(Debug)]
enum DirectScalarPhysicalV1 { Plain(Binding), Invoke { invoke: Binding, normal: Binding } }
#[derive(Debug)]
pub(super) struct DirectScalarCallProjectionV1 {
    origin: ScalarCallSourceV1,
    source: LocalCallObservationV1,
    arguments: Vec<ValueId>,
    physical: DirectScalarPhysicalV1,
    original: Vec<Binding>,
    dependencies: Vec<Binding>,
    params: Vec<ValueId>,
    param_types: Vec<crate::mir::MirType>,
    carriers: Option<Box<[crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1]>>,
}
impl DirectScalarCallProjectionV1 {
    fn validate_source(&self, context: CallValidationContextV1<'_>) -> Result<ValueId, String> {
        if context.ledger().lexical_i64_call_source(self.source.site()) != Some(&self.source)
            || self.source.result() != LocalCallResultClassV1::I64
            || !self.source.prior_homes().is_empty() || self.source.local_binding().is_none() {
            return Err(freeze("direct-scalar/source"));
        }
        let (observations, argument_sites) = match &self.origin {
            ScalarCallSourceV1::Direct(row) => {
                row.validate_static_source_v1(context.ledger())?;
                if row.taken_site() != Some(self.source.site()) || row.result() != InvokeCallResultKind::I64 {
                    return Err(freeze("direct-scalar/taken-source"));
                }
                if row.static_source_v1().is_some() {
                    let projections = row.argument_projections_v1().ok_or_else(|| freeze("direct-scalar/argument-projections-missing"))?;
                    if projections.len() != self.source.arguments().len() { return Err(freeze("direct-scalar/argument-projections-cardinality")); }
                    for (index, ((projection, source), site)) in projections.iter().zip(self.source.arguments()).zip(row.argument_sites()).enumerate() {
                        let value = projection.value_for_source_v1(source, self.source.owner(), site, None, Some(context))?;
                        if self.arguments.get(index) != Some(&value) { return Err(freeze("direct-scalar/argument-source-value")); }
                        match projection {
                            LexicalCallArgumentProjectionV1::Scalar(read) => {
                                if read.original_copy_dependencies_v1()?.iter().any(|copy| !self.dependencies.contains(copy)) {
                                    return Err(freeze("direct-scalar/original-argument-copy"));
                                }
                            }
                            LexicalCallArgumentProjectionV1::Integer(binding) if self.dependencies.contains(binding) => (),
                            LexicalCallArgumentProjectionV1::Expression(_) => (),
                            _ => return Err(freeze("direct-scalar/argument-source-producer")),
                        }
                    }
                }
                (row.observed_arguments().ok_or_else(|| freeze("direct-scalar/argument-descent-missing"))?, row.argument_sites())
            }
            ScalarCallSourceV1::Static { original, observations, projections, transport } => {
                let selected = context.ledger().source_required_static_scalar_call_v1(self.source.site())?
                    .ok_or_else(|| freeze("static-scalar/required-source-missing"))?;
                if !Rc::ptr_eq(original, &selected) { return Err(freeze("static-scalar/original-source")); }
                if projections.len() != observations.len() || transport.len() != observations.len() || self.arguments.len() != observations.len()
                    || self.source.arguments().len() != observations.len() {
                    return Err(freeze("direct-scalar/argument-projections-cardinality"));
                }
                for (index, ((projection, relation), (_, site, value))) in projections.iter().zip(transport).zip(observations).enumerate() {
                    if projection.value_for_source_v1(&self.source.arguments()[index], self.source.owner(), site, None, Some(context))? != *value
                        || relation.values() != (*value, self.arguments[index]) {
                        return Err(freeze("direct-scalar/argument-source-value"));
                    }
                }
                (observations.as_slice(), original.argument_sites())
            }
        };
        if observations.len() != self.arguments.len() || self.source.arguments().len() != observations.len()
            || observations.iter().enumerate().any(|(index, (ordinal, site, value))|
                *ordinal as usize != index || argument_sites.get(index) != Some(site)
                    || self.arguments.get(index) != Some(&match &self.origin {
                        ScalarCallSourceV1::Static { transport, .. } => transport[index].values().1,
                        ScalarCallSourceV1::Direct(_) => *value,
                    })) {
            return Err(freeze("direct-scalar/argument-descent"));
        }
        match &self.physical {
            DirectScalarPhysicalV1::Plain(binding) => {
                let MirInstruction::Call(actual) = &binding.1 else { return Err(freeze("direct-scalar/plain-call")) };
                let dst = actual.dst.ok_or_else(|| freeze("direct-scalar/plain-result"))?;
                let expected = match &self.origin {
                    ScalarCallSourceV1::Direct(row) => row.scalar_emission().map_err(|_| freeze("direct-scalar/scalar-source"))?
                        .materialize_call(Some(dst), self.arguments.clone()).map_err(|_| freeze("direct-scalar/materialization"))?,
                    ScalarCallSourceV1::Static { original, .. } => {
                        let target = original.target().canonical_global_target_v1().map_err(|_| freeze("static-scalar/target"))?;
                        let mut expected = actual.clone();
                        expected.callee = crate::mir::Callee::Global(target);
                        expected.args = self.arguments.clone();
                        expected
                    }
                };
                if actual != &expected || match &self.origin {
                    ScalarCallSourceV1::Direct(_) => self.original.as_slice() != [binding.clone()],
                    ScalarCallSourceV1::Static { .. } => !self.original.contains(binding),
                } {
                    return Err(format!("{}: site={:?} actual={actual:?} expected={expected:?}",
                        freeze("direct-scalar/plain-materialization"), self.source.site()));
                }
                Ok(dst)
            }
            DirectScalarPhysicalV1::Invoke { invoke, normal } => {
                let ScalarCallSourceV1::Direct(row) = &self.origin else { return Err(freeze("static-scalar/physical-route")) };
                let call = row.lifecycle_emission().map_err(|_| freeze("direct-scalar/lifecycle-source"))?
                    .materialize_call(None, self.arguments.clone()).map_err(|_| freeze("direct-scalar/materialization"))?;
                let MirInstruction::Invoke { operation: InvokeOperation::Call { call: actual, result: InvokeCallResultKind::I64 },
                    normal_landing, fault_landing, fault_frame } = &invoke.1 else { return Err(freeze("direct-scalar/invoke")) };
                let MirInstruction::InvokeNormalResult { invoke_block, dst } = normal.1 else { return Err(freeze("direct-scalar/normal")) };
                if actual != &call || *normal_landing != normal.0 || invoke_block != invoke.0
                    || !self.original.contains(invoke) || !self.original.contains(normal)
                    || !self.original.contains(&(*fault_landing, MirInstruction::ReturnFault { fault_frame: *fault_frame }))
                    || !self.original.iter().any(|(_, row)| matches!(row, MirInstruction::FaultFrameEnter { dst, .. } if dst == fault_frame)) {
                    return Err(freeze("direct-scalar/call-or-continuation"));
                }
                Ok(dst)
            }
        }
    }
    pub(super) fn corroborate_loop_continuation_v2(&self, entry: BasicBlockId, end: BasicBlockId,
        function: &MirFunction, check_coordinates: bool,
    ) -> Result<(), String> {
        let (call, normal) = match &self.physical {
            DirectScalarPhysicalV1::Plain(binding) => (binding.0, binding.0),
            DirectScalarPhysicalV1::Invoke { invoke, normal } => (invoke.0, normal.0),
        };
        if end != normal || (check_coordinates && !crate::mir::verification::utils::compute_dominators(function).dominates(entry, call)) {
            return Err(freeze("source-loop/call-continuation"));
        }
        Ok(())
    }

    pub(super) fn argument_calls_v1(&self) -> impl Iterator<Item = &EmittedLexicalCallProjectionV1> {
        let projections = match &self.origin {
            ScalarCallSourceV1::Direct(row) => row.argument_projections_v1().unwrap_or(&[]),
            ScalarCallSourceV1::Static { projections, .. } => projections.as_slice(),
        };
        projections.iter().flat_map(|argument| argument.expression_calls_v1())
    }
    pub(super) fn value(&self, source: &LocalCallObservationV1, context: CallValidationContextV1<'_>) -> Result<ValueId, String> {
        if source != &self.source { return Err(freeze("direct-scalar/foreign-observation")); }
        self.validate_source(context)
    }
    pub(super) fn validate(&self, context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&Boundary>, bindings: &[Binding], coordinates: Option<&Boundary>,
    ) -> Result<(), String> {
        self.validate_source(context)?;
        if let (Some(boundary), CallValidationContextV1::Finalized(_)) = (finishing, context) {
            boundary.validate_finished_sequences_v1(function)?;
        }
        if function.params != self.params || function.signature.params != self.param_types
            || function.metadata.physical_param_carriers != self.carriers
            || self.params.iter().copied().collect::<std::collections::BTreeSet<_>>().len() != self.params.len() {
            return Err(freeze("direct-scalar/original-entry-layout"));
        }
        let projected;
        let expected = match coordinates { Some(boundary) => { projected = boundary.bindings(&self.original)?; &projected }, None => &self.original };
        if expected.as_slice() != bindings { return Err(freeze("direct-scalar/recorded-membership")); }
        let require_completed = finishing.is_some() || matches!(context, CallValidationContextV1::Finalized(_));
        let projections = match &self.origin {
            ScalarCallSourceV1::Direct(row) => row.argument_projections_v1().unwrap_or(&[]),
            ScalarCallSourceV1::Static { projections, transport, .. } => {
                for relation in transport {
                    if require_completed { relation.require_completed_v1()?; }
                    match finishing {
                        Some(boundary) => relation.verify_projected(function, &mut |row| boundary.relation_binding(function, row))?,
                        None => relation.verify(function)?,
                    }
                }
                projections.as_slice()
            }
        };
        for argument in projections {
            argument.validate_expression_physical_v1(context, function, finishing, bindings, coordinates)?;
        }
        for original in self.original.iter().chain(&self.dependencies) {
            let resolved;
            let original = if let ScalarCallSourceV1::Static { transport, .. } = &self.origin {
                let mut current = original.clone();
                for relation in transport {
                    if let Some(row) = relation.resolve_observed_binding_v1(&current, require_completed)? { current = row; }
                }
                resolved = current;
                &resolved
            } else { original };
            let actual = match (finishing, original.1.dst_value()) {
                (Some(boundary), Some(dst)) => {
                    if boundary.captured_definition(dst)? != original { return Err(freeze("direct-scalar/captured-producer")); }
                    match context {
                        CallValidationContextV1::Finalized(_) => boundary.relation_binding(function, original)?,
                        CallValidationContextV1::Lowering(_) => continue,
                    }
                }
                (Some(boundary), None) if matches!(original.1, MirInstruction::Invoke { .. }) => Some(match context {
                    CallValidationContextV1::Finalized(_) => boundary.required_effect_binding(function, original)?,
                    CallValidationContextV1::Lowering(_) => boundary.captured_effect_binding(function, original)?,
                }),
                (Some(boundary), None) => boundary.binding(original.0, &original.1)?,
                (None, _) => Some(original.clone()),
            };
            if let Some((block, row)) = actual {
                if !super::super::super::physical_boundary::check_binding(function, None, block, &row)? {
                    return Err(freeze("direct-scalar/physical-binding"));
                }
                if let Some(dst) = row.dst_value() {
                    let mut definitions = function.blocks.iter().flat_map(|(block, body)| body.all_instructions().map(move |row| (*block, row)))
                        .filter(|(_, row)| row.dst_value() == Some(dst));
                    if function.params.contains(&dst) || definitions.next() != Some((block, &row)) || definitions.next().is_some() {
                        return Err(freeze("direct-scalar/unique-producer"));
                    }
                }
            }
        }
        Ok(())
    }
}

/// Borrow the original physical dependency cone of the completed descent.
/// No MIR shape here assigns a source type or selects a call route.
fn argument_dependencies(function: &MirFunction, arguments: &[ValueId]) -> Result<Vec<Binding>, String> {
    let originals: Vec<_> = function.blocks.iter().flat_map(|(block, body)|
        body.all_instructions().map(move |row| (*block, row.clone()))).collect();
    super::super::super::physical_boundary::original_value_dependencies_v1(&originals, &function.params, arguments)
}
impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir) fn source_required_static_scalar_call_v1(&self, site: &OwnedExprSiteV1)
        -> Result<Option<Rc<StaticIncomingSourceV1>>, String> {
        let Some(flow) = self.completion_for_owner(site.owner()).and_then(|completion| completion.cleanup().root_flow()) else { return Ok(None) };
        if !flow.initializer_scalar_call_dependency_v1(site) { return Ok(None) }
        let original = self.borrow_original_static_source_v1(site)?
            .ok_or_else(|| freeze("static-scalar/original-call-missing"))?;
        if !matches!(original.source_result(), crate::mir::callable_result_representation::VerifiedCallableResultDispositionV1::ExactI64 { .. }) {
            return Err(freeze("static-scalar/original-result-not-i64"));
        }
        self.verify_original_static_packet_source_v1(original)?;
        Ok(Some(Rc::clone(original)))
    }
    pub(in crate::mir) fn record_source_required_static_scalar_call_v1(&self, original: Rc<StaticIncomingSourceV1>,
        observations: Vec<(u32, SourceExprSiteV1, ValueId)>, projections: Vec<LexicalCallArgumentProjectionV1>,
        transport: Vec<Rc<crate::mir::builder::LocalSsaPhysicalRelationV1>>, operand_bindings: Vec<Binding>, binding: Binding, function: &MirFunction,
    ) -> Result<(), String> {
        let source = self.lexical_i64_call_source(original.call_site()).ok_or_else(|| freeze("static-scalar/local-source"))?.clone();
        let MirInstruction::Call(call) = &binding.1 else { return Err(freeze("direct-scalar/plain-call")); };
        let arguments = call.args.clone();
        let context = CallValidationContextV1::Lowering(self);
        let mut bindings = Vec::new();
        let mut dependencies = Vec::new();
        for projection in &projections {
            match projection {
                LexicalCallArgumentProjectionV1::Integer(row) => bindings.push(row.clone()),
                LexicalCallArgumentProjectionV1::Scalar(read) => dependencies.extend_from_slice(read.original_copy_dependencies_v1()?),
                LexicalCallArgumentProjectionV1::Expression(_) => bindings.extend_from_slice(projection.expression_bindings_v1()),
                _ => return Err(freeze("direct-scalar/argument-source-producer")),
            }
            dependencies.extend(projection.expression_copy_dependencies_v1(context)?);
        }
        for relation in &transport {
            relation.verify(function)?;
            let mut rows = Vec::new();
            relation.append_observed_bindings_v1(&mut rows);
            for row in rows {
                if !dependencies.contains(&row) { dependencies.push(row); }
            }
            if let Some(row) = relation.target_binding() {
                if operand_bindings.contains(row) && !bindings.contains(row) { bindings.push(row.clone()); }
            }
        }
        bindings.push(binding.clone());
        let packet = DirectScalarCallProjectionV1 { origin: ScalarCallSourceV1::Static { original, observations, projections, transport }, source,
            dependencies, arguments,
            physical: DirectScalarPhysicalV1::Plain(binding), original: bindings.clone(), params: function.params.clone(),
            param_types: function.signature.params.clone(), carriers: function.metadata.physical_param_carriers.clone() };
        packet.validate(CallValidationContextV1::Lowering(self), function, None, &bindings, None)?;
        let owner = packet.source.owner();
        let group = RootLifecyclePrefixBindingGroupV1 { site: packet.source.site().clone(), bindings,
            proof: PrefixPhysicalProofV1::DirectScalar(Rc::new(packet)), projected: false };
        self.prepare_local_call_binding_group(owner, group)?.commit()
    }
    pub(crate) fn record_source_required_plain_call_v1(&self, row: DirectCallDispositionRowV1,
        arguments: Vec<ValueId>, binding: Binding, function: &MirFunction,
    ) -> Result<(), String> {
        let site = row.taken_site().ok_or_else(|| freeze("direct-scalar/taken-site"))?;
        let Some(flow) = self.completion_for_owner(site.owner()).and_then(|completion| completion.cleanup().root_flow()) else { return Ok(()) };
        if !flow.initializer_scalar_call_dependency_v1(site) { return Ok(()) }
        let source = self.lexical_i64_call_source(site).ok_or_else(|| freeze("direct-scalar/required-call-source"))?.clone();
        let bindings = vec![binding.clone()];
        let packet = DirectScalarCallProjectionV1 { origin: ScalarCallSourceV1::Direct(row), source, dependencies: argument_dependencies(function, &arguments)?,
            arguments, physical: DirectScalarPhysicalV1::Plain(binding), original: bindings.clone(), params: function.params.clone(),
            param_types: function.signature.params.clone(), carriers: function.metadata.physical_param_carriers.clone() };
        packet.validate(CallValidationContextV1::Lowering(self), function, None, &bindings, None)?;
        let owner = packet.source.owner();
        let group = RootLifecyclePrefixBindingGroupV1 { site: packet.source.site().clone(), bindings,
            proof: PrefixPhysicalProofV1::DirectScalar(Rc::new(packet)), projected: false };
        self.prepare_local_call_binding_group(owner, group)?.commit()
    }
    pub(crate) fn record_root_direct_scalar_call_bindings_v1(&self,
        row: DirectCallDispositionRowV1, source: LocalCallObservationV1, arguments: Vec<ValueId>,
        invoke: Binding, normal: Binding, bindings: Vec<Binding>, function: &MirFunction,
    ) -> Result<(), String> {
        let mut combined = Vec::new();
        for binding in row.argument_projections_v1().unwrap_or(&[]).iter().flat_map(|argument| argument.expression_bindings_v1()).chain(&bindings) {
            if !combined.contains(binding) { combined.push(binding.clone()); }
        }
        let bindings = combined;
        let packet = DirectScalarCallProjectionV1 { origin: ScalarCallSourceV1::Direct(row), source, dependencies: argument_dependencies(function, &arguments)?,
            arguments, physical: DirectScalarPhysicalV1::Invoke { invoke, normal }, original: bindings.clone(), params: function.params.clone(),
            param_types: function.signature.params.clone(), carriers: function.metadata.physical_param_carriers.clone() };
        packet.validate(CallValidationContextV1::Lowering(self), function, None, &bindings, None)?;
        let owner = packet.source.owner();
        let group = RootLifecyclePrefixBindingGroupV1 { site: packet.source.site().clone(), bindings,
            proof: PrefixPhysicalProofV1::DirectScalar(Rc::new(packet)), projected: false };
        self.prepare_local_call_binding_group(owner, group)?.commit()
    }
}
