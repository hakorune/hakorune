use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::core_method_result_kind::{
    lookup_core_method_result_row_v2, CoreMethodEffectV1, CoreMethodResultKindV1,
};
use std::collections::BTreeSet;

/// The narrow source relation selected by the StringBox migration D0.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Stage1StringBoxReceiverV1 {
    DirectStringLiteral,
    NewStringBox,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Stage1StringBoxArgumentKindV1 {
    DirectStringLiteral,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Stage1SourceSpanV1 {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) line: usize,
    pub(crate) column: usize,
}

impl From<Span> for Stage1SourceSpanV1 {
    fn from(span: Span) -> Self {
        Self {
            start: span.start,
            end: span.end,
            line: span.line,
            column: span.column,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Stage1StringBoxMethodContractV1 {
    pub(crate) selector: String,
    pub(crate) arity: usize,
    pub(crate) result_kind: &'static str,
    pub(crate) effect_kind: &'static str,
    pub(crate) canonical_selector: &'static str,
}

impl Stage1StringBoxMethodContractV1 {
    fn for_selector(selector: &str, arity: usize) -> Result<Self, String> {
        let row = lookup_core_method_result_row_v2("StringBox", selector, arity as u32)
            .ok_or_else(|| {
                format!(
                    "[stage1/stringbox-source-artifact/contract-missing] {} / {}",
                    selector, arity
                )
            })?;
        if row.result_kind != CoreMethodResultKindV1::I64Value
            || row.effect != CoreMethodEffectV1::PureRead
        {
            return Err(format!(
                "[stage1/stringbox-source-artifact/contract-invalid] {} / {}",
                selector, arity
            ));
        }
        Ok(Self {
            selector: selector.to_string(),
            arity,
            result_kind: row.result_kind.as_manifest_name(),
            effect_kind: row.effect.as_manifest_name(),
            canonical_selector: row.canonical,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Stage1StringBoxSourceRelationV1 {
    pub(crate) owner: &'static str,
    pub(crate) path: String,
    pub(crate) anchor: u32,
    pub(crate) span: Stage1SourceSpanV1,
    pub(crate) receiver: Stage1StringBoxReceiverV1,
    pub(crate) selector: String,
    pub(crate) arity: usize,
    pub(crate) constructor_arg_count: usize,
    pub(crate) constructor_argument_kinds: Vec<Stage1StringBoxArgumentKindV1>,
    pub(crate) field_initializer_count: usize,
    pub(crate) type_arguments: Vec<String>,
    pub(crate) contract: Stage1StringBoxMethodContractV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct Stage1StringBoxSourceProductV1 {
    pub(crate) relations: Vec<Stage1StringBoxSourceRelationV1>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Stage1ProgramJsonCrosswalkEntryV1 {
    pub(crate) owner: &'static str,
    pub(crate) path: String,
    pub(crate) anchor: u32,
    pub(crate) span: Stage1SourceSpanV1,
    pub(crate) receiver: Stage1StringBoxReceiverV1,
    pub(crate) expected_box_name: &'static str,
    pub(crate) expected_method: String,
    pub(crate) expected_arity: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct Stage1ProgramJsonCrosswalkV1 {
    pub(crate) entries: Vec<Stage1ProgramJsonCrosswalkEntryV1>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Stage1ProgramJsonSourceArtifactV1 {
    pub(crate) program_json: String,
    pub(crate) product: Stage1StringBoxSourceProductV1,
    pub(crate) crosswalk: Stage1ProgramJsonCrosswalkV1,
}

#[derive(Debug, Default)]
pub(crate) struct Stage1StringBoxSourceCollector {
    relations: Vec<Stage1StringBoxSourceRelationV1>,
}

impl Stage1StringBoxSourceCollector {
    pub(crate) fn observe_method_call(
        &mut self,
        object: &ASTNode,
        method: &str,
        arguments: &[ASTNode],
    ) -> Result<Option<u32>, String> {
        if !matches!(method, "length" | "size") || !arguments.is_empty() {
            return Ok(None);
        }

        let (
            receiver,
            constructor_arg_count,
            constructor_argument_kinds,
            field_initializer_count,
            type_arguments,
            span,
        ) = match object {
            ASTNode::Literal {
                value: LiteralValue::String(_),
                span,
            } => (
                Stage1StringBoxReceiverV1::DirectStringLiteral,
                0,
                Vec::new(),
                0,
                Vec::new(),
                *span,
            ),
            ASTNode::New {
                class,
                arguments,
                field_initializers,
                type_arguments,
                span,
            } if class == "StringBox"
                && arguments.len() == 1
                && matches!(
                    arguments.first(),
                    Some(ASTNode::Literal {
                        value: LiteralValue::String(_),
                        ..
                    })
                )
                && field_initializers.is_empty()
                && type_arguments.is_empty() =>
            {
                (
                    Stage1StringBoxReceiverV1::NewStringBox,
                    arguments.len(),
                    vec![Stage1StringBoxArgumentKindV1::DirectStringLiteral],
                    field_initializers.len(),
                    type_arguments.clone(),
                    *span,
                )
            }
            _ => return Ok(None),
        };

        let source_span = Stage1SourceSpanV1::from(span);
        let anchor = self.relations.len() as u32;
        let path = format!("Main.main/0#stringbox-anchor-{}", anchor);
        let contract = Stage1StringBoxMethodContractV1::for_selector(method, arguments.len())?;
        self.relations.push(Stage1StringBoxSourceRelationV1 {
            owner: "Main.main/0",
            path,
            anchor,
            span: source_span,
            receiver,
            selector: method.to_string(),
            arity: arguments.len(),
            constructor_arg_count,
            constructor_argument_kinds,
            field_initializer_count,
            type_arguments,
            contract,
        });
        Ok(Some(anchor))
    }

    pub(crate) fn finish(
        self,
    ) -> Result<(Stage1StringBoxSourceProductV1, Stage1ProgramJsonCrosswalkV1), String> {
        let mut seen = BTreeSet::new();
        for relation in &self.relations {
            if !seen.insert(relation.anchor) {
                return Err(format!(
                    "[stage1/stringbox-source-artifact/duplicate] source relation {}",
                    relation.path
                ));
            }
            if relation.contract.selector != relation.selector
                || relation.contract.arity != relation.arity
                || relation.anchor as usize >= self.relations.len()
            {
                return Err(format!(
                    "[stage1/stringbox-source-artifact/contract-mismatch] source relation {}",
                    relation.path
                ));
            }
        }

        let entries = self
            .relations
            .iter()
            .map(|relation| Stage1ProgramJsonCrosswalkEntryV1 {
                owner: relation.owner,
                path: relation.path.clone(),
                anchor: relation.anchor,
                span: relation.span.clone(),
                receiver: relation.receiver.clone(),
                expected_box_name: "RuntimeDataBox",
                expected_method: relation.selector.clone(),
                expected_arity: relation.arity,
            })
            .collect();

        Ok((
            Stage1StringBoxSourceProductV1 {
                relations: self.relations,
            },
            Stage1ProgramJsonCrosswalkV1 { entries },
        ))
    }
}
