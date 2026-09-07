//! Physical bindings beside the source continuation; source classification stays there.
use super::*;
use crate::mir::builder::collection_literals::array_emission::{
    last_instruction, ArrayLiteralEmission,
};
use crate::mir::builder::raw_structured_child_scope::PreparedRawChildSourceV1;
use crate::mir::builder::stmts::LocalInitializerObservationV1;
use crate::mir::resolved_semantics::{ResolvedInitializerRelationV1, SourceExprSiteV1};
use crate::mir::{BasicBlockId, ConstValue, MirBuilder, MirFunction, MirInstruction};

#[derive(Debug, Default)]
pub(super) struct ArrayEmissionBindings {
    rows: BTreeMap<BindingRefV1, BoundArray>,
    root_bound: bool,
    terminal: Option<RootReturnEmission>,
}

#[derive(Debug)]
struct BoundArray {
    initializer: SourceExprSiteV1,
    spec: crate::typed_array_contract_spec::ArrayElementContractSpec,
    children: Vec<SourceExprSiteV1>,
    literal: ArrayLiteralEmission,
    local: ValueId,
    local_slot: crate::mir::LocalSlotId,
    local_site: (BasicBlockId, usize),
}

#[derive(Debug)]
struct RootReturnEmission {
    block: BasicBlockId,
    instruction: MirInstruction,
    definition: Option<((BasicBlockId, usize), MirInstruction)>,
}

impl ScriptSemanticLoweringState {
    pub(in crate::mir::builder) fn record_array_local_emission(
        &mut self,
        builder: &MirBuilder,
        relation: &ResolvedInitializerRelationV1,
        local: ValueId,
        observations: Vec<LocalInitializerObservationV1>,
    ) -> Result<(), String> {
        let Some((spec, children)) = self.continuation.array_element_sites(relation)? else {
            return Ok(());
        };
        let [observation]: [LocalInitializerObservationV1; 1] = observations
            .try_into()
            .map_err(|_| fault("initializer-count"))?;
        let initializer = relation
            .initializer_site()
            .ok_or_else(|| fault("initializer-missing"))?;
        if observation.ordinal != 0
            || !matches!(&observation.source,
            PreparedRawChildSourceV1::Exact(context) if context.site() == Some(initializer.node()))
        {
            return Err(fault("initializer-source"));
        }
        let literal = observation
            .array
            .ok_or_else(|| fault("literal-emission-missing"))?;
        if observation.value != literal.allocation || children.len() != literal.elements.len() {
            return Err(fault("initializer-value-or-children"));
        }
        let (local_site, instruction) = last_instruction(builder)?;
        if !matches!(instruction, MirInstruction::Copy { dst, src }
            if *dst == local && *src == literal.allocation)
        {
            return Err(fault("local-emission"));
        }
        let function = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(|| fault("root-missing"))?;
        let carriers: Vec<_> = function
            .metadata
            .typed_array_contract_sources
            .iter()
            .filter(|source| source.contract_id == literal.claim)
            .collect();
        let [carrier] = carriers.as_slice() else {
            return Err(fault("claim-source-carrier"));
        };
        let crate::mir::function::TypedArrayContractSourceIdentity::LocalSlot(local_slot) =
            &carrier.source_identity
        else {
            return Err(fault("claim-local-slot"));
        };
        if self.array_emissions.rows.contains_key(&relation.binding()) {
            return Err(fault("duplicate-local"));
        }
        self.array_emissions.rows.insert(
            relation.binding(),
            BoundArray {
                initializer: initializer.clone(),
                spec,
                children,
                literal,
                local,
                local_slot: *local_slot,
                local_site,
            },
        );
        Ok(())
    }

