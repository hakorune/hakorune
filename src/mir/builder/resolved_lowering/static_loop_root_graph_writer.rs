//! Canonical physical adapter for the existing root Home graph algorithm.
//! The selected Static caller will consume its deferred Returns after DraftSeal.

use crate::mir::builder::ordinary_new_admission::selected::root_graph::RootGraphWriterPortV1;
use crate::mir::builder::resolved_lowering::canonical_cfg::CanonicalCfgSessionV1;
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, MirInstruction};

pub(super) struct CanonicalRootGraphWriterV1<'session> {
    cfg: &'session mut CanonicalCfgSessionV1,
    created: Vec<BasicBlockId>,
    deferred_clean: Vec<(BasicBlockId, MirInstruction)>,
}

impl<'session> CanonicalRootGraphWriterV1<'session> {
    pub(super) fn new(cfg: &'session mut CanonicalCfgSessionV1) -> Self {
        Self {
            cfg,
            created: Vec::new(),
            deferred_clean: Vec::new(),
        }
    }

    pub(super) fn created(&self) -> &[BasicBlockId] {
        &self.created
    }

    pub(super) fn deferred_clean(&self) -> &[(BasicBlockId, MirInstruction)] {
        &self.deferred_clean
    }

    fn write_terminator(
        &mut self,
        builder: &mut MirBuilder,
        block: BasicBlockId,
        instruction: &MirInstruction,
    ) -> Result<(), String> {
        let function = builder
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| "[freeze:contract][root-graph/no-function]".to_owned())?;
        match instruction {
            MirInstruction::Return { .. } => {
                if !self.created.contains(&block)
                    || self
                        .deferred_clean
                        .iter()
                        .any(|(recorded, _)| *recorded == block)
                    || function
                        .get_block(block)
                        .is_none_or(|target| target.is_terminated() || target.is_sealed())
                {
                    return Err("[freeze:contract][root-graph/deferred-return-target]".into());
                }
                self.deferred_clean.push((block, instruction.clone()));
                Ok(())
            }
            MirInstruction::ReturnFault { fault_frame } => self
                .cfg
                .emit_root_fault(function, block, *fault_frame)
                .map_err(|error| error.to_string()),
            MirInstruction::Jump {
                target,
                edge_args: None,
            } => self
                .cfg
                .emit_jump(function, block, *target)
                .map_err(|error| error.to_string()),
            MirInstruction::Invoke {
                operation,
                fault_frame,
                normal_landing,
                fault_landing,
            } => self
                .cfg
                .emit_root_invoke(
                    function,
                    block,
                    operation.clone(),
                    *fault_frame,
                    *normal_landing,
                    *fault_landing,
                )
                .map_err(|error| error.to_string()),
            _ => Err("[freeze:contract][root-graph/terminator-unsupported]".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::MirCall;
    use crate::mir::instruction::{
        FaultFrameMode, InvokeCallResultKind, InvokeOperation, MapInvokeOperation,
    };
    use crate::mir::{
        BasicBlock, Callee, EffectMask, FunctionSignature, MirFunction, MirType, ValueId,
    };

    fn owner() -> (MirBuilder, CanonicalCfgSessionV1, BasicBlockId, ValueId) {
        let mut builder = MirBuilder::new();
        let entry = builder.next_block_id();
        let frame = ValueId::new(17);
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "canonical_root_graph/0".into(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::CONTROL,
            },
            entry,
        );
        let mut block = BasicBlock::new(entry);
        block.add_instruction(MirInstruction::FaultFrameEnter {
            dst: frame,
            mode: FaultFrameMode::RootOwned,
        });
        function.add_block(block);
        builder.function_state.current_function = Some(function);
        builder.function_state.current_block = Some(entry);
        (builder, CanonicalCfgSessionV1::new(), entry, frame)
    }

    #[test]
    fn canonical_root_graph_preserves_bindings_and_predecessors() {
        let (mut builder, mut cfg, entry, frame) = owner();
        let mut bindings = Vec::new();
        let (clean, fault_terminal, origin, normal, fault) = {
            let mut writer = CanonicalRootGraphWriterV1::new(&mut cfg);
            let clean = writer.reserve_block(&mut builder).unwrap();
            writer
                .append_terminated(
                    &mut builder,
                    clean,
                    MirInstruction::Return {
                        value: Some(ValueId::new(5)),
                    },
                    &mut bindings,
                )
                .unwrap();
            let fault_terminal = writer.reserve_block(&mut builder).unwrap();
            writer
                .append_terminated(
                    &mut builder,
                    fault_terminal,
                    MirInstruction::ReturnFault { fault_frame: frame },
                    &mut bindings,
                )
                .unwrap();
            let origin = writer.reserve_block(&mut builder).unwrap();
            let normal = writer.reserve_block(&mut builder).unwrap();
            let fault = writer.reserve_block(&mut builder).unwrap();
            for (landing, target) in [(normal, clean), (fault, fault_terminal)] {
                writer
                    .append_terminated(
                        &mut builder,
                        landing,
                        MirInstruction::Jump {
                            target,
                            edge_args: None,
                        },
                        &mut bindings,
                    )
                    .unwrap();
            }
            writer
                .append_terminated(
                    &mut builder,
                    origin,
                    MirInstruction::Invoke {
                        operation: InvokeOperation::Map(MapInvokeOperation::End {
                            map: ValueId::new(8),
                        }),
                        fault_frame: frame,
                        normal_landing: normal,
                        fault_landing: fault,
                    },
                    &mut bindings,
                )
                .unwrap();
            let ingress = writer
                .emit_current(
                    &mut builder,
                    MirInstruction::Jump {
                        target: origin,
                        edge_args: None,
                    },
                )
                .unwrap();
            bindings.push(ingress);
            assert_eq!(
                writer.created(),
                &[clean, fault_terminal, origin, normal, fault]
            );
            assert_eq!(writer.deferred_clean(), &bindings[..1]);
            (clean, fault_terminal, origin, normal, fault)
        };
        let function = builder.function_state.current_function.as_mut().unwrap();
        cfg.emit_return(function, clean, Some(ValueId::new(5)))
            .unwrap();
        assert_eq!(function.blocks[&origin].predecessors.len(), 1);
        assert_eq!(function.blocks[&normal].predecessors.len(), 1);
        assert_eq!(function.blocks[&fault].predecessors.len(), 1);
        assert_eq!(function.blocks[&fault_terminal].predecessors.len(), 1);
        assert!(matches!(bindings[0].1, MirInstruction::Return { .. }));
        assert!(matches!(bindings[1].1, MirInstruction::ReturnFault { .. }));
        assert!(matches!(
            bindings.last().unwrap().1,
            MirInstruction::Jump { .. }
        ));
        for block in function.block_ids() {
            cfg.seal_block(function, block).unwrap();
        }
        assert_eq!(cfg.finish(function).unwrap().blocks().len(), 6);
        assert_eq!(entry, function.entry_block);
    }

    #[test]
    fn canonical_root_terminal_call_accepts_prepopulated_projection() {
        let (mut builder, mut cfg, entry, frame) = owner();
        let mut bindings = Vec::new();
        let clean = {
            let mut writer = CanonicalRootGraphWriterV1::new(&mut cfg);
            let clean = writer.reserve_block(&mut builder).unwrap();
            writer
                .append_terminated(
                    &mut builder,
                    clean,
                    MirInstruction::Return {
                        value: Some(ValueId::new(31)),
                    },
                    &mut bindings,
                )
                .unwrap();
            let fault_terminal = writer.reserve_block(&mut builder).unwrap();
            writer
                .append_terminated(
                    &mut builder,
                    fault_terminal,
                    MirInstruction::ReturnFault { fault_frame: frame },
                    &mut bindings,
                )
                .unwrap();
            let normal = writer.reserve_block(&mut builder).unwrap();
            let fault = writer.reserve_block(&mut builder).unwrap();
            for (landing, target) in [(normal, clean), (fault, fault_terminal)] {
                writer
                    .append_terminated(
                        &mut builder,
                        landing,
                        MirInstruction::Jump {
                            target,
                            edge_args: None,
                        },
                        &mut bindings,
                    )
                    .unwrap();
            }
            writer
                .insert_projection(
                    &mut builder,
                    normal,
                    MirInstruction::InvokeNormalResult {
                        invoke_block: entry,
                        dst: ValueId::new(31),
                    },
                )
                .unwrap();
            let key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::static_box_method(
                "RootGraph",
                "terminal",
                0,
            );
            let call = MirCall::new(
                None,
                Callee::Global(key.canonical_global_target_v1().unwrap()),
                vec![],
            );
            writer
                .emit_current(
                    &mut builder,
                    MirInstruction::Invoke {
                        operation: InvokeOperation::Call {
                            call,
                            result: InvokeCallResultKind::I64,
                        },
                        fault_frame: frame,
                        normal_landing: normal,
                        fault_landing: fault,
                    },
                )
                .unwrap();
            assert_eq!(writer.created().len(), 4);
            clean
        };
        let function = builder.function_state.current_function.as_mut().unwrap();
        cfg.emit_return(function, clean, Some(ValueId::new(31)))
            .unwrap();
        for block in function.block_ids() {
            cfg.seal_block(function, block).unwrap();
        }
        cfg.finish(function).unwrap();
    }

    #[test]
    fn canonical_root_writer_rejects_foreign_shapes_before_mutation() {
        let (mut builder, mut cfg, entry, frame) = owner();
        let mut writer = CanonicalRootGraphWriterV1::new(&mut cfg);
        let clean = writer.reserve_block(&mut builder).unwrap();
        let normal = writer.reserve_block(&mut builder).unwrap();
        let fault = writer.reserve_block(&mut builder).unwrap();
        let mut bindings = Vec::new();
        assert!(writer
            .append_terminated(
                &mut builder,
                clean,
                MirInstruction::Const {
                    dst: ValueId::new(22),
                    value: crate::mir::ConstValue::Integer(1),
                },
                &mut bindings,
            )
            .unwrap_err()
            .contains("terminator-unsupported"));
        assert!(bindings.is_empty());
        assert!(writer
            .cfg
            .create_block(
                builder.function_state.current_function.as_mut().unwrap(),
                clean
            )
            .is_err());
        for (landing, target) in [(normal, clean), (fault, clean)] {
            writer
                .append_terminated(
                    &mut builder,
                    landing,
                    MirInstruction::Jump {
                        target,
                        edge_args: None,
                    },
                    &mut bindings,
                )
                .unwrap();
        }
        let invoke = |fault_frame| MirInstruction::Invoke {
            operation: InvokeOperation::Map(MapInvokeOperation::End {
                map: ValueId::new(8),
            }),
            fault_frame,
            normal_landing: normal,
            fault_landing: fault,
        };
        assert!(writer
            .emit_current(&mut builder, invoke(ValueId::new(99)))
            .unwrap_err()
            .contains("root frame is not exactly defined"));
        assert!(builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .blocks[&entry]
            .terminator
            .is_none());
        writer
            .insert_projection(
                &mut builder,
                normal,
                MirInstruction::InvokeNormalResult {
                    invoke_block: clean,
                    dst: ValueId::new(9),
                },
            )
            .unwrap();
        assert!(writer
            .emit_current(&mut builder, invoke(frame))
            .unwrap_err()
            .contains("occupied or foreign"));
        assert!(builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .blocks[&entry]
            .terminator
            .is_none());
        writer
            .cfg
            .seal_block(
                builder.function_state.current_function.as_mut().unwrap(),
                entry,
            )
            .unwrap();
        assert!(writer
            .emit_current(&mut builder, invoke(frame))
            .unwrap_err()
            .contains("already sealed"));
    }
}

impl RootGraphWriterPortV1 for CanonicalRootGraphWriterV1<'_> {
    fn reserve_block(&mut self, builder: &mut MirBuilder) -> Result<BasicBlockId, String> {
        let block = builder.next_block_id();
        let function = builder
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| "[freeze:contract][root-graph/no-function]".to_owned())?;
        self.cfg
            .create_block(function, block)
            .map_err(|error| error.to_string())?;
        self.created.push(block);
        Ok(block)
    }

    fn append_terminated(
        &mut self,
        builder: &mut MirBuilder,
        id: BasicBlockId,
        terminator: MirInstruction,
        bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String> {
        if !self.created.contains(&id) {
            return Err("[freeze:contract][root-graph/unreserved-block]".into());
        }
        self.write_terminator(builder, id, &terminator)?;
        bindings.push((id, terminator));
        Ok(())
    }

    fn emit_current(
        &mut self,
        builder: &mut MirBuilder,
        instruction: MirInstruction,
    ) -> Result<(BasicBlockId, MirInstruction), String> {
        let block = builder
            .function_state
            .current_block
            .ok_or_else(|| "[freeze:contract][root-graph/no-block]".to_owned())?;
        self.write_terminator(builder, block, &instruction)?;
        Ok((block, instruction))
    }

    fn insert_projection(
        &mut self,
        builder: &mut MirBuilder,
        landing: BasicBlockId,
        projection: MirInstruction,
    ) -> Result<(), String> {
        let MirInstruction::InvokeNormalResult { .. } = projection else {
            return Err("[freeze:contract][root-graph/projection-unsupported]".into());
        };
        if !self.created.contains(&landing) {
            return Err("[freeze:contract][root-graph/unreserved-landing]".into());
        }
        let function = builder
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| "[freeze:contract][root-graph/no-function]".to_owned())?;
        let block = function
            .get_block_mut(landing)
            .ok_or_else(|| "[freeze:contract][root-graph/missing-landing]".to_owned())?;
        if block.is_sealed()
            || !block.instructions.is_empty()
            || !matches!(
                block.terminator,
                Some(MirInstruction::Jump {
                    edge_args: None,
                    ..
                })
            )
        {
            return Err("[freeze:contract][root-graph/occupied-landing]".into());
        }
        block.add_instruction(projection);
        Ok(())
    }
}
