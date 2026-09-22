//! Caller-zero consumer for exact static-call result publication.
//!
//! The source demand is AST-free and Builder-free.  This box is the only
//! owner allowed to turn a successful physical Call receipt into a type fact.
//! It is intentionally disconnected from raw route selection and GenericLoop
//! until a later activation row proves the whole-source wiring.

use crate::mir::builder::calls::unified_emitter::CompletedUnifiedValueCallEmissionV1;
use crate::mir::builder::{MirBuilder, ValueId};
use crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1;
use crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationDemandV1;
use crate::mir::MirType;

fn exact_physical_result_type(representation: &VerifiedCallableResultRepresentationV1) -> MirType {
    match representation {
        VerifiedCallableResultRepresentationV1::ExactI64 => MirType::Integer,
        VerifiedCallableResultRepresentationV1::ExactBool => MirType::Bool,
        VerifiedCallableResultRepresentationV1::ExactString => MirType::String,
        VerifiedCallableResultRepresentationV1::ExactNominalBox { box_name } => {
            MirType::Box(box_name.clone())
        }
    }
}

#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedStaticCallResultPublicationV1 {
    demand: VerifiedStaticCallResultPublicationDemandV1,
    destination: ValueId,
    _seal: PreparedStaticCallResultPublicationSealV1,
}

#[derive(Debug)]
struct PreparedStaticCallResultPublicationSealV1;

impl PreparedStaticCallResultPublicationV1 {
    pub(in crate::mir::builder) fn prepare(
        demand: VerifiedStaticCallResultPublicationDemandV1,
        emission: CompletedUnifiedValueCallEmissionV1,
    ) -> Self {
        Self {
            demand,
            destination: emission.final_destination(),
            _seal: PreparedStaticCallResultPublicationSealV1,
        }
    }

    pub(in crate::mir::builder) const fn destination(&self) -> ValueId {
        self.destination
    }

    pub(in crate::mir::builder) const fn demand(
        &self,
    ) -> &VerifiedStaticCallResultPublicationDemandV1 {
        &self.demand
    }

    /// Consume the publication exactly once after physical Call success.
    pub(in crate::mir::builder) fn commit(self, builder: &mut MirBuilder) -> Result<(), String> {
        if builder
            .function_state
            .type_ctx
            .get_type(self.destination)
            .is_some()
        {
            return Err("[freeze:contract][static-call-result-publication/duplicate]".to_owned());
        }
        let exact_type = exact_physical_result_type(self.demand.representation());
        builder
            .function_state
            .type_ctx
            .set_type(self.destination, exact_type);
        Ok(())
    }

