//! Caller-zero PHI materialization for the bounded portable Loop JoinSig.
//!
//! This is a builder-side seam only.  It consumes a verified logical JoinSig
//! and a sealed logical-to-physical map; it never selects a route, discovers a
//! carrier, or repairs CFG/SSA facts.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::builder::emission::phi_lifecycle::PhiTxn;
use crate::mir::builder::ssa::binding::{BindingSsaIrV1, MirBindingSsaAdapterV1};
use crate::mir::builder::MirBuilder;
use crate::mir::loop_recipe_contract::{
    LoopBindingKeyV1, LoopJoinEdgeRoleV1, LoopJoinPortV1, LoopNodeKeyV1, LoopValueClassV1,
    LoopValueKeyV1, VerifiedLoopJoinSigV1,
};
use crate::mir::{BasicBlockId, MirType, ValueId};

const MATERIALIZER_TAG: &str = "loop_recipe_m6b";

#[derive(Debug)]
pub(in crate::mir::builder) struct LoopLogicalToPhysicalMapInputV1 {
    pub(in crate::mir::builder) ports: Vec<(LoopNodeKeyV1, LoopJoinPortV1, BasicBlockId)>,
    pub(in crate::mir::builder) values: Vec<(LoopValueKeyV1, ValueId, LoopValueClassV1)>,
    pub(in crate::mir::builder) destinations: Vec<(LoopNodeKeyV1, LoopBindingKeyV1, ValueId)>,
    pub(in crate::mir::builder) predecessors: Vec<(BasicBlockId, Vec<BasicBlockId>)>,
}

#[derive(Debug)]
pub(in crate::mir::builder) struct VerifiedLoopLogicalToPhysicalMapV1 {
    ports: BTreeMap<(LoopNodeKeyV1, LoopJoinPortV1), BasicBlockId>,
    values: BTreeMap<LoopValueKeyV1, (ValueId, LoopValueClassV1)>,
    destinations: BTreeMap<(LoopNodeKeyV1, LoopBindingKeyV1), ValueId>,
    predecessors: BTreeMap<BasicBlockId, Box<[BasicBlockId]>>,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum LoopPhiMaterializerErrorV1 {
    Map(String),
    Preflight(String),
    Transaction(String),
}

impl std::fmt::Display for LoopPhiMaterializerErrorV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Map(error) | Self::Preflight(error) | Self::Transaction(error) => {
                f.write_str(error)
            }
        }
    }
}

impl std::error::Error for LoopPhiMaterializerErrorV1 {}

