//! Complete physical program projection for the selected lifecycle consumer.
//!
//! The final view lends its finalized handoff and matching MIR bodies.
//! This projection validates retained correspondence and borrows the result;
//! it issues neither source facts nor C execution permission.

use crate::mir::instruction::InvokeCallResultKind;
use std::collections::BTreeSet;

use hakorune_mir_defs::SameModuleCallableNamespaceV1;

use crate::mir::definitions::MirCall;
use crate::mir::instruction::InvokeOperation;
use crate::mir::{
    BasicBlockId, BinaryOp, Callee, ConstValue, EdgeArgs, MirFunction, MirInstruction, MirModule,
    ValueId,
};
use hakorune_mir_defs::{CanonicalFieldRefV1, CanonicalObjectIdV1};

use super::{CompiledEntryRootResultV1, PublishedMirBackendView, PublishedStaticMethodRouteV1};
use crate::mir::finalized_root_handoff::FinalizedRootHandoffV1;

#[path = "physical_program_projection.rs"]
mod projection;

#[path = "physical_program_scalar.rs"]
mod scalar;
#[path = "physical_program_object_end.rs"]
mod object_end;
#[path = "physical_program_shared_initializer.rs"]
mod shared_initializer;

#[path = "physical_program_call_helpers.rs"]
mod call_helpers;
#[path = "physical_program_field_ref.rs"]
mod field_ref;
#[path = "physical_program_object_identity.rs"]
mod object_identity;
pub(crate) use call_helpers::{ordinary_call_receiver, ordinary_callable_key};
use field_ref::prepare_field_ref;
use crate::mir::normal_callable_semantic_package::FinalizedObjectReturnOutcomeV1;

#[path = "physical_program_role.rs"]
mod role;
pub(crate) use role::PublishedLifecyclePhysicalFunctionRoleV1;

#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedLifecyclePhysicalInstructionRefV1<'module> {
    index: u32,
    instruction: &'module MirInstruction,
    field_ref: Option<CanonicalFieldRefV1>,
}

impl<'module> PublishedLifecyclePhysicalInstructionRefV1<'module> {
    pub(crate) const fn index(self) -> u32 {
        self.index
    }

    pub(crate) fn instruction(self) -> &'module MirInstruction {
        self.instruction
    }

    pub(crate) const fn field_ref(self) -> Option<CanonicalFieldRefV1> {
        self.field_ref
    }
}

/// One physical CFG edge copied from the final block terminator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PublishedLifecyclePhysicalEdgeV1 {
    target: BasicBlockId,
    args: Option<EdgeArgs>,
}

impl PublishedLifecyclePhysicalEdgeV1 {
    pub(crate) const fn target(&self) -> BasicBlockId {
        self.target
    }