    /// Commit a publication whose destination type was preallocated by the
    /// source normalizer. Existing matching type information is accepted as
    /// the same destination contract; conflicting type information still
    /// fails closed.
    pub(in crate::mir::builder) fn commit_external_destination(
        self,
        builder: &mut MirBuilder,
    ) -> Result<(), String> {
        let exact_type = exact_physical_result_type(self.demand.representation());
        if let Some(existing) = builder.function_state.type_ctx.get_type(self.destination) {
            if existing != &exact_type {
                return Err(
                    "[freeze:contract][static-call-result-publication/type-mismatch]".to_owned(),
                );
            }
            return Ok(());
        }
        builder
            .function_state
            .type_ctx
            .set_type(self.destination, exact_type);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::calls::call_target::CallTarget;
    use crate::mir::builder::calls::unified_emitter::UnifiedCallEmitterBox;
    use crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationDemandV1;
    use crate::mir::resolved_semantics::{SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1};

    fn demand() -> VerifiedStaticCallResultPublicationDemandV1 {
        VerifiedStaticCallResultPublicationDemandV1::from_test_parts(
            crate::mir::builder::CanonicalSameModuleCallableKeyV1::test_static_box_method(
                "StringHelpers",
                "int_to_str",
                1,
            ),
            SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::Initializer(0),
            ])),
            crate::mir::builder::CanonicalSameModuleCallableKeyV1::test_static_box_method(
                "StringHelpers",
                "to_i64",
                1,
            ),
        )
    }

    fn builder(name: &str) -> MirBuilder {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test(name.to_owned());
        builder
    }

    fn source_result_demand(result: &str) -> VerifiedStaticCallResultPublicationDemandV1 {
        use crate::mir::builder::{
            SameModuleCallableNamespaceV1, VerifiedSameModuleCallableDeclarationCatalogV1,
        };
        use crate::mir::callable_result_representation::{
            StaticCallResultPublicationTakeV1, VerifiedSameModuleCallableResultCatalogV1,
            VerifiedStaticCallResultPublicationOwnerV1,
        };
        use crate::mir::source_call_target::{
            VerifiedSourceMethodCallSiteV1, VerifiedSourceStaticCallTargetCatalogV1,
            VerifiedStaticImportAliasViewV1,
        };
        let ast = crate::parser::NyashParser::parse_from_string(&format!(
            "static box Text {{ caller() {{ return me.text() }} text() {{ return {result} }} }}"
        ))
        .unwrap();
        let declarations =
            VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&ast).unwrap();
        let caller = declarations
            .declaration_for(
                SameModuleCallableNamespaceV1::StaticBoxMethod,
                "Text",
                "caller",
                0,
            )
            .unwrap()
            .key()
            .clone();
        let site = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::Value,
        ]));
        let imports = VerifiedStaticImportAliasViewV1::seal(
            &declarations,
            std::iter::empty::<(String, String)>(),
        )
        .unwrap();
        let call =
            VerifiedSourceMethodCallSiteV1::verify(&declarations, &caller, site.clone()).unwrap();
        let targets =
            VerifiedSourceStaticCallTargetCatalogV1::seal_qualified(&imports, std::iter::empty())
                .unwrap()
                .extend_current_owner([&call])
                .unwrap();
        let results =
            VerifiedSameModuleCallableResultCatalogV1::verify(&declarations, &targets).unwrap();
        let mut owner =
            VerifiedStaticCallResultPublicationOwnerV1::issue(&declarations, &targets, &results)
                .unwrap();
        let StaticCallResultPublicationTakeV1::Selected(handoff) = owner
            .take_for_source(&declarations, &caller, &site)
            .unwrap()
        else {
            panic!("source String handoff")
        };
        owner.finish_empty().unwrap();
        handoff.consume().0
    }

    #[test]
    fn source_string_and_bool_publication_cover_destinations_and_mismatch() {
        crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
            for (result, expected) in [("\"猫\"", MirType::String), ("true", MirType::Bool)] {
                for external in [false, true] {
                    let mut builder = builder("string_publication/0");
                    let demand = source_result_demand(result);
                    let destination = builder.alloc_value_for_test();
                    let target = demand.target().canonical_global_target_v1().unwrap();
                    let emission =
                        UnifiedCallEmitterBox::emit_unified_value_call_with_lookup_receipt_v1(
                            &mut builder,
                            destination,
                            CallTarget::Global(target),
                            vec![],
                            None,
                        )
                        .unwrap();
                    let publication =
                        PreparedStaticCallResultPublicationV1::prepare(demand, emission);
                    if external {
                        builder
                            .function_state
                            .type_ctx
                            .set_type(destination, expected.clone());
                        publication
                            .commit_external_destination(&mut builder)
                            .unwrap();
                    } else {
                        publication.commit(&mut builder).unwrap();
                    }
                    assert_eq!(
                        builder.function_state.type_ctx.get_type(destination),
                        Some(&expected)
                    );
                }
                let mut builder = builder("string_publication_mismatch/0");
                let demand = source_result_demand(result);
                let destination = builder.alloc_value_for_test();
                let target = demand.target().canonical_global_target_v1().unwrap();
                let emission =
                    UnifiedCallEmitterBox::emit_unified_value_call_with_lookup_receipt_v1(
                        &mut builder,
                        destination,
                        CallTarget::Global(target),
                        vec![],
                        None,
                    )
                    .unwrap();
                builder
                    .function_state
                    .type_ctx
                    .set_type(destination, MirType::Integer);
                let error = PreparedStaticCallResultPublicationV1::prepare(demand, emission)
                    .commit_external_destination(&mut builder)
                    .unwrap_err();
                assert!(error.contains("type-mismatch"));
                assert_eq!(
                    builder.function_state.type_ctx.get_type(destination),
                    Some(&MirType::Integer)
                );
            }
        });
    }

    #[test]
    fn successful_physical_receipt_publishes_integer_once() {
        crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
            let mut builder = builder("static_result_publication/0");
            let destination = builder.alloc_value_for_test();
            let emission = UnifiedCallEmitterBox::emit_unified_value_call_with_lookup_receipt_v1(
                &mut builder,
                destination,
                CallTarget::Global(crate::mir::test_global_target(
                    "StringHelpers.to_i64/1".to_owned(),
                )),
                vec![],
                None,
            )
            .expect("generic physical receipt");
            let publication = PreparedStaticCallResultPublicationV1::prepare(demand(), emission);
            assert_eq!(publication.destination(), destination);
            assert_eq!(publication.demand().target().name(), "to_i64");
            publication.commit(&mut builder).expect("one publication");
            assert_eq!(
                builder.function_state.type_ctx.get_type(destination),
                Some(&MirType::Integer)
            );
        });
    }

    #[test]
    fn exact_nominal_box_representation_projects_to_the_matching_mir_type() {
        assert_eq!(
            exact_physical_result_type(&VerifiedCallableResultRepresentationV1::ExactNominalBox {
                box_name: "ParserNodeProductV1".to_owned(),
            },),
            MirType::Box("ParserNodeProductV1".to_owned())
        );
    }

    #[test]
    fn publication_rejects_existing_type_before_second_write() {
        crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
            let mut builder = builder("static_result_publication_duplicate/0");
            let destination = builder.alloc_value_for_test();
            builder
                .function_state
                .type_ctx
                .set_type(destination, MirType::Integer);
            let emission = UnifiedCallEmitterBox::emit_unified_value_call_with_lookup_receipt_v1(
                &mut builder,
                destination,
                CallTarget::Global(crate::mir::test_global_target(
                    "StringHelpers.to_i64/1".to_owned(),
                )),
                vec![],
                None,
            )
            .expect("generic physical receipt");
            let error = PreparedStaticCallResultPublicationV1::prepare(demand(), emission)
                .commit(&mut builder)
                .expect_err("duplicate publication");
            assert!(error.contains("static-call-result-publication/duplicate"));
        });
    }

    #[test]
    fn failed_physical_call_cannot_create_publication() {
        crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
            let mut builder = builder("static_result_publication_failure/0");
            let destination = builder.alloc_value_for_test();
            builder.function_state.current_block = None;
            let result = UnifiedCallEmitterBox::emit_unified_value_call_with_lookup_receipt_v1(
                &mut builder,
                destination,
                CallTarget::Global(crate::mir::test_global_target(
                    "StringHelpers.to_i64/1".to_owned(),
                )),
                vec![],
                None,
            );
            assert!(result.is_err());
            assert_eq!(builder.function_state.type_ctx.get_type(destination), None);
        });
    }
}
