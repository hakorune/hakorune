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

#[path = "physical_program_call_helpers.rs"]
mod call_helpers;
#[path = "physical_program_field_ref.rs"]
mod field_ref;
#[path = "physical_program_object_identity.rs"]
mod object_identity;
pub(crate) use call_helpers::{ordinary_call_receiver, ordinary_callable_key};
use field_ref::prepare_field_ref;

/// One exact selected function in the physical lifecycle program.
#[derive(Debug, Clone)]
pub(crate) enum PublishedLifecyclePhysicalFunctionRoleV1 {
    Root {
        result: CompiledEntryRootResultV1,
    },
    BirthUnit {
        abi: crate::mir::normal_callable_semantic_package::BirthAbiHandoffV1,
    },
    OrdinaryI64 {
        key: hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        receiver_object: Option<CanonicalObjectIdV1>,
    },
    /// Caller-owned opaque Map storage result; never an i64 handle.
    OrdinaryMap {
        key: hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        receiver_object: Option<CanonicalObjectIdV1>,
    },
    /// Caller-owned transferred object result (`return new` callee); the
    /// caller holds the handle and owes exactly one release.
    OrdinaryHandle {
        key: hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        receiver_object: Option<CanonicalObjectIdV1>,
    },
    /// Caller-owned nullable object result (`NullableObject` claim): the
    /// wire value is a live handle or the null sentinel, and the caller
    /// owes a checked release, never an unconditional one.
    OrdinaryNullableHandle {
        key: hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        receiver_object: Option<CanonicalObjectIdV1>,
    },
}

impl PublishedLifecyclePhysicalFunctionRoleV1 {
    pub(crate) const fn wire_name(&self) -> &'static str {
        match self {
            Self::Root {
                result: CompiledEntryRootResultV1::I64,
                ..
            } => "root_i64",
            Self::Root {
                result: CompiledEntryRootResultV1::Unit,
                ..
            } => "root_unit",
            Self::BirthUnit { .. } => "birth_unit",
            Self::OrdinaryI64 { .. } => "ordinary_i64",
            Self::OrdinaryMap { .. } => "ordinary_map",
            Self::OrdinaryHandle { .. } => "ordinary_handle",
            Self::OrdinaryNullableHandle { .. } => "ordinary_nullable_handle",
        }
    }

    pub(crate) fn birth_target(
        &self,
    ) -> Option<&hakorune_mir_defs::CanonicalSameModuleCallableKeyV1> {
        match self {
            Self::BirthUnit { abi } => Some(abi.target()),
            Self::Root { .. }
            | Self::OrdinaryI64 { .. }
            | Self::OrdinaryMap { .. }
            | Self::OrdinaryHandle { .. }
            | Self::OrdinaryNullableHandle { .. } => None,
        }
    }

    pub(crate) fn ordinary_target(
        &self,
    ) -> Option<&hakorune_mir_defs::CanonicalSameModuleCallableKeyV1> {
        match self {
            Self::OrdinaryI64 { key, .. }
            | Self::OrdinaryMap { key, .. }
            | Self::OrdinaryHandle { key, .. }
            | Self::OrdinaryNullableHandle { key, .. } => Some(key),
            Self::Root { .. } | Self::BirthUnit { .. } => None,
        }
    }

    pub(crate) const fn receiver_object(&self) -> Option<CanonicalObjectIdV1> {
        match self {
            Self::OrdinaryI64 {
                receiver_object, ..
            }
            | Self::OrdinaryMap {
                receiver_object, ..
            }
            | Self::OrdinaryHandle {
                receiver_object, ..
            }
            | Self::OrdinaryNullableHandle {
                receiver_object, ..
            } => *receiver_object,
            Self::Root { .. } | Self::BirthUnit { .. } => None,
        }
    }

    pub(crate) fn has_receiver(&self) -> bool {
        match self {
            Self::BirthUnit { .. } => true,
            Self::OrdinaryI64 { key, .. }
            | Self::OrdinaryMap { key, .. }
            | Self::OrdinaryHandle { key, .. }
            | Self::OrdinaryNullableHandle { key, .. } => {
                key.namespace() == SameModuleCallableNamespaceV1::InstanceBoxMethod
            }
            Self::Root { .. } => false,
        }
    }

    pub(crate) fn receiver_value(&self, params: &[ValueId]) -> Option<ValueId> {
        self.has_receiver()
            .then(|| params.first().copied())
            .flatten()
    }
}

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
}

