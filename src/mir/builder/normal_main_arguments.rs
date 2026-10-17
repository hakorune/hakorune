//! Main wrapper ingress resource, distinct from source-local Home/New.
use super::NormalRuntimeInputSnapshotV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::instruction::InvokeOperation;
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1, SourceBindingSiteV1};
use crate::mir::{
    BasicBlock, BasicBlockId, MirBuilder, MirFunction, MirInstruction, MirType, ValueId,
};
use crate::parser::CallableDeclarationIdentityV1;
use std::{
    collections::{BTreeMap, VecDeque},
    rc::Rc,
};

#[derive(Debug)]
pub(in crate::mir) struct MainArgumentsEmissionV1 {
    identity: CallableDeclarationIdentityV1,
    owner: FunctionOwnerIdV1,
    parameters: Box<[(BindingRefV1, String, ValueId)]>,
    snapshot: Rc<[String]>,
    physical: MainArgumentsPhysicalBindingsV1,
}

/// Physical coordinates only; the original source product stays in its owner.
#[derive(Debug)]
struct MainArgumentsPhysicalBindingsV1 {
    allocation: Option<(BasicBlockId, MirInstruction, BasicBlockId, MirInstruction)>,
    releases: Vec<(BasicBlockId, MirInstruction)>,
    allocation_fault: Option<(BasicBlockId, MirInstruction)>,
    defaults: Vec<(BasicBlockId, MirInstruction)>,
    frame_binding: Option<(BasicBlockId, MirInstruction)>,
}

impl MainArgumentsEmissionV1 {
    /// Called only inside the original AppMain source loan after root selection.
    pub(super) fn emit(
        builder: &mut MirBuilder,
        input: ResolvedFunctionLoweringInputV1<'_>,
        identity: &CallableDeclarationIdentityV1,
        runtime: &NormalRuntimeInputSnapshotV1,
        frame_binding: Option<(BasicBlockId, MirInstruction)>,
    ) -> Result<Self, String> {
        let owner = input.function();
        let mut rows = owner
            .declaration_sites()
            .filter_map(|site| match site {
                SourceBindingSiteV1::Parameter { index } => Some(*index),
                _ => None,
            })
            .collect::<Vec<_>>();
        rows.sort_unstable();
        let mut source_parameters = Vec::new();
        for (ordinal, index) in rows.into_iter().enumerate() {
            if index as usize != ordinal {
                return Err(fault("parameter-order"));
            }
            let binding = owner
                .declaration_binding(&SourceBindingSiteV1::Parameter { index })
                .ok_or_else(|| fault("parameter-binding"))?;
            let name = owner
                .binding(binding)
                .ok_or_else(|| fault("parameter-record"))?
                .diagnostic_name()
                .to_owned();
            source_parameters.push((binding, name));
        }
        if source_parameters
            .iter()
            .filter(|(_, name)| name == "args")
            .count()
            > 1
        {
            return Err(fault("duplicate-args"));
        }
        let mut emission = Self {
            identity: identity.clone(),
            owner: input.owner(),
            parameters: Box::new([]),
            snapshot: runtime.script_args_snapshot(),
            physical: MainArgumentsPhysicalBindingsV1 {
                allocation: None,
                releases: Vec::new(),
                allocation_fault: None,
                defaults: Vec::new(),
                frame_binding,
            },
        };
        let mut parameters = Vec::new();
        for (binding, name) in source_parameters {
            let value = if name == "args" {
                let frame = match emission.physical.frame_binding.as_ref().map(|row| &row.1) {
                    Some(MirInstruction::FaultFrameEnter {
                        dst,
                        mode: crate::mir::instruction::FaultFrameMode::RootOwned,
                    }) => *dst,
                    _ => return Err(fault("frame-missing")),
                };
                let origin = builder
                    .current_block_id()
                    .ok_or_else(|| fault("block-missing"))?;
                let normal = builder.next_block_id();
                let fault_block = builder.next_block_id();
                let mut failure = BasicBlock::new(fault_block);
                let failed = MirInstruction::ReturnFault { fault_frame: frame };
                failure.set_terminator(failed.clone());
                emission.physical.allocation_fault = Some((fault_block, failed));
                if builder
                    .function_state
                    .current_function
                    .as_ref()
                    .map(|f| f.entry_block)
                    != Some(origin)
                {
                    return Err(fault("allocation-entry"));
                }
                builder
                    .function_state
                    .current_function
                    .as_mut()
                    .ok_or_else(|| fault("function-missing"))?
                    .add_block(failure);
                let operation = MirInstruction::Invoke {
                    operation: InvokeOperation::ScriptArgumentsNew {
                        values: emission.snapshot.as_ref().into(),
                    },
                    fault_frame: frame,
                    normal_landing: normal,
                    fault_landing: fault_block,
                };
                builder.emit_instruction(operation.clone())?;
                builder.start_new_block(normal)?;
                let value = builder.next_value_id();
                let result = MirInstruction::InvokeNormalResult {
                    dst: value,
                    invoke_block: origin,
                };
                builder.emit_instruction(result.clone())?;
                builder
                    .function_state
                    .type_ctx
                    .value_origin_newbox
                    .insert(value, "ArrayBox".into());
                builder
                    .function_state
                    .type_ctx
                    .value_types
                    .insert(value, MirType::Box("ArrayBox".into()));
                emission.physical.allocation = Some((origin, operation, normal, result));
                value
            } else {
                let value = crate::mir::builder::emission::constant::emit_void(builder)?;
                emission.physical.defaults.push((
                    builder
                        .current_block_id()
                        .ok_or_else(|| fault("block-missing"))?,
                    MirInstruction::Const {
                        dst: value,
                        value: crate::mir::ConstValue::Void,
                    },
                ));
                value
            };
            builder
                .function_state
                .variable_ctx
                .variable_map
                .insert(name.clone(), value);
            parameters.push((binding, name, value));
        }
        emission.parameters = parameters.into();
        Ok(emission)
    }

