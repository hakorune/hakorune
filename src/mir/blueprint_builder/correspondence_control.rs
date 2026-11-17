//! Actual CFG traversal and source-derived binding/Phi relations.
use super::*;
fn same_inputs(actual: &[(BasicBlockId, ValueId)], expected: &[(BasicBlockId, ValueId)]) -> bool {
    let a: BTreeMap<_, _> = actual.iter().copied().collect();
    let b: BTreeMap<_, _> = expected.iter().copied().collect();
    a.len() == actual.len() && b.len() == expected.len() && a == b
}
impl Checker<'_> {
    fn phi(
        &mut self,
        block: BasicBlockId,
        binding: BindingId,
        expected: Option<&[(BasicBlockId, ValueId)]>,
    ) -> Result<(ValueId, Vec<(BasicBlockId, ValueId)>), String> {
        let recorded = *self
            .records
            .phis
            .get(&(block, binding))
            .ok_or_else(|| bad("phi-missing"))?;
        if !self.phis.insert((block, binding)) {
            return Err(bad("phi-repeated"));
        }
        let MirInstruction::Phi {
            dst,
            inputs,
            type_hint,
        } = self.take(None)?
        else {
            return Err(bad("phi-kind"));
        };
        if type_hint != Some(self.source.bindings[binding.0].ty.mir())
            || dst != recorded
            || expected.is_some_and(|expected| !same_inputs(&inputs, expected))
        {
            return Err(bad("phi-relation"));
        }
        self.typed(dst, self.source.bindings[binding.0].ty)?;
        Ok((dst, inputs))
    }
    fn merge(
        &mut self,
        block: BasicBlockId,
        states: &[State],
        bindings: &[BindingId],
    ) -> Result<(), String> {
        if states.is_empty() {
            self.current = None;
            return Ok(());
        }
        self.enter(block, Environment::new());
        for binding in bindings {
            let mut inputs = Vec::new();
            for state in states {
                inputs.push((
                    state.cursor.block,
                    *state
                        .values
                        .get(binding)
                        .ok_or_else(|| bad("merge-binding"))?,
                ));
            }
            let value = if self.records.phis.contains_key(&(block, *binding)) {
                self.phi(block, *binding, Some(&inputs))?.0
            } else if inputs.iter().all(|(_, v)| *v == inputs[0].1) {
                inputs[0].1
            } else {
                return Err(bad("merge-phi-missing"));
            };
            self.bind(*binding, value)?;
        }
        Ok(())
    }
    pub(super) fn conditional(
        &mut self,
        node: NodeId,
        condition: &Expression,
        yes: &[Statement],
        no: &[Statement],
        bindings: &[BindingId],
    ) -> Result<(), String> {
        self.joins.insert(node);
        let join = *self
            .records
            .joins
            .get(&node)
            .ok_or_else(|| bad("join-missing"))?;
        let condition = self.operand(condition)?;
        let (state, yes_block, no_block) = self.branch(node, condition)?;
        let mut exits = Vec::new();
        self.enter(yes_block, state.values.clone());
        self.block(yes)?;
        if self.current.is_some() {
            exits.push(self.jump(None, join)?);
        }
        self.enter(no_block, state.values);
        self.block(no)?;
        if self.current.is_some() {
            exits.push(self.jump(None, join)?);
        }
        self.merge(join, &exits, bindings)
    }
    pub(super) fn logical(
        &mut self,
        source: &Expression,
        left: &Expression,
        right: &Expression,
        and: bool,
    ) -> Result<ValueId, String> {
        self.joins.insert(source.id);
        let join = *self
            .records
            .joins
            .get(&source.id)
            .ok_or_else(|| bad("logical-join"))?;
        let lhs = self.operand(left)?;
        let (state, yes, no) = self.branch(source.id, lhs)?;
        let (rhs, skip) = if and { (yes, no) } else { (no, yes) };
        let bindings: Vec<_> = state.values.keys().copied().collect();
        self.enter(skip, state.values.clone());
        let skipped = self.jump(None, join)?;
        self.enter(rhs, state.values);
        let right = self.operand(right)?;
        let evaluated = self.jump(None, join)?;
        let inputs = vec![(skipped.cursor.block, lhs), (evaluated.cursor.block, right)];
        self.merge(join, &[skipped, evaluated], &bindings)?;
        let MirInstruction::Phi {
            dst,
            inputs: actual,
            type_hint: Some(crate::mir::MirType::Bool),
        } = self.take(Some((source.id, Role::Value)))?
        else {
            return Err(bad("logical-result-kind"));
        };
        if !same_inputs(&actual, &inputs) {
            return Err(bad("logical-result-relation"));
        }
        Ok(dst)
    }
    pub(super) fn loop_statement(
        &mut self,
        node: NodeId,
        contract: &LoopContract,
    ) -> Result<(), String> {
        self.loop_nodes.insert(node);
        let image = *self
            .records
            .loops
            .get(&node)
            .ok_or_else(|| bad("loop-image"))?;
        let range = match &contract.test {
            LoopTest::Range { index, start, end } => {
                let start = self.operand(start)?;
                let end = self.operand(end)?;
                self.bind(*index, start)?;
                Some((*index, end))
            }
            _ => None,
        };
        if image.step.is_some() != range.is_some() {
            return Err(bad("loop-step-role"));
        }
        let initial = self.jump(None, image.header)?;
        let mut carriers = contract.carriers.clone();
        let mut header_bindings = contract.assigned.clone();
        if let Some((index, _)) = range {
            carriers.push(index);
            header_bindings.push(index);
        }
        self.enter(image.header, initial.values.clone());
        let mut header_phis = Vec::new();
        for binding in &header_bindings {
            let (value, inputs) = self.phi(image.header, *binding, None)?;
            self.bind(*binding, value)?;
            header_phis.push((*binding, inputs));
        }
        let (condition, always) = match &contract.test {
            LoopTest::Condition { expression, always } => (self.operand(expression)?, *always),
            LoopTest::Range { .. } => {
                let (index, end) = range.unwrap();
                let lhs = self.read(index)?;
                let MirInstruction::Compare {
                    dst,
                    op: CompareOp::Lt,
                    lhs: actual,
                    rhs,
                } = self.take(Some((node, Role::RangeTest)))?
                else {
                    return Err(bad("range-test"));
                };
                if actual != lhs || rhs != end {
                    return Err(bad("range-bound"));
                }
                self.typed(dst, Type::Bool)?;
                (dst, false)
            }
        };
        let condition_state = if always {
            self.jump(Some((node, Role::Control)), image.body)?
        } else {
            let (state, yes, no) = self.branch(node, condition)?;
            if yes != image.body || no != image.exit {
                return Err(bad("loop-branch-direction"));
            }
            state
        };
        self.enter(image.body, condition_state.values.clone());
        self.loops.push(LoopFrame {
            id: node,
            image,
            breaks: Vec::new(),
            continues: Vec::new(),
        });
        self.block(&contract.body)?;
        if self.current.is_some() {
            let state = self.jump(None, image.step.unwrap_or(image.header))?;
            self.loops.last_mut().unwrap().continues.push(state);
        }
        let frame = self.loops.pop().ok_or_else(|| bad("loop-frame"))?;
        let mut backs = frame.continues;
        if let Some((index, _)) = range {
            self.merge(image.step.unwrap(), &backs, &carriers)?;
            backs.clear();
            if self.current.is_some() {
                let old = self.read(index)?;
                let MirInstruction::Const {
                    dst: one,
                    value: ConstValue::Integer(1),
                } = self.take(Some((node, Role::RangeOne)))?
                else {
                    return Err(bad("range-one"));
                };
                self.typed(one, Type::I64)?;
                let MirInstruction::BinOp {
                    dst,
                    op: BinaryOp::Add,
                    lhs,
                    rhs,
                } = self.take(Some((node, Role::RangeStep)))?
                else {
                    return Err(bad("range-step"));
                };
                if lhs != old || rhs != one {
                    return Err(bad("range-step-operands"));
                }
                self.typed(dst, Type::I64)?;
                self.bind(index, dst)?;
                backs.push(self.jump(None, image.header)?);
            }
        }
        // A carrier omitted from header Phis must stay invariant on every
        // backedge. Missing write information must not silently lose updates.
        for binding in carriers.iter().filter(|b| !header_bindings.contains(b)) {
            for state in &backs {
                if state.values.get(binding) != initial.values.get(binding) {
                    return Err(bad("unassigned-carrier-changed"));
                }
            }
        }
        for (binding, actual) in header_phis {
            let mut expected = vec![(
                initial.cursor.block,
                *initial
                    .values
                    .get(&binding)
                    .ok_or_else(|| bad("initial-binding"))?,
            )];
            for state in &backs {
                expected.push((
                    state.cursor.block,
                    *state
                        .values
                        .get(&binding)
                        .ok_or_else(|| bad("backedge-binding"))?,
                ));
            }
            if !same_inputs(&actual, &expected) {
                return Err(bad("header-phi-inputs"));
            }
        }
        let mut exits = frame.breaks;
        if !always {
            exits.push(condition_state);
        }
        self.merge(image.exit, &exits, &contract.carriers)
    }
}
