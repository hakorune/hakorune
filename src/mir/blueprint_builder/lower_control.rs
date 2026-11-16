//! Structured control emission. No route policy, family test, or acceptance.
use super::*;
impl Workspace<'_> {
    pub(super) fn conditional(
        &mut self,
        node: NodeId,
        condition: &Expression,
        yes: &[Statement],
        no: &[Statement],
        carriers: &[BindingId],
    ) -> Result<(), String> {
        let Some(condition) = self.expression(condition)?.value()? else {
            return Ok(());
        };
        let environment = self.environment()?;
        let a = self.new_block(environment.clone());
        let b = self.new_block(environment.clone());
        let join = self.new_block(Environment::new());
        self.correspondence.joins.insert(node, join);
        self.branch(node, condition, a, b)?;
        let mut predecessors = Vec::new();
        self.current = Some(a);
        self.block(yes)?;
        if self.current.is_some() {
            predecessors.push(self.jump(None, join)?);
        }
        self.current = Some(b);
        self.block(no)?;
        if self.current.is_some() {
            predecessors.push(self.jump(None, join)?);
        }
        self.merge(join, &predecessors, carriers)
    }
    pub(super) fn logical(
        &mut self,
        source: &Expression,
        left: &Expression,
        right: &Expression,
        and: bool,
    ) -> Result<Option<ValueId>, String> {
        let Some(lhs) = self.expression(left)?.value()? else {
            return Ok(None);
        };
        let environment = self.environment()?;
        let bindings: Vec<_> = environment.keys().copied().collect();
        let rhs = self.new_block(environment.clone());
        let skip = self.new_block(environment);
        let join = self.new_block(Environment::new());
        self.correspondence.joins.insert(source.id, join);
        if and {
            self.branch(source.id, lhs, rhs, skip)?;
        } else {
            self.branch(source.id, lhs, skip, rhs)?;
        }
        self.current = Some(skip);
        let skipped = self.jump(None, join)?;
        self.current = Some(rhs);
        let right = self.expression(right)?.value()?;
        let mut incoming = vec![(skipped, lhs)];
        let mut predecessors = vec![skipped];
        if let Some(value) = right {
            let predecessor = self.jump(None, join)?;
            incoming.push((predecessor, value));
            predecessors.push(predecessor);
        }
        self.merge(join, &predecessors, &bindings)?;
        let result = self.value(Type::Bool);
        self.emit(
            source.id,
            Role::Value,
            MirInstruction::Phi {
                dst: result,
                inputs: incoming,
                type_hint: Some(Type::Bool.mir()),
            },
        )?;
        Ok(Some(result))
    }
    /// All source loop forms use this one structural writer and one workspace.
    pub(super) fn loop_statement(
        &mut self,
        node: NodeId,
        contract: &LoopContract,
    ) -> Result<(), String> {
        let range = match &contract.test {
            LoopTest::Range { index, start, end } => {
                let Some(start) = self.expression(start)?.value()? else {
                    return Ok(());
                };
                let Some(end) = self.expression(end)?.value()? else {
                    return Ok(());
                };
                self.bind(*index, start)?;
                Some((*index, end))
            }
            _ => None,
        };
        let initial = self.current()?;
        let incoming = self.environment()?;
        let mut carriers = contract.carriers.clone();
        if let Some((index, _)) = range {
            carriers.push(index);
        }
        let header = self.new_block(incoming.clone());
        let body = self.new_block(Environment::new());
        let step = range.map(|_| self.new_block(Environment::new()));
        let exit = self.new_block(Environment::new());
        let image = LoopImage {
            header,
            body,
            step,
            exit,
        };
        self.correspondence.loops.insert(node, image);
        self.jump(None, header)?;
        self.current = Some(header);
        let mut environment = Environment::new();
        for binding in &carriers {
            let value = *incoming
                .get(binding)
                .ok_or("blueprint/internal/header-binding")?;
            let phi = self.phi(header, *binding, vec![(initial, value)]);
            environment.insert(*binding, phi);
        }
        self.environments.insert(header, environment);
        let (condition, always) = match &contract.test {
            LoopTest::Condition { expression, always } => (
                self.expression(expression)?
                    .value()?
                    .ok_or("blueprint/internal/condition-ended")?,
                *always,
            ),
            LoopTest::Range { .. } => {
                let (index, end) = range.unwrap();
                let lhs = self.read(index)?;
                let condition = self.value(Type::Bool);
                self.emit(
                    node,
                    Role::RangeTest,
                    MirInstruction::Compare {
                        dst: condition,
                        op: CompareOp::Lt,
                        lhs,
                        rhs: end,
                    },
                )?;
                (condition, false)
            }
        };
        let condition_end = self.current()?;
        let body_environment = self.environment()?;
        if always {
            self.jump(Some((node, Role::Control)), body)?;
        } else {
            self.branch(node, condition, body, exit)?;
        }
        self.environments.insert(body, body_environment);
        self.current = Some(body);
        self.loops.push(LoopFrame {
            id: node,
            image,
            breaks: Vec::new(),
            continues: Vec::new(),
        });
        self.block(&contract.body)?;
        if self.current.is_some() {
            let p = self.jump(None, step.unwrap_or(header))?;
            self.loops.last_mut().unwrap().continues.push(p);
        }
        let frame = self.loops.pop().ok_or("blueprint/internal/loop-frame")?;
        let mut backedges = frame.continues;
        if let Some((index, _)) = range {
            let step = step.unwrap();
            self.merge(step, &backedges, &carriers)?;
            backedges.clear();
            if self.current.is_some() {
                let old = self.read(index)?;
                let one = self.value(Type::I64);
                self.emit(
                    node,
                    Role::RangeOne,
                    MirInstruction::Const {
                        dst: one,
                        value: ConstValue::Integer(1),
                    },
                )?;
                let next = self.value(Type::I64);
                self.emit(
                    node,
                    Role::RangeStep,
                    MirInstruction::BinOp {
                        dst: next,
                        op: BinaryOp::Add,
                        lhs: old,
                        rhs: one,
                    },
                )?;
                self.bind(index, next)?;
                backedges.push(self.jump(None, header)?);
            }
        }
        for binding in &carriers {
            let dst = self.correspondence.phis[&(header, *binding)];
            let mut inputs = vec![(initial, incoming[binding])];
            for predecessor in &backedges {
                inputs.push((
                    *predecessor,
                    *self.environments[predecessor]
                        .get(binding)
                        .ok_or("blueprint/internal/backedge-binding")?,
                ));
            }
            let operation = self
                .function
                .blocks
                .get_mut(&header)
                .unwrap()
                .instructions
                .iter_mut()
                .find(|op| matches!(op,MirInstruction::Phi{dst:actual,..}if *actual==dst))
                .ok_or("blueprint/internal/header-phi")?;
            let MirInstruction::Phi { inputs: actual, .. } = operation else {
                unreachable!()
            };
            *actual = inputs;
        }
        let mut exits = frame.breaks;
        if !always {
            exits.push(condition_end);
        }
        self.merge(exit, &exits, &contract.carriers)
    }
}
