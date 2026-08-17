//! Source-backed declaration/header projection for Generic G0 TopLevel.
//!
//! This module owns no physical ABI, effect, Completion, or Builder state. It
//! only freezes the exact declaration facts needed by the later Generic
//! physical-entry cohort. The input source view is borrowed from the same
//! parent transaction that issues the Generic recipe product.

use crate::mir::resolved_semantics::{
    CallableHeaderSyntaxViewV1, FunctionOriginV1, FunctionOwnerIdV1,
    SemanticOwnerSourceKindV1,
};

use super::function_input::ResolvedFunctionLoweringInputV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericG0TopLevelDeclarationHeaderRejectV1 {
    SourceOwnerMismatch,
    SourceRootNotFunction,
    SourceKindMismatch,
    OverrideNotAllowed,
    ParameterCountMismatch,
    ParameterNameMismatch,
    MetadataNotEmpty,
    ParameterOrdinalOverflow,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GenericG0DeclarationParameterV1 {
    ordinal: u32,
    name: Box<str>,
    declared_type_name: Option<Box<str>>,
}

impl GenericG0DeclarationParameterV1 {
    pub(crate) const fn ordinal(&self) -> u32 {
        self.ordinal
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn declared_type_name(&self) -> Option<&str> {
        self.declared_type_name.as_deref()
    }
}

/// Mechanical source declaration facts retained by the Generic source parent.
/// It is intentionally non-Clone so a later physical cohort can only borrow
/// the exact row issued by the source-parent transaction.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericG0TopLevelDeclarationHeaderV1 {
    owner: FunctionOwnerIdV1,
    origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    name: Box<str>,
    parameters: Box<[GenericG0DeclarationParameterV1]>,
    return_type_name: Option<Box<str>>,
    is_static: bool,
    metadata_is_empty: bool,
}

impl VerifiedGenericG0TopLevelDeclarationHeaderV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn origin(&self) -> FunctionOriginV1 {
        self.origin
    }

    pub(crate) const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.source_kind
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn parameters(&self) -> &[GenericG0DeclarationParameterV1] {
        &self.parameters
    }

    pub(crate) fn return_type_name(&self) -> Option<&str> {
        self.return_type_name.as_deref()
    }

    pub(crate) const fn is_static(&self) -> bool {
        self.is_static
    }

    pub(crate) const fn metadata_is_empty(&self) -> bool {
        self.metadata_is_empty
    }
}

pub(crate) fn issue_generic_g0_top_level_declaration_header_v1(
    input: &ResolvedFunctionLoweringInputV1<'_>,
) -> Result<
    VerifiedGenericG0TopLevelDeclarationHeaderV1,
    GenericG0TopLevelDeclarationHeaderRejectV1,
> {
    let source = input.source();
    if source.owner() != input.owner() {
        return Err(GenericG0TopLevelDeclarationHeaderRejectV1::SourceOwnerMismatch);
    }
    if input.function().source_kind() != SemanticOwnerSourceKindV1::DeclaredFunction {
        return Err(GenericG0TopLevelDeclarationHeaderRejectV1::SourceKindMismatch);
    }
    let header = CallableHeaderSyntaxViewV1::from_function_ast(source.root())
        .ok_or(GenericG0TopLevelDeclarationHeaderRejectV1::SourceRootNotFunction)?;
    if header.is_override() {
        return Err(GenericG0TopLevelDeclarationHeaderRejectV1::OverrideNotAllowed);
    }
    if !header.metadata_is_empty() {
        return Err(GenericG0TopLevelDeclarationHeaderRejectV1::MetadataNotEmpty);
    }
    if header.params().len() != header.param_decls().len() {
        return Err(GenericG0TopLevelDeclarationHeaderRejectV1::ParameterCountMismatch);
    }

    let mut parameters = Vec::with_capacity(header.params().len());
    for (index, (name, declaration)) in header
        .params()
        .iter()
        .zip(header.param_decls().iter())
        .enumerate()
    {
        let ordinal = u32::try_from(index)
            .map_err(|_| GenericG0TopLevelDeclarationHeaderRejectV1::ParameterOrdinalOverflow)?;
        if name != &declaration.name {
            return Err(GenericG0TopLevelDeclarationHeaderRejectV1::ParameterNameMismatch);
        }
        parameters.push(GenericG0DeclarationParameterV1 {
            ordinal,
            name: name.as_str().into(),
            declared_type_name: declaration
                .declared_type_name
                .as_deref()
                .map(Into::into),
        });
    }

    Ok(VerifiedGenericG0TopLevelDeclarationHeaderV1 {
        owner: input.owner(),
        origin: input.function().function_origin(),
        source_kind: input.function().source_kind(),
        name: header.name().into(),
        parameters: parameters.into_boxed_slice(),
        return_type_name: header.return_type_name().map(Into::into),
        is_static: header.is_static(),
        metadata_is_empty: header.metadata_is_empty(),
    })
}
