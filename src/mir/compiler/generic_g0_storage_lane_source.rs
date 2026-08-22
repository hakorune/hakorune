//! Source-only Generic G0 storage/lane projection.
//!
//! This row keeps the source receiver policy, declaration metadata, and the
//! already-sealed explicit entry bindings together.  It is not a physical ABI
//! descriptor: it owns no ValueId, MirFunction, EffectMask, Builder, or
//! session state.  The carrier tag is mechanical and deliberately local to
//! Generic G0; S6C physical rows are not an authority here.

use crate::mir::exact_trivial_return_abi::ExactTrivialReturnAbiV1;
use crate::mir::loop_recipe_contract::VerifiedGenericRecipeProductG0;
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingOriginV1, BindingRefV1, FunctionOriginV1, FunctionOwnerIdV1,
    ReceiverPolicyV1, SemanticOwnerSourceKindV1, SourceBindingSiteV1, SourcePathSegmentV1,
    LoopExecutionFrameKeyV1, VerifiedResolvedBodyShapeInventoryV1,
};

use super::function_input::ResolvedFunctionLoweringInputV1;
use super::generic_g0_source_parent::VerifiedGenericG0EntryBindingV1;
use super::generic_g0_top_level_declaration_header::
    VerifiedGenericG0TopLevelDeclarationHeaderV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericG0StorageLaneCarrierV1 {
    ExistingCallableI64,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GenericG0StorageLaneReceiverRowV1 {
    binding: BindingRefV1,
    carrier: GenericG0StorageLaneCarrierV1,
}

impl GenericG0StorageLaneReceiverRowV1 {
    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) const fn carrier(&self) -> GenericG0StorageLaneCarrierV1 {
        self.carrier
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GenericG0StorageLaneFormalRowV1 {
    ordinal: u32,
    binding: BindingRefV1,
    declared_type_name: Box<str>,
    abi: ExactTrivialReturnAbiV1,
    carrier: GenericG0StorageLaneCarrierV1,
}

impl GenericG0StorageLaneFormalRowV1 {
    pub(crate) const fn ordinal(&self) -> u32 {
        self.ordinal
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) fn declared_type_name(&self) -> &str {
        &self.declared_type_name
    }

    pub(crate) const fn abi(&self) -> ExactTrivialReturnAbiV1 {
        self.abi
    }

    pub(crate) const fn carrier(&self) -> GenericG0StorageLaneCarrierV1 {
        self.carrier
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GenericG0StorageLaneSourceRejectV1 {
    OwnerMismatch,
    OriginMismatch,
    SourceKindMismatch,
    BodyRootMismatch,
    FrameMismatch,
    HeaderParameterCountMismatch,
    HeaderParameterOrdinalMismatch,
    EntryCountMismatch,
    EntryBindingOwnerMismatch,
    EntryBindingKindMismatch,
    EntryBindingOriginMismatch,
    EntryBindingMismatch,
    EntryTypeMismatch,
    EntryAbiMismatch,
    ReceiverPolicyUnsupported,
    ReceiverBindingMissing,
    ReceiverBindingUnexpected,
    ReceiverBindingOwnerMismatch,
    ReceiverBindingKindMismatch,
    ReceiverBindingOriginMismatch,
    ReceiverBindingDuplicatesFormal,
    SourceLogicalArityOverflow,
    PhysicalLaneCountOverflow,
}

/// One parent-owned source projection.  The receiver is intentionally not an
/// explicit formal row; its lane count is a separate callable-prefix axis.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericG0StorageLaneSourceProjectionV1 {
    owner: FunctionOwnerIdV1,
    origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    body_root: SourcePathSegmentV1,
    frame: LoopExecutionFrameKeyV1,
    uses: Box<[Box<str>]>,
    attrs: Box<[Box<str>]>,
    receiver_policy: ReceiverPolicyV1,
    receiver: Option<GenericG0StorageLaneReceiverRowV1>,
    formals: Box<[GenericG0StorageLaneFormalRowV1]>,
    source_logical_arity: u32,
    physical_callable_lane_count: u32,
}

impl VerifiedGenericG0StorageLaneSourceProjectionV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn origin(&self) -> FunctionOriginV1 {
        self.origin
    }

    pub(crate) const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.source_kind
    }

    pub(crate) fn body_root(&self) -> &SourcePathSegmentV1 {
        &self.body_root
    }

    pub(crate) fn frame(&self) -> &LoopExecutionFrameKeyV1 {
        &self.frame
    }

    pub(crate) fn uses(&self) -> &[Box<str>] {
        &self.uses
    }

    pub(crate) fn attrs(&self) -> &[Box<str>] {
        &self.attrs
    }

    pub(crate) const fn receiver_policy(&self) -> ReceiverPolicyV1 {
        self.receiver_policy
    }

    pub(crate) fn receiver(&self) -> Option<&GenericG0StorageLaneReceiverRowV1> {
        self.receiver.as_ref()
    }

    pub(crate) fn formals(&self) -> &[GenericG0StorageLaneFormalRowV1] {
        &self.formals
    }

    pub(crate) fn source_logical_arity(&self) -> u32 {
        self.source_logical_arity
    }

    pub(crate) const fn receiver_lane_count(&self) -> u32 {
        if self.receiver.is_some() { 1 } else { 0 }
    }

    pub(crate) fn physical_formal_lane_count(&self) -> u32 {
        self.source_logical_arity()
    }

    pub(crate) fn physical_callable_lane_count(&self) -> u32 {
        self.physical_callable_lane_count
    }
}

pub(crate) fn issue_generic_g0_storage_lane_source_projection_v1(
    input: &ResolvedFunctionLoweringInputV1<'_>,
    product: &VerifiedGenericRecipeProductG0,
    header: &VerifiedGenericG0TopLevelDeclarationHeaderV1,
    body_shape: &VerifiedResolvedBodyShapeInventoryV1,
    entries: &[VerifiedGenericG0EntryBindingV1],
) -> Result<
    VerifiedGenericG0StorageLaneSourceProjectionV1,
    GenericG0StorageLaneSourceRejectV1,
> {
    let function = input.function();
    if header.owner() != input.owner() || product.context().owner() != input.owner() {
        return Err(GenericG0StorageLaneSourceRejectV1::OwnerMismatch);
    }
    if header.origin() != function.function_origin()
        || product.context().origin() != function.function_origin()
    {
        return Err(GenericG0StorageLaneSourceRejectV1::OriginMismatch);
    }
    if header.source_kind() != function.source_kind()
        || product.context().source_kind() != function.source_kind()
    {
        return Err(GenericG0StorageLaneSourceRejectV1::SourceKindMismatch);
    }
    if body_shape.owner() != input.owner()
        || body_shape.body_root() != &function.root_profile().body_root()
    {
        return Err(GenericG0StorageLaneSourceRejectV1::BodyRootMismatch);
    }
    let (loop_source, _) = function
        .resolved_loop_source_context(product.context().loop_site())
        .map_err(|_| GenericG0StorageLaneSourceRejectV1::FrameMismatch)?;
    if !product.context().frame().matches(&loop_source.frame_key()) {
        return Err(GenericG0StorageLaneSourceRejectV1::FrameMismatch);
    }
    if header.parameters().len() != entries.len() {
        return Err(GenericG0StorageLaneSourceRejectV1::HeaderParameterCountMismatch);
    }
    let mut formals = Vec::with_capacity(entries.len());
    for (index, entry) in entries.iter().enumerate() {
        let ordinal = u32::try_from(index)
            .map_err(|_| GenericG0StorageLaneSourceRejectV1::SourceLogicalArityOverflow)?;
        let parameter = header
            .parameters()
            .get(index)
            .ok_or(GenericG0StorageLaneSourceRejectV1::HeaderParameterCountMismatch)?;
        if parameter.ordinal() != ordinal || entry.parameter_index() != ordinal {
            return Err(GenericG0StorageLaneSourceRejectV1::HeaderParameterOrdinalMismatch);
        }
        let record = function
            .binding(entry.binding())
            .ok_or(GenericG0StorageLaneSourceRejectV1::EntryBindingMismatch)?;
        if entry.binding().owner() != input.owner() {
            return Err(GenericG0StorageLaneSourceRejectV1::EntryBindingOwnerMismatch);
        }
        if record.kind() != (BindingKindV1::Parameter { index: ordinal }) {
            return Err(GenericG0StorageLaneSourceRejectV1::EntryBindingKindMismatch);
        }
        if record.origin()
            != &BindingOriginV1::Source(SourceBindingSiteV1::Parameter { index: ordinal })
        {
            return Err(GenericG0StorageLaneSourceRejectV1::EntryBindingOriginMismatch);
        }
        if parameter.declared_type_name() != Some("i64") {
            return Err(GenericG0StorageLaneSourceRejectV1::EntryTypeMismatch);
        }
        if entry.abi() != ExactTrivialReturnAbiV1::I64 {
            return Err(GenericG0StorageLaneSourceRejectV1::EntryAbiMismatch);
        }
        formals.push(GenericG0StorageLaneFormalRowV1 {
            ordinal,
            binding: entry.binding(),
            declared_type_name: parameter.declared_type_name().unwrap().into(),
            abi: entry.abi(),
            carrier: GenericG0StorageLaneCarrierV1::ExistingCallableI64,
        });
    }

    let receiver_policy = function.root_profile().receiver_policy();
    let receiver_binding = function.declaration_binding(&SourceBindingSiteV1::Receiver);
    let receiver = match receiver_policy {
        ReceiverPolicyV1::DeclaredInstance => {
            let binding = receiver_binding
                .ok_or(GenericG0StorageLaneSourceRejectV1::ReceiverBindingMissing)?;
            let record = function
                .binding(binding)
                .ok_or(GenericG0StorageLaneSourceRejectV1::ReceiverBindingOwnerMismatch)?;
            if binding.owner() != input.owner() {
                return Err(GenericG0StorageLaneSourceRejectV1::ReceiverBindingOwnerMismatch);
            }
            if record.kind() != BindingKindV1::Receiver {
                return Err(GenericG0StorageLaneSourceRejectV1::ReceiverBindingKindMismatch);
            }
            if record.origin() != &BindingOriginV1::Source(SourceBindingSiteV1::Receiver) {
                return Err(GenericG0StorageLaneSourceRejectV1::ReceiverBindingOriginMismatch);
            }
            if formals.iter().any(|row| row.binding() == binding) {
                return Err(GenericG0StorageLaneSourceRejectV1::ReceiverBindingDuplicatesFormal);
            }
            Some(GenericG0StorageLaneReceiverRowV1 {
                binding,
                carrier: GenericG0StorageLaneCarrierV1::ExistingCallableI64,
            })
        }
        ReceiverPolicyV1::Absent => {
            if receiver_binding.is_some() {
                return Err(GenericG0StorageLaneSourceRejectV1::ReceiverBindingUnexpected);
            }
            None
        }
        ReceiverPolicyV1::StaticCurrentOwner => {
            return Err(GenericG0StorageLaneSourceRejectV1::ReceiverPolicyUnsupported)
        }
    };

    let source_logical_arity = u32::try_from(formals.len())
        .map_err(|_| GenericG0StorageLaneSourceRejectV1::SourceLogicalArityOverflow)?;
    let physical_callable_lane_count = u32::from(receiver.is_some())
        .checked_add(source_logical_arity)
        .ok_or(GenericG0StorageLaneSourceRejectV1::PhysicalLaneCountOverflow)?;

    let attrs = header
        .attrs()
        .iter()
        .map(|attr| attr.name().into())
        .collect::<Vec<_>>()
        .into_boxed_slice();
    let uses = header
        .uses()
        .iter()
        .map(|use_name| use_name.as_ref().into())
        .collect::<Vec<_>>()
        .into_boxed_slice();

    Ok(VerifiedGenericG0StorageLaneSourceProjectionV1 {
        owner: input.owner(),
        origin: function.function_origin(),
        source_kind: function.source_kind(),
        body_root: function.root_profile().body_root(),
        frame: product.context().frame().clone(),
        uses,
        attrs,
        receiver_policy,
        receiver,
        formals: formals.into_boxed_slice(),
        source_logical_arity,
        physical_callable_lane_count,
    })
}
