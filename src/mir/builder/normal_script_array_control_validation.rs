//! Exact Script control coverage; source obligations come only from retained Recipes.
use super::*;
use crate::mir::builder::collection_literals::array_emission::{
    ArrayCleanupEmission, ArrayInvokeSite,
};
use crate::mir::builder::normal_script_source_continuation::ArrayReleaseRoleV1;
use crate::mir::instruction::{FaultFrameMode, InvokeOperation};

type Position = (BasicBlockId, usize);
type Located<'a> = (Position, &'a MirInstruction);

impl ArrayEmissionBindings {
    pub(super) fn check_bindings(&self, expected: &[BindingRefV1]) -> Result<(), String> {
        if self.rows.keys().copied().collect::<BTreeSet<_>>() != expected.iter().copied().collect()
        {
            return Err(fault("source-binding-set"));
        }
        Ok(())
    }

    pub(super) fn validate(
        &self,
        root: &MirFunction,
        finishing: bool,
        bindings: &[BindingRefV1],
    ) -> Result<(), String> {
        if self.rows.is_empty() {
            return if self.terminal.is_none() {
                Ok(())
            } else {
                Err(fault("unexpected-return-binding"))
            };
        }
        let index = EmissionIndex::build(root)?;
        let mut covered = BTreeSet::new();
        let mut prior = None;
        for binding in bindings {
            let row = self
                .rows
                .get(binding)
                .ok_or_else(|| fault("source-binding-set"))?;
            let literal = &row.literal;
            if root.entry_block != literal.entry
                || literal.recipe.elements().len() != literal.elements.len()
            {
                return Err(fault("source-or-root-drift"));
            }
            let frame = index.definition(literal.frame)?;
            if frame.0 .0 != root.entry_block {
                return Err(fault("frame-entry"));
            }
            cover(
                root,
                &mut covered,
                frame.0,
                &MirInstruction::FaultFrameEnter {
                    dst: literal.frame,
                    mode: FaultFrameMode::RootOwned,
                },
            )?;
            validate_cleanup(
                root,
                &mut covered,
                &literal.allocation_fault,
                literal.recipe.allocation_fault(),
                None,
                literal.frame,
                &self.rows,
            )?;
            validate_cleanup(
                root,
                &mut covered,
                &literal.acquired_fault,
                literal.recipe.acquired_fault(),
                Some(literal.allocation),
                literal.frame,
                &self.rows,
            )?;
            if literal.allocation_control.fault != literal.allocation_fault.block
                || literal.claim_control.fault != literal.acquired_fault.block
                || literal.claim_control.origin != literal.allocation_control.normal
            {
                return Err(fault("allocation-claim-control"));
            }
            let origin = invoke(
                root,
                &mut covered,
                &literal.allocation_control,
                literal.frame,
                InvokeOperation::IntrinsicArrayNew,
            )?;
            if let Some(previous) = prior {
                ordered(previous, origin)?;
            } else if origin.0 != root.entry_block {
                return Err(fault("allocation-entry"));
            }
            let allocation = index.definition(literal.allocation)?;
            check_position(allocation.0, literal.allocation_site, finishing)?;
            if allocation.0 .0 != literal.allocation_control.normal {
                return Err(fault("allocation-result-block"));
            }
            cover(
                root,
                &mut covered,
                allocation.0,
                &MirInstruction::InvokeNormalResult {
                    dst: literal.allocation,
                    invoke_block: literal.allocation_control.origin,
                },
            )?;
            let claim = invoke(
                root,
                &mut covered,
                &literal.claim_control,
                literal.frame,
                InvokeOperation::ArrayStateContractClaim {
                    contract_id: literal.claim.clone(),
                    array: literal.allocation,
                },
            )?;
            ordered(allocation.0, claim)?;
            let source = index
                .carriers
                .get(literal.claim.as_str())
                .ok_or_else(|| fault("claim-source-carrier"))?;
            if source.element_spec != literal.recipe.spec()
                || source.boundary_value
                    != crate::mir::function::TypedArrayBoundaryValue::Value(literal.allocation)
                || source.boundary != crate::mir::function::TypedArrayContractBoundary::LocalInit
                || source.source_identity
                    != crate::mir::function::TypedArrayContractSourceIdentity::LocalSlot(
                        row.local_slot,
                    )
            {
                return Err(fault("claim-source-carrier"));
            }
            let mut normal = literal.claim_control.normal;
            for element in &literal.elements {
                if element.control.origin != normal
                    || element.control.fault != literal.acquired_fault.block
                {
                    return Err(fault("write-control"));
                }
                let definition = index.definition(element.value)?;
                check_position(definition.0, element.definition_site, finishing)?;
                validate_definition(element.value, &element.definition, definition.1)?;
                let write = invoke(
                    root,
                    &mut covered,
                    &element.control,
                    literal.frame,
                    InvokeOperation::ArrayElementWrite {
                        site_id: element.write,
                        kind: crate::mir::ArrayElementWriteKind::LiteralAppend,
                        producer: crate::mir::ArrayWriteProducerKind::Literal,
                        receiver: literal.allocation,
                        index: None,
                        value: element.value,
                    },
                )?;
                ordered(definition.0, write)?;
                normal = element.control.normal;
            }
            // Every committed Home remains used by its exact cleanup operand.
            let local = index.definition(row.local)?;
            if !matches!(local.1, MirInstruction::Copy { src, .. } if *src == literal.allocation)
                || local.0 .0 != normal
            {
                return Err(fault("local-operand-drift"));
            }
            check_position(local.0, row.local_site, finishing)?;
            prior = Some(local.0);
        }
        let terminal = self
            .terminal
            .as_ref()
            .ok_or_else(|| fault("return-unbound"))?;
        let body = root
            .blocks
            .get(&terminal.block)
            .ok_or_else(|| fault("return-block"))?;
        cover(
            root,
            &mut covered,
            (terminal.block, body.instructions.len()),
            &terminal.instruction,
        )?;
        let expected_releases = home_values(terminal.recipe.releases(), None, &self.rows)?;
        if terminal.releases != expected_releases {
            return Err(fault("return-home-binding"));
        }
        let start = body
            .instructions
            .len()
            .checked_sub(expected_releases.len())
            .ok_or_else(|| fault("return-release-count"))?;
        for (offset, value) in expected_releases.iter().enumerate() {
            cover(
                root,
                &mut covered,
                (terminal.block, start + offset),
                &MirInstruction::ArrayResidenceRelease { value: *value },
            )?;
        }
        if let Some((position, definition)) = &terminal.definition {
            let value = definition
                .dst_value()
                .ok_or_else(|| fault("return-definition"))?;
            let current = index.definition(value)?;
            check_position(current.0, *position, finishing)?;
            validate_definition(value, definition, current.1)?;
            ordered(current.0, (terminal.block, start))?;
            if let Some(previous) = prior {
                ordered(previous, current.0)?;
            }
        } else if let Some(previous) = prior {
            ordered(previous, (terminal.block, start))?;
        }
        // No selected operation/edge may hide outside the source-bound coverage.
        for (block, body) in &root.blocks {
            for (offset, instruction) in body.all_instructions().enumerate() {
                if (instruction.requires_lifecycle_validation()
                    || offset == body.instructions.len())
                    && !covered.contains(&(*block, offset))
                {
                    return Err(fault("uncovered-control"));
                }
                if matches!(
                    instruction,
                    MirInstruction::ArrayStateContractClaim { .. }
                        | MirInstruction::ArrayElementWrite { .. }
                        | MirInstruction::NewBox {
                            target: crate::mir::ConstructionTarget::IntrinsicArray,
                            ..
                        }
                ) {
                    return Err(fault("standalone-array-operation"));
                }
            }
        }
        crate::mir::MirVerifier::new_strict()
            .verify_function(root)
            .map_err(|errors| format!("{} {errors:?}", fault("invalid-control")))
    }
}

