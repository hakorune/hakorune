//! Script-only ExactI64 publication after a generic Call receipt.
//!
//! The source result owner remains the authority for representation.  This
//! session-local box only projects that already-verified `ExactI64` fact into
//! the destination of one completed generic Call, exactly once.

use crate::mir::builder::calls::unified_emitter::CompletedUnifiedValueCallEmissionV1;
use crate::mir::builder::{MirBuilder, ValueId};
use crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1;
use crate::mir::MirType;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum ScriptDirectStaticPublicationErrorV1 {
    RepresentationMismatch,
    DuplicatePublication,
}

impl std::fmt::Display for ScriptDirectStaticPublicationErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RepresentationMismatch => write!(
                formatter,
                "[freeze:contract][script-direct-static/publication-representation]"
            ),
            Self::DuplicatePublication => write!(
                formatter,
                "[freeze:contract][script-direct-static/publication-duplicate]"
            ),
        }
    }
}

#[derive(Debug)]
pub(super) struct PreparedScriptDirectStaticResultPublicationV1 {
    destination: ValueId,
    _seal: PreparedScriptDirectStaticResultPublicationSealV1,
}

#[derive(Debug)]
struct PreparedScriptDirectStaticResultPublicationSealV1;

impl PreparedScriptDirectStaticResultPublicationV1 {
    pub(super) fn prepare(
        representation: &VerifiedCallableResultRepresentationV1,
        emission: CompletedUnifiedValueCallEmissionV1,
    ) -> Result<Self, ScriptDirectStaticPublicationErrorV1> {
        if !matches!(
            representation,
            VerifiedCallableResultRepresentationV1::ExactI64
        ) {
            return Err(ScriptDirectStaticPublicationErrorV1::RepresentationMismatch);
        }
        Ok(Self {
            destination: emission.final_destination(),
            _seal: PreparedScriptDirectStaticResultPublicationSealV1,
        })
    }

    pub(super) fn commit(
        self,
        builder: &mut MirBuilder,
    ) -> Result<ValueId, ScriptDirectStaticPublicationErrorV1> {
        if builder
            .function_state
            .type_ctx
            .get_type(self.destination)
            .is_some()
        {
            return Err(ScriptDirectStaticPublicationErrorV1::DuplicatePublication);
        }
        builder
            .function_state
            .type_ctx
            .set_type(self.destination, MirType::Integer);
        Ok(self.destination)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::calls::call_target::CallTarget;
    use crate::mir::builder::calls::unified_emitter::UnifiedCallEmitterBox;

    fn builder() -> MirBuilder {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("script-direct-static/publication/0".to_owned());
        builder
    }

    #[test]
    fn exact_i64_publication_is_one_shot() {
        crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
            let mut builder = builder();
            let destination = builder.alloc_value_for_test();
            let emission = UnifiedCallEmitterBox::emit_unified_value_call_with_lookup_receipt_v1(
                &mut builder,
                destination,
                CallTarget::Global("Helpers.run/1".to_owned()),
                vec![],
                None,
            )
            .expect("generic receipt");
            let publication = PreparedScriptDirectStaticResultPublicationV1::prepare(
                &VerifiedCallableResultRepresentationV1::ExactI64,
                emission,
            )
            .expect("ExactI64");
            assert_eq!(publication.commit(&mut builder).unwrap(), destination);
            assert_eq!(
                builder.function_state.type_ctx.get_type(destination),
                Some(&MirType::Integer)
            );
        });
    }

    #[test]
    fn non_exact_representation_is_rejected_before_publication() {
        crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
            let mut builder = builder();
            let destination = builder.alloc_value_for_test();
            let emission = UnifiedCallEmitterBox::emit_unified_value_call_with_lookup_receipt_v1(
                &mut builder,
                destination,
                CallTarget::Global("Helpers.run/1".to_owned()),
                vec![],
                None,
            )
            .expect("generic receipt");
            let error = PreparedScriptDirectStaticResultPublicationV1::prepare(
                &VerifiedCallableResultRepresentationV1::ExactNominalBox {
                    box_name: "Box".to_owned(),
                },
                emission,
            )
            .expect_err("non-ExactI64 must be typed");
            assert_eq!(
                error,
                ScriptDirectStaticPublicationErrorV1::RepresentationMismatch
            );
        });
    }

    #[test]
    fn duplicate_destination_is_rejected_as_typed_error() {
        crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
            let mut builder = builder();
            let destination = builder.alloc_value_for_test();
            builder
                .function_state
                .type_ctx
                .set_type(destination, MirType::Integer);
            let emission = UnifiedCallEmitterBox::emit_unified_value_call_with_lookup_receipt_v1(
                &mut builder,
                destination,
                CallTarget::Global("Helpers.run/1".to_owned()),
                vec![],
                None,
            )
            .expect("generic receipt");
            let publication = PreparedScriptDirectStaticResultPublicationV1::prepare(
                &VerifiedCallableResultRepresentationV1::ExactI64,
                emission,
            )
            .expect("ExactI64");
            assert_eq!(
                publication.commit(&mut builder),
                Err(ScriptDirectStaticPublicationErrorV1::DuplicatePublication)
            );
        });
    }
}
