//! Append generated declarations inside the existing source transaction.
use super::model::OpenParserPostpassProductV1;
use crate::ast::{ASTNode, Span};
use crate::parser::callable_source_anchor::{
    GeneratedCallableOriginV1, PreparedCallableSourceV1, PreparedGeneratedCallableSourceV1,
};
use crate::parser::default_derive_source::{DefaultDeriveKindV1, GeneratedDefaultDeriveOriginV1};
use crate::parser::ParseError;
use crate::r#macro::NormalMacroPolicyV1;

impl OpenParserPostpassProductV1 {
    pub(super) fn issue_default_derives(
        mut self,
        policy: Option<&NormalMacroPolicyV1>,
    ) -> Result<Self, ParseError> {
        let Some(policy) = policy.filter(|p| p.enabled()) else {
            return Ok(self);
        };
        let ASTNode::Program { statements, .. } = &mut self.ast else {
            return Err(reject("program-missing"));
        };
        let mut ordinary_index = 0;
        for node in statements {
            let ASTNode::BoxDeclaration {
                name,
                methods,
                public_fields,
                is_static,
                is_interface,
                is_record,
                is_sync,
                constructors,
                ..
            } = node
            else {
                continue;
            };
            if *is_static || *is_interface || *is_record {
                continue;
            }
            let path = self
                .final_box_paths
                .get(ordinary_index)
                .ok_or_else(|| reject("box-path-missing"))?;
            ordinary_index += 1;
            let seal = self
                .source_session
                .prepared_source_seals
                .iter_mut()
                .find(|seal| seal.box_site().path() == path)
                .ok_or_else(|| reject("box-source-missing"))?;
            // Validate the old source/authorized delegate stage before extending
            // its expected inventory. Never bless an arbitrary final AST snapshot.
            seal.validate_against(name, *is_sync, methods, constructors)
                .map_err(super::finalize::map_error)?;
            let selection = policy.selection(false, methods);
            if !(selection.equals || selection.to_string) {
                continue;
            }
            if !public_fields.is_empty() {
                return Err(reject("dynamic-field-text-contract-missing"));
            }
            for kind in [DefaultDeriveKindV1::Equals, DefaultDeriveKindV1::ToString] {
                let selected = match kind {
                    DefaultDeriveKindV1::Equals => selection.equals,
                    DefaultDeriveKindV1::ToString => selection.to_string,
                };
                if !selected {
                    continue;
                }
                let declaration = kind.build(name, public_fields);
                let placement = methods
                    .try_push_generated(
                        kind.name(),
                        declaration.clone(),
                        kind.provenance(),
                        Span::unknown(),
                    )
                    .map_err(|_| reject("generated-placement"))?;
                let origin = GeneratedDefaultDeriveOriginV1::issue(
                    seal.box_site().clone(),
                    kind,
                    placement,
                    &declaration,
                )
                .map_err(|_| reject("generated-parameter-coverage"))?;
                self.source_session
                    .callable_rows
                    .push(PreparedCallableSourceV1::Generated(
                        PreparedGeneratedCallableSourceV1::issue(
                            seal.brand.clone(),
                            GeneratedCallableOriginV1::DefaultDerive(origin),
                            kind.name(),
                        ),
                    ));
            }
            // The same issuer just constructed every appended body. Subsequent
            // source finalization checks this exact inventory, including bodies.
            seal.inventory = methods.clone();
        }
        if ordinary_index != self.final_box_paths.len() {
            return Err(reject("box-coverage"));
        }
        Ok(self)
    }
}

fn reject(reason: &str) -> ParseError {
    ParseError::GrammarContract {
        stable_reject_tag: "parser/default-derive-source",
        detail: reason.into(),
        line: 0,
    }
}
