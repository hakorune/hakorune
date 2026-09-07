use super::*;
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, ResolveScriptOutcomeV1, ScriptRootResolvedDemandV1,
    ScriptRootRuntimeDispositionV1, ScriptRootSemanticDispositionV1, ScriptSyntaxViewV1,
    SourcePathV1, VerifiedResolvedScriptV1, VerifiedScriptRootDemandEntryV1,
    VerifiedScriptRootDemandWindowV1,
};

fn source_owner(unit: u32) -> VerifiedResolvedScriptV1 {
    let ast = crate::parser::NyashParser::parse_from_string("local a: Array<i64> = []").unwrap();
    let statement = SourcePathV1::program_body()
        .child(SourcePathSegmentV1::ProgramBody(0))
        .stmt();
    let window = VerifiedScriptRootDemandWindowV1::seal(
        vec![VerifiedScriptRootDemandEntryV1::new(
            statement,
            ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::LexicalCore),
            ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
        )],
        1,
    )
    .unwrap();
    let mut resolver = FunctionSemanticResolverSessionV1::new(unit).unwrap();
    let ResolveScriptOutcomeV1::Complete(owner) = resolver
        .resolve_script(ScriptSyntaxViewV1::from_program(&ast).unwrap(), &window)
        .unwrap()
    else {
        panic!("Complete")
    };
    owner
}

#[test]
fn local_relation_projection_rejects_missing_duplicate_foreign_and_missing_expression() {
    let owner = source_owner(0);
    let foreign = source_owner(1);
    let core = owner.core();
    let owner_id = core.data().owner;
    let relation = owner.expression_source().initializers().next().unwrap();
    let SourceBindingSiteV1::Local { statement, .. } = relation.declaration_site() else {
        panic!("Local")
    };
    let bindings = BTreeMap::from([(statement.node().clone(), relation.binding())]);
    assert!(seal_local_relations(
        owner_id,
        bindings.clone(),
        [relation].into_iter(),
        core.expression_sites()
    )
    .is_ok());
    assert!(seal_local_relations(
        owner_id,
        bindings.clone(),
        [].into_iter(),
        core.expression_sites()
    )
    .unwrap_err()
    .contains("missing-local-relation"));
    assert!(seal_local_relations(
        owner_id,
        bindings.clone(),
        [relation, relation].into_iter(),
        core.expression_sites()
    )
    .is_err());
    assert!(seal_local_relations(
        owner_id,
        bindings.clone(),
        foreign.expression_source().initializers(),
        core.expression_sites()
    )
    .unwrap_err()
    .contains("local-relation-binding"));
    assert!(
        seal_local_relations(owner_id, bindings, [relation].into_iter(), [].into_iter())
            .unwrap_err()
            .contains("local-relation-initializer")
    );
}
