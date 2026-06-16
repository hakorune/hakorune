/*!
 * MIR-owned Map representation plans.
 *
 * This module records proof-bearing map representation families without
 * changing lowering behavior. The v0 slice only records the current generic
 * hash runtime surface derived from generic method routes; later slices may
 * promote fixed / enum / interned subsets into the same plan family.
 */

use super::generic_method_route_facts::{
    const_i64_value, GenericMethodKeyRoute, GenericMethodPublicationPolicy,
    GenericMethodReturnShape, GenericMethodValueDemand,
};
use super::generic_method_route_plan::GenericMethodRoute;
use super::value_origin::{build_value_def_map, ValueDefMap};
use super::{BasicBlockId, MirFunction, MirInstruction, MirModule, ValueId};
use crate::object_storage_plan::{
    AliasClassId, LocalFastPathFact, LocalFastPathSiteId, ObjectBasicBlockId,
    ObjectInstructionIndex, ObjectStoragePlanId, ObjectValueId, RoutePlanId,
};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapReprKind {
    GenericHashRuntime,
    LocalI64KeyMapShadow,
    FixedStatic,
    FixedSmallLinear,
    FixedOpenAddress,
    EnumKeyDense,
    InternedKeyHash,
    InternedKeyFixed,
}

impl MapReprKind {
    pub fn as_metadata_name(self) -> &'static str {
        match self {
            Self::GenericHashRuntime => "generic_hash_runtime",
            Self::LocalI64KeyMapShadow => "local_i64_key_map_shadow",
            Self::FixedStatic => "fixed_static",
            Self::FixedSmallLinear => "fixed_small_linear",
            Self::FixedOpenAddress => "fixed_open_address",
            Self::EnumKeyDense => "enum_key_dense",
            Self::InternedKeyHash => "interned_key_hash",
            Self::InternedKeyFixed => "interned_key_fixed",
        }
    }
}

impl std::fmt::Display for MapReprKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_metadata_name())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapReprPlan {
    block: BasicBlockId,
    instruction_index: usize,
    route_id: &'static str,
    repr_kind: MapReprKind,
    source_route_id: &'static str,
    source_route_kind: &'static str,
    source_helper_symbol: &'static str,
    surface_box_name: String,
    receiver_origin_box: Option<String>,
    method: String,
    receiver_value: ValueId,
    key_value: Option<ValueId>,
    result_value: Option<ValueId>,
    key_route: Option<GenericMethodKeyRoute>,
    return_shape: Option<GenericMethodReturnShape>,
    value_demand: GenericMethodValueDemand,
    publication_policy: Option<GenericMethodPublicationPolicy>,
    proof_tag: &'static str,
    lowering_tier: Option<&'static str>,
}

impl MapReprPlan {
    fn generic_hash_runtime(route: &GenericMethodRoute) -> Option<Self> {
        let receiver_origin_box = route.receiver_origin_box();
        if receiver_origin_box != Some("MapBox") {
            return None;
        }
        Some(Self {
            block: route.block(),
            instruction_index: route.instruction_index(),
            route_id: "map_repr.generic_hash_runtime",
            repr_kind: MapReprKind::GenericHashRuntime,
            source_route_id: route.route_id(),
            source_route_kind: route.route_kind_tag(),
            source_helper_symbol: route.helper_symbol(),
            surface_box_name: route.box_name().to_string(),
            receiver_origin_box: receiver_origin_box.map(str::to_string),
            method: route.method().to_string(),
            receiver_value: route.receiver_value(),
            key_value: route.key_value(),
            result_value: route.result_value(),
            key_route: route.key_route(),
            return_shape: route.return_shape(),
            value_demand: route.value_demand(),
            publication_policy: route.publication_policy(),
            proof_tag: route.proof_tag(),
            lowering_tier: route.lowering_tier().map(|tier| tier.as_json_name()),
        })
    }

