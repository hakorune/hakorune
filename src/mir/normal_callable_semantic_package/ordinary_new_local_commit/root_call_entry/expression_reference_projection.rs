//! Independent expression reference projection.
use super::*;

impl ExpressionPhysicalProjectionRefV1<'_> {
    /// Project the same source acquisition positions through finishing. A mapped
    /// physical producer can change coordinates, never the source node census.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_acquisition_projection_v1(
        &self,
        acquired: &[(usize, ValueId)],
        function: &MirFunction,
        finishing: Option<&super::super::super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        let Some(boundary) = finishing else {
            return validate_expression_acquisition_scope_v1(
                self.frame,
                None,
                None,
                acquired,
                self.values,
                function,
            );
        };
        let mut values = self.values.to_vec();
        let mut projected = Vec::with_capacity(acquired.len());
        for &(node, value) in acquired {
            if values.get(node) != Some(&Some(Operand::OwnedString(value).into())) {
                return Err(freeze("expression-call/acquisition-node-value"));
            }
            let original = self.require_unique(|row| matches!(row, MirInstruction::InvokeNormalResult { dst, .. } if *dst == value))?;
            let actual = boundary
                .binding(original.0, &original.1)?
                .ok_or_else(|| freeze("expression-call/acquisition-projection"))?;
            let MirInstruction::InvokeNormalResult { dst, .. } = actual.1 else {
                return Err(freeze("expression-call/acquisition-projection"));
            };
            projected.push((node, dst));
            values[node] = Some(Operand::OwnedString(dst).into());
        }
        validate_expression_acquisition_scope_v1(
            self.frame, None, None, &projected, &values, function,
        )
    }
}

impl ExpressionPhysicalProjectionRefV1<'_> {
    pub(in super::super) fn validate_reference_dependencies(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&super::super::super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        self.validate_source(context)?;
        for (&node, observed) in self.reference_reads {
            observed.validate_dependencies_v1(self.frame, node, context, function, finishing)?;
        }
        for (observation, packet) in self.source_calls().zip(self.calls()) {
            packet.validate_self_entry_receiver_v1(observation, context, function, finishing)?;
        }
        let source = self.frame.source();
        let roots = self.original_field_roots_v1(context)?;
        for read in self.path_reads {
            let (site, _, _) = read.source_value();
            if let Some(access) = self
                .frame
                .cores()
                .values()
                .filter(|core| self.node_selected(core.node()))
                .filter_map(|core| core.field_array_access_v1())
                .find(|access| access.receiver_alias_v1().is_some() && access.receiver() == &site)
            {
                let (_, original) = context
                    .ledger()
                    .array_alias_operation_read_v1(access, read, context)?;
                let alias = access
                    .receiver_alias_v1()
                    .ok_or_else(|| freeze("print-expression/path-root-source"))?;
                for dependency in read
                    .loan_local_initializer_v1(alias.binding())?
                    .1
                    .iter()
                    .chain(read.original_copy_dependencies_v1()?)
                {
                    let actual = match finishing {
                        Some(projection) => projection.relation_binding(function, dependency)?,
                        None => Some(dependency.clone()),
                    };
                    if let Some(actual) = actual {
                        if !super::super::super::physical_boundary::check_binding(
                            function, None, actual.0, &actual.1,
                        )? {
                            return Err(freeze("print-expression/path-copy-missing"));
                        }
                    }
                }
                let actual = match finishing {
                    Some(projection) => projection
                        .relation_binding(function, &original)?
                        .ok_or_else(|| freeze("print-expression/required-producer-removed"))?,
                    None => original,
                };
                if !super::super::super::physical_boundary::check_binding(
                    function, None, actual.0, &actual.1,
                )? {
                    return Err(freeze("print-expression/path-copy-missing"));
                }
                continue;
            }
            let (_, first) = roots
                .get(&site)
                .ok_or_else(|| freeze("print-expression/path-root-source"))?;
            if source.nodes().iter().enumerate().any(|(index, node)| {
                node.site() == first.site()
                    && self
                        .frame
                        .direct_array_receiver_v1(index)
                        .is_some_and(|core| self.node_selected(core.node()))
            }) {
                let (binding, _) = roots
                    .get(&site)
                    .ok_or_else(|| freeze("print-expression/path-root-source"))?;
                let entry = *function
                    .params
                    .first()
                    .ok_or_else(|| freeze("print-expression/path-root-source"))?;
                for dependency in read.loan_forwarded_copies(*binding, entry)? {
                    let actual = match finishing {
                        Some(projection) => projection.relation_binding(function, dependency)?,
                        None => Some(dependency.clone()),
                    };
                    if let Some(actual) = actual {
                        if !super::super::super::physical_boundary::check_binding(
                            function, None, actual.0, &actual.1,
                        )? {
                            return Err(freeze("print-expression/path-copy-missing"));
                        }
                    }
                }
            }
            let bindings = context
                .ledger()
                .expression_field_read_bindings_v1(first, source, context)?;
            let mut operations = bindings.iter().filter(|(_, row)| {
                matches!(
                    row,
                    MirInstruction::ObjectFieldGet { .. }
                        | MirInstruction::Invoke {
                            operation: InvokeOperation::FieldGet { .. }
                                | InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::FieldMaterialize { .. }),
                            ..
                        }
                )
            });
            let consumer = operations
                .next()
                .ok_or_else(|| freeze("print-expression/path-root-value"))?;
            if operations.next().is_some() {
                return Err(freeze("print-expression/path-root-value"));
            }
            if let Some((frame, exclusion)) = context
                .ledger()
                .stored_get_field_root_source_v1(first, read, context)?
            {
                context.validate_stored_get_nonnull_read_v1(
                    &frame, &exclusion, read, function, finishing, consumer,
                )?;
            }
        }
        Ok(())
    }
}
