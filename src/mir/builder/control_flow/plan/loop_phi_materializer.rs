//! Caller-zero PHI materialization for the bounded portable Loop JoinSig.
//!
//! This is a builder-side seam only.  It consumes a verified logical JoinSig
//! and a sealed logical-to-physical map; it never selects a route, discovers a
//! carrier, or repairs CFG/SSA facts.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::builder::control_flow::plan::loop_physical_edge_path::LoopPhysicalEdgePathV1;
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
    pub(in crate::mir::builder) edge_paths: Vec<LoopPhysicalEdgePathV1>,
}

#[derive(Debug)]
pub(in crate::mir::builder) struct VerifiedLoopLogicalToPhysicalMapV1 {
    ports: BTreeMap<(LoopNodeKeyV1, LoopJoinPortV1), BasicBlockId>,
    values: BTreeMap<LoopValueKeyV1, (ValueId, LoopValueClassV1)>,
    destinations: BTreeMap<(LoopNodeKeyV1, LoopBindingKeyV1), ValueId>,
    predecessors: BTreeMap<BasicBlockId, Box<[BasicBlockId]>>,
    edge_paths: BTreeMap<(LoopNodeKeyV1, LoopJoinEdgeRoleV1), Box<[LoopPhysicalEdgePathV1]>>,
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
        let edge_paths = grouped_edge_paths(input.edge_paths)?;
        let map = Self {
            ports,
            values,
            destinations,
            predecessors,
            edge_paths,
        };
        map.validate_signature(sig)?;
        Ok(map)
    }

    fn validate_signature(
        &self,
        sig: &VerifiedLoopJoinSigV1,
    ) -> Result<(), LoopPhiMaterializerErrorV1> {
        let mut known_edge_roles = BTreeSet::new();
        for row in &sig.as_sig().loops {
            for edge in &row.edges {
                known_edge_roles.insert((row.key, edge.role));
                self.port(row.key, edge.from)?;
                self.port(row.key, edge.to)?;
                self.validate_edge_paths(row.key, edge)?;
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
        if self
            .edge_paths
            .keys()
            .any(|key| !known_edge_roles.contains(key))
        {
            return Err(map_error("physical edge path has no logical JoinSig edge"));
        }
        Ok(())
    }

    fn validate_edge_paths(
        &self,
        loop_key: LoopNodeKeyV1,
        edge: &crate::mir::loop_recipe_contract::LoopJoinEdgeV1,
    ) -> Result<(), LoopPhiMaterializerErrorV1> {
        let paths = self.edge_paths.get(&(loop_key, edge.role)).ok_or_else(|| {
            map_error(format!(
                "missing physical edge path loop={loop_key:?} role={:?}",
                edge.role
            ))
        })?;
        let from = self.port(loop_key, edge.from)?;
        let to = self.port(loop_key, edge.to)?;
        let matching = paths
            .iter()
            .filter(|path| path.blocks.first() == Some(&from) && path.blocks.last() == Some(&to))
            .count();
        if matching == 0 {
            return Err(map_error(format!(
                "edge path endpoint mismatch loop={loop_key:?} role={:?} from={from:?} to={to:?}",
                edge.role
            )));
        }
        if is_header_input(edge.role) && paths.len() != 1 {
            return Err(map_error(format!(
                "header edge path must be unique loop={loop_key:?} role={:?} count={}",
                edge.role,
                paths.len()
            )));
        }
        for path in paths {
            self.validate_edge_path(path, from, to)?;
        }
        Ok(())
    }

    fn validate_edge_path(
        &self,
        path: &LoopPhysicalEdgePathV1,
        from: BasicBlockId,
        to: BasicBlockId,
    ) -> Result<(), LoopPhiMaterializerErrorV1> {
        if path.blocks.len() < 2 {
            return Err(map_error(format!(
                "physical edge path is too short loop={:?} role={:?}",
                path.loop_key, path.role
            )));
        }
        if path.blocks.first() != Some(&from) || path.blocks.last() != Some(&to) {
            return Err(map_error(format!(
                "physical edge path endpoints loop={:?} role={:?} expected=({from:?},{to:?}) actual={:?}",
                path.loop_key, path.role, path.blocks
            )));
        }
        if path.terminal_predecessor != path.blocks[path.blocks.len() - 2] {
            return Err(map_error(format!(
                "physical edge path terminal predecessor mismatch loop={:?} role={:?}",
                path.loop_key, path.role
            )));
        }
        for pair in path.blocks.windows(2) {
            let predecessors = self.predecessors.get(&pair[1]).ok_or_else(|| {
                map_error(format!(
                    "missing predecessor witness target={:?} loop={:?} role={:?}",
                    pair[1], path.loop_key, path.role
                ))
            })?;
            if !predecessors.contains(&pair[0]) {
                return Err(map_error(format!(
                    "physical edge path predecessor mismatch target={:?} predecessor={:?} loop={:?} role={:?}",
                    pair[1], pair[0], path.loop_key, path.role
                )));
            }
        }
        Ok(())
    }

    fn header_path(
        &self,
        loop_key: LoopNodeKeyV1,
        edge: &crate::mir::loop_recipe_contract::LoopJoinEdgeV1,
    ) -> Result<&LoopPhysicalEdgePathV1, LoopPhiMaterializerErrorV1> {
        let paths = self
            .edge_paths
            .get(&(loop_key, edge.role))
            .ok_or_else(|| map_error(format!("missing header edge path loop={loop_key:?}")))?;
        if paths.len() != 1 {
            return Err(map_error(format!(
                "header edge path is not unique loop={loop_key:?} role={:?}",
                edge.role
            )));
        }
        Ok(&paths[0])
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
                    inputs.push((
                        self.header_path(row.key, edge)?.terminal_predecessor,
                        physical,
                    ));
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

/// Test-only two-phase carrier-PHI seam for the caller-zero physicalizer.
///
/// The handle deliberately keeps the existing `PhiTxn` alive between
/// provisional definition and input finalization.  It is not a second PHI
/// writer and it does not expose `PendingPhiV1` or `PhiToken` to callers.
#[cfg(test)]
pub(in crate::mir::builder) struct LoopPhiMaterializationHandleV1 {
    txn: Option<PhiTxn>,
    pending: Vec<(
        PendingPhiV1,
        crate::mir::builder::emission::phi_lifecycle::PhiToken,
    )>,
}

#[cfg(test)]
impl LoopPhiMaterializationHandleV1 {
    pub(in crate::mir::builder) fn begin(
        builder: &mut MirBuilder,
        sig: &VerifiedLoopJoinSigV1,
        map: VerifiedLoopLogicalToPhysicalMapV1,
    ) -> Result<Self, LoopPhiMaterializerErrorV1> {
        let pending = map.pending_phis(sig)?;
        preflight_builder(builder, &pending)?;
        let mut txn = PhiTxn::begin(MATERIALIZER_TAG);
        let mut rows = Vec::with_capacity(pending.len());
        for row in pending {
            let token =
                match txn.define_provisional_phi(builder, row.block, row.dst, MATERIALIZER_TAG) {
                    Ok(token) => token,
                    Err(error) => return Err(transaction_error(builder, txn, error)),
                };
            rows.push((row, token));
        }
        Ok(Self {
            txn: Some(txn),
            pending: rows,
        })
    }

    pub(in crate::mir::builder) fn destination_values(&self) -> Vec<ValueId> {
        self.pending.iter().map(|(row, _)| row.dst).collect()
    }

    pub(in crate::mir::builder) fn finalize(
        mut self,
        builder: &mut MirBuilder,
    ) -> Result<LoopPhiMaterializationReceiptV1, LoopPhiMaterializerErrorV1> {
        let mut txn = self
            .txn
            .take()
            .expect("LoopPhiMaterializationHandleV1 transaction already consumed");
        let mut sites = Vec::with_capacity(self.pending.len());
        for (row, token) in self.pending {
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

    pub(in crate::mir::builder) fn abort(
        self,
        builder: &mut MirBuilder,
        error: impl Into<String>,
    ) -> LoopPhiMaterializerErrorV1 {
        let txn = self
            .txn
            .expect("LoopPhiMaterializationHandleV1 transaction already consumed");
        transaction_error(builder, txn, error)
    }
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

fn grouped_edge_paths(
    rows: Vec<LoopPhysicalEdgePathV1>,
) -> Result<
    BTreeMap<(LoopNodeKeyV1, LoopJoinEdgeRoleV1), Box<[LoopPhysicalEdgePathV1]>>,
    LoopPhiMaterializerErrorV1,
> {
    let mut grouped =
        BTreeMap::<(LoopNodeKeyV1, LoopJoinEdgeRoleV1), Vec<LoopPhysicalEdgePathV1>>::new();
    for path in rows {
        grouped
            .entry((path.loop_key, path.role))
            .or_default()
            .push(path);
    }
    Ok(grouped
        .into_iter()
        .map(|(key, mut paths)| {
            paths.sort_by_key(|path| path.blocks.clone());
            (key, paths.into_boxed_slice())
        })
        .collect())
}

fn header_inputs(
    map: &VerifiedLoopLogicalToPhysicalMapV1,
    row: &crate::mir::loop_recipe_contract::LoopJoinLoopV1,
) -> Result<Vec<BasicBlockId>, LoopPhiMaterializerErrorV1> {
    let mut incoming = row
        .edges
        .iter()
        .filter(|edge| is_header_input(edge.role))
        .map(|edge| {
            map.header_path(row.key, edge)
                .map(|path| path.terminal_predecessor)
        })
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
#[path = "loop_phi_materializer_tests.rs"]
mod tests;
