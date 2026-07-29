//! One consuming production lifecycle for raw Lambda capture and publication.

use super::raw_lambda_lexical_observation::RawLambdaLexicalObservationV1;
use super::{MirBuilder, MirInstruction, MirType, ValueId};
use crate::ast::ASTNode;
use crate::mir::function::ClosureBodyId;
use std::collections::BTreeMap;

pub(super) struct PreparedRawLambdaLexicalCaptureLifecycleV1 {
    params: Vec<String>,
    body: Vec<ASTNode>,
    observation: RawLambdaLexicalObservationV1,
}

impl PreparedRawLambdaLexicalCaptureLifecycleV1 {
    pub(super) fn prepare(params: Vec<String>, body: Vec<ASTNode>) -> Result<Self, String> {
        let observation = RawLambdaLexicalObservationV1::observe(&params, &body)
            .map_err(|error| error.to_string())?;
        Ok(Self {
            params,
            body,
            observation,
        })
    }

    pub(super) fn lower_with_builder_v1(self, builder: &mut MirBuilder) -> Result<ValueId, String> {
        let environment = RawLambdaCaptureEnvironmentV1::snapshot(builder);
        let captures = environment.materialize_captures(&self.observation, builder)?;
        let receiver = environment.materialize_receiver(&self.observation, builder)?;
        let publication = PreparedClosureBodyPublicationV1::prepare(builder, self.body);

        match publication {
            PreparedClosureBodyPublicationV1::Inline { body } => {
                let dst = builder.next_value_id();
                builder.emit_instruction(MirInstruction::NewClosure {
                    dst,
                    params: self.params,
                    body_id: None,
                    body,
                    captures,
                    me: receiver,
                })?;
                install_function_box_type(builder, dst);
                Ok(dst)
            }
            PreparedClosureBodyPublicationV1::External { expected_id, body } => {
                let dst = builder.next_value_id();
                builder.emit_instruction(MirInstruction::NewClosure {
                    dst,
                    params: self.params,
                    body_id: Some(expected_id),
                    body: Vec::new(),
                    captures,
                    me: receiver,
                })?;
                builder
                    .current_module
                    .as_mut()
                    .expect("[freeze:contract][mir_builder/raw_lambda_missing_reserved_module]")
                    .commit_reserved_closure_body(expected_id, body);
                install_function_box_type(builder, dst);
                Ok(dst)
            }
        }
    }
}

fn install_function_box_type(builder: &mut MirBuilder, value: ValueId) {
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(value, MirType::Box("FunctionBox".into()));
}

/// Explicit compatibility boundary between source observation and the legacy
/// raw variable map. It is a read-only snapshot, never a fabricated resolver
/// binding identity.
struct RawLambdaCaptureEnvironmentV1 {
    names: BTreeMap<Box<str>, ValueId>,
    receiver: Option<ValueId>,
}

impl RawLambdaCaptureEnvironmentV1 {
    fn snapshot(builder: &MirBuilder) -> Self {
        Self {
            names: builder
                .function_state
                .variable_ctx
                .variable_map
                .iter()
                .map(|(name, value)| (Box::<str>::from(name.as_str()), *value))
                .collect(),
            receiver: builder
                .function_state
                .variable_ctx
                .variable_map
                .get("me")
                .copied(),
        }
    }

    fn materialize_captures(
        &self,
        observation: &RawLambdaLexicalObservationV1,
        builder: &MirBuilder,
    ) -> Result<Vec<(String, ValueId)>, String> {
        observation
            .capture_names()
            .iter()
            .map(|name| {
                let value = self
                    .names
                    .get(name.as_ref())
                    .copied()
                    .ok_or_else(|| builder.undefined_variable_message(name))?;
                Ok((name.to_string(), value))
            })
            .collect()
    }

    fn materialize_receiver(
        &self,
        observation: &RawLambdaLexicalObservationV1,
        builder: &MirBuilder,
    ) -> Result<Option<ValueId>, String> {
        if !observation.receiver_required() {
            return Ok(None);
        }
        self.receiver
            .ok_or_else(|| {
                format!(
                    "{} [freeze:contract][mir_builder/raw_lambda_receiver_unavailable]",
                    builder.undefined_variable_message("me")
                )
            })
            .map(Some)
    }
}

enum PreparedClosureBodyPublicationV1 {
    Inline {
        body: Vec<ASTNode>,
    },
    External {
        expected_id: ClosureBodyId,
        body: Vec<ASTNode>,
    },
}

