//! Canonical fallible control reuses existing Call and allocation operands.
//! This enum is not a target resolver or an arbitrary instruction wrapper.

use crate::mir::definitions::MirCall;
use crate::mir::{Effect, EffectMask, ValueId};

/// Checked field mutation semantics, selected by the retained source packet.
/// This mode is not a source capability or a runtime storage classifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldSetModeV1 {
    /// Unit on Normal; validation Fault leaves the slot unchanged.
    ExactSlot,
    /// No-Home I64 candidate. Validation Fault leaves the slot unchanged;
    /// commit detaches the old residence, then End consumes it. An End Fault
    /// does not roll back the new slot. Both Faults unwind the same caller Homes.
    DynamicI64Replace,
    /// Birth first-write: runtime checks Empty before committing the scalar.
    DynamicI64Initialize,
    /// Birth first-write: Empty validation precedes a move into stable Map storage.
    DynamicMapInitialize,
}

/// Source-selected field End law; neither mode creates an owning child Home.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldResidenceEndModeV1 {
    ArrayResidence,
    /// Take the current slot to Empty, then consume/End/dispose the detached
    /// outcome. End Fault does not restore the slot and cleanup continues.
    DynamicResidence,
}

/// The source-approved complete End body, in execution order. Only the
/// enclosing operation carries an SSA value; these steps issue no ownership.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectEndStepV1 {
    FieldResidence {
        field: hakorune_mir_defs::CanonicalFieldRefV1,
        mode: FieldResidenceEndModeV1,
    },
    ObjectField {
        field: hakorune_mir_defs::CanonicalFieldRefV1,
        child: hakorune_mir_defs::CanonicalObjectIdV1,
    },
    ReleaseOuter,
}


#[path = "invoke_map.rs"]
mod map;
#[path = "invoke_owned_array.rs"]
mod owned_array;
pub use owned_array::{OwnedArrayInvokeOperation, OwnedArraySlotDescriptorV1};
#[path = "invoke_string.rs"]
mod string;
pub use string::{StringInvokeOperand, StringInvokeOperation};
pub use map::{InvokeNormalResultKind, MapInvokeOperation, MapValueKind};

impl crate::mir::MirInstruction {
    /// Identifies physical sites needing retained lifecycle validation.
    /// Presence is an obligation, never source eligibility or permission.
    pub(crate) fn requires_lifecycle_validation(&self) -> bool {
        matches!(
            self,
            Self::Invoke { .. }
                | Self::InvokeNormalResult { .. }
                | Self::ReturnFault { .. }
                | Self::FaultFrameEnter { .. }
                | Self::ArrayResidenceRelease { .. }
                | Self::StringResidenceInstall { .. }
        ) || matches!(self, Self::Call(call)
                if matches!(call.callee, crate::mir::Callee::BirthConstructor { .. }))
    }
}

/// Hidden physical entry role, never a source parameter or runtime box type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultFrameMode {
    RootOwned,
    Borrowed,
}

/// Logical support for a non-owning object result. This descriptor contains
/// no runtime value, acquired Home, or source admission. The selected producer
/// and both physical consumers must independently prove the same relation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorrowedResultSupportV1 {
    /// Null or the original logical non-receiver argument.
    Formal(u32),
    /// The receiver's exact Map field owns the payload at this Text-key
    /// argument. The key is not a payload formal; holder and payload differ.
    ReceiverFieldSlot {
        field: hakorune_mir_defs::CanonicalFieldRefV1,
        key_ordinal: u32,
        object: hakorune_mir_defs::CanonicalObjectIdV1,
    },
}
impl BorrowedResultSupportV1 {
    /// Formal-only consumers must explicitly decline storage-supported results.
    pub fn formal_ordinal(self) -> Option<u32> {
        match self { Self::Formal(ordinal) => Some(ordinal), Self::ReceiverFieldSlot { .. } => None }
    }
}

