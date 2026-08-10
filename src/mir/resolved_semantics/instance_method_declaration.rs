//! Resolver-owned declaration/signature issuer for bounded Box instance methods.
//!
//! This module consumes the parser handoff exactly once and issues semantic
//! declaration facts. It deliberately has no Home, Query-behavior, target,
//! Recipe, MIR, or physical-ABI authority.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::parser::{
    CallableContractSyntaxV1, ParserBoxResolverSourceHandoffV1, ResolverBoxMethodSourceRowV1,
    ResolverBoxMethodSourceSiteV1, ResolverSourceInvocationProvenanceV1,
};

static NEXT_RESOLVER_CATALOG_BRAND: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ResolverCatalogBrandV1(u64);

impl ResolverCatalogBrandV1 {
    fn issue() -> Result<Self, ResolverNominalTypeEnvironmentIssueV1> {
        let value = NEXT_RESOLVER_CATALOG_BRAND
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                current.checked_add(1)
            })
            .map_err(|_| ResolverNominalTypeEnvironmentIssueV1::BrandExhausted)?;
        Ok(Self(value))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ResolverNominalBoxTypeIdV1 {
    brand: ResolverCatalogBrandV1,
    slot: u32,
}

impl ResolverNominalBoxTypeIdV1 {
    pub(crate) const fn brand(self) -> ResolverCatalogBrandV1 {
        self.brand
    }