    pub(crate) fn args(&self) -> Option<&EdgeArgs> {
        self.args.as_ref()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PublishedLifecyclePhysicalBlockV1<'module> {
    id: BasicBlockId,
    instructions: Box<[PublishedLifecyclePhysicalInstructionRefV1<'module>]>,
    terminator: PublishedLifecyclePhysicalInstructionRefV1<'module>,
    edges: Box<[PublishedLifecyclePhysicalEdgeV1]>,
}

impl<'module> PublishedLifecyclePhysicalBlockV1<'module> {
    pub(crate) const fn id(&self) -> BasicBlockId {
        self.id
    }

    pub(crate) fn instructions(&self) -> &[PublishedLifecyclePhysicalInstructionRefV1<'module>] {
        &self.instructions
    }

    pub(crate) const fn terminator(&self) -> PublishedLifecyclePhysicalInstructionRefV1<'module> {
        self.terminator
    }

    pub(crate) fn edges(&self) -> &[PublishedLifecyclePhysicalEdgeV1] {
        &self.edges
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PublishedLifecyclePhysicalFunctionV1<'module> {
    function: &'module MirFunction,
    name: &'module str,
    role: PublishedLifecyclePhysicalFunctionRoleV1,
    params: &'module [ValueId],
    /// Positionally aligned with `params` (the signature's declared types).
    /// Drives the wire `representation` — a `Box("MapBox")` formal rides as
    /// borrowed checked-map storage, not an i64.
    param_types: &'module [crate::mir::MirType],
    /// Signature-aligned physical carriers issued beside the signature —
    /// the wire `representation` authority. `CheckedMapStorage` keeps its
    /// identity so the name is never re-read to spell `ptr`.
    param_carriers:
        Option<&'module [crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1]>,
    /// The caller-side actual representation record: the same sealed
    /// `value_types` map the MIR verifier corroborates against callee
    /// formals. A `Box("MapBox")` actual is the only map-kind evidence;
    /// an unrecorded or `Integer` actual is the scalar lane.
    value_types: &'module std::collections::BTreeMap<ValueId, crate::mir::MirType>,
    entry: BasicBlockId,
    blocks: Box<[PublishedLifecyclePhysicalBlockV1<'module>]>,
}

impl<'module> PublishedLifecyclePhysicalFunctionV1<'module> {
    pub(crate) fn function(&self) -> &'module MirFunction { self.function }
    pub(crate) fn name(&self) -> &'module str {
        self.name
    }

    pub(crate) fn role(&self) -> &PublishedLifecyclePhysicalFunctionRoleV1 {
        &self.role
    }

    pub(crate) fn params(&self) -> &'module [ValueId] {
        self.params
    }

    pub(crate) fn param_types(&self) -> &'module [crate::mir::MirType] {
        self.param_types
    }

    pub(in crate::mir) fn param_carriers(
        &self,
    ) -> Option<
        &'module [crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1],
    >{
        self.param_carriers
    }

    pub(crate) fn value_types(
        &self,
    ) -> &'module std::collections::BTreeMap<ValueId, crate::mir::MirType> {
        self.value_types
    }

    pub(crate) const fn entry(&self) -> BasicBlockId {
        self.entry
    }

    pub(crate) fn blocks(&self) -> &[PublishedLifecyclePhysicalBlockV1<'module>] {
        &self.blocks
    }
}

/// Complete, deterministic physical program issued by the activated final view.
#[derive(Debug, Clone)]
pub(crate) struct PublishedLifecyclePhysicalProgramV1<'module> {
    functions: Box<[PublishedLifecyclePhysicalFunctionV1<'module>]>,
    handoff: &'module FinalizedRootHandoffV1,
}

