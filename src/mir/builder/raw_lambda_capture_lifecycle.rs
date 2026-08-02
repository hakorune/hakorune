//! One consuming production lifecycle for raw Lambda capture and publication.

use super::raw_invocation_source_transport::RawSourceTransportPortV1;
use super::raw_lambda_closure_emission::PreparedRawLambdaClosureEmissionV1;
use super::raw_lambda_lexical_observation::RawLambdaLexicalObservationV1;
use super::recursive_child_lowering::{RawInvocationChildPortV1, RawLegacyChildLoweringPortV1};
use super::{MirBuilder, ValueId};
use crate::ast::ASTNode;
use std::collections::BTreeMap;

pub(super) struct PreparedRawLambdaLexicalCaptureLifecycleV1 {
    params: Vec<String>,
    body: Vec<ASTNode>,
    observation: RawLambdaLexicalObservationV1,
}

impl PreparedRawLambdaLexicalCaptureLifecycleV1 {
    pub(super) fn lower_with_selected_captures_v1(
        params: Vec<String>,
        body: Vec<ASTNode>,
        captures: Vec<(String, ValueId)>,
        builder: &mut MirBuilder,
    ) -> Result<ValueId, String> {
        PreparedRawLambdaClosureEmissionV1::prepare(params, body, captures, None)
            .lower_with_builder_v1(builder)
    }

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
        PreparedRawLambdaClosureEmissionV1::prepare(self.params, self.body, captures, receiver)
            .lower_with_builder_v1(builder)
    }
}

pub(in crate::mir::builder) trait RawLambdaCaptureDemandPortV1 {
    fn selected_lambda_captures_v1(&self) -> Result<Option<Vec<(String, ValueId)>>, String>;
}

impl RawLambdaCaptureDemandPortV1 for RawLegacyChildLoweringPortV1 {
    fn selected_lambda_captures_v1(&self) -> Result<Option<Vec<(String, ValueId)>>, String> {
        Ok(None)
    }
}

impl RawLambdaCaptureDemandPortV1 for RawInvocationChildPortV1<'_, '_> {
    fn selected_lambda_captures_v1(&self) -> Result<Option<Vec<(String, ValueId)>>, String> {
        if let Some(ledger) = &self.callable_ledger {
            let site = self
                .current_source_context_v1()
                .and_then(|context| context.site().cloned())
                .ok_or_else(|| "[freeze:contract][callable-lambda/missing-site]".to_owned())?;
            return ledger.borrow_mut().direct_lambda_captures(&site).map(Some);
        }
        let Some(ledger) = &self.semantic_ledger else {
            return Ok(None);
        };
        let site = self
            .current_source_context_v1()
            .and_then(|context| context.site().cloned())
            .ok_or_else(|| "[freeze:contract][script-lambda/missing-site]".to_owned())?;
        ledger
            .borrow()
            .lambda_captures(&site)
            .transpose()?
            .map(Some)
            .ok_or_else(|| "[freeze:contract][script-lambda/missing-sealed-receipt]".to_owned())
    }
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

    #[test]
    fn selected_capture_receipt_uses_the_existing_closure_emitter() {
        let mut builder = MirBuilder::new();
        builder.prepare_module().unwrap();

        let dst = PreparedRawLambdaLexicalCaptureLifecycleV1::lower_with_selected_captures_v1(
            vec![],
            vec![variable("outer")],
            vec![("outer".into(), crate::mir::ValueId(7))],
            &mut builder,
        )
        .unwrap();

        let function = builder.function_state.current_function.as_ref().unwrap();
        assert!(function.blocks.values().flat_map(|block| &block.instructions).any(
            |instruction| matches!(instruction, MirInstruction::NewClosure { dst: value, captures, .. } if *value == dst && captures == &vec![("outer".into(), crate::mir::ValueId(7))])
        ));
    }

    #[test]
    fn selected_script_lambda_capture_matches_legacy_lowering() {
        use crate::mir::{MirCompiler, MirPrinter, NormalCompileRequestV1};
        use crate::parser::NyashParser;

        let source = "local outer = 7\nlocal f = fn() { outer }\nf";
        let mut legacy = MirCompiler::with_options(false);
        let legacy = legacy
            .compile_with_source(
                NyashParser::parse_from_string(source).unwrap(),
                Some("lambda"),
            )
            .unwrap();
        let normal = MirCompiler::with_options(false)
            .compile_normal(
                NormalCompileRequestV1::for_mir_mode(
                    NyashParser::parse_from_string(source).unwrap(),
                    Some("lambda"),
                    std::collections::HashMap::new(),
                )
                .unwrap(),
            )
            .unwrap();
        assert_eq!(
            MirPrinter::new().print_module(&normal.module),
            MirPrinter::new().print_module(&legacy.module)
        );
        assert_eq!(normal.verification_result, legacy.verification_result);
    }
}