    pub(in crate::mir::builder) fn record_array_root_return(
        &mut self,
        builder: &MirBuilder,
        site: &SourceNodeSiteV1,
    ) -> Result<(), String> {
        use super::super::normal_script_source_continuation::RootResult;
        let Some(source) = self.continuation.array_root_terminal()? else {
            return Ok(());
        };
        if source.site().node() != site || self.array_emissions.terminal.is_some() {
            return Err(fault("return-source-or-duplicate"));
        }
        let root = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(|| fault("root-missing"))?;
        let block = builder
            .function_state
            .current_block
            .ok_or_else(|| fault("return-block"))?;
        let instruction = root
            .blocks
            .get(&block)
            .and_then(|body| body.terminator.as_ref())
            .ok_or_else(|| fault("return-missing"))?;
        let definition = match (source.result(), instruction) {
            (RootResult::Unit, MirInstruction::Return { value: None }) => None,
            (result, MirInstruction::Return { value: Some(value) }) => {
                let (position, definition) = last_instruction(builder)?;
                match (result, definition) {
                    (
                        RootResult::Unit,
                        MirInstruction::Const {
                            dst,
                            value: ConstValue::Void,
                        },
                    ) if dst == value => {}
                    (
                        RootResult::Integer {
                            value: expected, ..
                        },
                        MirInstruction::Const {
                            dst,
                            value: ConstValue::Integer(actual),
                        },
                    ) if dst == value && actual == expected => {}
                    _ => return Err(fault("return-result-binding")),
                }
                Some((position, definition.clone()))
            }
            _ => return Err(fault("return-operation")),
        };
        self.array_emissions.terminal = Some(RootReturnEmission {
            block,
            instruction: instruction.clone(),
            definition,
        });
        Ok(())
    }

    pub(in crate::mir::builder) fn bind_array_root(
        &mut self,
        root: &MirFunction,
    ) -> Result<(), String> {
        self.finish_source_claims()?;
        let bindings = self.continuation.array_bindings()?;
        self.array_emissions.check_bindings(&bindings)?;
        if self.array_emissions.root_bound {
            return Err(fault("duplicate-root-bind"));
        }
        self.array_emissions.validate(root, false, &bindings)?;
        self.array_emissions.root_bound = true;
        Ok(())
    }

    pub(in crate::mir::builder) fn validate_finished_array_root(
        &mut self,
        root: &MirFunction,
    ) -> Result<(), String> {
        self.finish_source_claims()?;
        let bindings = self.continuation.array_bindings()?;
        self.array_emissions.check_bindings(&bindings)?;
        if !self.array_emissions.root_bound {
            return Err(fault("root-unbound"));
        }
        self.array_emissions.validate(root, true, &bindings)
    }
}

impl ArrayEmissionBindings {
    fn check_bindings(&self, expected: &[BindingRefV1]) -> Result<(), String> {
        if self.rows.keys().copied().collect::<BTreeSet<_>>()
            != expected.iter().copied().collect::<BTreeSet<_>>()
        {
            return Err(fault("source-binding-set"));
        }
        Ok(())
    }

    fn validate(
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
        let mut prior = None;
        for binding in bindings {
            let row = self
                .rows
                .get(binding)
                .ok_or_else(|| fault("source-binding-set"))?;
            let literal = &row.literal;
            if root.entry_block != literal.entry
                || row.children.len() != literal.elements.len()
                || row.children.iter().any(|site| site == &row.initializer)
            {
                return Err(fault("source-or-root-drift"));
            }
            let allocation = index.definition(literal.allocation)?;
            if !matches!(allocation.1, MirInstruction::NewBox { target: crate::mir::ConstructionTarget::IntrinsicArray, args, .. } if args.is_empty())
            {
                return Err(fault("allocation-drift"));
            }
            check_position(allocation.0, literal.allocation_site, finishing)?;
            if let Some(previous) = prior {
                ordered(previous, allocation.0)?;
            }
            let claim = index
                .claims
                .get(literal.claim.as_str())
                .copied()
                .ok_or_else(|| fault("operation-missing"))?;
            if !matches!(claim.1, MirInstruction::ArrayStateContractClaim { array, .. } if *array == literal.allocation)
            {
                return Err(fault("claim-operand-drift"));
            }
            let source = index
                .carriers
                .get(literal.claim.as_str())
                .ok_or_else(|| fault("claim-source-carrier"))?;
            if source.element_spec != row.spec
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
            check_position(claim.0, literal.claim_site, finishing)?;
            ordered(allocation.0, claim.0)?;
            let mut previous = claim.0;
            for element in &literal.elements {
                let definition = index.definition(element.value)?;
                check_position(definition.0, element.definition_site, finishing)?;
                validate_definition(element.value, &element.definition, definition.1)?;
                let write = index
                    .writes
                    .get(&element.write)
                    .copied()
                    .ok_or_else(|| fault("operation-missing"))?;
                if !matches!(write.1, MirInstruction::ArrayElementWrite {
                    dst: None, kind: crate::mir::ArrayElementWriteKind::LiteralAppend,
                    producer: crate::mir::ArrayWriteProducerKind::Literal,
                    receiver, index: None, value, ..
                } if *receiver == literal.allocation && *value == element.value)
                {
                    return Err(fault("write-operand-drift"));
                }
                check_position(write.0, element.write_site, finishing)?;
                ordered(previous, definition.0)?;
                ordered(definition.0, write.0)?;
                previous = write.0;
            }
            let local = index.definitions.get(&row.local).copied();
            if let Some((site, instruction)) = local {
                if !matches!(instruction, MirInstruction::Copy { src, .. } if *src == literal.allocation)
                {
                    return Err(fault("local-operand-drift"));
                }
                check_position(site, row.local_site, finishing)?;
                ordered(previous, site)?;
                previous = site;
            } else if !finishing || index.used.contains(&row.local) {
                return Err(fault("local-missing"));
            }
            prior = Some(previous);
        }
        if !self.rows.is_empty() {
            let terminal = self
                .terminal
                .as_ref()
                .ok_or_else(|| fault("return-unbound"))?;
            let body = root
                .blocks
                .get(&terminal.block)
                .ok_or_else(|| fault("return-block"))?;
            if body.terminator.as_ref() != Some(&terminal.instruction) {
                return Err(fault("return-operand-drift"));
            }
            if root
                .blocks
                .values()
                .filter(|body| matches!(body.terminator, Some(MirInstruction::Return { .. })))
                .count()
                != 1
            {
                return Err(fault("return-cardinality"));
            }
            if let Some((position, definition)) = &terminal.definition {
                let value = definition
                    .dst_value()
                    .ok_or_else(|| fault("return-definition"))?;
                let current = index.definition(value)?;
                check_position(current.0, *position, finishing)?;
                validate_definition(value, definition, current.1)?;
                if let Some(previous) = prior {
                    ordered(previous, current.0)?;
                }
            } else if let Some(previous) = prior {
                ordered(previous, (terminal.block, body.instructions.len()))?;
            }
        } else if self.terminal.is_some() {
            return Err(fault("unexpected-return-binding"));
        }
        Ok(())
    }
}

