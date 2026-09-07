//! Parser-owned generated origin; an implicit derive has a Box parent, not an
//! invented as-written method ordinal. Parameters come from its emitted declaration.
use super::source_authority::SourceBoxDeclarationSiteV1;
use super::source_path::SourceProgramDeclarationPathV1;
use crate::ast::{ASTNode, BoxMethodGeneratedProvenanceV1, BoxMethodInventoryOrdinalV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum DefaultDeriveKindV1 {
    Equals,
    ToString,
}

impl DefaultDeriveKindV1 {
    pub(in crate::parser) fn name(self) -> &'static str {
        match self {
            Self::Equals => "equals",
            Self::ToString => "toString",
        }
    }
    fn parameter_count(self) -> usize {
        match self {
            Self::Equals => 1,
            Self::ToString => 0,
        }
    }
    pub(in crate::parser) fn provenance(self) -> BoxMethodGeneratedProvenanceV1 {
        BoxMethodGeneratedProvenanceV1::MacroOrImport {
            generator: match self {
                Self::Equals => "macro-derive-equals",
                Self::ToString => "macro-derive-to-string",
            }
            .into(),
        }
    }
    pub(in crate::parser) fn build(self, name: &str, fields: &Vec<String>) -> ASTNode {
        match self {
            Self::Equals => crate::r#macro::default_derive::build_equals_method(name, fields),
            Self::ToString => crate::r#macro::default_derive::build_tostring_method(name, fields),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::parser) struct GeneratedDefaultDeriveOriginV1 {
    declaration: SourceProgramDeclarationPathV1,
    parent: SourceBoxDeclarationSiteV1,
    kind: DefaultDeriveKindV1,
    placement: BoxMethodInventoryOrdinalV1,
    parameters: Box<[Box<str>]>,
}
impl GeneratedDefaultDeriveOriginV1 {
    pub(in crate::parser) fn issue(
        parent: SourceBoxDeclarationSiteV1,
        kind: DefaultDeriveKindV1,
        placement: BoxMethodInventoryOrdinalV1,
        syntax: &ASTNode,
    ) -> Result<Self, String> {
        let ASTNode::FunctionDeclaration {
            name,
            params,
            param_decls,
            is_static,
            return_type_name,
            ..
        } = syntax
        else {
            return Err("generated declaration missing".into());
        };
        if name != kind.name()
            || *is_static
            || return_type_name.is_some()
            || params.len() != kind.parameter_count()
            || params.len() != param_decls.len()
            || params
                .iter()
                .zip(param_decls)
                .any(|(name, decl)| name != &decl.name || decl.declared_type_name.is_some())
        {
            return Err("generated declaration/parameter mismatch".into());
        }
        Ok(Self {
            declaration: SourceProgramDeclarationPathV1::from_parser_path(parent.path().clone()),
            parent,
            kind,
            placement,
            parameters: params.iter().map(|s| s.clone().into_boxed_str()).collect(),
        })
    }
    pub(in crate::parser) fn declaration(&self) -> &SourceProgramDeclarationPathV1 {
        &self.declaration
    }
    pub(in crate::parser) fn parent(&self) -> &SourceBoxDeclarationSiteV1 {
        &self.parent
    }
    pub(in crate::parser) fn placement(&self) -> BoxMethodInventoryOrdinalV1 {
        self.placement
    }
    pub(in crate::parser) fn kind(&self) -> DefaultDeriveKindV1 {
        self.kind
    }
    pub(in crate::parser) fn parameters(&self) -> &[Box<str>] {
        &self.parameters
    }
    pub(in crate::parser) fn validates(&self, syntax: &ASTNode) -> bool {
        let ASTNode::FunctionDeclaration {
            name,
            params,
            param_decls,
            is_static,
            return_type_name,
            ..
        } = syntax
        else {
            return false;
        };
        name == self.kind.name()
            && !is_static
            && return_type_name.is_none()
            && params.len() == self.parameters.len()
            && param_decls.len() == self.parameters.len()
            && params
                .iter()
                .zip(param_decls)
                .zip(self.parameters.iter())
                .all(|((name, decl), source)| {
                    name.as_str() == source.as_ref()
                        && decl.name.as_str() == source.as_ref()
                        && decl.declared_type_name.is_none()
                })
    }
}

#[cfg(test)]
#[path = "default_derive_source_tests.rs"]
mod tests;