impl<'module> PublishedLifecyclePhysicalProgramV1<'module> {
    pub(crate) fn handoff(&self) -> &'module FinalizedRootHandoffV1 {
        self.handoff
    }

    pub(crate) fn is_native_array(&self) -> bool {
        self.handoff.script_array().is_some()
    }

    pub(crate) fn functions(&self) -> &[PublishedLifecyclePhysicalFunctionV1<'module>] {
        &self.functions
    }

    /// A function whose range contracts need no check: no resolved call chain
    /// reaches it from the Root AND it has no physical function. A physical
    /// function keeps its checks even without an edge; an unresolved walk
    /// (`reachable == None`) keeps every function in scope.
    pub(crate) fn range_contracts_are_inert_v1(
        &self,
        reachable: &Option<std::collections::BTreeSet<&str>>,
        name: &str,
    ) -> bool {
        reachable.as_ref().is_some_and(|reached| !reached.contains(name))
            && !self.functions().iter().any(|physical| physical.name() == name)
    }

    /// Function symbols reachable from the program's Root through resolved call
    /// edges of the published MIR itself (not any source plan). `None` when any
    /// reachable function holds a call this walk cannot resolve to a module
    /// symbol; every function then stays in scope, exactly as without the walk.
    pub(crate) fn reachable_symbols_v1<'m>(
        &self,
        module: &'m crate::mir::MirModule,
    ) -> Option<std::collections::BTreeSet<&'m str>> {
        use crate::mir::definitions::Callee;
        use crate::mir::instruction::InvokeOperation;
        let mut reached = std::collections::BTreeSet::new();
        let mut pending = Vec::new();
        for physical in self.functions() {
            if matches!(physical.role(), PublishedLifecyclePhysicalFunctionRoleV1::Root { .. }) {
                let (symbol, _) = module.functions.get_key_value(physical.name())?;
                pending.push(symbol.as_str());
            }
        }
        if pending.is_empty() {
            return None;
        }
        while let Some(symbol) = pending.pop() {
            if !reached.insert(symbol) {
                continue;
            }
            for block in module.functions.get(symbol)?.blocks.values() {
                for instruction in block.all_instructions() {
                    let key = match instruction {
                        MirInstruction::Call(call) => match &call.callee {
                            Callee::BirthConstructor { key, .. } => key.clone(),
                            Callee::Extern(_) => continue,
                            _ => ordinary_callable_key(&call.callee).ok()?,
                        },
                        MirInstruction::Invoke { operation, .. } => match operation {
                            InvokeOperation::Birth(birth) => birth.key.clone(),
                            InvokeOperation::Call { call, .. } => match &call.callee {
                                Callee::BirthConstructor { key, .. } => key.clone(),
                                Callee::Extern(_) => continue,
                                _ => ordinary_callable_key(&call.callee).ok()?,
                            },
                            // A selector-dispatched call has no static target.
                            InvokeOperation::DynamicInvocation(_) => return None,
                            // Every other operation has no user call target (Object End
                            // has no user hook: lifecycle programs are `NoHook` only).
                            // Listed in full: a new variant must be classified here.
                            InvokeOperation::NewBox { .. }
                            | InvokeOperation::IntrinsicArrayNew
                            | InvokeOperation::ScriptArgumentsNew { .. }
                            | InvokeOperation::Map(_)
                            | InvokeOperation::String(_)
                            | InvokeOperation::DynamicArrayGet { .. }
                            | InvokeOperation::DynamicCarrierInteger { .. }
                            | InvokeOperation::DynamicCarrierSubI64 { .. }
                            | InvokeOperation::DynamicCarrierEnd { .. }
                            | InvokeOperation::OwnedArray(_)
                            | InvokeOperation::Shared(_)
                            | InvokeOperation::DynamicCarrierArray(_)
                            | InvokeOperation::ArrayStateContractClaim { .. }
                            | InvokeOperation::ArrayElementWrite { .. }
                            | InvokeOperation::FieldGet { .. }
                            | InvokeOperation::ObjectFieldBorrow { .. }
                            | InvokeOperation::FieldSet { .. }
                            | InvokeOperation::ObjectFieldSet { .. }
                            | InvokeOperation::HomeRelease { .. }
                            | InvokeOperation::HomeReleaseIfLive { .. }
                            | InvokeOperation::ObjectEndIfOwned { .. }
                            | InvokeOperation::ReclaimUnpublished { .. }
                            | InvokeOperation::OwnedFieldResidenceRelease { .. }
                            | InvokeOperation::OwnedObjectFieldRelease { .. } => continue,
                        },
                        MirInstruction::LegacyCallV0 { .. }
                        | MirInstruction::NewClosure { .. }
                        | MirInstruction::NewBox { .. }
                        | MirInstruction::CheckedCallOut { .. } => return None,
                        // No call target. Listed in full so a new instruction must be classified.
                        MirInstruction::Const { .. } | MirInstruction::BinOp { .. } | MirInstruction::UnaryOp { .. } | MirInstruction::Compare { .. } | MirInstruction::Load { .. } | MirInstruction::StaticDataLoad { .. } | MirInstruction::Store { .. } | MirInstruction::MapLiteralEntryWrite { .. } | MirInstruction::ArrayElementWrite { .. } | MirInstruction::StringLength { .. } | MirInstruction::StringIndexOf { .. } | MirInstruction::StringCompare { .. } | MirInstruction::ArrayRead { .. } | MirInstruction::ArrayStateContractClaim { .. } | MirInstruction::MemOp { .. } | MirInstruction::PinnedTextOp { .. } | MirInstruction::PinnedTextResidenceFinish { .. } | MirInstruction::ObjectFieldGet { .. } | MirInstruction::FieldGet { .. } | MirInstruction::FieldSet { .. } | MirInstruction::WeakFieldWrite { .. } | MirInstruction::VariantMake { .. } | MirInstruction::VariantTag { .. } | MirInstruction::VariantProject { .. } | MirInstruction::Branch { .. } | MirInstruction::Jump { .. } | MirInstruction::Return { .. } | MirInstruction::InvokeNormalResult { .. } | MirInstruction::ReturnFault { .. } | MirInstruction::FaultFrameEnter { .. } | MirInstruction::PinnedTextResidenceEnter { .. } | MirInstruction::PinnedTextResidenceTrap { .. } | MirInstruction::CheckedCallOutNormalResult { .. } | MirInstruction::CheckedCallOutEnd { .. } | MirInstruction::CheckedCallOutFault { .. } | MirInstruction::Phi { .. } | MirInstruction::TypeOp { .. } | MirInstruction::Copy { .. } | MirInstruction::CopyOwned { .. } | MirInstruction::DestroyOwned { .. } | MirInstruction::ArrayResidenceRelease { .. } | MirInstruction::StringResidenceInstall { .. } | MirInstruction::LocalContractWrite { .. } | MirInstruction::RecordFieldContractCheck { .. } | MirInstruction::RecordValuePublish { .. }
                        | MirInstruction::Debug { .. } | MirInstruction::KeepAlive { .. } | MirInstruction::ReleaseStrong { .. } | MirInstruction::Throw { .. } | MirInstruction::Catch { .. } | MirInstruction::Safepoint { .. } | MirInstruction::RefNew { .. } | MirInstruction::WeakRef { .. } | MirInstruction::Barrier { .. } | MirInstruction::FutureNew { .. } | MirInstruction::FutureSet { .. } | MirInstruction::Await { .. } | MirInstruction::Select { .. } => continue,
                    };
                    let target = module.canonical_callable_definition_symbol(&key)?;
                    let (target, _) = module.functions.get_key_value(target)?;
                    pending.push(target.as_str());
                }
            }
        }
        Some(reached)
    }
}

