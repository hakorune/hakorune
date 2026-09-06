//! Birth formal declarations co-sealed with their selected construction uses.
//!
//! This owner classifies only the exact constructor declaration loan.  It does
//! not inspect New arguments, MIR values, or C transport tags.

use crate::ast::{ASTNode, ParamDecl};
use crate::mir::exact_text_parameter_abi::ExactTextFormalAbiV1;
use crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1;
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingOriginV1, BindingRefV1, SourceBindingSiteV1,
};
use crate::mir::resolved_semantics::SourceExprSiteV1;

use super::super::instance_construction::{ConstructionEligibilityV1, ConstructionStoreRhsV1};
use super::VerifiedInstanceConstructorSemanticRowV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BirthFormalDeclarationClassV1 {
    Unannotated,
    ExactI64,
    ExactText,
    ExplicitUnsupported,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BirthFormalUseCoverageV1 {
    NoUse,
    I64FieldStores { sites: Box<[SourceExprSiteV1]> },
    UncoveredSelectedBody,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BirthFormalPhysicalDispositionV1 {
    DeferredActualBinding,
    UnavailableTaggedOrCheckedRepresentation,
    UnavailableUnsupportedDeclaration,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BirthFormalContractV1 {
    ordinal: u32,
    binding: BindingRefV1,
    declaration: BirthFormalDeclarationClassV1,
    uses: BirthFormalUseCoverageV1,
    disposition: BirthFormalPhysicalDispositionV1,
}

impl BirthFormalContractV1 {
    pub(crate) const fn ordinal(&self) -> u32 {
        self.ordinal
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) const fn declaration(&self) -> BirthFormalDeclarationClassV1 {
        self.declaration
    }

    pub(crate) fn uses(&self) -> &BirthFormalUseCoverageV1 {
        &self.uses
    }

    pub(crate) const fn disposition(&self) -> BirthFormalPhysicalDispositionV1 {
        self.disposition
    }
}

#[derive(Debug)]
pub(crate) enum BirthFormalContractIssueV1 {
    Declaration,
    Arity,
    MissingBinding(u32),
    ForeignBinding(u32),
    BindingMismatch(u32),
    DuplicateBinding,
}

pub(super) fn issue_birth_formal_contracts(
    declaration: &ASTNode,
    row: &VerifiedInstanceConstructorSemanticRowV1,
) -> Result<Box<[BirthFormalContractV1]>, BirthFormalContractIssueV1> {
    let ASTNode::FunctionDeclaration {
        params, param_decls, ..
    } = declaration
    else {
        return Err(BirthFormalContractIssueV1::Declaration);
    };
    let source_parameters = ParamDecl::with_name_fallback(param_decls, params);
    if source_parameters.len() != usize::try_from(row.source_arity()).map_err(|_| BirthFormalContractIssueV1::Arity)? {
        return Err(BirthFormalContractIssueV1::Arity);
    }
    let [owner] = row.forest().roots() else {
        return Err(BirthFormalContractIssueV1::Declaration);
    };
    let owner = row
        .forest()
        .semantic_owner(*owner)
        .ok_or(BirthFormalContractIssueV1::Declaration)?;
    let mut seen = std::collections::BTreeSet::new();
    source_parameters
        .iter()
        .enumerate()
        .map(|(index, parameter)| {
            let ordinal = u32::try_from(index).map_err(|_| BirthFormalContractIssueV1::Arity)?;
            let binding = owner
                .declaration_binding(&SourceBindingSiteV1::Parameter { index: ordinal })
                .ok_or(BirthFormalContractIssueV1::MissingBinding(ordinal))?;
            if binding.owner() != owner.owner() {
                return Err(BirthFormalContractIssueV1::ForeignBinding(ordinal));
            }
            let record = owner
                .binding(binding)
                .ok_or(BirthFormalContractIssueV1::ForeignBinding(ordinal))?;
            if record.kind() != (BindingKindV1::Parameter { index: ordinal })
                || record.origin()
                    != &BindingOriginV1::Source(SourceBindingSiteV1::Parameter { index: ordinal })
                || record.diagnostic_name() != parameter.name
            {
                return Err(BirthFormalContractIssueV1::BindingMismatch(ordinal));
            }
            if !seen.insert(binding) {
                return Err(BirthFormalContractIssueV1::DuplicateBinding);
            }
            let declaration = classify_declaration(parameter);
            let uses = classify_uses(row.construction(), binding);
            Ok(BirthFormalContractV1 {
                ordinal,
                binding,
                declaration,
                disposition: disposition(declaration, &uses),
                uses,
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .map(Vec::into_boxed_slice)
}

fn classify_declaration(parameter: &ParamDecl) -> BirthFormalDeclarationClassV1 {
    match parameter.declared_type_name.as_deref() {
        None => BirthFormalDeclarationClassV1::Unannotated,
        Some(source) if ExactTrivialParameterAbiV1::classify(source).is_some() => {
            BirthFormalDeclarationClassV1::ExactI64
        }
        Some(source) if ExactTextFormalAbiV1::classify(source).is_some() => {
            BirthFormalDeclarationClassV1::ExactText
        }
        Some(_) => BirthFormalDeclarationClassV1::ExplicitUnsupported,
    }
}

fn classify_uses(
    construction: &ConstructionEligibilityV1,
    binding: BindingRefV1,
) -> BirthFormalUseCoverageV1 {
    let Ok(plan) = construction else {
        return BirthFormalUseCoverageV1::UncoveredSelectedBody;
    };
    let sites = plan
        .stores()
        .iter()
        .filter_map(|store| match store.rhs() {
            ConstructionStoreRhsV1::Parameter { site, binding: rhs } if *rhs == binding => {
                Some(site.clone())
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    if sites.is_empty() {
        BirthFormalUseCoverageV1::NoUse
    } else {
        BirthFormalUseCoverageV1::I64FieldStores {
            sites: sites.into_boxed_slice(),
        }
    }
}

fn disposition(
    declaration: BirthFormalDeclarationClassV1,
    uses: &BirthFormalUseCoverageV1,
) -> BirthFormalPhysicalDispositionV1 {
    match declaration {
        BirthFormalDeclarationClassV1::ExplicitUnsupported => {
            BirthFormalPhysicalDispositionV1::UnavailableUnsupportedDeclaration
        }
        BirthFormalDeclarationClassV1::ExactText
        | BirthFormalDeclarationClassV1::Unannotated
            if !matches!(uses, BirthFormalUseCoverageV1::NoUse) => {
            BirthFormalPhysicalDispositionV1::UnavailableTaggedOrCheckedRepresentation
        }
        BirthFormalDeclarationClassV1::ExactI64 | BirthFormalDeclarationClassV1::Unannotated => {
            BirthFormalPhysicalDispositionV1::DeferredActualBinding
        }
        BirthFormalDeclarationClassV1::ExactText => {
            BirthFormalPhysicalDispositionV1::UnavailableTaggedOrCheckedRepresentation
        }
    }
}