    fn local_i64_key_map_shadow(
        route: &GenericMethodRoute,
        receiver_value: ValueId,
    ) -> Option<Self> {
        let receiver_origin_box = route.receiver_origin_box();
        if receiver_origin_box != Some("MapBox") {
            return None;
        }
        Some(Self {
            block: route.block(),
            instruction_index: route.instruction_index(),
            route_id: "map_repr.local_i64_key_map_shadow",
            repr_kind: MapReprKind::LocalI64KeyMapShadow,
            source_route_id: route.route_id(),
            source_route_kind: route.route_kind_tag(),
            source_helper_symbol: route.helper_symbol(),
            surface_box_name: route.box_name().to_string(),
            receiver_origin_box: receiver_origin_box.map(str::to_string),
            method: route.method().to_string(),
            receiver_value,
            key_value: route.key_value(),
            result_value: route.result_value(),
            key_route: route.key_route(),
            return_shape: route.return_shape(),
            value_demand: route.value_demand(),
            publication_policy: route.publication_policy(),
            proof_tag: "local_i64_key_map_shadow",
            lowering_tier: None,
        })
    }

    pub fn route_id(&self) -> &'static str {
        self.route_id
    }

    pub fn block(&self) -> BasicBlockId {
        self.block
    }

    pub fn instruction_index(&self) -> usize {
        self.instruction_index
    }

    pub fn repr_kind_tag(&self) -> &'static str {
        self.repr_kind.as_metadata_name()
    }

    pub fn source_route_id(&self) -> &'static str {
        self.source_route_id
    }

    pub fn source_route_kind(&self) -> &'static str {
        self.source_route_kind
    }

    pub fn source_helper_symbol(&self) -> &'static str {
        self.source_helper_symbol
    }

    pub fn surface_box_name(&self) -> &str {
        self.surface_box_name.as_str()
    }

    pub fn receiver_origin_box(&self) -> Option<&str> {
        self.receiver_origin_box.as_deref()
    }

    pub fn method(&self) -> &str {
        self.method.as_str()
    }

    pub fn receiver_value(&self) -> ValueId {
        self.receiver_value
    }

    pub fn key_value(&self) -> Option<ValueId> {
        self.key_value
    }

    pub fn result_value(&self) -> Option<ValueId> {
        self.result_value
    }

    pub fn key_route_tag(&self) -> Option<&'static str> {
        self.key_route.map(|route| route.as_metadata_name())
    }

    pub fn return_shape_tag(&self) -> Option<&'static str> {
        self.return_shape.map(|shape| shape.as_metadata_name())
    }

    pub fn value_demand_tag(&self) -> &'static str {
        self.value_demand.as_metadata_name()
    }

    pub fn publication_policy_tag(&self) -> Option<&'static str> {
        self.publication_policy
            .map(|policy| policy.as_metadata_name())
    }

    pub fn proof_tag(&self) -> &'static str {
        self.proof_tag
    }

    pub fn lowering_tier_tag(&self) -> Option<&'static str> {
        self.lowering_tier
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalMapStorageRealizationPlan {
    receiver_value: ValueId,
    representation: &'static str,
    candidate_set_count: usize,
    candidate_scalar_get_count: usize,
    publication_materialization_required: bool,
    backend_lowering_enabled: bool,
    runtime_helper_enabled: bool,
}

impl LocalMapStorageRealizationPlan {
    fn local_i64_key_map(receiver_value: ValueId, candidate: &LocalI64MapShadowCandidate) -> Self {
        Self {
            receiver_value,
            representation: "local_i64_key_map",
            candidate_set_count: candidate.i64_set_count,
            candidate_scalar_get_count: candidate.scalar_get_count,
            publication_materialization_required: true,
            backend_lowering_enabled: false,
            runtime_helper_enabled: false,
        }
    }

    pub fn receiver_value(&self) -> ValueId {
        self.receiver_value
    }

