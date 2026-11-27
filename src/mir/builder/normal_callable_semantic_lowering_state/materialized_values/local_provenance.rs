//! Existing local provenance queries, separate from value-slot installation.
use super::*;

impl LocalProvenanceV1 {
    /// Original source locator only; equality to the use is owed by completed MIR.
    pub(in crate::mir) fn recorded_root_value_v1(
        &self,
        binding: BindingRefV1,
    ) -> Result<ValueId, String> {
        if self.root != binding {
            return Err("[freeze:contract][forwarded-copy/root]".into());
        }
        Ok(self.root_value)
    }

    pub(in crate::mir) fn loan_scalar_join_v1(
        &self,
        source: &crate::mir::resolved_semantics::home_new_prefix::CurrentScalarProducerV1,
        result: ValueId,
    ) -> Result<&Rc<ScalarJoinPointValueV1>, String> {
        let LocalProducerPurposeV1::ScalarJoin(point) = &self.purpose else {
            return Err("[freeze:contract][local-read/loop-point-purpose]".into());
        };
        if point.source() != source {
            return Err("[freeze:contract][local-read/loop-point-source]".into());
        }
        self.loan_copies(point.binding(), point.value(), result)?;
        Ok(point)
    }
    pub(in crate::mir) fn loan_text_join_v1(
        &self,
        source: &crate::mir::resolved_semantics::home_new_prefix::SourceLoopTextProducerV1,
        result: ValueId,
    ) -> Result<&Rc<TextJoinPointValueV1>, String> {
        let LocalProducerPurposeV1::TextJoin(point) = &self.purpose else {
            return Err("[freeze:contract][local-read/text-loop-point-purpose]".into());
        };
        if point.source() != source {
            return Err("[freeze:contract][local-read/text-loop-point-source]".into());
        }
        self.loan_copies(point.binding(), point.value(), result)?;
        Ok(point)
    }
    pub(in crate::mir) fn loan_text_branch_v1(
        &self,
        source: &crate::mir::resolved_semantics::home_new_prefix::SourceLoopTextProducerV1,
        result: ValueId,
    ) -> Result<&Rc<crate::mir::normal_callable_semantic_package::BranchTextPhiObservationV1>, String>
    {
        let LocalProducerPurposeV1::BranchText(point) = &self.purpose else {
            return Err("[freeze:contract][local-read/text-branch-purpose]".into());
        };
        if point.source().as_ref() != match source {
            crate::mir::resolved_semantics::home_new_prefix::SourceLoopTextProducerV1::BranchNormal(source) => source.as_ref(),
            _ => return Err("[freeze:contract][local-read/text-branch-source]".into()),
        } {
            return Err("[freeze:contract][local-read/text-branch-source]".into());
        }
        self.loan_copies(
            point.source().binding(),
            point.relation().values().0,
            result,
        )?;
        Ok(point)
    }
    pub(in crate::mir) fn loan_dynamic_join_v1(
        &self,
        source: &crate::mir::resolved_semantics::home_new_prefix::SourceLoopDynamicProducerV1,
        result: ValueId,
    ) -> Result<&Rc<DynamicJoinPointValueV1>, String> {
        let LocalProducerPurposeV1::DynamicJoin(point) = &self.purpose else {
            return Err("[freeze:contract][local-read/dynamic-loop-point-purpose]".into());
        };
        if point.source() != source {
            return Err("[freeze:contract][local-read/dynamic-loop-point-source]".into());
        }
        self.loan_copies(point.binding(), point.value(), result)?;
        Ok(point)
    }
    /// Recorded instructions only; callers must check actual MIR membership.
    /// This inventory grants no Copy-origin, use or omission permission.
    pub(in crate::mir) fn recorded_copy_inventory_v1(&self) -> &[(BasicBlockId, MirInstruction)] {
        &self.copies
    }
    pub(in crate::mir) fn original_copy_dependencies_v1(
        &self,
        result: ValueId,
    ) -> Result<&[(BasicBlockId, MirInstruction)], String> {
        self.loan_copies(self.root, self.root_value, result)
    }
    /// Borrow the existing local initializer and its complete alias chain.
    /// The caller still owes the original source initializer and actual producer.
    pub(in crate::mir) fn loan_initializer_v1(
        &self,
        root: BindingRefV1,
        result: ValueId,
    ) -> Result<(ValueId, &[(BasicBlockId, MirInstruction)]), String> {
        self.local_initializer
            .as_ref()
            .ok_or_else(|| "[freeze:contract][local-read/initializer-missing]".to_owned())?
            .loan(root, result)
    }
    /// Lend the original root value only through the same complete copy proof.
    pub(in crate::mir) fn loan_root_initializer_v1(
        &self,
        original: BindingRefV1,
        result: ValueId,
    ) -> Result<ValueId, String> {
        self.loan_copies(original, self.root_value, result)?;
        Ok(self.root_value)
    }