/// One admitted ordinary call site with its physical result kind. The MirCall
/// itself carries no result kind; the invoke operation selects the callee ABI.
#[derive(Debug, Clone)]
pub(super) struct OrdinaryCallSite {
    pub(super) call: MirCall,
    pub(super) result: InvokeCallResultKind,
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
            let MirInstruction::Invoke {
                operation:
                    InvokeOperation::Call {
                        call,
                        result:
                            result @ (InvokeCallResultKind::I64
                            | InvokeCallResultKind::Map
                            | InvokeCallResultKind::NullableHandle),
                    },
                ..
            } = instruction
            else {
                continue;
            };
            ordinary_callable_key(&call.callee)?;
            if call.dst.is_some() {
                return Err(fault("ordinary-destination"));
            }
            calls.push(OrdinaryCallSite {
                call: call.clone(),
                result: *result,
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
    issue_function_with_module(None, function, role, script, ordinary_calls)
}

fn issue_function_with_module<'module>(
    module: Option<&'module MirModule>,
    function: &'module MirFunction,
    role: PublishedLifecyclePhysicalFunctionRoleV1,
    script: bool,
    ordinary_calls: &[OrdinaryCallSite],
) -> Result<PublishedLifecyclePhysicalFunctionV1<'module>, String> {
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
            let field_ref = prepare_field_ref(module, function, block.id, index, instruction)?;
            validate_instruction_with_context(field_ref, instruction, script, ordinary_calls)?;
            instructions.push(PublishedLifecyclePhysicalInstructionRefV1 {
                index: as_u32(index, "instruction-index")?,
                instruction,
                field_ref,
            });
        }
        let terminator_index = block.instructions.len();
        let terminator_field_ref =
            prepare_field_ref(module, function, block.id, terminator_index, terminator)?;
        validate_instruction_with_context(
            terminator_field_ref,
            terminator,
            script,
            ordinary_calls,
        )?;
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
        .filter_map(|row| {
            let MirInstruction::Invoke {
                operation:
                    InvokeOperation::Call {
                        call,
                        result:
                            result @ (InvokeCallResultKind::I64
                            | InvokeCallResultKind::Map
                            | InvokeCallResultKind::NullableHandle),
                    },
                ..
            } = row.instruction()
            else {
                return None;
            };
            Some((call, *result))
        })
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
    validate_instruction_with_context(None, instruction, script, ordinary_calls)
}

fn validate_instruction_with_context(
    field_ref: Option<CanonicalFieldRefV1>,
    instruction: &MirInstruction,
    script: bool,
    ordinary_calls: &[OrdinaryCallSite],
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
    let ordinary = matches!(
        instruction,
        MirInstruction::Invoke {
            operation: InvokeOperation::Call {
                call,
                result:
                    result @ (InvokeCallResultKind::I64
                    | InvokeCallResultKind::Map
                    | InvokeCallResultKind::NullableHandle),
            },
            ..
        } if ordinary_calls
            .iter()
            .any(|expected| expected.call == *call && expected.result == *result)
    );
    let field_get = matches!(instruction, MirInstruction::FieldGet { .. }) && field_ref.is_some();
    let field_set =
        matches!(instruction, MirInstruction::FieldSet { .. }) && field_ref.is_some();
    let supported = ordinary
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
                op: BinaryOp::Add,
                ..
            } | MirInstruction::Compare { .. }
                | MirInstruction::Copy { .. }
                | MirInstruction::Phi { .. }
                | MirInstruction::ObjectFieldGet { .. }
                | MirInstruction::Invoke {
                    operation: InvokeOperation::Map(_)
                        | InvokeOperation::NewBox { .. }
                        | InvokeOperation::IntrinsicArrayNew
                        | InvokeOperation::FieldSet { .. }
                        | InvokeOperation::ObjectFieldSet { .. }
                        | InvokeOperation::HomeRelease { .. }
                        | InvokeOperation::HomeReleaseIfLive { .. }
                        | InvokeOperation::OwnedFieldResidenceRelease { .. }
                        | InvokeOperation::OwnedObjectFieldRelease { .. }
                        | InvokeOperation::ReclaimUnpublished { .. }
                        | InvokeOperation::Call {
                            call: MirCall {
                                callee: Callee::BirthConstructor { .. },
                                ..
                            },
                            result: InvokeCallResultKind::Unit
                        },
                    ..
                }
                | MirInstruction::InvokeNormalResult { .. }
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