fn home_values(
    roles: &[ArrayReleaseRoleV1],
    incomplete: Option<ValueId>,
    rows: &BTreeMap<BindingRefV1, BoundArray>,
) -> Result<Vec<ValueId>, String> {
    roles
        .iter()
        .map(|role| match role {
            ArrayReleaseRoleV1::IncompleteResidence => {
                incomplete.ok_or_else(|| fault("unacquired-release"))
            }
            ArrayReleaseRoleV1::Home(binding) => rows
                .get(binding)
                .map(|row| row.local)
                .ok_or_else(|| fault("home-binding")),
        })
        .collect()
}
fn validate_cleanup(
    root: &MirFunction,
    covered: &mut BTreeSet<Position>,
    cleanup: &ArrayCleanupEmission,
    roles: &[ArrayReleaseRoleV1],
    incomplete: Option<ValueId>,
    frame: ValueId,
    rows: &BTreeMap<BindingRefV1, BoundArray>,
) -> Result<(), String> {
    let values = home_values(roles, incomplete, rows)?;
    if cleanup
        .releases
        .iter()
        .map(|(role, _)| role)
        .ne(roles.iter())
        || cleanup
            .releases
            .iter()
            .map(|(_, value)| *value)
            .ne(values.iter().copied())
    {
        return Err(fault("fault-home-binding"));
    }
    let body = root
        .blocks
        .get(&cleanup.block)
        .ok_or_else(|| fault("fault-block"))?;
    if body.instructions.len() != values.len() {
        return Err(fault("fault-release-count"));
    }
    for (offset, value) in values.iter().enumerate() {
        cover(
            root,
            covered,
            (cleanup.block, offset),
            &MirInstruction::ArrayResidenceRelease { value: *value },
        )?;
    }
    cover(
        root,
        covered,
        (cleanup.block, values.len()),
        &MirInstruction::ReturnFault { fault_frame: frame },
    )
}
fn invoke(
    root: &MirFunction,
    covered: &mut BTreeSet<Position>,
    site: &ArrayInvokeSite,
    frame: ValueId,
    operation: InvokeOperation,
) -> Result<Position, String> {
    let body = root
        .blocks
        .get(&site.origin)
        .ok_or_else(|| fault("invoke-block"))?;
    let position = (site.origin, body.instructions.len());
    cover(
        root,
        covered,
        position,
        &MirInstruction::Invoke {
            operation,
            fault_frame: frame,
            normal_landing: site.normal,
            fault_landing: site.fault,
        },
    )?;
    Ok(position)
}
fn cover(
    root: &MirFunction,
    covered: &mut BTreeSet<Position>,
    position: Position,
    expected: &MirInstruction,
) -> Result<(), String> {
    let actual = root
        .blocks
        .get(&position.0)
        .and_then(|body| body.all_instructions().nth(position.1));
    if actual != Some(expected) {
        return Err(fault("control-binding-drift"));
    }
    covered.insert(position);
    Ok(())
}
struct EmissionIndex<'a> {
    definitions: BTreeMap<ValueId, Located<'a>>,
    carriers: BTreeMap<&'a str, &'a crate::mir::function::TypedArrayContractSource>,
}
impl<'a> EmissionIndex<'a> {
    fn build(root: &'a MirFunction) -> Result<Self, String> {
        let mut index = Self {
            definitions: BTreeMap::new(),
            carriers: BTreeMap::new(),
        };
        for (block, body) in &root.blocks {
            for (offset, instruction) in body.instructions.iter().enumerate() {
                if let Some(value) = instruction.dst_value() {
                    if index
                        .definitions
                        .insert(value, ((*block, offset), instruction))
                        .is_some()
                    {
                        return Err(fault("duplicate-operation"));
                    }
                }
            }
        }
        for source in &root.metadata.typed_array_contract_sources {
            if index.carriers.insert(&source.contract_id, source).is_some() {
                return Err(fault("claim-source-carrier"));
            }
        }
        Ok(index)
    }
    fn definition(&self, value: ValueId) -> Result<Located<'a>, String> {
        self.definitions
            .get(&value)
            .copied()
            .ok_or_else(|| fault("operation-missing"))
    }
}
fn ordered(before: Position, after: Position) -> Result<(), String> {
    if before.0 != after.0 || before.1 >= after.1 {
        return Err(fault("operation-order"));
    }
    Ok(())
}
fn check_position(actual: Position, emitted: Position, finishing: bool) -> Result<(), String> {
    if actual.0 != emitted.0 || (!finishing && actual.1 != emitted.1) {
        return Err(fault("operation-position"));
    }
    Ok(())
}
fn validate_definition(
    value: ValueId,
    original: &MirInstruction,
    current: &MirInstruction,
) -> Result<(), String> {
    match (original, current) {
        (
            MirInstruction::Const {
                dst,
                value: expected,
            },
            MirInstruction::Const {
                dst: actual,
                value: observed,
            },
        ) if *dst == value && actual == dst && same_constant(expected, observed) => Ok(()),
        _ => Err(fault("primitive-definition-drift")),
    }
}
fn same_constant(left: &ConstValue, right: &ConstValue) -> bool {
    match (left, right) {
        (ConstValue::Float(a), ConstValue::Float(b)) => a.to_bits() == b.to_bits(),
        _ => left == right,
    }
}
