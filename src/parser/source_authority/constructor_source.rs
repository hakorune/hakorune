//! Parser-owned source inventory for instance-Box constructors.
//!
//! Constructor AST nodes remain in the legacy `HashMap` carrier.  This file
//! owns the source identity that the map cannot express: written member/gate
//! sites, declaration order, and generated-birth initializer provenance.

use std::collections::{HashMap, HashSet};

use crate::ast::ASTNode;

use super::SourceAuthorityErrorV1;
use super::{OpenBoxMethodSourceTransactionV1, SourceBoxMethodSiteV1};
use crate::parser::ParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum GeneratedBirthTriggerKindV1 {
    BirthOnceProperty,
    StoredFieldInitializer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::parser) struct GeneratedBirthTriggerSourceV1 {
    source_site: SourceBoxMethodSiteV1,
    kind: GeneratedBirthTriggerKindV1,
}

impl GeneratedBirthTriggerSourceV1 {
    pub(in crate::parser) fn source_site(&self) -> &SourceBoxMethodSiteV1 {
        &self.source_site
    }

    pub(in crate::parser) const fn kind(&self) -> GeneratedBirthTriggerKindV1 {
        self.kind
    }

    pub(super) fn prepend_selected_gate(
        &mut self,
        gate_member_ordinal: u32,
        branch_member_ordinal: u32,
    ) {
        self.source_site
            .prepend_selected_gate(gate_member_ordinal, branch_member_ordinal);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::parser) enum ConstructorSourceOriginV1 {
    Direct(SourceBoxMethodSiteV1),
    GeneratedBirthInitializer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::parser) struct ConstructorSourceRelationV1 {
    key: Box<str>,
    origin: ConstructorSourceOriginV1,
    initializer_triggers: Box<[GeneratedBirthTriggerSourceV1]>,
}

impl ConstructorSourceRelationV1 {
    pub(in crate::parser) fn key(&self) -> &str {
        &self.key
    }

    pub(in crate::parser) fn origin(&self) -> &ConstructorSourceOriginV1 {
        &self.origin
    }

    pub(in crate::parser) fn initializer_triggers(&self) -> &[GeneratedBirthTriggerSourceV1] {
        &self.initializer_triggers
    }

    pub(super) fn source_member_ordinal(&self) -> Option<u32> {
        match &self.origin {
            ConstructorSourceOriginV1::Direct(site) => Some(site.source_member_ordinal()),
            ConstructorSourceOriginV1::GeneratedBirthInitializer => None,
        }
    }

    pub(super) fn prepend_selected_gate(
        &mut self,
        gate_member_ordinal: u32,
        branch_member_ordinal: u32,
    ) {
        if let ConstructorSourceOriginV1::Direct(site) = &mut self.origin {
            site.prepend_selected_gate(gate_member_ordinal, branch_member_ordinal);
        }
        for trigger in &mut self.initializer_triggers {
            trigger.prepend_selected_gate(gate_member_ordinal, branch_member_ordinal);
        }
    }
}

pub(super) fn canonical_constructor_key(node: &ASTNode) -> Option<String> {
    let ASTNode::FunctionDeclaration { name, params, .. } = node else {
        return None;
    };
    if !matches!(name.as_str(), "init" | "pack" | "birth") {
        return None;
    }
    Some(format!("{name}/{}", params.len()))
}

pub(in crate::parser) fn validate_constructor_rows(
    rows: &[ConstructorSourceRelationV1],
    constructors: &HashMap<String, ASTNode>,
) -> Result<(), SourceAuthorityErrorV1> {
    if rows.len() != constructors.len() {
        return Err(SourceAuthorityErrorV1::ConstructorCoverageMismatch(
            rows.len(),
            constructors.len(),
        ));
    }
    let mut keys = HashSet::with_capacity(rows.len());
    for row in rows {
        if !keys.insert(row.key.clone()) {
            return Err(SourceAuthorityErrorV1::DuplicateConstructorKey(
                row.key.clone(),
            ));
        }
        let node = constructors
            .get(row.key.as_ref())
            .ok_or_else(|| SourceAuthorityErrorV1::ConstructorMissing(row.key.clone()))?;
        let actual = canonical_constructor_key(node)
            .ok_or_else(|| SourceAuthorityErrorV1::ConstructorShapeMismatch(row.key.clone()))?;
        if actual != row.key.as_ref() {
            return Err(SourceAuthorityErrorV1::ConstructorShapeMismatch(
                row.key.clone(),
            ));
        }
    }
    Ok(())
}

impl OpenBoxMethodSourceTransactionV1 {
    pub(in crate::parser) fn commit_constructor_at_current(
        &mut self,
        key: &str,
        declaration: &ASTNode,
    ) -> Result<(), ParseError> {
        let actual =
            canonical_constructor_key(declaration).ok_or_else(|| ParseError::BuildCfg {
                message: "constructor source row is not a supported function declaration"
                    .to_owned(),
                line: 0,
            })?;
        if actual != key {
            return Err(ParseError::BuildCfg {
                message: format!(
                    "constructor source key `{key}` does not match declaration `{actual}`"
                ),
                line: 0,
            });
        }
        if self
            .constructor_relations
            .iter()
            .any(|row| row.key() == key)
        {
            return Err(ParseError::BuildCfg {
                message: format!("duplicate constructor source key `{key}`"),
                line: 0,
            });
        }
        self.constructor_relations
            .push(ConstructorSourceRelationV1 {
                key: key.to_owned().into_boxed_str(),
                origin: ConstructorSourceOriginV1::Direct(SourceBoxMethodSiteV1::Direct {
                    member: self.current_member_site(),
                }),
                initializer_triggers: Box::new([]),
            });
        Ok(())
    }

    pub(in crate::parser) fn record_generated_birth_trigger_at_current(
        &mut self,
        kind: GeneratedBirthTriggerKindV1,
    ) {
        self.generated_birth_triggers
            .push(GeneratedBirthTriggerSourceV1 {
                source_site: SourceBoxMethodSiteV1::Direct {
                    member: self.current_member_site(),
                },
                kind,
            });
    }

    pub(super) fn seal_constructor_inventory(
        &mut self,
        constructors: &HashMap<String, ASTNode>,
    ) -> Result<(), ParseError> {
        if !self.generated_birth_triggers.is_empty()
            && !self
                .constructor_relations
                .iter()
                .any(|row| row.key() == "birth/0")
        {
            if !constructors.contains_key("birth/0") {
                return Err(ParseError::BuildCfg {
                    message: "generated birth initializer has no birth/0 constructor".to_owned(),
                    line: 0,
                });
            }
            self.constructor_relations
                .push(ConstructorSourceRelationV1 {
                    key: "birth/0".into(),
                    origin: ConstructorSourceOriginV1::GeneratedBirthInitializer,
                    initializer_triggers: self.generated_birth_triggers.clone().into_boxed_slice(),
                });
        } else if !self.generated_birth_triggers.is_empty() {
            for row in self
                .constructor_relations
                .iter_mut()
                .filter(|row| row.key() == "birth/0")
            {
                row.initializer_triggers = self.generated_birth_triggers.clone().into_boxed_slice();
            }
        }
        validate_constructor_rows(&self.constructor_relations, constructors)
            .map_err(super::source_authority_to_parse_error)
    }
}