/// One admitted ordinary call site with its physical result kind. The MirCall
/// itself carries no result kind; the invoke operation selects the callee ABI.
#[derive(Debug, Clone)]
pub(super) struct OrdinaryCallSite {
    pub(super) call: MirCall,
    pub(super) result: InvokeCallResultKind,
}

/// One physical classification shared by selection, exact coverage and ABI.
/// Birth retains its own source/ABI owner even though its result is also Unit.
pub(in crate::mir::compiler::normal_default_pipeline) fn ordinary_call_site_v1(
    instruction: &MirInstruction,
) -> Option<(&MirCall, InvokeCallResultKind)> {
    match instruction {
        MirInstruction::Invoke { operation: InvokeOperation::Call { call, result }, .. }
            if !matches!(&call.callee, Callee::BirthConstructor { .. }) => Some((call, *result)),
        _ => None,
    }
}

fn collect_ordinary_calls(function: &MirFunction) -> Result<Vec<OrdinaryCallSite>, String> {
    let mut calls = Vec::new();
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();
    for block_id in block_ids {
        let block = function
            .blocks
            .get(&block_id)
            .ok_or_else(|| fault("function-block-membership"))?;
        for instruction in block.all_instructions() {
            let Some((call, result)) = ordinary_call_site_v1(instruction) else { continue; };
            ordinary_callable_key(&call.callee)?;
            if call.dst.is_some() {
                return Err(fault("ordinary-destination"));
            }
            calls.push(OrdinaryCallSite {
                call: call.clone(),
                result,
            });
        }
    }
    Ok(calls)
}

pub(super) fn issue_function<'module>(
    function: &'module MirFunction,
    role: PublishedLifecyclePhysicalFunctionRoleV1,
    script: bool,
    ordinary_calls: &[OrdinaryCallSite],
) -> Result<PublishedLifecyclePhysicalFunctionV1<'module>, String> {
    issue_function_with_module(None, function, role, script, ordinary_calls, None)
}