    pub(super) fn corroborate_adoption(
        &self,
        builder: &MirBuilder,
        state: &super::normal_callable_semantic_lowering_state::CallableSemanticLoweringState,
    ) -> Result<(), String> {
        if self.parameters.iter().any(|(_, name, value)| {
            builder.function_state.variable_ctx.variable_map.get(name) != Some(value)
        }) || self.parameters.iter().any(|(binding, _, value)| {
            state.value_for_exact_binding(self.owner, *binding).ok() != Some(*value)
        }) {
            return Err(fault("entry-adoption"));
        }
        Ok(())
    }

    /// The outer wrapper releases after the source body's own cleanup, without
    /// manufacturing a source Home. Only live paths receive this exact suffix.
    pub(super) fn finish(&mut self, function: &mut MirFunction) -> Result<(), String> {
        let Some((_, _, normal, _)) = &self.physical.allocation else {
            return Ok(());
        };
        let value = self.argv_value()?;
        let mut pending = VecDeque::from([*normal]);
        let mut visited = std::collections::BTreeSet::new();
        while let Some(id) = pending.pop_front() {
            if !visited.insert(id) {
                continue;
            }
            let block = function
                .blocks
                .get_mut(&id)
                .ok_or_else(|| fault("block-missing"))?;
            if matches!(
                block.terminator,
                Some(MirInstruction::Return { .. } | MirInstruction::ReturnFault { .. })
            ) {
                let release = MirInstruction::ArrayResidenceRelease { value };
                block.add_instruction_before_terminator(release.clone());
                self.physical.releases.push((id, release));
            } else {
                pending.extend(block.out_edges().into_iter().map(|edge| edge.target));
            }
        }
        self.validate(function)
    }

    fn argv_value(&self) -> Result<ValueId, String> {
        self.parameters
            .iter()
            .find(|(_, name, _)| name == "args")
            .map(|(_, _, value)| *value)
            .ok_or_else(|| fault("args-binding"))
    }