type Position = (BasicBlockId, usize);
type Located<'a> = (Position, &'a MirInstruction);
// Ephemeral physical lookup, built once per validation. It never creates a
// source correspondence; every requested identity was retained at emission.
struct EmissionIndex<'a> {
    definitions: BTreeMap<ValueId, Located<'a>>,
    claims: BTreeMap<&'a str, Located<'a>>,
    writes: BTreeMap<crate::mir::ArrayWriteSiteId, Located<'a>>,
    carriers: BTreeMap<&'a str, &'a crate::mir::function::TypedArrayContractSource>,
    used: BTreeSet<ValueId>,
}
impl<'a> EmissionIndex<'a> {
    fn build(root: &'a MirFunction) -> Result<Self, String> {
        let mut index = Self {
            definitions: BTreeMap::new(),
            claims: BTreeMap::new(),
            writes: BTreeMap::new(),
            carriers: BTreeMap::new(),
            used: BTreeSet::new(),
        };
        for (block, body) in &root.blocks {
            for (position, instruction) in body.instructions.iter().enumerate() {
                let location = ((*block, position), instruction);
                if let Some(dst) = instruction.dst_value() {
                    if index.definitions.insert(dst, location).is_some() {
                        return Err(fault("duplicate-operation"));
                    }
                }
                match instruction {
                    MirInstruction::ArrayStateContractClaim { contract_id, .. } => {
                        if index.claims.insert(contract_id, location).is_some() {
                            return Err(fault("duplicate-operation"));
                        }
                    }
                    MirInstruction::ArrayElementWrite { site_id, .. } => {
                        if index.writes.insert(*site_id, location).is_some() {
                            return Err(fault("duplicate-operation"));
                        }
                    }
                    _ => {}
                }
            }
            for instruction in body.all_instructions() {
                index.used.extend(instruction.used_values());
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
                value: constant,
            },
            MirInstruction::Const {
                dst: actual,
                value: actual_value,
            },
        ) if *dst == value && actual == dst && same_constant(actual_value, constant) => Ok(()),
        _ => Err(fault("primitive-definition-drift")),
    }
}
fn fault(reason: &str) -> String {
    format!("[freeze:contract][script-array/emission/{reason}]")
}

fn same_constant(left: &ConstValue, right: &ConstValue) -> bool {
    match (left, right) {
        (ConstValue::Float(left), ConstValue::Float(right)) => left.to_bits() == right.to_bits(),
        _ => left == right,
    }
}