/// The same final consumer checks retained source outcomes against the complete
/// current Return inventory. It never reissues an original source projection.
pub(super) fn validate_object_return_census(
    function: &MirFunction, role: &PublishedLifecyclePhysicalFunctionRoleV1,
) -> Result<(), String> {
    if let Some(returns) = role.object_return_outcomes() {
        let key = role.ordinary_target().expect("object result role retains original key");
        if key.mir_symbol_projection() != function.signature.name
            || key.arity() as usize + usize::from(role.has_receiver()) != function.params.len()
        { return Err(fault("object-return/function-identity")); }
        let mut expected = BTreeSet::new();
        for row in returns {
            row.validate_physical_function_v1(function, role.ordinary_result().unwrap(), role.has_receiver())?;
            if !expected.insert(row.returned().0) { return Err(fault("object-return/duplicate")); }
        }
        let actual: BTreeSet<_> = function.blocks.iter().filter_map(|(id, block)|
            matches!(block.terminator, Some(MirInstruction::Return { .. })).then_some(*id)).collect();
        if expected.is_empty() || expected != actual { return Err(fault("object-return/census")); }
    }
    Ok(())
}

fn issue_function_with_module<'module>(
    module: Option<&'module MirModule>,
    function: &'module MirFunction,
    role: PublishedLifecyclePhysicalFunctionRoleV1,
    script: bool,
    ordinary_calls: &[OrdinaryCallSite],
    selected_view: Option<&PublishedMirBackendView<'module>>,
) -> Result<PublishedLifecyclePhysicalFunctionV1<'module>, String> {
    validate_object_return_census(function, &role)?;
    let source_field_reads = field_ref::source_array_reads(module, function, selected_view, &role)?;
    let received_field_reads = field_ref::source_received_reads(module, function, selected_view, &role)?;
    let received_field_binding = |block: BasicBlockId, instruction: &MirInstruction|
        received_field_reads.iter().any(|(id, original)| *id == block && original == instruction);
    let field_appends = match (module, selected_view) {
        (Some(module), Some(view)) => {
            if !std::ptr::eq(module, view.module()) { return Err(fault("field-append/view-membership")); }
            match view.retained_root_source() {
                Some(source) => source.finished_field_array_appends_v1(module, function, view.validated_named_arrays()?)?,
                None => Vec::new(),
            }
        }
        _ => Vec::new(),
    };
    let initializer_installs = match (module, selected_view.and_then(|view| view.retained_root_source())) {
        (Some(module), Some(source)) => source.finished_initializer_installs_v1(module, function)?,
        _ => Vec::new(),
    };
    let source_initializer_install = |block, instruction: &MirInstruction| initializer_installs
        .iter().any(|(bound_block, bound)| *bound_block == block && bound == instruction);
    let shared_initializers = shared_initializer::bindings(module, function, selected_view)?;
    let source_shared_binding = |block, instruction: &MirInstruction| shared_initializers
        .iter().any(|(bound_block, bound)| *bound_block == block && bound == instruction);
    let end_bindings = object_end::bindings(module, function, selected_view)?;
    let source_end_binding = |block, instruction: &MirInstruction| end_bindings
        .iter().any(|(bound_block, bound)| *bound_block == block && bound == instruction);
    let scalar_bindings =
        scalar::validate_selected_scalar_operations(module, function, selected_view)?;
    let main_argument_bindings = match (module, selected_view.and_then(|view| view.retained_root_source())) {
        (Some(module), Some(source)) => source.main_arguments_bindings_v1(module, function)?,
        _ => Vec::new(),
    };
    let main_argument_binding = |block, instruction: &MirInstruction| main_argument_bindings
        .iter().any(|(bound_block, bound)| *bound_block == block && bound == instruction);
    let array_construction_bindings = match (module, selected_view.and_then(|view| view.retained_root_source())) {
        (Some(module), Some(source)) => source.finished_array_construction_bindings_v1(module, function)?,
        _ => Vec::new(),
    };
    let array_construction_binding = |block, instruction: &MirInstruction| array_construction_bindings
        .iter().any(|(bound_block, bound)| *bound_block == block && bound == instruction);
    // Borrow the source-checked full-holder image, never admit Array from
    // its instruction shape or runtime descriptor alone.
    let mut owned_array_bindings = Vec::new();
    if let Some(view) = selected_view {
        for row in view.validated_named_arrays()? {
            if let crate::mir::normal_callable_semantic_package::EmittedNamedArrayRequirementV1::OwnedHolder(holder) = row {
                if holder.finished_symbol_v1()? == function.signature.name {
                    owned_array_bindings.push(holder.finished_bindings_v1()?);
                }
            }
        }
    }
    let array_counts = field_ref::source_array_counts(module, function, selected_view)?;
    let source_count_binding = |block: BasicBlockId, instruction: &MirInstruction| array_counts
        .iter().any(|(bound_block, bound)| *bound_block == block && bound == instruction);
    let source_array_binding = |block: BasicBlockId, instruction: &MirInstruction| {
        field_appends.iter().any(|(bound_block, bound)| *bound_block == block && bound == instruction)
            || array_construction_binding(block, instruction)
            || matches!(instruction, MirInstruction::Invoke { operation: InvokeOperation::OwnedArray(_), .. }
            | MirInstruction::Invoke { operation: InvokeOperation::DynamicCarrierArray(_), .. }
            | MirInstruction::ArrayResidenceRelease { .. })
            && owned_array_bindings.iter().flat_map(|rows| rows.iter())
                .any(|(bound_block, bound)| *bound_block == block && bound == instruction)
    };
    let mut ids: Vec<_> = function.blocks.keys().copied().collect();
    ids.sort();
    if ids.is_empty() || !function.blocks.contains_key(&function.entry_block) {
        return Err(fault("function-block-membership"));
    }
    let mut blocks = Vec::with_capacity(ids.len());
    for id in ids {
        let block = function
            .blocks
            .get(&id)
            .expect("sorted final block id must remain present");
        let terminator = block
            .terminator
            .as_ref()
            .ok_or_else(|| fault("block-terminator-missing"))?;
        let mut instructions = Vec::with_capacity(block.instructions.len());
        for (index, instruction) in block.instructions.iter().enumerate() {
            // A `.set` element write publishes only through the resolver's
            // own ArrayStoreAny route at this exact coordinate: the route is
            // the receiver's array proof and pins receiver/index operands.
            if let MirInstruction::ArrayElementWrite {
                site_id,
                kind,
                producer,
                receiver,
                index: element_index,
                value: _,
                dst,
            } = instruction
            {
                let routed = *kind == crate::mir::ArrayElementWriteKind::Set
                    && *producer == crate::mir::ArrayWriteProducerKind::MethodCall
                    && element_index.is_some()
                    && function.metadata.generic_method_routes.iter().any(|route| {
                        route.route_kind()
                            == crate::mir::generic_method_route_plan::GenericMethodRouteKind::ArrayStoreAny
                            && route.block() == block.id
                            && route.instruction_index() == index
                            && route.array_write_site_id() == Some(*site_id)
                            && route.receiver_value() == *receiver
                            && route.key_value() == *element_index
                            && route.result_value() == *dst
                    });
                if !routed {
                    let candidates: Vec<_> = function.metadata.generic_method_routes.iter()
                        .filter(|route| route.array_write_site_id() == Some(*site_id)
                            || (route.block() == block.id && route.instruction_index() == index))
                        .collect();
                    return Err(format!("{} function={} role={} block={:?} index={index} instruction={instruction:?} routes={candidates:?}",
                        fault("array-set-unsupported"), function.signature.name, role.wire_name(), block.id));
                }
            }
            let field_ref = prepare_field_ref(module, function, block.id, index, instruction, &source_field_reads)?;
            if !source_count_binding(id, instruction) && !source_initializer_install(id, instruction) && !source_shared_binding(id, instruction) && !source_end_binding(id, instruction) && !main_argument_binding(id, instruction) && !source_array_binding(id, instruction) && !scalar_bindings
                .iter()
                .any(|(bound_block, bound)| *bound_block == id && bound == instruction)
            {
                validate_instruction_with_context(field_ref, instruction, script, ordinary_calls, received_field_binding(id, instruction))
                    .map_err(|error| format!(
                        "{error} function={} role={} block={:?} instruction_index={index} instruction={instruction:?}",
                        function.signature.name, role.wire_name(), block.id,
                    ))?;
            }
            instructions.push(PublishedLifecyclePhysicalInstructionRefV1 {
                index: as_u32(index, "instruction-index")?,
                instruction,
                field_ref,
            });
        }
        let terminator_index = block.instructions.len();
        let terminator_field_ref =
            prepare_field_ref(module, function, block.id, terminator_index, terminator, &source_field_reads)?;
        if !source_end_binding(id, terminator) && !source_shared_binding(id, terminator) && !main_argument_binding(id, terminator) && !source_array_binding(id, terminator) {
            validate_instruction_with_context(
                terminator_field_ref,
                terminator,
                script,
                ordinary_calls,
                received_field_binding(id, terminator),
            ).map_err(|error| format!(
                "{error} function={} role={} block={:?} instruction_index={terminator_index} instruction={terminator:?}",
                function.signature.name, role.wire_name(), block.id,
            ))?;
        }
        let terminator_index = as_u32(terminator_index, "terminator-index")?;
        let edges = block
            .out_edges()
            .into_iter()
            .map(|edge| PublishedLifecyclePhysicalEdgeV1 {
                target: edge.target,
                args: edge.args,
            })
            .collect();
        blocks.push(PublishedLifecyclePhysicalBlockV1 {
            id,
            instructions: instructions.into_boxed_slice(),
            terminator: PublishedLifecyclePhysicalInstructionRefV1 {
                index: terminator_index,
                instruction: terminator,
                field_ref: terminator_field_ref,
            },
            edges,
        });
    }
    let ordinary_rows = blocks
        .iter()
        .flat_map(|block| {
            block
                .instructions()
                .iter()
                .copied()
                .chain(std::iter::once(block.terminator()))
        })
        .filter_map(|row| ordinary_call_site_v1(row.instruction()))
        .collect::<Vec<_>>();
    let mut consumed = vec![false; ordinary_rows.len()];
    for expected in ordinary_calls {
        let Some(index) = ordinary_rows
            .iter()
            .enumerate()
            .find_map(|(index, actual)| {
                (!consumed[index] && *actual.0 == expected.call && actual.1 == expected.result)
                    .then_some(index)
            })
        else {
            return Err(fault("ordinary-call-membership"));
        };
        consumed[index] = true;
    }
    if consumed.iter().any(|used| !used) {
        return Err(fault("ordinary-call-membership"));
    }
    Ok(PublishedLifecyclePhysicalFunctionV1 {
        function,
        name: function.signature.name.as_str(),
        role,
        params: &function.params,
        param_types: &function.signature.params,
        param_carriers: function.metadata.physical_param_carriers.as_deref(),
        value_types: &function.metadata.value_types,
        entry: function.entry_block,
        blocks: blocks.into_boxed_slice(),
    })
}