#[derive(Debug, PartialEq, Eq)]
struct PendingPhiV1 {
    loop_key: LoopNodeKeyV1,
    binding: LoopBindingKeyV1,
    block: BasicBlockId,
    dst: ValueId,
    class: LoopValueClassV1,
    expected_predecessors: Box<[BasicBlockId]>,
    inputs: Vec<(BasicBlockId, ValueId)>,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct LoopPhiSiteReceiptV1 {
    pub(in crate::mir::builder) loop_key: LoopNodeKeyV1,
    pub(in crate::mir::builder) binding: LoopBindingKeyV1,
    pub(in crate::mir::builder) block: BasicBlockId,
    pub(in crate::mir::builder) dst: ValueId,
    pub(in crate::mir::builder) inputs: Box<[(BasicBlockId, ValueId)]>,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct LoopPhiMaterializationReceiptV1 {
    sites: Box<[LoopPhiSiteReceiptV1]>,
}

impl LoopPhiMaterializationReceiptV1 {
    pub(in crate::mir::builder) fn sites(&self) -> &[LoopPhiSiteReceiptV1] {
        &self.sites
    }
}

impl VerifiedLoopLogicalToPhysicalMapV1 {
    pub(in crate::mir::builder) fn try_new(
        sig: &VerifiedLoopJoinSigV1,
        input: LoopLogicalToPhysicalMapInputV1,
    ) -> Result<Self, LoopPhiMaterializerErrorV1> {
        let ports = unique_ports(input.ports)?;
        let values = unique_values(input.values)?;
        let destinations = unique_destinations(input.destinations)?;
        let predecessors = input
            .predecessors
            .into_iter()
            .map(|(block, mut rows)| {
                rows.sort_unstable();
                if rows.windows(2).any(|pair| pair[0] == pair[1]) {
                    return Err(map_error(format!(
                        "duplicate predecessor witness block={block:?}"
                    )));
                }
                Ok((block, rows.into_boxed_slice()))
            })
            .collect::<Result<BTreeMap<_, _>, _>>()?;
        let map = Self {
            ports,
            values,
            destinations,
            predecessors,
        };
        map.validate_signature(sig)?;
        Ok(map)
    }

    fn validate_signature(
        &self,
        sig: &VerifiedLoopJoinSigV1,
    ) -> Result<(), LoopPhiMaterializerErrorV1> {
        for row in &sig.as_sig().loops {
            for edge in &row.edges {
                self.port(row.key, edge.from)?;
                self.port(row.key, edge.to)?;
                for payload in &edge.payload {
                    let (_, class) = self.value(payload.value)?;
                    if class != payload.class {
                        return Err(map_error(format!(
                            "payload class mismatch loop={:?} value={:?}",
                            row.key, payload.value
                        )));
                    }
                }
            }
            for carrier in &row.carriers {
                let _ = self.destination(row.key, carrier.binding)?;
                let header = self.port(row.key, LoopJoinPortV1::Header)?;
                let incoming = header_inputs(self, row)?;
                let expected = self.predecessors.get(&header).ok_or_else(|| {
                    map_error(format!("missing predecessor witness block={header:?}"))
                })?;
                if expected.as_ref() != incoming.as_slice() {
                    return Err(map_error(format!(
                        "predecessor mismatch loop={:?} expected={expected:?} actual={incoming:?}",
                        row.key
                    )));
                }
                for edge in row.edges.iter().filter(|edge| is_header_input(edge.role)) {
                    let count = edge
                        .payload
                        .iter()
                        .filter(|payload| payload.binding == carrier.binding)
                        .count();
                    if count != 1 {
                        return Err(map_error(format!(
                            "carrier payload count={} loop={:?} binding={:?}",
                            count, row.key, carrier.binding
                        )));
                    }
                }
            }
        }
        Ok(())
    }

    fn port(
        &self,
        loop_key: LoopNodeKeyV1,
        port: LoopJoinPortV1,
    ) -> Result<BasicBlockId, LoopPhiMaterializerErrorV1> {
        self.ports.get(&(loop_key, port)).copied().ok_or_else(|| {
            map_error(format!(
                "missing physical port loop={loop_key:?} port={port:?}"
            ))
        })
    }

    fn value(
        &self,
        key: LoopValueKeyV1,
    ) -> Result<(ValueId, LoopValueClassV1), LoopPhiMaterializerErrorV1> {
        self.values
            .get(&key)
            .copied()
            .ok_or_else(|| map_error(format!("missing physical value key={key:?}")))
    }

    fn destination(
        &self,
        loop_key: LoopNodeKeyV1,
        binding: LoopBindingKeyV1,
    ) -> Result<ValueId, LoopPhiMaterializerErrorV1> {
        self.destinations
            .get(&(loop_key, binding))
            .copied()
            .ok_or_else(|| {
                map_error(format!(
                    "missing PHI destination loop={loop_key:?} binding={binding:?}"
                ))
            })
    }

    fn pending_phis(
        &self,
        sig: &VerifiedLoopJoinSigV1,
    ) -> Result<Vec<PendingPhiV1>, LoopPhiMaterializerErrorV1> {
        sig.as_sig()
            .loops
            .iter()
            .flat_map(|row| row.carriers.iter().map(move |carrier| (row, carrier)))
            .map(|(row, carrier)| {
                let block = self.port(row.key, LoopJoinPortV1::Header)?;
                let mut inputs = Vec::new();
                let mut class = None;
                for edge in row.edges.iter().filter(|edge| is_header_input(edge.role)) {
                    let incoming = edge
                        .payload
                        .iter()
                        .find(|payload| payload.binding == carrier.binding)
                        .ok_or_else(|| {
                            map_error(format!(
                                "missing carrier payload loop={:?} binding={:?}",
                                row.key, carrier.binding
                            ))
                        })?
                        .value;
                    let (physical, incoming_class) = self.value(incoming)?;
                    class.get_or_insert(incoming_class);
                    inputs.push((self.port(row.key, edge.from)?, physical));
                }
                inputs.sort_unstable_by_key(|(block, value)| (*block, *value));
                Ok(PendingPhiV1 {
                    loop_key: row.key,
                    binding: carrier.binding,
                    block,
                    dst: self.destination(row.key, carrier.binding)?,
                    class: class.ok_or_else(|| {
                        map_error(format!("carrier has no header input loop={:?}", row.key))
                    })?,
                    expected_predecessors: self.predecessors.get(&block).cloned().ok_or_else(
                        || map_error(format!("missing predecessor witness block={block:?}")),
                    )?,
                    inputs,
                })
            })
            .collect()
    }
}

pub(in crate::mir::builder) fn materialize_loop_phis(
    builder: &mut MirBuilder,
    sig: &VerifiedLoopJoinSigV1,
    map: VerifiedLoopLogicalToPhysicalMapV1,
) -> Result<LoopPhiMaterializationReceiptV1, LoopPhiMaterializerErrorV1> {
    materialize_impl(builder, sig, map, None)
}

fn materialize_impl(
    builder: &mut MirBuilder,
    sig: &VerifiedLoopJoinSigV1,
    map: VerifiedLoopLogicalToPhysicalMapV1,
    #[cfg(test)] fail_after: Option<usize>,
    #[cfg(not(test))] _fail_after: Option<usize>,
) -> Result<LoopPhiMaterializationReceiptV1, LoopPhiMaterializerErrorV1> {
    let pending = map.pending_phis(sig)?;
    preflight_builder(builder, &pending)?;
    let mut txn = PhiTxn::begin(MATERIALIZER_TAG);
    let mut sites = Vec::with_capacity(pending.len());
    for (index, row) in pending.into_iter().enumerate() {
        let token = match txn.define_provisional_phi(builder, row.block, row.dst, MATERIALIZER_TAG)
        {
            Ok(token) => token,
            Err(error) => return Err(transaction_error(builder, txn, error)),
        };
        #[cfg(test)]
        if fail_after == Some(index) {
            return Err(transaction_error(
                builder,
                txn,
                "injected M6-B failure after provisional PHI",
            ));
        }
        if let Err(error) =
            txn.patch_phi_inputs(builder, token, row.inputs.clone(), MATERIALIZER_TAG)
        {
            return Err(transaction_error(builder, txn, error));
        }
        sites.push(LoopPhiSiteReceiptV1 {
            loop_key: row.loop_key,
            binding: row.binding,
            block: row.block,
            dst: row.dst,
            inputs: row.inputs.into_boxed_slice(),
        });
    }
    txn.commit(builder)
        .map_err(|error| LoopPhiMaterializerErrorV1::Transaction(error.to_string()))?;
    Ok(LoopPhiMaterializationReceiptV1 {
        sites: sites.into_boxed_slice(),
    })
}

fn preflight_builder(
    builder: &mut MirBuilder,
    rows: &[PendingPhiV1],
) -> Result<(), LoopPhiMaterializerErrorV1> {
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| preflight_error("no current function"))?;
    let mut destinations = BTreeSet::new();
    for row in rows {
        let block = function
            .get_block(row.block)
            .ok_or_else(|| preflight_error(format!("missing PHI block={:?}", row.block)))?;
        let mut actual_predecessors = block.predecessors.iter().copied().collect::<Vec<_>>();
        actual_predecessors.sort_unstable();
        if actual_predecessors.as_slice() != row.expected_predecessors.as_ref() {
            return Err(preflight_error(format!(
                "sealed predecessor mismatch block={:?} expected={:?} actual={actual_predecessors:?}",
                row.block, row.expected_predecessors
            )));
        }
        if !destinations.insert(row.dst) || function_defines(function, row.dst) {
            return Err(preflight_error(format!(
                "PHI destination already defined dst={:?}",
                row.dst
            )));
        }
        for (predecessor, value) in &row.inputs {
            let actual = builder
                .function_state
                .type_ctx
                .value_types
                .get(value)
                .ok_or_else(|| {
                    preflight_error(format!("missing type for incoming value={value:?}"))
                })?;
            if *actual == MirType::Unknown {
                return Err(preflight_error(format!(
                    "unknown type for incoming value={value:?}"
                )));
            }
            if *actual != mir_type_for_class(row.class) {
                return Err(preflight_error(format!(
                    "type/class mismatch incoming value={value:?} actual={actual:?} expected={:?}",
                    mir_type_for_class(row.class)
                )));
            }
            let _ = predecessor;
        }
    }
    let mut txn = PhiTxn::begin(MATERIALIZER_TAG);
    {
        let adapter = MirBindingSsaAdapterV1::new(builder, &mut txn);
        for row in rows {
            for (predecessor, value) in &row.inputs {
                BindingSsaIrV1::verify_phi_input(&adapter, *predecessor, *value)
                    .map_err(preflight_error)?;
            }
        }
    }
    let _ = txn;
    Ok(())
}

fn transaction_error(
    builder: &mut MirBuilder,
    txn: PhiTxn,
    error: impl Into<String>,
) -> LoopPhiMaterializerErrorV1 {
    LoopPhiMaterializerErrorV1::Transaction(txn.abort_on_err(builder, error.into()).to_string())
}

fn function_defines(function: &crate::mir::MirFunction, value: ValueId) -> bool {
    function.params.contains(&value)
        || function
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .any(|instruction| instruction.dst_value() == Some(value))
}

fn unique_ports(
    rows: Vec<(LoopNodeKeyV1, LoopJoinPortV1, BasicBlockId)>,
) -> Result<BTreeMap<(LoopNodeKeyV1, LoopJoinPortV1), BasicBlockId>, LoopPhiMaterializerErrorV1> {
    let mut map = BTreeMap::new();
    for (loop_key, port, block) in rows {
        if map.insert((loop_key, port), block).is_some() {
            return Err(map_error(format!(
                "duplicate port loop={loop_key:?} port={port:?}"
            )));
        }
    }
    Ok(map)
}

fn unique_values(
    rows: Vec<(LoopValueKeyV1, ValueId, LoopValueClassV1)>,
) -> Result<BTreeMap<LoopValueKeyV1, (ValueId, LoopValueClassV1)>, LoopPhiMaterializerErrorV1> {
    let mut map = BTreeMap::new();
    for (key, value, class) in rows {
        if map.insert(key, (value, class)).is_some() {
            return Err(map_error(format!("duplicate value key={key:?}")));
        }
    }
    Ok(map)
}

fn unique_destinations(
    rows: Vec<(LoopNodeKeyV1, LoopBindingKeyV1, ValueId)>,
) -> Result<BTreeMap<(LoopNodeKeyV1, LoopBindingKeyV1), ValueId>, LoopPhiMaterializerErrorV1> {
    let mut map = BTreeMap::new();
    for (loop_key, binding, dst) in rows {
        if map.insert((loop_key, binding), dst).is_some() {
            return Err(map_error(format!(
                "duplicate destination loop={loop_key:?} binding={binding:?}"
            )));
        }
    }
    Ok(map)
}

fn header_inputs(
    map: &VerifiedLoopLogicalToPhysicalMapV1,
    row: &crate::mir::loop_recipe_contract::LoopJoinLoopV1,
) -> Result<Vec<BasicBlockId>, LoopPhiMaterializerErrorV1> {
    let mut incoming = row
        .edges
        .iter()
        .filter(|edge| is_header_input(edge.role))
        .map(|edge| map.port(row.key, edge.from))
        .collect::<Result<Vec<_>, _>>()?;
    incoming.sort_unstable();
    if incoming.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(map_error(format!(
            "duplicate PHI predecessor loop={:?}",
            row.key
        )));
    }
    Ok(incoming)
}

