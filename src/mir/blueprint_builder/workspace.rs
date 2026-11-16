//! The sole transient binding×block table and Phi writer.
use super::*;
impl Workspace<'_> {
    pub(super) fn mark(&mut self, node: NodeId) -> Result<(), String> {
        if !self.correspondence.nodes.insert(node) {
            return Err("blueprint/internal/duplicate-source-node".into());
        }
        Ok(())
    }
    pub(super) fn value(&mut self, ty: Type) -> ValueId {
        let value = self.function.next_value_id();
        self.function.metadata.value_types.insert(value, ty.mir());
        value
    }
    pub(super) fn current(&self) -> Result<BasicBlockId, String> {
        self.current
            .ok_or_else(|| "blueprint/internal/no-continuation".into())
    }
    pub(super) fn environment(&self) -> Result<Environment, String> {
        Ok(self.environments[&self.current()?].clone())
    }
    pub(super) fn read(&self, binding: BindingId) -> Result<ValueId, String> {
        self.environments[&self.current()?]
            .get(&binding)
            .copied()
            .ok_or_else(|| "blueprint/internal/uninitialized-binding".into())
    }
    pub(super) fn bind(&mut self, binding: BindingId, value: ValueId) -> Result<(), String> {
        let block = self.current()?;
        self.environments
            .get_mut(&block)
            .ok_or("blueprint/internal/missing-environment")?
            .insert(binding, value);
        Ok(())
    }
    pub(super) fn new_block(&mut self, environment: Environment) -> BasicBlockId {
        let block = BasicBlockId::new(self.function.blocks.len() as u32);
        self.function.add_block(BasicBlock::new(block));
        self.environments.insert(block, environment);
        block
    }
    pub(super) fn emit(
        &mut self,
        node: NodeId,
        role: Role,
        instruction: MirInstruction,
    ) -> Result<(), String> {
        let block = self.current()?;
        let body = self
            .function
            .blocks
            .get_mut(&block)
            .ok_or("blueprint/internal/block")?;
        if body.terminator.is_some() {
            return Err("blueprint/internal/write-after-exit".into());
        }
        let position = Cursor {
            block,
            index: body.instructions.len(),
        };
        if self
            .correspondence
            .operations
            .insert((node, role), position)
            .is_some()
        {
            return Err("blueprint/internal/duplicate-role".into());
        }
        body.add_instruction(instruction);
        Ok(())
    }
    pub(super) fn terminate(
        &mut self,
        source: Option<(NodeId, Role)>,
        operation: MirInstruction,
    ) -> Result<BasicBlockId, String> {
        let block = self.current()?;
        let body = self
            .function
            .blocks
            .get_mut(&block)
            .ok_or("blueprint/internal/block")?;
        if body.terminator.is_some() {
            return Err("blueprint/internal/second-exit".into());
        }
        if let Some(role) = source {
            if self
                .correspondence
                .operations
                .insert(
                    role,
                    Cursor {
                        block,
                        index: body.instructions.len(),
                    },
                )
                .is_some()
            {
                return Err("blueprint/internal/duplicate-role".into());
            }
        }
        body.set_terminator(operation);
        self.current = None;
        Ok(block)
    }
    pub(super) fn jump(
        &mut self,
        source: Option<(NodeId, Role)>,
        target: BasicBlockId,
    ) -> Result<BasicBlockId, String> {
        self.terminate(
            source,
            MirInstruction::Jump {
                target,
                edge_args: None,
            },
        )
    }
    pub(super) fn branch(
        &mut self,
        node: NodeId,
        condition: ValueId,
        yes: BasicBlockId,
        no: BasicBlockId,
    ) -> Result<BasicBlockId, String> {
        self.terminate(
            Some((node, Role::Control)),
            MirInstruction::Branch {
                condition,
                then_bb: yes,
                else_bb: no,
                then_edge_args: None,
                else_edge_args: None,
            },
        )
    }
    pub(super) fn phi(
        &mut self,
        block: BasicBlockId,
        binding: BindingId,
        inputs: Vec<(BasicBlockId, ValueId)>,
    ) -> ValueId {
        let dst = self.value(self.source.bindings[binding.0].ty);
        self.function
            .blocks
            .get_mut(&block)
            .unwrap()
            .add_instruction(MirInstruction::Phi {
                dst,
                inputs,
                type_hint: Some(self.source.bindings[binding.0].ty.mir()),
            });
        self.correspondence.phis.insert((block, binding), dst);
        dst
    }
    pub(super) fn merge(
        &mut self,
        block: BasicBlockId,
        pred: &[BasicBlockId],
        bindings: &[BindingId],
    ) -> Result<(), String> {
        if pred.is_empty() {
            self.current = None;
            return Ok(());
        }
        let mut environment = Environment::new();
        for binding in bindings {
            let mut inputs = Vec::new();
            for p in pred {
                let value = *self.environments[p]
                    .get(binding)
                    .ok_or("blueprint/internal/merge-binding")?;
                inputs.push((*p, value));
            }
            let value = if inputs.iter().all(|(_, v)| *v == inputs[0].1) {
                inputs[0].1
            } else {
                self.phi(block, *binding, inputs)
            };
            environment.insert(*binding, value);
        }
        self.environments.insert(block, environment);
        self.current = Some(block);
        Ok(())
    }
}