    pub(in crate::mir) fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir) fn corroborates_source(
        &self,
        owner: FunctionOwnerIdV1,
        identity: &CallableDeclarationIdentityV1,
    ) -> bool {
        self.owner == owner
            && self.identity.same_as(identity)
            && self
                .parameters
                .iter()
                .all(|(binding, _, _)| binding.owner() == owner)
    }

    pub(in crate::mir) fn original_bindings(&self) -> Vec<(BasicBlockId, MirInstruction)> {
        self.physical.bindings()
    }

    /// Every original effect must survive the SAME checked finishing projection.
    /// Only after all originals map may converged physical release rows coalesce.
    pub(in crate::mir) fn projected_bindings(
        &self,
        function: &MirFunction,
        mut map: impl FnMut(
            &(BasicBlockId, MirInstruction),
        ) -> Result<(BasicBlockId, MirInstruction), String>,
    ) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        let allocation = self
            .physical
            .allocation
            .as_ref()
            .map(|(block, invoke, normal, result)| {
                let (block, invoke) = map(&(*block, invoke.clone()))?;
                let (normal, result) = map(&(*normal, result.clone()))?;
                Ok::<_, String>((block, invoke, normal, result))
            })
            .transpose()?;
        let mut releases = Vec::new();
        for original in &self.physical.releases {
            let mapped = map(original)?;
            if !releases.contains(&mapped) {
                releases.push(mapped);
            }
        }
        let physical = MainArgumentsPhysicalBindingsV1 {
            allocation,
            releases,
            allocation_fault: self
                .physical
                .allocation_fault
                .as_ref()
                .map(&mut map)
                .transpose()?,
            defaults: self
                .physical
                .defaults
                .iter()
                .map(&mut map)
                .collect::<Result<_, _>>()?,
            frame_binding: self
                .physical
                .frame_binding
                .as_ref()
                .map(&mut map)
                .transpose()?,
        };
        self.validate_physical(function, &physical)?;
        Ok(physical.bindings())
    }

    pub(in crate::mir) fn validate(&self, function: &MirFunction) -> Result<(), String> {
        self.validate_physical(function, &self.physical)
    }

    fn validate_physical(
        &self,
        function: &MirFunction,
        physical: &MainArgumentsPhysicalBindingsV1,
    ) -> Result<(), String> {
        for (block, instruction) in physical
            .defaults
            .iter()
            .chain(physical.frame_binding.iter())
        {
            if function.blocks.get(block).map_or(true, |block| {
                block
                    .instructions
                    .iter()
                    .filter(|row| *row == instruction)
                    .count()
                    != 1
            }) {
                return Err(fault("default-binding"));
            }
        }
        let expected = usize::from(physical.allocation.is_some());
        let actual = function
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .filter(|row| {
                matches!(
                    row,
                    MirInstruction::Invoke {
                        operation: InvokeOperation::ScriptArgumentsNew { .. },
                        ..
                    }
                )
            })
            .count();
        if actual != expected {
            return Err(fault("allocation-census"));
        }
        let Some((origin, operation, normal, result)) = &physical.allocation else {
            return Ok(());
        };
        if !matches!(operation, MirInstruction::Invoke { operation: InvokeOperation::ScriptArgumentsNew { values }, .. }
            if values.as_ref() == self.snapshot.as_ref())
            || function
                .blocks
                .get(origin)
                .and_then(|block| block.terminator.as_ref())
                != Some(operation)
            || function.blocks.get(normal).map_or(true, |block| {
                block
                    .instructions
                    .iter()
                    .filter(|row| *row == result)
                    .count()
                    != 1
            })
        {
            return Err(fault("allocation-binding"));
        }
        if function.entry_block != *origin {
            return Err(fault("allocation-entry"));
        }
        let (failed, terminal) = physical
            .allocation_fault
            .as_ref()
            .ok_or_else(|| fault("allocation-fault-missing"))?;
        let failure = function
            .blocks
            .get(failed)
            .ok_or_else(|| fault("allocation-fault-missing"))?;
        if !failure.instructions.is_empty() || failure.terminator.as_ref() != Some(terminal) {
            return Err(fault("allocation-fault-binding"));
        }
        let value = self.argv_value()?;
        let origins = crate::mir::value_origin::AcyclicUnanimousValueOrigins::new(function);
        // Exact physical coverage includes disconnected blocks; the CFG walk
        // below separately proves reachable residence and cleanup ordering.
        let mut physical_releases = function
            .blocks
            .iter()
            .flat_map(|(id, block)| {
                block.instructions.iter().filter_map(|row| match row {
                    MirInstruction::ArrayResidenceRelease { value: released }
                        if origins.origin(*released) == Some(value) =>
                    {
                        Some((*id, row.clone()))
                    }
                    _ => None,
                })
            })
            .collect::<Vec<_>>();
        let mut states = BTreeMap::new();
        let mut pending = VecDeque::from([(function.entry_block, false)]);
        let mut seen_releases = Vec::new();
        while let Some((id, mut live)) = pending.pop_front() {
            if let Some(old) = states.insert(id, live) {
                if old != live {
                    return Err(fault("mixed-residence-path"));
                }
                continue;
            }
            let block = function
                .blocks
                .get(&id)
                .ok_or_else(|| fault("block-missing"))?;
            for (index, row) in block.instructions.iter().enumerate() {
                let uses_argv = row
                    .used_values()
                    .into_iter()
                    .any(|used| origins.origin(used) == Some(value));
                if uses_argv && !live {
                    return Err(fault("use-outside-residence"));
                }
                if let MirInstruction::Phi { dst, .. } = row {
                    if uses_argv && origins.origin(*dst) != Some(value) {
                        return Err(fault("ambiguous-argv-alias"));
                    }
                }
                if uses_argv
                    && !matches!(
                        row,
                        MirInstruction::Copy { .. }
                            | MirInstruction::Phi { .. }
                            | MirInstruction::ArrayResidenceRelease { .. }
                    )
                {
                    return Err(fault("argument-use-source-unconnected"));
                }
                if matches!(row, MirInstruction::ArrayResidenceRelease { value: released }
                    if origins.origin(*released) == Some(value))
                {
                    if !matches!(row, MirInstruction::ArrayResidenceRelease { value: released } if *released == value)
                    {
                        return Err(fault("alias-release"));
                    }
                    if !live
                        || index + 1 != block.instructions.len()
                        || !matches!(
                            block.terminator,
                            Some(
                                MirInstruction::Return { .. } | MirInstruction::ReturnFault { .. }
                            )
                        )
                    {
                        return Err(fault("release-order"));
                    }
                    live = false;
                    seen_releases.push((id, row.clone()));
                }
            }
            if let Some(terminal) = block.terminator.as_ref() {
                let uses_argv = terminal
                    .used_values()
                    .into_iter()
                    .any(|used| origins.origin(used) == Some(value));
                // Original body operations need their own source-issued borrow
                // consumer. This wrapper resource grants transport and teardown,
                // never permission to call/store/transfer an Array by its word.
                if uses_argv {
                    return Err(fault("argument-use-source-unconnected"));
                }
            }
            match block.terminator.as_ref() {
                Some(row) if id == *origin && row == operation => {
                    if live {
                        return Err(fault("allocation-reentry"));
                    }
                    let MirInstruction::Invoke {
                        normal_landing,
                        fault_landing,
                        ..
                    } = row
                    else {
                        unreachable!()
                    };
                    pending.push_back((*normal_landing, true));
                    pending.push_back((*fault_landing, false));
                }
                Some(MirInstruction::Return { .. } | MirInstruction::ReturnFault { .. }) => {
                    if live {
                        return Err(fault("unreleased-exit"));
                    }
                }
                Some(_) => pending.extend(
                    block
                        .out_edges()
                        .into_iter()
                        .map(|edge| (edge.target, live)),
                ),
                None => return Err(fault("terminator-missing")),
            }
        }
        if !states.contains_key(origin) || !states.contains_key(normal) {
            return Err(fault("allocation-unreachable"));
        }
        let mut expected_releases = physical.releases.clone();
        seen_releases.sort_by_key(|(block, _)| *block);
        expected_releases.sort_by_key(|(block, _)| *block);
        physical_releases.sort_by_key(|(block, _)| *block);
        if seen_releases != expected_releases || physical_releases != expected_releases {
            return Err(fault("release-census"));
        }
        Ok(())
    }
}
impl MainArgumentsPhysicalBindingsV1 {
    fn bindings(&self) -> Vec<(BasicBlockId, MirInstruction)> {
        let mut bindings = self.defaults.clone();
        bindings.extend(self.frame_binding.iter().cloned());
        bindings.extend(self.releases.clone());
        if let Some((block, operation, normal, result)) = &self.allocation {
            bindings.push((*block, operation.clone()));
            bindings.push((*normal, result.clone()));
        }
        bindings.extend(self.allocation_fault.iter().cloned());
        bindings
    }
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][main-arguments/{reason}]")
}