/// Physical result projection; never source admission or inferred from a name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvokeCallResultKind {
    /// Owned checked String pair, acquired only on the Call Normal edge.
    OwnedString,
    Unit,
    I64,
    /// Exact source Bool; machine storage does not grant Integer semantics.
    Bool,
    /// The callee transfers a live checked-Map lease into caller-owned storage.
    Map,
    /// The callee transfers an owned object across the Return edge; the
    /// projected ValueId is a `MirType::Box(class)` handle the caller owns
    /// and must release exactly once.
    Handle,
    /// The callee's sealed claim is `NullableObject(C)`: the Return edge
    /// carries `BoxRef` or the `Void` null sentinel. The caller owns the
    /// object when non-null and owes a checked release — never the
    /// unconditional `Handle` contract.
    NullableHandle,
    /// Non-owning payload with its independently retained lifetime support.
    /// This result creates no Home or lease.
    BorrowedNullableHandle { support: BorrowedResultSupportV1 },
    /// Same Normal value carries a source-issued Null/Fresh/Forward outcome.
    /// Non-null does not establish ownership; Forward borrows this actual.
    MixedHandle { formal_ordinal: u32 },
}

impl InvokeCallResultKind {
    pub const fn borrowed_nullable_formal(formal_ordinal: u32) -> Self {
        Self::BorrowedNullableHandle { support: BorrowedResultSupportV1::Formal(formal_ordinal) }
    }
}

/// Physical Array write payload. Text keeps its complete descriptor; a
/// static byte string has no invented scalar SSA identity or String Home.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArrayElementWriteValueV1 {
    Scalar(ValueId),
    Text(StringInvokeOperand),
}
impl serde::Serialize for ArrayElementWriteValueV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Scalar(value) => serializer.serialize_u32(value.as_u32()),
            Self::Text(value) => serde::Serialize::serialize(value, serializer),
        }
    }
}
impl ArrayElementWriteValueV1 {
    pub fn scalar(&self) -> Option<ValueId> {
        match self { Self::Scalar(value) => Some(*value), Self::Text(_) => None }
    }
    pub fn used_values(&self) -> std::option::IntoIter<ValueId> {
        match self { Self::Scalar(value) => Some(*value), Self::Text(operand) => operand.value() }.into_iter()
    }
    pub fn rewrite_values(&mut self, rewrite: impl FnMut(&mut ValueId)) {
        match self { Self::Scalar(value) => { let mut rewrite = rewrite; rewrite(value); }, Self::Text(operand) => operand.rewrite_value(rewrite) }
    }
    pub fn valid_for(&self, kind: crate::mir::ArrayElementWriteKind, producer: crate::mir::ArrayWriteProducerKind, index: Option<ValueId>) -> bool {
        match self {
            Self::Scalar(_) => crate::mir::array_element_write::validate_shape(kind, index).is_ok(),
            Self::Text(value) => kind == crate::mir::ArrayElementWriteKind::Push
                && producer == crate::mir::ArrayWriteProducerKind::MethodCall && index.is_none() && value.is_text(),
        }
    }
}

/// One logical Birth actual. Static Text has no invented SSA value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BirthArgumentV1 {
    Value(ValueId),
    Text(StringInvokeOperand),
}
impl BirthArgumentV1 {
    pub fn value(&self) -> Option<ValueId> {
        match self { Self::Value(value) => Some(*value), Self::Text(text) => text.value() }
    }
    pub fn rewrite_value(&mut self, rewrite: impl FnMut(&mut ValueId)) {
        match self { Self::Value(value) => { let mut rewrite = rewrite; rewrite(value); }, Self::Text(text) => text.rewrite_value(rewrite) }
    }
}