    /// Resolve an aliased local initializer to its source producer value.
    /// The source and this local's initializer operand must occur in that
    /// order on the same verified physical copy chain.
    pub(in crate::mir) fn loan_forwarded_initializer_v1(
        &self,
        local_binding: BindingRefV1,
        result: ValueId,
        expected_source_value: Option<ValueId>,
    ) -> Result<ValueId, String> {
        let (initializer, _) = self
            .local_initializer
            .as_ref()
            .ok_or_else(|| "[freeze:contract][local-read/initializer-missing]".to_owned())?
            .loan(local_binding, result)?;
        self.loan_copies(self.root, self.root_value, result)?;
        let Some(source_value) = expected_source_value else {
            return Ok(initializer);
        };
        let mut current = self.root_value;
        let mut source_position = (current == source_value).then_some(0usize);
        let mut initializer_position = (current == initializer).then_some(0usize);
        for (index, (_, row)) in self.copies.iter().enumerate() {
            let Some((src, dst)) = local_initializer_operands(row) else {
                return Err(
                    "[freeze:contract][local-read/forwarded-initializer-instruction]".into(),
                );
            };
            if src != current {
                return Err("[freeze:contract][local-read/forwarded-initializer-chain]".into());
            }
            current = dst;
            let position = index + 1;
            if current == source_value && source_position.is_none() {
                source_position = Some(position);
            }
            if current == initializer && initializer_position.is_none() {
                initializer_position = Some(position);
            }
        }
        if source_position
            .zip(initializer_position)
            .is_none_or(|(source, target)| source > target)
        {
            return Err("[freeze:contract][local-read/forwarded-initializer-source]".into());
        }
        Ok(source_value)
    }
    pub(in crate::mir::builder::normal_callable_semantic_lowering_state) fn with_local_initializer_v1(
        mut self,
        binding: BindingRefV1,
        row: &CompletedLocalBindingV1,
    ) -> Option<Self> {
        self.local_initializer = Some(LocalInitializerProofV1::from_completed(binding, row)?);
        Some(self)
    }
    pub(in crate::mir) fn loan_checked_rebind_v1(
        &self,
        frame: &std::rc::Rc<
            crate::mir::resolved_semantics::home_new_prefix::CheckedBindingRebindFrameV1,
        >,
        result: ValueId,
    ) -> Result<(ValueId, &[(BasicBlockId, MirInstruction)]), String> {
        if !matches!(&self.purpose, LocalProducerPurposeV1::BindingRebind(original) if std::rc::Rc::ptr_eq(original, frame))
        {
            return Err("[freeze:contract][local-read/rebind-purpose]".into());
        }
        Ok((
            self.root_value,
            self.loan_copies(frame.target(), self.root_value, result)?,
        ))
    }
    pub(in crate::mir) fn source_read_branch_v1(
        &self,
        result: ValueId,
    ) -> Result<
        Option<Rc<crate::mir::normal_callable_semantic_package::BranchReadPhiObservationV1>>,
        String,
    > {
        let LocalProducerPurposeV1::BranchRead(point) = &self.purpose else {
            return Ok(None);
        };
        self.loan_copies(point.binding(), point.relation().values().0, result)?;
        Ok(Some(Rc::clone(point)))
    }
    pub(in crate::mir) fn source_declared_read_rebind_v1(
        &self,
        result: ValueId,
    ) -> Result<
        Option<
            std::rc::Rc<
                crate::mir::resolved_semantics::home_new_prefix::CheckedBindingRebindFrameV1,
            >,
        >,
        String,
    > {
        let LocalProducerPurposeV1::BindingRebind(frame) = &self.purpose else {
            return Ok(None);
        };
        if frame.declared_read_v1().is_none() {
            return Ok(None);
        }
        self.loan_checked_rebind_v1(frame, result)?;
        Ok(Some(std::rc::Rc::clone(frame)))
    }
    pub(in crate::mir) fn source_dynamic_rebind_v1(
        &self,
        root: BindingRefV1,
        result: ValueId,
        site: &crate::mir::resolved_semantics::SourceExprSiteV1,
    ) -> Result<
        Option<
            std::rc::Rc<
                crate::mir::resolved_semantics::home_new_prefix::CheckedBindingRebindFrameV1,
            >,
        >,
        String,
    > {
        let LocalProducerPurposeV1::BindingRebind(frame) = &self.purpose else {
            return Ok(None);
        };
        let dynamic = crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1::Value(
            crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::Dynamic,
        );
        if self.root != root
            || self.root_value != result
            || frame.target() != root
            || !frame.is_dynamic_loop_carrier_v1()
            || frame.evaluation().root_class() != dynamic
            || frame
                .source()
                .nodes()
                .first()
                .is_none_or(|node| node.site() != site)
        {
            return Err("[freeze:contract][local-read/dynamic-rebind-source]".into());
        }
        Ok(Some(std::rc::Rc::clone(frame)))
    }
    pub(in crate::mir::builder::normal_callable_semantic_lowering_state) fn matches(
        &self,
        binding: BindingRefV1,
        value: ValueId,
    ) -> bool {
        self.root == binding && self.root_value == value
    }

    pub(in crate::mir) fn loan_copies(
        &self,
        binding: BindingRefV1,
        value: ValueId,
        result: ValueId,
    ) -> Result<&[(BasicBlockId, MirInstruction)], String> {
        if !self.matches(binding, value) {
            if std::env::var_os("NYASH_DEBUG_LOCAL_CALLS").is_some() {
                eprintln!("[local-call-bindings] copy-root expected=({binding:?},{value:?}) actual=({:?},{:?}) result={result:?} purpose={:?} backtrace={}",
                    self.root, self.root_value, self.purpose, std::backtrace::Backtrace::force_capture());
            }
            return Err("[freeze:contract][forwarded-copy/root]".into());
        }
        let mut current = value;
        for (index, copy) in self.copies.iter().enumerate() {
            let Some((src, dst)) = local_initializer_operands(&copy.1) else {
                return Err("[freeze:contract][forwarded-copy/instruction]".into());
            };
            if src != current || self.copies[..index].contains(copy) {
                return Err("[freeze:contract][forwarded-copy/chain]".into());
            }
            current = dst;
        }
        if current != result {
            return Err("[freeze:contract][forwarded-copy/result]".into());
        }
        Ok(&self.copies)
    }
}