fn is_header_input(role: LoopJoinEdgeRoleV1) -> bool {
    matches!(
        role,
        LoopJoinEdgeRoleV1::Enter | LoopJoinEdgeRoleV1::Backedge | LoopJoinEdgeRoleV1::Continue
    )
}

fn mir_type_for_class(class: LoopValueClassV1) -> MirType {
    match class {
        LoopValueClassV1::I64 => MirType::Integer,
        LoopValueClassV1::Bool => MirType::Bool,
        LoopValueClassV1::Unit => MirType::Void,
    }
}

fn map_error(error: impl Into<String>) -> LoopPhiMaterializerErrorV1 {
    LoopPhiMaterializerErrorV1::Map(format!(
        "[freeze:contract][loop_phi_materializer/map] {}",
        error.into()
    ))
}

fn preflight_error(error: impl Into<String>) -> LoopPhiMaterializerErrorV1 {
    LoopPhiMaterializerErrorV1::Preflight(format!(
        "[freeze:contract][loop_phi_materializer/preflight] {}",
        error.into()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::module_invocation_session::BuilderCoreSeedPolicyV1;
    use crate::mir::builder::{
        BuilderCommitReadinessErrorV1, BuilderInvocationConfigV1, MirBuilder,
        ModuleBuilderInvocationSessionV1,
    };
    use crate::mir::loop_recipe_contract::{
        LoopJoinSigElaboratorV1, LoopRecipeArtifactV1, LoopRecipeVerifierV1,
    };
    use crate::mir::{
        BasicBlock, ConstValue, EffectMask, FunctionSignature, MirFunction, MirInstruction,
    };

    const GOLDEN: &str =
        include_str!("../../../loop_recipe_contract/fixtures/accum_nested_v1.json");

    fn verified_sig() -> VerifiedLoopJoinSigV1 {
        let artifact: LoopRecipeArtifactV1 = serde_json::from_str(GOLDEN).expect("golden JSON");
        let verified =
            LoopRecipeVerifierV1::verify(artifact.recipe().clone()).expect("recipe shape");
        LoopJoinSigElaboratorV1::elaborate(&verified).expect("bounded JoinSig")
    }

    fn bb(id: u32) -> BasicBlockId {
        BasicBlockId::new(id)
    }

    fn seed_builder(builder: &mut MirBuilder) {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "m6b/accum/0".to_string(),
                params: Vec::new(),
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            bb(0),
        );
        for id in 1..8 {
            function.add_block(BasicBlock::new(bb(id)));
        }
        {
            let entry = function.get_block_mut(bb(0)).unwrap();
            entry.add_instruction(MirInstruction::Const {
                dst: ValueId::new(30),
                value: ConstValue::Bool(true),
            });
            entry.add_instruction(MirInstruction::Const {
                dst: ValueId::new(10),
                value: ConstValue::Integer(0),
            });
            entry.add_instruction(MirInstruction::Const {
                dst: ValueId::new(12),
                value: ConstValue::Integer(0),
            });
            entry.set_terminator(MirInstruction::Jump {
                target: bb(1),
                edge_args: None,
            });
        }
        function
            .get_block_mut(bb(1))
            .unwrap()
            .set_terminator(MirInstruction::Branch {
                condition: ValueId::new(30),
                then_bb: bb(2),
                else_bb: bb(3),
                then_edge_args: None,
                else_edge_args: None,
            });
        {
            let body = function.get_block_mut(bb(2)).unwrap();
            body.add_instruction(MirInstruction::Const {
                dst: ValueId::new(11),
                value: ConstValue::Integer(1),
            });
            body.add_instruction(MirInstruction::Const {
                dst: ValueId::new(13),
                value: ConstValue::Integer(1),
            });
            body.set_terminator(MirInstruction::Jump {
                target: bb(1),
                edge_args: None,
            });
        }
        function
            .get_block_mut(bb(3))
            .unwrap()
            .set_terminator(MirInstruction::Return { value: None });
        function
            .get_block_mut(bb(1))
            .unwrap()
            .add_predecessor(bb(0));
        function
            .get_block_mut(bb(1))
            .unwrap()
            .add_predecessor(bb(2));
        function
            .get_block_mut(bb(2))
            .unwrap()
            .add_predecessor(bb(1));
        function
            .get_block_mut(bb(3))
            .unwrap()
            .add_predecessor(bb(1));
        for (value, ty) in [
            (30, MirType::Bool),
            (10, MirType::Integer),
            (11, MirType::Integer),
            (12, MirType::Integer),
            (13, MirType::Integer),
        ] {
            function
                .metadata
                .value_types
                .insert(ValueId::new(value), ty);
        }
        let value_types = function.metadata.value_types.clone();
        builder.function_state.current_function = Some(function);
        builder.function_state.type_ctx.value_types = value_types;
    }

    fn seeded_builder() -> MirBuilder {
        let mut builder = MirBuilder::new();
        seed_builder(&mut builder);
        builder
    }

    fn candidate_session(live: &MirBuilder) -> ModuleBuilderInvocationSessionV1 {
        let config = BuilderInvocationConfigV1::snapshot_with_policy(
            live,
            BuilderCoreSeedPolicyV1::ContinueLive,
        );
        let mut session = ModuleBuilderInvocationSessionV1::open(live, config);
        seed_builder(session.builder_mut());
        session
    }

    fn map_input() -> LoopLogicalToPhysicalMapInputV1 {
        use LoopJoinPortV1::*;
        LoopLogicalToPhysicalMapInputV1 {
            ports: vec![
                (LoopNodeKeyV1::new(0), Preheader, bb(0)),
                (LoopNodeKeyV1::new(0), Header, bb(1)),
                (LoopNodeKeyV1::new(0), Body, bb(2)),
                (LoopNodeKeyV1::new(0), After, bb(3)),
                (LoopNodeKeyV1::new(1), Preheader, bb(2)),
                (LoopNodeKeyV1::new(1), Header, bb(4)),
                (LoopNodeKeyV1::new(1), Body, bb(5)),
                (LoopNodeKeyV1::new(1), After, bb(6)),
            ],
            values: vec![
                (
                    LoopValueKeyV1::new(0),
                    ValueId::new(10),
                    LoopValueClassV1::I64,
                ),
                (
                    LoopValueKeyV1::new(3),
                    ValueId::new(12),
                    LoopValueClassV1::I64,
                ),
                (
                    LoopValueKeyV1::new(5),
                    ValueId::new(11),
                    LoopValueClassV1::I64,
                ),
                (
                    LoopValueKeyV1::new(6),
                    ValueId::new(13),
                    LoopValueClassV1::I64,
                ),
            ],
            destinations: vec![
                (
                    LoopNodeKeyV1::new(0),
                    LoopBindingKeyV1::new(0),
                    ValueId::new(20),
                ),
                (
                    LoopNodeKeyV1::new(0),
                    LoopBindingKeyV1::new(1),
                    ValueId::new(21),
                ),
            ],
            predecessors: vec![(bb(1), vec![bb(0), bb(2)])],
        }
    }

    fn materializer_input(sig: &VerifiedLoopJoinSigV1) -> VerifiedLoopLogicalToPhysicalMapV1 {
        VerifiedLoopLogicalToPhysicalMapV1::try_new(sig, map_input()).expect("sealed map")
    }

    #[test]
    fn map_rejects_duplicate_predecessor_before_builder_effect() {
        let sig = verified_sig();
        let mut input = map_input();
        input.predecessors[0].1.push(bb(0));
        let error = VerifiedLoopLogicalToPhysicalMapV1::try_new(&sig, input).unwrap_err();
        assert!(error.to_string().contains("duplicate predecessor"));
    }

    #[test]
    fn materializer_emits_exact_accum_header_phis() {
        let sig = verified_sig();
        let mut builder = seeded_builder();
        let before = builder.function_state.variable_ctx.variable_map.clone();
        let receipt = materialize_loop_phis(&mut builder, &sig, materializer_input(&sig))
            .expect("PHI materialization");
        assert_eq!(receipt.sites().len(), 2);
        assert_eq!(
            receipt.sites()[0].inputs.as_ref(),
            &[(bb(0), ValueId::new(10)), (bb(2), ValueId::new(11))]
        );
        assert_eq!(
            receipt.sites()[1].inputs.as_ref(),
            &[(bb(0), ValueId::new(12)), (bb(2), ValueId::new(13))]
        );
        let function = builder.function_state.current_function.as_ref().unwrap();
        let phis = function
            .get_block(bb(1))
            .unwrap()
            .instructions
            .iter()
            .filter_map(|instruction| match instruction {
                MirInstruction::Phi { dst, inputs, .. } => Some((*dst, inputs.clone())),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(phis.len(), 2);
        assert_eq!(builder.function_state.variable_ctx.variable_map, before);
    }

    #[test]
    fn provisional_failure_rolls_back_empty_phi() {
        let sig = verified_sig();
        let mut builder = seeded_builder();
        let map = materializer_input(&sig);
        let error = materialize_impl(&mut builder, &sig, map, Some(0)).unwrap_err();
        assert!(error.to_string().contains("txn_abort"));
        assert!(builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .get_block(bb(1))
            .unwrap()
            .instructions
            .iter()
            .all(|instruction| !matches!(instruction, MirInstruction::Phi { .. })));
    }

    #[test]
    fn stale_cfg_witness_rejects_before_phi_effect() {
        let sig = verified_sig();
        let mut builder = seeded_builder();
        builder
            .function_state
            .current_function
            .as_mut()
            .unwrap()
            .get_block_mut(bb(1))
            .unwrap()
            .predecessors
            .remove(&bb(2));
        let error =
            materialize_loop_phis(&mut builder, &sig, materializer_input(&sig)).unwrap_err();
        assert!(error.to_string().contains("sealed predecessor mismatch"));
        assert!(builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .get_block(bb(1))
            .unwrap()
            .instructions
            .iter()
            .all(|instruction| !matches!(instruction, MirInstruction::Phi { .. })));
    }

    #[test]
    fn fresh_builder_reuse_is_deterministic() {
        let sig = verified_sig();
        let mut left = seeded_builder();
        let mut right = seeded_builder();
        let left_receipt =
            materialize_loop_phis(&mut left, &sig, materializer_input(&sig)).unwrap();
        let right_receipt =
            materialize_loop_phis(&mut right, &sig, materializer_input(&sig)).unwrap();
        assert_eq!(left_receipt, right_receipt);
    }

    #[test]
    fn candidate_abort_after_m6b_effect_allows_fresh_retry() {
        let live = MirBuilder::new();
        let before = live.loop_candidate_test_fingerprint();
        let sig = verified_sig();
        let mut first = candidate_session(&live);
        let first_receipt =
            materialize_loop_phis(first.builder_mut(), &sig, materializer_input(&sig))
                .expect("first candidate materialization");
        let first_error = first.prepare_external_commit().unwrap_err();
        assert_eq!(
            first_error,
            BuilderCommitReadinessErrorV1::CurrentFunctionOpen
        );
        assert_eq!(live.loop_candidate_test_fingerprint(), before);

        let mut second = candidate_session(&live);
        let second_receipt =
            materialize_loop_phis(second.builder_mut(), &sig, materializer_input(&sig))
                .expect("fresh candidate materialization");
        assert_eq!(first_receipt, second_receipt);
        let second_error = second.prepare_external_commit().unwrap_err();
        assert_eq!(
            second_error,
            BuilderCommitReadinessErrorV1::CurrentFunctionOpen
        );
        assert_eq!(live.loop_candidate_test_fingerprint(), before);
    }
}