    pub fn representation(&self) -> &'static str {
        self.representation
    }

    pub fn candidate_set_count(&self) -> usize {
        self.candidate_set_count
    }

    pub fn candidate_scalar_get_count(&self) -> usize {
        self.candidate_scalar_get_count
    }

    pub fn publication_materialization_required(&self) -> bool {
        self.publication_materialization_required
    }

    pub fn backend_lowering_enabled(&self) -> bool {
        self.backend_lowering_enabled
    }

    pub fn runtime_helper_enabled(&self) -> bool {
        self.runtime_helper_enabled
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalI64MapDirectStoragePlan {
    receiver_value: ValueId,
    representation: &'static str,
    known_i64_key_set_count: usize,
    scalar_get_count: usize,
    entry_value_tracking_enabled: bool,
    publication_materialization_required: bool,
    backend_lowering_enabled: bool,
    runtime_helper_enabled: bool,
}

impl LocalI64MapDirectStoragePlan {
    fn closed_world_i64_key_value_table(
        receiver_value: ValueId,
        candidate: &LocalI64MapShadowCandidate,
    ) -> Self {
        Self {
            receiver_value,
            representation: "closed_world_i64_key_value_table",
            known_i64_key_set_count: candidate.i64_set_count,
            scalar_get_count: candidate.scalar_get_count,
            entry_value_tracking_enabled: false,
            publication_materialization_required: true,
            backend_lowering_enabled: false,
            runtime_helper_enabled: false,
        }
    }

    pub fn receiver_value(&self) -> ValueId {
        self.receiver_value
    }

    pub fn representation(&self) -> &'static str {
        self.representation
    }

    pub fn known_i64_key_set_count(&self) -> usize {
        self.known_i64_key_set_count
    }

    pub fn scalar_get_count(&self) -> usize {
        self.scalar_get_count
    }

    pub fn entry_value_tracking_enabled(&self) -> bool {
        self.entry_value_tracking_enabled
    }

    pub fn publication_materialization_required(&self) -> bool {
        self.publication_materialization_required
    }

    pub fn backend_lowering_enabled(&self) -> bool {
        self.backend_lowering_enabled
    }

    pub fn runtime_helper_enabled(&self) -> bool {
        self.runtime_helper_enabled
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalI64MapEntryValueTrackingPlan {
    receiver_value: ValueId,
    set_block: BasicBlockId,
    set_instruction_index: usize,
    key_value: ValueId,
    value_value: ValueId,
    key_const_if_known: Option<i64>,
    value_const_if_known: Option<i64>,
    backend_lowering_enabled: bool,
    runtime_helper_enabled: bool,
}

impl LocalI64MapEntryValueTrackingPlan {
    fn from_set_site(
        function: &MirFunction,
        def_map: &ValueDefMap,
        route: &GenericMethodRoute,
        receiver_value: ValueId,
        key_value: ValueId,
        value_value: ValueId,
    ) -> Self {
        Self {
            receiver_value,
            set_block: route.block(),
            set_instruction_index: route.instruction_index(),
            key_value,
            value_value,
            key_const_if_known: const_i64_value(function, def_map, key_value),
            value_const_if_known: const_i64_value(function, def_map, value_value),
            backend_lowering_enabled: false,
            runtime_helper_enabled: false,
        }
    }

    pub fn receiver_value(&self) -> ValueId {
        self.receiver_value
    }

    pub fn set_block(&self) -> BasicBlockId {
        self.set_block
    }

    pub fn set_instruction_index(&self) -> usize {
        self.set_instruction_index
    }

    pub fn key_value(&self) -> ValueId {
        self.key_value
    }

    pub fn value_value(&self) -> ValueId {
        self.value_value
    }

    pub fn key_const_if_known(&self) -> Option<i64> {
        self.key_const_if_known
    }

    pub fn value_const_if_known(&self) -> Option<i64> {
        self.value_const_if_known
    }

    pub fn backend_lowering_enabled(&self) -> bool {
        self.backend_lowering_enabled
    }

    pub fn runtime_helper_enabled(&self) -> bool {
        self.runtime_helper_enabled
    }
}

pub fn refresh_module_map_repr_plans(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_map_repr_plans(function);
    }
}

pub fn refresh_function_map_repr_plans(function: &mut MirFunction) {
    let mut plans = Vec::new();
    let local_i64_shadow_receivers = local_i64_key_map_shadow_receivers(function);
    let def_map = build_value_def_map(function);

    for route in &function.metadata.generic_method_routes {
        if let Some(plan) = MapReprPlan::generic_hash_runtime(route) {
            plans.push(plan);
        }
        let storage_receiver = map_storage_receiver_value(function, route);
        if local_i64_shadow_receivers.contains_key(&storage_receiver) {
            if let Some(plan) = MapReprPlan::local_i64_key_map_shadow(route, storage_receiver) {
                plans.push(plan);
            }
        }
    }

    plans.sort_by_key(|plan| (plan.block().as_u32(), plan.instruction_index()));
    function.metadata.local_fastpath_facts = build_local_fastpath_facts_from_map_repr_plans(&plans);
    function.metadata.local_map_storage_realization_plans =
        build_local_map_storage_realization_plans(&local_i64_shadow_receivers);
    function.metadata.local_i64_map_direct_storage_plans =
        build_local_i64_map_direct_storage_plans(&local_i64_shadow_receivers);
    function.metadata.local_i64_map_entry_value_tracking_plans =
        build_local_i64_map_entry_value_tracking_plans(
            function,
            &def_map,
            &local_i64_shadow_receivers,
        );
    function.metadata.map_repr_plans = plans;
}

fn build_local_fastpath_facts_from_map_repr_plans(plans: &[MapReprPlan]) -> Vec<LocalFastPathFact> {
    plans
        .iter()
        .enumerate()
        .filter_map(|(index, plan)| {
            if plan.route_id() != "map_repr.generic_hash_runtime" {
                return None;
            }
            if plan.source_route_kind() != "map_load_scalar_i64" {
                return None;
            }
            if plan.publication_policy_tag() != Some("no_publication") {
                return None;
            }
            if plan.return_shape_tag() != Some("scalar_i64_or_missing_zero") {
                return None;
            }
            Some(LocalFastPathFact::known_receiver_direct_call(
                LocalFastPathSiteId(index as u32),
                ObjectBasicBlockId(plan.block().as_u32()),
                ObjectInstructionIndex(plan.instruction_index() as u32),
                ObjectValueId(plan.receiver_value().as_u32()),
                AliasClassId(plan.receiver_value().as_u32()),
                RoutePlanId(index as u32),
                ObjectStoragePlanId(index as u32),
            ))
        })
        .collect()
}

fn build_local_map_storage_realization_plans(
    local_i64_candidates: &HashMap<ValueId, LocalI64MapShadowCandidate>,
) -> Vec<LocalMapStorageRealizationPlan> {
    let mut plans: Vec<_> = local_i64_candidates
        .iter()
        .map(|(receiver, candidate)| {
            LocalMapStorageRealizationPlan::local_i64_key_map(*receiver, candidate)
        })
        .collect();
    plans.sort_by_key(|plan| plan.receiver_value().as_u32());
    plans
}

fn build_local_i64_map_direct_storage_plans(
    local_i64_candidates: &HashMap<ValueId, LocalI64MapShadowCandidate>,
) -> Vec<LocalI64MapDirectStoragePlan> {
    let mut plans: Vec<_> = local_i64_candidates
        .iter()
        .map(|(receiver, candidate)| {
            LocalI64MapDirectStoragePlan::closed_world_i64_key_value_table(*receiver, candidate)
        })
        .collect();
    plans.sort_by_key(|plan| plan.receiver_value().as_u32());
    plans
}

fn build_local_i64_map_entry_value_tracking_plans(
    function: &MirFunction,
    def_map: &ValueDefMap,
    local_i64_candidates: &HashMap<ValueId, LocalI64MapShadowCandidate>,
) -> Vec<LocalI64MapEntryValueTrackingPlan> {
    let mut plans = Vec::new();
    for route in &function.metadata.generic_method_routes {
        let receiver = map_storage_receiver_value(function, route);
        if !local_i64_candidates.contains_key(&receiver) {
            continue;
        }
        if !is_i64_map_set_route(route) {
            continue;
        }
        let Some((key_value, value_value)) = set_route_key_value_operands(function, route) else {
            continue;
        };
        plans.push(LocalI64MapEntryValueTrackingPlan::from_set_site(
            function,
            def_map,
            route,
            receiver,
            key_value,
            value_value,
        ));
    }
    plans.sort_by_key(|plan| {
        (
            plan.receiver_value().as_u32(),
            plan.set_block().as_u32(),
            plan.set_instruction_index(),
        )
    });
    plans
}

#[derive(Debug, Default)]
struct LocalI64MapShadowCandidate {
    i64_set_count: usize,
    scalar_get_count: usize,
    disallowed_route_count: usize,
}

fn local_i64_key_map_shadow_receivers(
    function: &MirFunction,
) -> HashMap<ValueId, LocalI64MapShadowCandidate> {
    let mut candidates: HashMap<ValueId, LocalI64MapShadowCandidate> = HashMap::new();

    for route in &function.metadata.generic_method_routes {
        if route.receiver_origin_box() != Some("MapBox") {
            continue;
        }
        let receiver = map_storage_receiver_value(function, route);
        let entry = candidates.entry(receiver).or_default();
        if is_i64_map_set_route(route) {
            entry.i64_set_count += 1;
        } else if is_scalar_i64_map_get_route(route) {
            entry.scalar_get_count += 1;
        } else if is_public_map_get_read_route(route) {
            // A later public read forces the generic fallback path for that site,
            // but it does not invalidate pre-publication scalar get candidates.
        } else {
            entry.disallowed_route_count += 1;
        }
    }

    candidates
        .into_iter()
        .filter_map(|(receiver, candidate)| {
            (candidate.i64_set_count > 0
                && candidate.scalar_get_count > 0
                && candidate.disallowed_route_count == 0)
                .then_some((receiver, candidate))
        })
        .collect()
}

fn is_i64_map_set_route(route: &GenericMethodRoute) -> bool {
    route.route_id() == "generic_method.set"
        && route.route_kind_tag() == "map_store_any"
        && route.key_route() == Some(GenericMethodKeyRoute::I64Const)
}

fn is_scalar_i64_map_get_route(route: &GenericMethodRoute) -> bool {
    route.route_id() == "generic_method.get" && route.route_kind_tag() == "map_load_scalar_i64"
}

fn is_public_map_get_read_route(route: &GenericMethodRoute) -> bool {
    route.route_id() == "generic_method.get"
        && route.route_kind_tag() == "map_load_any"
        && route.value_demand() == GenericMethodValueDemand::ReadRef
}

fn map_storage_receiver_value(function: &MirFunction, route: &GenericMethodRoute) -> ValueId {
    if !is_i64_map_set_route(route) {
        return route.receiver_value();
    }
    let Some(block) = function.blocks.get(&route.block()) else {
        return route.receiver_value();
    };
    let Some(MirInstruction::Call { args, .. }) = block.instructions.get(route.instruction_index())
    else {
        return route.receiver_value();
    };
    let Some(first) = args.first().copied() else {
        return route.receiver_value();
    };
    if first == route.receiver_value() || Some(first) == route.key_value() {
        route.receiver_value()
    } else {
        first
    }
}

fn set_route_key_value_operands(
    function: &MirFunction,
    route: &GenericMethodRoute,
) -> Option<(ValueId, ValueId)> {
    let block = function.blocks.get(&route.block())?;
    let MirInstruction::Call { args, .. } = block.instructions.get(route.instruction_index())?
    else {
        return None;
    };
    let first = args.first().copied()?;
    let offset = if first == route.receiver_value() || Some(first) != route.key_value() {
        1
    } else {
        0
    };
    let key = args.get(offset).copied()?;
    let value = args.get(offset + 1).copied()?;
    Some((key, value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{
        BasicBlock, BinaryOp, Callee, ConstValue, EffectMask, FunctionSignature, MirInstruction,
        MirType,
    };

    fn method_call(
        dst: Option<u32>,
        box_name: &str,
        method: &str,
        receiver: u32,
        args: Vec<u32>,
    ) -> MirInstruction {
        MirInstruction::Call {
            dst: dst.map(ValueId::new),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: box_name.to_string(),
                method: method.to_string(),
                receiver: Some(ValueId::new(receiver)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: args.into_iter().map(ValueId::new).collect(),
            effects: EffectMask::PURE,
        }
    }

    fn make_function() -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    #[test]
    fn refresh_function_map_repr_plans_emits_generic_hash_runtime_rows() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "MapBox".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Integer(-1),
        });
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Integer(7),
        });
        block.add_instruction(method_call(Some(4), "MapBox", "set", 1, vec![2, 3]));

        crate::mir::generic_method_route_plan::refresh_function_generic_method_routes(
            &mut function,
        );
        refresh_function_map_repr_plans(&mut function);

        assert_eq!(function.metadata.map_repr_plans.len(), 1);
        let plan = &function.metadata.map_repr_plans[0];
        assert_eq!(plan.route_id(), "map_repr.generic_hash_runtime");
        assert_eq!(plan.repr_kind_tag(), "generic_hash_runtime");
        assert_eq!(plan.source_route_id(), "generic_method.set");
        assert_eq!(plan.surface_box_name(), "MapBox");
        assert_eq!(plan.receiver_origin_box(), Some("MapBox"));
        assert_eq!(plan.method(), "set");
        assert_eq!(plan.receiver_value(), ValueId::new(1));
        assert_eq!(plan.key_route_tag(), Some("i64_const"));
        assert_eq!(plan.value_demand_tag(), "write_any");
        assert_eq!(plan.proof_tag(), "set_surface_policy");
    }

    #[test]
    fn refresh_function_map_repr_plans_emits_local_i64_key_map_shadow_rows() {
        let mut function = make_function();
        let entry_id = BasicBlockId::new(0);
        let body_id = BasicBlockId::new(1);
        let entry = function.blocks.get_mut(&entry_id).expect("entry");
        entry.successors.insert(body_id);
        entry.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "MapBox".to_string(),
            args: vec![],
        });
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Integer(0),
        });
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Integer(1),
        });
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::Integer(2),
        });
        entry.add_instruction(method_call(Some(5), "MapBox", "set", 1, vec![2, 3]));
        entry.add_instruction(method_call(Some(6), "MapBox", "set", 1, vec![3, 4]));
        entry.add_instruction(method_call(Some(7), "MapBox", "set", 1, vec![4, 2]));

        let mut body = BasicBlock::new(body_id);
        body.predecessors.insert(entry_id);
        body.successors.insert(body_id);
        body.add_instruction(MirInstruction::Phi {
            dst: ValueId::new(10),
            inputs: vec![(entry_id, ValueId::new(2)), (body_id, ValueId::new(13))],
            type_hint: None,
        });
        body.add_instruction(MirInstruction::Const {
            dst: ValueId::new(11),
            value: ConstValue::Integer(3),
        });
        body.add_instruction(MirInstruction::BinOp {
            dst: ValueId::new(12),
            op: BinaryOp::Mod,
            lhs: ValueId::new(10),
            rhs: ValueId::new(11),
        });
        body.add_instruction(method_call(Some(20), "RuntimeDataBox", "get", 1, vec![12]));
        body.add_instruction(MirInstruction::BinOp {
            dst: ValueId::new(13),
            op: BinaryOp::Add,
            lhs: ValueId::new(10),
            rhs: ValueId::new(3),
        });
        function.add_block(body);

        crate::mir::generic_method_route_plan::refresh_function_generic_method_routes(
            &mut function,
        );
        refresh_function_map_repr_plans(&mut function);

        let shadow_plans: Vec<_> = function
            .metadata
            .map_repr_plans
            .iter()
            .filter(|plan| plan.repr_kind_tag() == "local_i64_key_map_shadow")
            .collect();
        assert_eq!(shadow_plans.len(), 4);
        assert!(shadow_plans
            .iter()
            .any(|plan| plan.source_route_kind() == "map_load_scalar_i64"));
        assert!(shadow_plans.iter().all(|plan| {
            plan.receiver_value() == ValueId::new(1)
                && plan.proof_tag() == "local_i64_key_map_shadow"
        }));
        let storage_plans = &function.metadata.local_map_storage_realization_plans;
        assert_eq!(storage_plans.len(), 1);
        let storage_plan = &storage_plans[0];
        assert_eq!(storage_plan.receiver_value(), ValueId::new(1));
        assert_eq!(storage_plan.representation(), "local_i64_key_map");
        assert_eq!(storage_plan.candidate_set_count(), 3);
        assert_eq!(storage_plan.candidate_scalar_get_count(), 1);
        assert!(storage_plan.publication_materialization_required());
        assert!(!storage_plan.backend_lowering_enabled());
        assert!(!storage_plan.runtime_helper_enabled());
        let direct_storage_plans = &function.metadata.local_i64_map_direct_storage_plans;
        assert_eq!(direct_storage_plans.len(), 1);
        let direct_storage_plan = &direct_storage_plans[0];
        assert_eq!(direct_storage_plan.receiver_value(), ValueId::new(1));
        assert_eq!(
            direct_storage_plan.representation(),
            "closed_world_i64_key_value_table"
        );
        assert_eq!(direct_storage_plan.known_i64_key_set_count(), 3);
        assert_eq!(direct_storage_plan.scalar_get_count(), 1);
        assert!(!direct_storage_plan.entry_value_tracking_enabled());
        assert!(direct_storage_plan.publication_materialization_required());
        assert!(!direct_storage_plan.backend_lowering_enabled());
        assert!(!direct_storage_plan.runtime_helper_enabled());
        let entry_plans = &function.metadata.local_i64_map_entry_value_tracking_plans;
        assert_eq!(entry_plans.len(), 3);
        let first_entry = &entry_plans[0];
        assert_eq!(first_entry.receiver_value(), ValueId::new(1));
        assert_eq!(first_entry.set_block(), BasicBlockId::new(0));
        assert_eq!(first_entry.set_instruction_index(), 4);
        assert_eq!(first_entry.key_value(), ValueId::new(2));
        assert_eq!(first_entry.value_value(), ValueId::new(3));
        assert_eq!(first_entry.key_const_if_known(), Some(0));
        assert_eq!(first_entry.value_const_if_known(), Some(1));
        assert!(!first_entry.backend_lowering_enabled());
        assert!(!first_entry.runtime_helper_enabled());
    }

    #[test]
    fn refresh_function_map_repr_plans_joins_set_receiver_alias_and_later_public_read() {
        let mut function = make_function();
        let entry_id = BasicBlockId::new(0);
        let body_id = BasicBlockId::new(1);
        let exit_id = BasicBlockId::new(2);
        let entry = function.blocks.get_mut(&entry_id).expect("entry");
        entry.successors.insert(body_id);
        entry.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(3),
            box_type: "MapBox".to_string(),
            args: vec![],
        });
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(12),
            value: ConstValue::Integer(0),
        });
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(13),
            value: ConstValue::Integer(1),
        });
        entry.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(14),
            src: ValueId::new(3),
        });
        entry.add_instruction(method_call(Some(15), "MapBox", "set", 14, vec![3, 12, 13]));

        let mut body = BasicBlock::new(body_id);
        body.predecessors.insert(entry_id);
        body.successors.insert(exit_id);
        body.add_instruction(method_call(Some(20), "RuntimeDataBox", "get", 3, vec![12]));
        function.add_block(body);

        let mut exit = BasicBlock::new(exit_id);
        exit.predecessors.insert(body_id);
        exit.add_instruction(method_call(Some(30), "MapBox", "get", 3, vec![12]));
        function.add_block(exit);

        crate::mir::generic_method_route_plan::refresh_function_generic_method_routes(
            &mut function,
        );
        refresh_function_map_repr_plans(&mut function);

        let direct_storage_plans = &function.metadata.local_i64_map_direct_storage_plans;
        let direct_storage_plan = direct_storage_plans
            .iter()
            .find(|plan| plan.receiver_value() == ValueId::new(3))
            .expect("direct storage plan for canonical receiver");
        assert_eq!(direct_storage_plan.receiver_value(), ValueId::new(3));
        assert_eq!(direct_storage_plan.known_i64_key_set_count(), 1);
        assert!(direct_storage_plan.scalar_get_count() >= 1);

        let entry_plans = &function.metadata.local_i64_map_entry_value_tracking_plans;
        let entry = entry_plans
            .iter()
            .find(|plan| plan.receiver_value() == ValueId::new(3))
            .expect("entry tracking plan for canonical receiver");
        assert_eq!(entry.receiver_value(), ValueId::new(3));
        assert_eq!(entry.key_value(), ValueId::new(12));
        assert_eq!(entry.value_value(), ValueId::new(13));
        assert_eq!(entry.key_const_if_known(), Some(0));
        assert_eq!(entry.value_const_if_known(), Some(1));
    }

    #[test]
    fn refresh_function_map_repr_plans_emits_local_fastpath_facts_for_scalar_no_publication_get() {
        let mut function = make_function();
        let entry_id = BasicBlockId::new(0);
        let body_id = BasicBlockId::new(1);
        let entry = function.blocks.get_mut(&entry_id).expect("entry");
        entry.successors.insert(body_id);
        entry.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "MapBox".to_string(),
            args: vec![],
        });
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Integer(0),
        });
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Integer(1),
        });
        entry.add_instruction(method_call(Some(4), "MapBox", "set", 1, vec![2, 3]));

        let mut body = BasicBlock::new(body_id);
        body.predecessors.insert(entry_id);
        body.successors.insert(body_id);
        body.add_instruction(method_call(Some(20), "RuntimeDataBox", "get", 1, vec![2]));
        function.add_block(body);

        crate::mir::generic_method_route_plan::refresh_function_generic_method_routes(
            &mut function,
        );
        refresh_function_map_repr_plans(&mut function);

        let facts = &function.metadata.local_fastpath_facts;
        assert_eq!(facts.len(), 1);
        let source_plan = function
            .metadata
            .map_repr_plans
            .iter()
            .find(|plan| {
                plan.source_route_kind() == "map_load_scalar_i64"
                    && plan.publication_policy_tag() == Some("no_publication")
                    && plan.return_shape_tag() == Some("scalar_i64_or_missing_zero")
            })
            .expect("scalar no-publication map repr plan");
        assert_eq!(source_plan.route_id(), "map_repr.generic_hash_runtime");
        let fact = &facts[0];
        assert_eq!(fact.object_id, ObjectValueId(1));
        assert_eq!(fact.block_id, ObjectBasicBlockId(1));
        assert_eq!(fact.instruction_index, ObjectInstructionIndex(0));
        assert!(fact.valid_until_publication);
    }
}
