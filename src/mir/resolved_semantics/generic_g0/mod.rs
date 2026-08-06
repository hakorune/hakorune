//! AST-free source-type inventory for Generic G0 S0B.
//!
//! The compiler projector owns syntax navigation. This module owns the one
//! issuer that seals parameter/result/literal source facts and nothing beyond
//! them. Numeric representation, policy, portable lowering, and MIR remain
//! elsewhere.

use std::collections::BTreeSet;

use super::{
    BindingKindV1, BindingOriginV1, BindingRefV1, FunctionOriginV1, OwnedExprSiteV1,
    OwnedHeaderSiteV1, SemanticOwnerSourceKindV1, SourceBindingSiteV1, SourceHeaderSiteV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum GenericG0LiteralRoleV1 {
    OuterConditionRhs,
    InnerConditionRhs,
    OuterUpdateRhs,
    InnerUpdateRhs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum GenericG0LiteralKindV1 {
    NonLiteral,
    String,
    Float,
    Bool,
    Null,
    Void,
}

/// As-written literal syntax. No semantic numeric classification is stored.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GenericG0LiteralSyntaxV1 {
    PlainInteger(i64),
    TypedInteger {
        value: i64,
        declared_type_name: Box<str>,
    },
    Other(GenericG0LiteralKindV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GenericG0ParameterTypeRowV1 {
    pub(crate) index: u32,
    pub(crate) header: OwnedHeaderSiteV1,
    pub(crate) binding: BindingRefV1,
    pub(crate) binding_kind: BindingKindV1,
    pub(crate) binding_origin: BindingOriginV1,
    pub(crate) declared_type_name: Option<Box<str>>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GenericG0ResultTypeRowV1 {
    pub(crate) header: OwnedHeaderSiteV1,
    pub(crate) declared_type_name: Option<Box<str>>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GenericG0LiteralTypeRowV1 {
    pub(crate) role: GenericG0LiteralRoleV1,
    pub(crate) site: OwnedExprSiteV1,
    pub(crate) context: OwnedExprSiteV1,
    pub(crate) binding: BindingRefV1,
    pub(crate) syntax: GenericG0LiteralSyntaxV1,
}

/// AST-free observation created by the one compiler-side projector.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GenericG0SourceTypeObservationV1 {
    pub(crate) owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    pub(crate) origin: FunctionOriginV1,
    pub(crate) source_kind: SemanticOwnerSourceKindV1,
    pub(crate) parameters: Box<[GenericG0ParameterTypeRowV1]>,
    pub(crate) result: GenericG0ResultTypeRowV1,
    pub(crate) literals: Box<[GenericG0LiteralTypeRowV1]>,
}

/// Move-only S0B inventory. It is consumed by S0C and cannot be rebuilt from
/// names or source paths after this boundary.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericSourceTypeInventoryG0 {
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    parameters: Box<[GenericG0ParameterTypeRowV1]>,
    result: GenericG0ResultTypeRowV1,
    literals: Box<[GenericG0LiteralTypeRowV1]>,
}

impl VerifiedGenericSourceTypeInventoryG0 {
    pub(crate) const fn owner(&self) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn origin(&self) -> FunctionOriginV1 {
        self.origin
    }

    pub(crate) const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.source_kind
    }

    pub(crate) fn parameters(&self) -> &[GenericG0ParameterTypeRowV1] {
        &self.parameters
    }

    pub(crate) fn result(&self) -> &GenericG0ResultTypeRowV1 {
        &self.result
    }

    pub(crate) fn literals(&self) -> &[GenericG0LiteralTypeRowV1] {
        &self.literals
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GenericG0SourceTypeIssueV1 {
    Unresolved(GenericG0SourceTypeUnresolvedV1),
    Rejected(GenericG0SourceTypeRejectV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GenericG0SourceTypeUnresolvedV1 {
    MissingParameterAnnotation { index: u32 },
    MissingReturnAnnotation,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GenericG0SourceTypeRejectV1 {
    WrongSourceKind,
    ParameterCardinality,
    ParameterHeaderSite {
        index: u32,
    },
    ParameterBinding {
        index: u32,
    },
    ParameterNotI64 {
        index: u32,
        declared_type_name: Box<str>,
    },
    ReturnHeaderSite,
    ReturnNotI64 {
        declared_type_name: Box<str>,
    },
    ForeignOwner,
    LiteralCardinality,
    DuplicateLiteralRole,
    LiteralSiteOwner,
    LiteralContextOwner,
    NonIntegerLiteral {
        role: GenericG0LiteralRoleV1,
    },
}

pub(crate) fn issue_generic_g0_source_type_inventory_v1(
    observation: GenericG0SourceTypeObservationV1,
) -> Result<VerifiedGenericSourceTypeInventoryG0, GenericG0SourceTypeIssueV1> {
    if observation.source_kind != SemanticOwnerSourceKindV1::DeclaredFunction {
        return Err(GenericG0SourceTypeIssueV1::Rejected(
            GenericG0SourceTypeRejectV1::WrongSourceKind,
        ));
    }
    if observation.parameters.len() != 2 {
        return Err(GenericG0SourceTypeIssueV1::Rejected(
            GenericG0SourceTypeRejectV1::ParameterCardinality,
        ));
    }
    for (expected, row) in observation.parameters.iter().enumerate() {
        let index = expected as u32;
        if row.index != index
            || row.header.owner() != observation.owner
            || row.header.site() != (SourceHeaderSiteV1::Parameter { index })
        {
            return Err(GenericG0SourceTypeIssueV1::Rejected(
                GenericG0SourceTypeRejectV1::ParameterHeaderSite { index },
            ));
        }
        if row.binding.owner() != observation.owner
            || !binding_origin_is_parameter(&row.binding_origin, row.binding_kind, row.index)
        {
            return Err(GenericG0SourceTypeIssueV1::Rejected(
                GenericG0SourceTypeRejectV1::ParameterBinding { index },
            ));
        }
        let Some(type_name) = row.declared_type_name.as_deref() else {
            return Err(GenericG0SourceTypeIssueV1::Unresolved(
                GenericG0SourceTypeUnresolvedV1::MissingParameterAnnotation { index },
            ));
        };
        if type_name != "i64" {
            return Err(GenericG0SourceTypeIssueV1::Rejected(
                GenericG0SourceTypeRejectV1::ParameterNotI64 {
                    index,
                    declared_type_name: type_name.into(),
                },
            ));
        }
    }

    if observation.result.header.owner() != observation.owner
        || observation.result.header.site() != SourceHeaderSiteV1::ReturnAnnotation
    {
        return Err(GenericG0SourceTypeIssueV1::Rejected(
            GenericG0SourceTypeRejectV1::ReturnHeaderSite,
        ));
    }
    let Some(result_type) = observation.result.declared_type_name.as_deref() else {
        return Err(GenericG0SourceTypeIssueV1::Unresolved(
            GenericG0SourceTypeUnresolvedV1::MissingReturnAnnotation,
        ));
    };
    if result_type != "i64" {
        return Err(GenericG0SourceTypeIssueV1::Rejected(
            GenericG0SourceTypeRejectV1::ReturnNotI64 {
                declared_type_name: result_type.into(),
            },
        ));
    }

    if observation.literals.len() != 4 {
        return Err(GenericG0SourceTypeIssueV1::Rejected(
            GenericG0SourceTypeRejectV1::LiteralCardinality,
        ));
    }
    let mut roles = BTreeSet::new();
    for row in &observation.literals {
        if !roles.insert(row.role) {
            return Err(GenericG0SourceTypeIssueV1::Rejected(
                GenericG0SourceTypeRejectV1::DuplicateLiteralRole,
            ));
        }
        if row.site.owner() != observation.owner || row.binding.owner() != observation.owner {
            return Err(GenericG0SourceTypeIssueV1::Rejected(
                GenericG0SourceTypeRejectV1::LiteralSiteOwner,
            ));
        }
        if row.context.owner() != observation.owner {
            return Err(GenericG0SourceTypeIssueV1::Rejected(
                GenericG0SourceTypeRejectV1::LiteralContextOwner,
            ));
        }
        if matches!(&row.syntax, GenericG0LiteralSyntaxV1::Other(_)) {
            return Err(GenericG0SourceTypeIssueV1::Rejected(
                GenericG0SourceTypeRejectV1::NonIntegerLiteral { role: row.role },
            ));
        }
    }

    Ok(VerifiedGenericSourceTypeInventoryG0 {
        owner: observation.owner,
        origin: observation.origin,
        source_kind: observation.source_kind,
        parameters: observation.parameters,
        result: observation.result,
        literals: observation.literals,
    })
}

/// The projector copies this resolver record into the AST-free observation;
/// the issuer checks it without opening the resolver arena a second time.
pub(crate) fn binding_origin_is_parameter(
    origin: &BindingOriginV1,
    kind: BindingKindV1,
    index: u32,
) -> bool {
    kind == BindingKindV1::Parameter { index }
        && matches!(origin, BindingOriginV1::Source(SourceBindingSiteV1::Parameter { index: i }) if *i == index)
}