    pub(crate) const fn slot(self) -> u32 {
        self.slot
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolverNominalBoxDeclarationInputV1 {
    source_statement_ordinal: u32,
    source_name: Box<str>,
}

impl ResolverNominalBoxDeclarationInputV1 {
    pub(crate) fn new(source_statement_ordinal: u32, source_name: impl Into<Box<str>>) -> Self {
        Self {
            source_statement_ordinal,
            source_name: source_name.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ResolverNominalTypeEnvironmentIssueV1 {
    BrandExhausted,
    DuplicateSourceStatement { statement_ordinal: u32 },
    DuplicateSourceName { name: Box<str> },
}

#[derive(Debug)]
pub(crate) struct ResolverNominalTypeEnvironmentV1 {
    brand: ResolverCatalogBrandV1,
    declarations: BTreeMap<u32, ResolverNominalBoxDeclarationV1>,
}

#[derive(Debug)]
struct ResolverNominalBoxDeclarationV1 {
    source_name: Box<str>,
    type_id: ResolverNominalBoxTypeIdV1,
}

impl ResolverNominalTypeEnvironmentV1 {
    pub(crate) fn issue(
        inputs: impl IntoIterator<Item = ResolverNominalBoxDeclarationInputV1>,
    ) -> Result<Self, ResolverNominalTypeEnvironmentIssueV1> {
        let brand = ResolverCatalogBrandV1::issue()?;
        let mut declarations = BTreeMap::new();
        let mut names = BTreeMap::new();
        for (slot, input) in inputs.into_iter().enumerate() {
            if declarations.contains_key(&input.source_statement_ordinal) {
                return Err(
                    ResolverNominalTypeEnvironmentIssueV1::DuplicateSourceStatement {
                        statement_ordinal: input.source_statement_ordinal,
                    },
                );
            }
            if names
                .insert(input.source_name.clone(), input.source_statement_ordinal)
                .is_some()
            {
                return Err(ResolverNominalTypeEnvironmentIssueV1::DuplicateSourceName {
                    name: input.source_name,
                });
            }
            let slot = u32::try_from(slot)
                .map_err(|_| ResolverNominalTypeEnvironmentIssueV1::BrandExhausted)?;
            let type_id = ResolverNominalBoxTypeIdV1 { brand, slot };
            declarations.insert(
                input.source_statement_ordinal,
                ResolverNominalBoxDeclarationV1 {
                    source_name: input.source_name,
                    type_id,
                },
            );
        }
        Ok(Self {
            brand,
            declarations,
        })
    }

    fn brand(&self) -> ResolverCatalogBrandV1 {
        self.brand
    }

    fn declaration(
        &self,
        statement_ordinal: u32,
        source_name: &str,
    ) -> Result<&ResolverNominalBoxDeclarationV1, InstanceMethodDeclarationIssueV1> {
        let declaration = self.declarations.get(&statement_ordinal).ok_or(
            InstanceMethodDeclarationIssueV1::NominalBoxUnavailable {
                statement_ordinal,
                name: source_name.to_owned().into_boxed_str(),
            },
        )?;
        if declaration.source_name.as_ref() != source_name {
            return Err(InstanceMethodDeclarationIssueV1::NominalBoxSourceMismatch {
                statement_ordinal,
                expected: declaration.source_name.clone(),
                actual: source_name.to_owned().into_boxed_str(),
            });
        }
        Ok(declaration)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResolverSemanticValueTypeV1 {
    I64,
    Unit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedSemanticCallableSignatureV1 {
    parameters: Box<[ResolverSemanticValueTypeV1]>,
    result: ResolverSemanticValueTypeV1,
}

impl VerifiedSemanticCallableSignatureV1 {
    pub(crate) fn parameters(&self) -> &[ResolverSemanticValueTypeV1] {
        &self.parameters
    }

    pub(crate) const fn result(&self) -> ResolverSemanticValueTypeV1 {
        self.result
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedInstanceMethodDeclarationCatalogV1 {
    resolver_brand: ResolverCatalogBrandV1,
    parser_provenance: ResolverSourceInvocationProvenanceV1,
    declarations: Box<[VerifiedInstanceMethodDeclarationV1]>,
}

#[derive(Debug)]
pub(crate) struct VerifiedInstanceMethodDeclarationV1 {
    resolver_brand: ResolverCatalogBrandV1,
    nominal_box_type: ResolverNominalBoxTypeIdV1,
    box_statement_ordinal: u32,
    method_member_ordinal: u32,
    name: Box<str>,
    signature: VerifiedSemanticCallableSignatureV1,
    callable_contract: Option<CallableContractSyntaxV1>,
}

impl VerifiedInstanceMethodDeclarationCatalogV1 {
    pub(crate) fn declarations(&self) -> &[VerifiedInstanceMethodDeclarationV1] {
        &self.declarations
    }

    pub(crate) const fn resolver_brand(&self) -> ResolverCatalogBrandV1 {
        self.resolver_brand
    }

    pub(crate) fn parser_provenance(&self) -> &ResolverSourceInvocationProvenanceV1 {
        &self.parser_provenance
    }

    pub(crate) fn declaration_at_source_site(
        &self,
        source_site: ResolverBoxMethodSourceSiteV1,
    ) -> Option<&VerifiedInstanceMethodDeclarationV1> {
        self.declarations.iter().find(|declaration| {
            declaration.box_statement_ordinal == source_site.box_statement_ordinal()
                && declaration.method_member_ordinal == source_site.member_ordinal()
        })
    }
}

impl VerifiedInstanceMethodDeclarationV1 {
    pub(crate) const fn resolver_brand(&self) -> ResolverCatalogBrandV1 {
        self.resolver_brand
    }

    pub(crate) const fn nominal_box_type(&self) -> ResolverNominalBoxTypeIdV1 {
        self.nominal_box_type
    }

    pub(crate) const fn box_statement_ordinal(&self) -> u32 {
        self.box_statement_ordinal
    }

    pub(crate) const fn method_member_ordinal(&self) -> u32 {
        self.method_member_ordinal
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn signature(&self) -> &VerifiedSemanticCallableSignatureV1 {
        &self.signature
    }

    pub(crate) fn callable_contract(&self) -> Option<&CallableContractSyntaxV1> {
        self.callable_contract.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum InstanceMethodDeclarationIssueV1 {
    HandoffRowsEmpty,
    NominalBoxUnavailable {
        statement_ordinal: u32,
        name: Box<str>,
    },
    NominalBoxSourceMismatch {
        statement_ordinal: u32,
        expected: Box<str>,
        actual: Box<str>,
    },
    UnusedNominalBox {
        statement_ordinal: u32,
    },
    StaticMethodUnsupported {
        name: Box<str>,
    },
    MissingParameterType {
        method: Box<str>,
        index: usize,
    },
    UnsupportedType {
        method: Box<str>,
        type_name: Box<str>,
    },
    DuplicateSourceBoxRow {
        statement_ordinal: u32,
    },
    MethodSourceBoxMismatch {
        box_statement_ordinal: u32,
        method_statement_ordinal: u32,
    },
    DuplicateSourceMethodSite {
        statement_ordinal: u32,
        member_ordinal: u32,
    },
}

pub(crate) struct SemanticInstanceDeclarationIssuerV1;

impl SemanticInstanceDeclarationIssuerV1 {
    pub(crate) fn issue(
        handoff: ParserBoxResolverSourceHandoffV1,
        environment: ResolverNominalTypeEnvironmentV1,
    ) -> Result<VerifiedInstanceMethodDeclarationCatalogV1, InstanceMethodDeclarationIssueV1> {
        let (parser_provenance, boxes) = handoff.into_parts();
        if boxes.is_empty() {
            return Err(InstanceMethodDeclarationIssueV1::HandoffRowsEmpty);
        }

        let resolver_brand = environment.brand();
        let mut used_statements = BTreeMap::new();
        let mut used_method_sites = BTreeSet::new();
        let mut declarations = Vec::new();
        for box_row in boxes.iter() {
            let statement_ordinal = box_row.statement_ordinal();
            let nominal = environment.declaration(statement_ordinal, box_row.name())?;
            if used_statements
                .insert(statement_ordinal, box_row.name().to_owned())
                .is_some()
            {
                return Err(InstanceMethodDeclarationIssueV1::DuplicateSourceBoxRow {
                    statement_ordinal,
                });
            }
            for method in box_row.methods() {
                let source_site = method.source_site();
                if source_site.box_statement_ordinal() != statement_ordinal {
                    return Err(InstanceMethodDeclarationIssueV1::MethodSourceBoxMismatch {
                        box_statement_ordinal: statement_ordinal,
                        method_statement_ordinal: source_site.box_statement_ordinal(),
                    });
                }
                if !used_method_sites.insert((
                    source_site.box_statement_ordinal(),
                    source_site.member_ordinal(),
                )) {
                    return Err(
                        InstanceMethodDeclarationIssueV1::DuplicateSourceMethodSite {
                            statement_ordinal,
                            member_ordinal: source_site.member_ordinal(),
                        },
                    );
                }
                declarations.push(issue_method(resolver_brand, nominal.type_id, method)?);
            }
        }

        if environment
            .declarations
            .keys()
            .any(|statement| !used_statements.contains_key(statement))
        {
            let statement_ordinal = environment
                .declarations
                .keys()
                .find(|statement| !used_statements.contains_key(statement))
                .copied()
                .unwrap_or_default();
            return Err(InstanceMethodDeclarationIssueV1::UnusedNominalBox { statement_ordinal });
        }

        Ok(VerifiedInstanceMethodDeclarationCatalogV1 {
            resolver_brand,
            parser_provenance,
            declarations: declarations.into_boxed_slice(),
        })
    }
}

fn issue_method(
    resolver_brand: ResolverCatalogBrandV1,
    nominal_box_type: ResolverNominalBoxTypeIdV1,
    method: &ResolverBoxMethodSourceRowV1,
) -> Result<VerifiedInstanceMethodDeclarationV1, InstanceMethodDeclarationIssueV1> {
    if method.signature().is_static() {
        return Err(InstanceMethodDeclarationIssueV1::StaticMethodUnsupported {
            name: method.name().to_owned().into_boxed_str(),
        });
    }
    let signature = issue_signature(method)?;
    Ok(VerifiedInstanceMethodDeclarationV1 {
        resolver_brand,
        nominal_box_type,
        box_statement_ordinal: method.source_site().box_statement_ordinal(),
        method_member_ordinal: method.source_site().member_ordinal(),
        name: method.name().to_owned().into_boxed_str(),
        signature,
        callable_contract: method.callable_contract().cloned(),
    })
}

fn issue_signature(
    method: &ResolverBoxMethodSourceRowV1,
) -> Result<VerifiedSemanticCallableSignatureV1, InstanceMethodDeclarationIssueV1> {
    let parameters = method
        .signature()
        .parameters()
        .iter()
        .enumerate()
        .map(|(index, parameter)| {
            let type_name = parameter.declared_type_name().ok_or_else(|| {
                InstanceMethodDeclarationIssueV1::MissingParameterType {
                    method: method.name().to_owned().into_boxed_str(),
                    index,
                }
            })?;
            resolve_type(method.name(), type_name)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let result = match method.signature().return_type_name() {
        None => ResolverSemanticValueTypeV1::Unit,
        Some(type_name) => resolve_type(method.name(), type_name)?,
    };
    Ok(VerifiedSemanticCallableSignatureV1 {
        parameters: parameters.into_boxed_slice(),
        result,
    })
}

fn resolve_type(
    method: &str,
    type_name: &str,
) -> Result<ResolverSemanticValueTypeV1, InstanceMethodDeclarationIssueV1> {
    match type_name {
        "i64" => Ok(ResolverSemanticValueTypeV1::I64),
        other => Err(InstanceMethodDeclarationIssueV1::UnsupportedType {
            method: method.to_owned().into_boxed_str(),
            type_name: other.to_owned().into_boxed_str(),
        }),
    }
}

#[cfg(test)]
#[path = "instance_method_declaration_tests.rs"]
mod tests;
