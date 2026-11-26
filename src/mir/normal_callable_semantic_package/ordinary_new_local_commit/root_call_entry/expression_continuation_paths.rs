//! Checks that expression continuation delimiters still describe the original frame.
use super::*;

impl ExpressionPhysicalProjectionRefV1<'_> {
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
            if context
                .ledger()
                .expression_requires_fault_frame_v1(self.frame)
            {
                return Err(freeze("expression/fault-frame-required"));
            }
            if self.inline_range.is_none() {
                return Err(freeze("expression/inline-range-missing"));
            }
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
                if !context
                    .ledger()
                    .expression_root_continuation_v1(self.frame, continuation)?
                {
                    return Err(freeze("expression/inline-delimiter-drift"));
                }
                let actual = match finishing {
                    Some(boundary) => boundary
                        .binding(original.0, &original.1)?
                        .ok_or_else(|| freeze("initializer-expression/structural-jump-drift"))?,
                    None => original.clone(),
                };
                if !super::super::super::physical_boundary::check_binding(
                    function, None, actual.0, &actual.1,
                )? {
                    return Err(freeze("initializer-expression/structural-jump-drift"));
                }
                continue;
            }
            match finishing {
                Some(boundary) => {
                    boundary.structural_jump_binding(
                        function,
                        original,
                        context.is_finalized(),
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
        self.validate_continuation_v1(
            context,
            function,
            None,
            terminal,
            continuation,
            ExpressionContinuationPolicyV1 {
                entry: Some(entry),
                defer_owned_argument: true,
                ..Default::default()
            },
        )
        .unwrap();
        assert!(
            self.validate_continuation_v1(
                context,
                function,
                None,
                terminal,
                continuation,
                ExpressionContinuationPolicyV1 {
                    entry: Some(entry),
                    ..Default::default()
                },
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
            self.validate_continuation_v1(
                context,
                &changed,
                None,
                terminal,
                continuation,
                ExpressionContinuationPolicyV1 {
                    entry: Some(entry),
                    defer_owned_argument: true,
                    ..Default::default()
                },
            )
            .is_err(),
            "deferred delimiter must remain original"
        );
    }
}