impl PreparedClosureBodyPublicationV1 {
    fn prepare(builder: &MirBuilder, body: Vec<ASTNode>) -> Self {
        if body.is_empty() {
            return Self::Inline { body };
        }
        let Some(module) = builder.current_module.as_ref() else {
            return Self::Inline { body };
        };
        Self::External {
            expected_id: module.reserve_next_closure_body_id(),
            body,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreparedRawLambdaLexicalCaptureLifecycleV1;
    use crate::ast::{ASTNode, Span};
    use crate::mir::builder::MirBuilder;
    use crate::mir::{MirInstruction, MirModule};

    fn variable(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.into(),
            span: Span::unknown(),
        }
    }

    #[test]
    fn unavailable_capture_fails_before_metadata_or_value_effect() {
        let mut builder = MirBuilder::new();
        builder.current_module = Some(MirModule::new("lambda".into()));
        let next = builder
            .current_module
            .as_ref()
            .unwrap()
            .metadata
            .next_closure_body_id;

        let error =
            PreparedRawLambdaLexicalCaptureLifecycleV1::prepare(vec![], vec![variable("missing")])
                .unwrap()
                .lower_with_builder_v1(&mut builder)
                .unwrap_err();

        assert!(error.contains("Undefined variable: missing"));
        let module = builder.current_module.as_ref().unwrap();
        assert!(module.metadata.closure_bodies.is_empty());
        assert_eq!(module.metadata.next_closure_body_id, next);
    }

    #[test]
    fn direct_me_requires_an_explicit_receiver_before_metadata_effect() {
        let mut builder = MirBuilder::new();
        builder.current_module = Some(MirModule::new("lambda".into()));
        let next = builder
            .current_module
            .as_ref()
            .unwrap()
            .metadata
            .next_closure_body_id;

        let error = PreparedRawLambdaLexicalCaptureLifecycleV1::prepare(
            vec![],
            vec![ASTNode::Me {
                span: Span::unknown(),
            }],
        )
        .unwrap()
        .lower_with_builder_v1(&mut builder)
        .unwrap_err();

        assert!(error.contains("raw_lambda_receiver_unavailable"));
        let module = builder.current_module.as_ref().unwrap();
        assert!(module.metadata.closure_bodies.is_empty());
        assert_eq!(module.metadata.next_closure_body_id, next);
    }

    #[test]
    fn emit_failure_does_not_publish_reserved_body() {
        let mut builder = MirBuilder::new();
        builder.current_module = Some(MirModule::new("lambda".into()));
        let next = builder
            .current_module
            .as_ref()
            .unwrap()
            .metadata
            .next_closure_body_id;

        let error = PreparedRawLambdaLexicalCaptureLifecycleV1::prepare(
            vec![],
            vec![ASTNode::Literal {
                value: crate::ast::LiteralValue::Integer(1),
                span: Span::unknown(),
            }],
        )
        .unwrap()
        .lower_with_builder_v1(&mut builder)
        .unwrap_err();

        assert!(error.contains("No current basic block"));
        let module = builder.current_module.as_ref().unwrap();
        assert!(module.metadata.closure_bodies.is_empty());
        assert_eq!(module.metadata.next_closure_body_id, next);
    }

    #[test]
    fn external_body_commits_only_after_closure_emission() {
        let mut builder = MirBuilder::new();
        builder.prepare_module().unwrap();
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert("outer".into(), crate::mir::ValueId(7));

        let dst =
            PreparedRawLambdaLexicalCaptureLifecycleV1::prepare(vec![], vec![variable("outer")])
                .unwrap()
                .lower_with_builder_v1(&mut builder)
                .unwrap();

        let module = builder.current_module.as_ref().unwrap();
        assert_eq!(module.metadata.closure_bodies.len(), 1);
        let function = builder.function_state.current_function.as_ref().unwrap();
        let instruction = function
            .blocks
            .values()
            .flat_map(|block| block.instructions.iter())
            .find(|instruction| matches!(instruction, MirInstruction::NewClosure { dst: value, .. } if *value == dst))
            .unwrap();
        assert!(
            matches!(instruction, MirInstruction::NewClosure { body_id: Some(0), captures, .. } if captures == &vec![("outer".into(), crate::mir::ValueId(7))])
        );
    }
}