fn validate_instruction(
    instruction: &MirInstruction,
    script: bool,
    ordinary_calls: &[OrdinaryCallSite],
) -> Result<(), String> {
    validate_instruction_with_context(None, instruction, script, ordinary_calls, false)
}

fn validate_instruction_with_context(
    field_ref: Option<CanonicalFieldRefV1>,
    instruction: &MirInstruction,
    script: bool,
    ordinary_calls: &[OrdinaryCallSite],
    received_field: bool,
) -> Result<(), String> {
    if script {
        return if matches!(
            instruction,
            MirInstruction::Const {
                value: ConstValue::Integer(_)
                    | ConstValue::Bool(_)
                    | ConstValue::Float(_)
                    | ConstValue::Void,
                ..
            } | MirInstruction::Copy { .. }
                | MirInstruction::FaultFrameEnter { .. }
                | MirInstruction::Invoke {
                    operation: InvokeOperation::IntrinsicArrayNew
                        | InvokeOperation::ArrayStateContractClaim { .. }
                        | InvokeOperation::ArrayElementWrite { .. },
                    ..
                }
                | MirInstruction::InvokeNormalResult { .. }
                | MirInstruction::ArrayResidenceRelease { .. }
                | MirInstruction::ReturnFault { .. }
                | MirInstruction::Return { .. }
        ) {
            Ok(())
        } else {
            Err(fault("script-instruction-unsupported"))
        };
    }
    let ordinary = ordinary_call_site_v1(instruction).is_some_and(|(call, result)|
        ordinary_calls.iter().any(|expected| expected.call == *call && expected.result == result));
    let field_get = matches!(instruction, MirInstruction::FieldGet { .. }) && field_ref.is_some();
    let field_set = matches!(instruction, MirInstruction::FieldSet { .. }) && field_ref.is_some();
    let supported = ordinary
        || received_field && matches!(instruction, MirInstruction::Invoke { operation: InvokeOperation::FieldGet { .. } | InvokeOperation::ObjectFieldBorrow { .. } | InvokeOperation::Shared(crate::mir::instruction::SharedInvokeOperation::Borrow { .. }), .. })
        || field_get
        || field_set
        || matches!(
            instruction,
            MirInstruction::Const {
                value: ConstValue::Integer(_)
                    | ConstValue::Bool(_)
                    | ConstValue::String(_)
                    | ConstValue::Null
                    | ConstValue::Void,
                ..
            } | MirInstruction::BinOp {
                op: BinaryOp::Add | BinaryOp::Mul,
                ..
            } | MirInstruction::Compare { .. }
                | MirInstruction::Copy { .. }
                | MirInstruction::StringLength { .. } | MirInstruction::StringIndexOf { .. } | MirInstruction::StringCompare { .. }
                | MirInstruction::Phi { .. }
                | MirInstruction::KeepAlive { .. }
                | MirInstruction::ObjectFieldGet { .. }
                | MirInstruction::Invoke {
                    operation: InvokeOperation::Map(_)
                        | InvokeOperation::String(_)
                        | InvokeOperation::NewBox { .. }
                        | InvokeOperation::IntrinsicArrayNew
                        | InvokeOperation::FieldSet { .. }
                        | InvokeOperation::ObjectFieldSet { .. }
                        | InvokeOperation::ObjectFieldBorrow { .. }
                        | InvokeOperation::HomeRelease { .. }
                        | InvokeOperation::HomeReleaseIfLive { .. }
                        | InvokeOperation::OwnedFieldResidenceRelease { .. }
                        | InvokeOperation::OwnedObjectFieldRelease { .. }
                        | InvokeOperation::ReclaimUnpublished { .. }
                        | InvokeOperation::Birth(_)
                        | InvokeOperation::DynamicArrayGet { .. }
                        | InvokeOperation::DynamicCarrierEnd { .. },
                    ..
                }
                | MirInstruction::InvokeNormalResult { .. }
                | MirInstruction::ArrayElementWrite {
                    kind: crate::mir::ArrayElementWriteKind::Set,
                    producer: crate::mir::ArrayWriteProducerKind::MethodCall,
                    index: Some(_),
                    ..
                }
                | MirInstruction::ArrayRead { operation: crate::mir::instruction::ArrayReadOperationV1::Element { .. }, .. }
                | MirInstruction::ReturnFault { .. }
                | MirInstruction::FaultFrameEnter { .. }
                | MirInstruction::Branch { .. }
                | MirInstruction::Jump { .. }
                | MirInstruction::Return { .. }
                | MirInstruction::Call(MirCall {
                    callee: Callee::BirthConstructor { .. },
                    ..
                })
        );
    if supported {
        Ok(())
    } else {
        Err(fault("instruction-unsupported"))
    }
}

fn as_u32(value: usize, reason: &str) -> Result<u32, String> {
    u32::try_from(value).map_err(|_| fault(reason))
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][published-lifecycle-program/{reason}]")
}

#[cfg(test)]
#[path = "physical_program_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "physical_program_call_tests.rs"]
mod call_result_tests;

#[cfg(test)]
#[path = "physical_program_order_tests.rs"]
mod order_tests;

#[cfg(test)]
#[path = "borrowed_carrier_json_tests.rs"]
mod borrowed_carrier_json_tests;

#[cfg(test)]
#[path = "borrowed_source_publication_tests.rs"]
mod borrowed_source_publication_tests;

#[cfg(test)]
#[path = "borrowed_use_projection_tests.rs"]
mod borrowed_use_projection_tests;