/// Checked Birth is Unit-only. Its ordered actuals are the sole operand census;
/// no ordinary Call destination, result classification or parallel argv exists.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BirthInvokeV1 {
    pub key: hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
    pub receiver: ValueId,
    pub arguments: Box<[BirthArgumentV1]>,
    pub effects: EffectMask,
}
impl BirthInvokeV1 {
    pub fn from_value_arguments(key: hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        receiver: ValueId, arguments: Vec<ValueId>, effects: EffectMask) -> Self {
        Self { key, receiver, arguments: arguments.into_iter().map(BirthArgumentV1::Value).collect(), effects }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InvokeOperation {
    Birth(BirthInvokeV1),
    /// The embedded destination must be absent; the Normal projection owns it.
    Call {
        call: MirCall,
        result: InvokeCallResultKind,
    },
    /// Allocation only; constructor arguments belong to the subsequent Birth.
    NewBox {
        object: hakorune_mir_defs::CanonicalObjectIdV1,
    },
    /// Intrinsic Array allocation; one value exists only on Normal.
    IntrinsicArrayNew,
    /// Compiler-owned Main entry adapter. All immutable UTF-8 arguments are
    /// constructed before Normal publishes one owned Array residence.
    ScriptArgumentsNew { values: Box<[String]> },
    /// Checked non-host Map lifecycle; source obligations come from Completion.
    Map(MapInvokeOperation),
    /// Checked expression String operations; never an Object handle lane.
    String(StringInvokeOperation),
    /// Source-owned object slots; distinct from numeric Array state.
    OwnedArray(OwnedArrayInvokeOperation),
    /// Adopt the source-issued numeric state contract; Unit on Normal.
    ArrayStateContractClaim { contract_id: String, array: ValueId },
    /// Checked mutation; Unit on Normal and no mutation on Fault.
    ArrayElementWrite {
        site_id: crate::mir::ArrayWriteSiteId,
        kind: crate::mir::ArrayElementWriteKind,
        producer: crate::mir::ArrayWriteProducerKind,
        receiver: ValueId,
        index: Option<ValueId>,
        value: ArrayElementWriteValueV1,
    },
    /// Exact received-object scalar field; I64 only on Normal, no value on Fault.
    FieldGet {
        field: hakorune_mir_defs::CanonicalFieldRefV1,
        base: ValueId,
    },
    /// Checked source-selected mutation; the mode defines its commit/Fault law.
    FieldSet {
        mode: FieldSetModeV1,
        field: hakorune_mir_defs::CanonicalFieldRefV1,
        base: ValueId,
        value: ValueId,
    },
    /// Store a proven user-object child handle into an exact field. The
    /// lease moves into the object slot on both edges — a faulted checked
    /// store leaves the in-flight child owned by this frame, so the
    /// emitted discharge releases it under the child's own type id.
    ObjectFieldSet {
        field: hakorune_mir_defs::CanonicalFieldRefV1,
        base: ValueId,
        value: ValueId,
        child: hakorune_mir_defs::CanonicalObjectIdV1,
    },
    /// Consume a completed Home; both outcomes continue the cleanup chain.
    HomeRelease {
        object: hakorune_mir_defs::CanonicalObjectIdV1,
        value: ValueId,
    },
    /// Release a nullable received handle only when the value is non-null:
    /// the `Void` sentinel skips the reclaim, a `BoxRef` releases once.
    HomeReleaseIfLive {
        object: hakorune_mir_defs::CanonicalObjectIdV1,
        value: ValueId,
    },
    /// Complete End, including all children, only for the SAME value's Fresh
    /// outcome. Null/Forward consume no owned storage or supporting anchor.
    ObjectEndIfOwned {
        object: hakorune_mir_defs::CanonicalObjectIdV1,
        value: ValueId,
        end: Box<[ObjectEndStepV1]>,
    },
    /// Reclaim only incomplete outer storage; never invoke the parent's fini.
    ReclaimUnpublished {
        object: hakorune_mir_defs::CanonicalObjectIdV1,
        value: ValueId,
    },
    /// Read the field's owned residence handle and release it when live.
    /// Slots are zero-initialized, so an unwritten field is skipped; the
    /// operation itself only runs for proven provider residences.
    OwnedFieldResidenceRelease {
        mode: FieldResidenceEndModeV1,
        field: hakorune_mir_defs::CanonicalFieldRefV1,
        base: ValueId,
    },
    /// Read the field's owned user-object handle and release it when live.
    /// The child must be `PlainI64NoHook` or a sealed
    /// `OwnedArrayFieldsNoHook` — the physical consumer walks the child's
    /// `owned_residences` layout mark newest-first before
    /// `home_release_plain_i64_v1` under the child's own type id. Deeper
    /// teardown stays unadmitted.
    OwnedObjectFieldRelease {
        field: hakorune_mir_defs::CanonicalFieldRefV1,
        base: ValueId,
        child: hakorune_mir_defs::CanonicalObjectIdV1,
    },
}

impl InvokeOperation {
    pub fn normal_result_kind(&self) -> Option<InvokeNormalResultKind> {
        match self {
            Self::NewBox { .. } | Self::IntrinsicArrayNew | Self::ScriptArgumentsNew { .. } => Some(InvokeNormalResultKind::Handle),
            Self::FieldGet { .. } => Some(InvokeNormalResultKind::I64),
            Self::Map(operation) => operation.normal_result_kind(),
            Self::String(operation) => operation.normal_result_kind(),
            Self::OwnedArray(operation) => operation.normal_result_kind(),
            Self::Call {
                result: InvokeCallResultKind::I64,
                ..
            } => Some(InvokeNormalResultKind::I64),
            Self::Call {
                result: InvokeCallResultKind::Bool,
                ..
            } => Some(InvokeNormalResultKind::Bool),
            Self::Call { result: InvokeCallResultKind::OwnedString, .. } => Some(InvokeNormalResultKind::OwnedString),
            Self::Call {
                result: InvokeCallResultKind::Map,
                ..
            } => Some(InvokeNormalResultKind::Map),
            Self::Call {
                result: InvokeCallResultKind::Handle,
                ..
            } => Some(InvokeNormalResultKind::Handle),
            Self::Call {
                result: InvokeCallResultKind::NullableHandle,
                ..
            } => Some(InvokeNormalResultKind::NullableHandle),
            Self::Call {
                result: InvokeCallResultKind::BorrowedNullableHandle { .. },
                ..
            } => Some(InvokeNormalResultKind::BorrowedNullableHandle),
            Self::Call { result: InvokeCallResultKind::MixedHandle { .. }, .. } => {
                Some(InvokeNormalResultKind::MixedHandle)
            }
            Self::Call {
                result: InvokeCallResultKind::Unit,
                ..
            }
            | Self::Birth(_)
            | Self::FieldSet { .. }
            | Self::ObjectFieldSet { .. }
            | Self::ArrayStateContractClaim { .. }
            | Self::ArrayElementWrite { .. }
            | Self::HomeRelease { .. }
            | Self::HomeReleaseIfLive { .. }
            | Self::ObjectEndIfOwned { .. }
            | Self::ReclaimUnpublished { .. }
            | Self::OwnedFieldResidenceRelease { .. }
            | Self::OwnedObjectFieldRelease { .. } => None,
        }
    }

    pub fn effects(&self) -> EffectMask {
        match self {
            Self::Map(operation) => operation.effects(),
            Self::String(operation) => operation.effects(),
            Self::OwnedArray(operation) => operation.effects(),
            Self::Birth(call) => call.effects.add(Effect::Control),
            Self::Call { call, .. } => call.effects.add(Effect::Control),
            Self::NewBox { .. } | Self::IntrinsicArrayNew | Self::ScriptArgumentsNew { .. } => EffectMask::CONTROL.add(Effect::Alloc),
            Self::FieldGet { .. } => EffectMask::READ.add(Effect::Control),
            Self::ArrayElementWrite { value: ArrayElementWriteValueV1::Text(_), .. } => EffectMask::WRITE.add(Effect::Alloc).add(Effect::Control),
            Self::FieldSet { mode: FieldSetModeV1::DynamicMapInitialize, .. } =>
                EffectMask::WRITE.add(Effect::Alloc).add(Effect::Control),
            Self::FieldSet { .. }
            | Self::ObjectFieldSet { .. }
            | Self::ArrayStateContractClaim { .. }
            | Self::ArrayElementWrite { .. } => EffectMask::WRITE.add(Effect::Control),
            Self::HomeRelease { .. }
            | Self::HomeReleaseIfLive { .. }
            | Self::ObjectEndIfOwned { .. }
            | Self::ReclaimUnpublished { .. }
            | Self::OwnedFieldResidenceRelease { .. }
            | Self::OwnedObjectFieldRelease { .. } => EffectMask::WRITE
                .union(EffectMask::MUT)
                .union(EffectMask::IO)
                .add(Effect::Control),
        }
    }

    pub fn used_values(&self) -> Vec<ValueId> {
        match self {
            Self::Map(operation) => operation.used_values(),
            Self::String(operation) => operation.used_values(),
            Self::OwnedArray(operation) => operation.used_values(),
            Self::Birth(call) => std::iter::once(call.receiver).chain(call.arguments.iter().filter_map(BirthArgumentV1::value)).collect(),
            Self::Call { call, .. } => {
                let mut values = Vec::new();
                call.for_each_value_operand(|value| values.push(value));
                values
            }
            Self::NewBox { .. } | Self::IntrinsicArrayNew | Self::ScriptArgumentsNew { .. } => Vec::new(),
            Self::ArrayStateContractClaim { array, .. } => vec![*array],
            Self::ArrayElementWrite {
                receiver,
                index,
                value,
                ..
            } => {
                let mut values = vec![*receiver];
                values.extend(index.iter().copied());
                values.extend(value.used_values());
                values
            }
            Self::FieldSet { base, value, .. } | Self::ObjectFieldSet { base, value, .. } => {
                vec![*base, *value]
            }
            Self::FieldGet { base, .. }
            | Self::OwnedFieldResidenceRelease { base, .. }
            | Self::OwnedObjectFieldRelease { base, .. } => vec![*base],
            Self::HomeRelease { value, .. }
            | Self::HomeReleaseIfLive { value, .. }
            | Self::ObjectEndIfOwned { value, .. }
            | Self::ReclaimUnpublished { value, .. } => {
                vec![*value]
            }
        }
    }

    pub fn rewrite_values(&mut self, mut rewrite: impl FnMut(&mut ValueId)) {
        match self {
            Self::Map(operation) => operation.rewrite_values(rewrite),
            Self::String(operation) => operation.rewrite_values(rewrite),
            Self::OwnedArray(operation) => operation.rewrite_values(rewrite),
            Self::Birth(call) => {
                rewrite(&mut call.receiver);
                for argument in &mut call.arguments { argument.rewrite_value(&mut rewrite); }
            }
            Self::Call { call, .. } => {
                call.rewrite_value_operands(rewrite);
            }
            Self::NewBox { .. } | Self::IntrinsicArrayNew | Self::ScriptArgumentsNew { .. } => {}
            Self::ArrayStateContractClaim { array, .. } => rewrite(array),
            Self::ArrayElementWrite {
                receiver,
                index,
                value,
                ..
            } => {
                rewrite(receiver);
                if let Some(index) = index {
                    rewrite(index);
                }
                value.rewrite_values(rewrite);
            }
            Self::FieldGet { base, .. }
            | Self::OwnedFieldResidenceRelease { base, .. }
            | Self::OwnedObjectFieldRelease { base, .. } => rewrite(base),
            Self::HomeRelease { value, .. }
            | Self::HomeReleaseIfLive { value, .. }
            | Self::ObjectEndIfOwned { value, .. }
            | Self::ReclaimUnpublished { value, .. } => rewrite(value),
            Self::FieldSet { base, value, .. } | Self::ObjectFieldSet { base, value, .. } => {
                rewrite(base);
                rewrite(value);
            }
        }
    }
}
