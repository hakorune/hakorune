//! Independent expression continuation paths.
use super::*;

impl ExpressionPhysicalProjectionRefV1<'_> {
    pub(in crate::mir) fn validate_scalar_continuation_v1(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&super::super::super::physical_boundary::FinishedBindings>,
        entry: &Option<Binding>,
        terminal: &Option<Binding>,
        continuation: &Option<Binding>,
        inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        self.validate_local_continuation_v1(
            context,
            function,
            finishing,
            entry,
            terminal,
            continuation,
            None,
            inherited,
        )
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_residence_continuation_v1(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&super::super::super::physical_boundary::FinishedBindings>,
        entry: &Option<Binding>,
        terminal: &Option<Binding>,
        continuation: &Option<Binding>,
        residence: Option<&Binding>,
        displaced: Option<&InvokeOperation>,
        purpose_acquisition: Option<&[Binding; 2]>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        self.validate_local_delimiters_v1(
            context,
            function,
            finishing,
            entry,
            terminal,
            continuation,
        )?;
        self.validate_paths_with_policy(
            context,
            function,
            finishing,
            terminal,
            continuation,
            residence,
            None,
            false,
            displaced,
            purpose_acquisition,
            None,
        )
    }
    pub(in crate::mir) fn validate_local_continuation_v1(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&super::super::super::physical_boundary::FinishedBindings>,
        entry: &Option<Binding>,
        terminal: &Option<Binding>,
        continuation: &Option<Binding>,
        residence: Option<&Binding>,
        inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        self.validate_local_continuation_with_purpose_v1(
            context,
            function,
            finishing,
            entry,
            terminal,
            continuation,
            residence,
            inherited,
            None,
        )
    }
    /// A Local that owns a Text snapshot of an owned carrier also carries the `Materialize` pair.
    pub(in crate::mir) fn validate_local_continuation_with_purpose_v1(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&super::super::super::physical_boundary::FinishedBindings>,
        entry: &Option<Binding>,
        terminal: &Option<Binding>,
        continuation: &Option<Binding>,
        residence: Option<&Binding>,
        inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
        purpose_acquisition: Option<&[Binding; 2]>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        self.validate_local_delimiters_v1(
            context,
            function,
            finishing,
            entry,
            terminal,
            continuation,
        )?;
        self.validate_paths_with_policy(
            context,
            function,
            finishing,
            terminal,
            continuation,
            residence,
            inherited,
            false,
            None,
            purpose_acquisition,
            None,
        )
    }
    pub(in crate::mir) fn validate_dynamic_local_handoff_continuation_v1(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&super::super::super::physical_boundary::FinishedBindings>,
        entry: &Option<Binding>,
        terminal: &Option<Binding>,
        continuation: &Option<Binding>,
        carrier: ValueId,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        self.validate_dynamic_carrier_argument_continuation_v1(
            context,
            function,
            finishing,
            entry,
            terminal,
            continuation,
            None,
            carrier,
        )
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_deferred_owned_continuation_v1(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&super::super::super::physical_boundary::FinishedBindings>,
        entry: &Option<Binding>,
        terminal: &Option<Binding>,
        continuation: &Option<Binding>,
        inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
        purpose_acquisition: Option<&[Binding; 2]>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        self.validate_local_delimiters_v1(
            context,
            function,
            finishing,
            entry,
            terminal,
            continuation,
        )?;
        self.validate_paths_with_policy(
            context,
            function,
            finishing,
            terminal,
            continuation,
            None,
            inherited,
            true,
            None,
            purpose_acquisition,
            None,
        )
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_dynamic_carrier_argument_continuation_v1(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&super::super::super::physical_boundary::FinishedBindings>,
        entry: &Option<Binding>,
        terminal: &Option<Binding>,
        continuation: &Option<Binding>,
        inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
        carrier: ValueId,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        self.validate_local_delimiters_v1(
            context,
            function,
            finishing,
            entry,
            terminal,
            continuation,
        )?;
        self.validate_paths_with_policy(
            context,
            function,
            finishing,
            terminal,
            continuation,
            None,
            inherited,
            false,
            None,
            None,
            Some(carrier),
        )
    }
    pub(super) fn validate_local_delimiters_v1(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&super::super::super::physical_boundary::FinishedBindings>,
        entry: &Option<Binding>,
        terminal: &Option<Binding>,
        continuation: &Option<Binding>,
    ) -> Result<(), String> {
        let (Some(entry), Some(terminal), Some(continuation)) = (entry, terminal, continuation)
        else {
            if entry.is_some() || terminal.is_some() || continuation.is_some() {
                return Err(freeze("expression/inline-delimiter-drift"));
            }
            self.validate_inline_scope_v1(context, function, finishing)?;
            return Ok(());
        };
        if self.inline_range.is_some()
            || !context
                .ledger()
                .expression_requires_fault_frame_v1(self.frame)
        {
            return Err(freeze("expression/inline-delimiter-drift"));
        }
        for original in [entry, terminal, continuation] {
            if original == continuation && self.root_fault.is_some() {
                if !context.ledger().expression_root_continuation_v1(self.frame, continuation)? {
                    return Err(freeze("expression/inline-delimiter-drift"));
                }
                let actual = match finishing {
                    Some(boundary) => boundary.binding(original.0, &original.1)?.ok_or_else(|| freeze("initializer-expression/structural-jump-drift"))?,
                    None => original.clone(),
                };
                if !super::super::super::physical_boundary::check_binding(function, None, actual.0, &actual.1)? {
                    return Err(freeze("initializer-expression/structural-jump-drift"));
                }
                continue;
            }
            match finishing {
                Some(boundary) => {
                    boundary.structural_jump_binding(
                        function,
                        original,
                        matches!(context, CallValidationContextV1::Finalized(_)),
                    )?;
                }
                None => {
                    if function
                        .blocks
                        .get(&original.0)
                        .and_then(|block| block.terminator.as_ref())
                        != Some(&original.1)
                    {
                        return Err(freeze("initializer-expression/structural-jump-drift"));
                    }
                }
            }
        }
        Ok(())
    }

    #[cfg(test)]
    pub(in crate::mir) fn assert_original_deferred_argument_paths_for_test(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        entry: &Option<Binding>,
        terminal: &Option<Binding>,
        continuation: &Option<Binding>,
    ) {
        self.validate_deferred_owned_continuation_v1(
            context,
            function,
            None,
            entry,
            terminal,
            continuation,
            None,
            None,
        )
        .unwrap();
        assert!(
            self.validate_scalar_continuation_v1(
                context,
                function,
                None,
                entry,
                terminal,
                continuation,
                None
            )
            .is_err(),
            "scalar completion cannot skip its own Ends"
        );
        let mut changed = function.clone();
        changed
            .blocks
            .get_mut(&terminal.as_ref().expect("owned argument has a terminal").0)
            .unwrap()
            .terminator = Some(MirInstruction::Jump {
            target: BasicBlockId(999_995),
            edge_args: None,
        });
        assert!(
            self.validate_deferred_owned_continuation_v1(
                context,
                &changed,
                None,
                entry,
                terminal,
                continuation,
                None,
                None
            )
            .is_err(),
            "deferred delimiter must remain original"
        );
    }
}
