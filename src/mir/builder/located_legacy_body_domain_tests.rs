use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::callable_result_representation::VerifiedCallableResultLegacySourceViewV1;
use crate::mir::resolved_semantics::{BodyChildRoleV1, SourcePathSegmentV1};
use crate::mir::{BasicBlockId, MirInstruction, ValueId};

use super::local_tests::{builder_for, caller, instructions, seal_plan, site, CallSiteSpecV1};
use super::{LocatedLegacyLoweringErrorV1, LocatedLegacyLoweringSessionV1};

const SOURCE: &str = r#"
    box ParserBox {
        parse(text, pos) {
            if pos {
                local then_value = Helpers.step(1)
            } else {
                local else_value = Helpers.step(2)
            }
            loop(pos < 1) {
                local loop_value = Helpers.step(3)
                break
            }
            return 0
        }
    }
    static box Helpers { step(value) { return value } }
"#;

#[derive(Debug, PartialEq)]
struct BuilderEffectSnapshotV1 {
    current_block: Option<BasicBlockId>,
    function_next_value: u32,
    core_next_value: ValueId,
    core_next_block: BasicBlockId,
    blocks: Vec<(
        BasicBlockId,
        Vec<MirInstruction>,
        Option<MirInstruction>,
        Vec<BasicBlockId>,
        Vec<BasicBlockId>,
        bool,
        bool,
    )>,
}

fn effect_snapshot(builder: &crate::mir::MirBuilder) -> BuilderEffectSnapshotV1 {
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .expect("body-domain function");
    let mut blocks = function
        .blocks
        .values()
        .map(|block| {
            (
                block.id,
                block.instructions.clone(),
                block.terminator.clone(),
                block.predecessors.iter().copied().collect(),
                block.successors.iter().copied().collect(),
                block.reachable,
                block.sealed,
            )
        })
        .collect::<Vec<_>>();
    blocks.sort_by_key(|row| row.0);
    BuilderEffectSnapshotV1 {
        current_block: builder.function_state.current_block,
        function_next_value: function.next_value_id,
        core_next_value: builder.core_ctx.peek_next_value(),
        core_next_block: builder.core_ctx.peek_next_block(),
        blocks,
    }
}

fn plan() -> crate::mir::callable_result_representation::VerifiedCallableResultActivationPlanV1 {
    seal_plan(
        SOURCE,
        vec![
            CallSiteSpecV1 {
                site: site(vec![
                    SourcePathSegmentV1::Body(0),
                    SourcePathSegmentV1::IfThen(0),
                    SourcePathSegmentV1::Initializer(0),
                ]),
            },
            CallSiteSpecV1 {
                site: site(vec![
                    SourcePathSegmentV1::Body(0),
                    SourcePathSegmentV1::IfElse(0),
                    SourcePathSegmentV1::Initializer(0),
                ]),
            },
            CallSiteSpecV1 {
                site: site(vec![
                    SourcePathSegmentV1::Body(1),
                    SourcePathSegmentV1::LoopBody(0),
                    SourcePathSegmentV1::Initializer(0),
                ]),
            },
        ],
    )
}

#[test]
fn active_then_else_and_loop_body_domains_fail_before_raw_effects() {
    let plan = plan();
    let caller = caller(plan.declaration_catalog());
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let root = view.root_body();
    let if_statement = view.body_stmt(&root, 0).unwrap();
    let loop_statement = view.body_stmt(&root, 1).unwrap();
    let bodies = [
        view.child_body_from_stmt(&if_statement, BodyChildRoleV1::IfThen)
            .unwrap(),
        view.child_body_from_stmt(&if_statement, BodyChildRoleV1::IfElse)
            .unwrap(),
        view.child_body_from_stmt(&loop_statement, BodyChildRoleV1::LoopBody)
            .unwrap(),
    ];

    for (index, body) in bodies.into_iter().enumerate() {
        let mut session = LocatedLegacyLoweringSessionV1::verify(&plan, &caller).unwrap();
        let mut builder = builder_for(SOURCE, &format!("body_domain_reject_{index}/0"));
        let _scope = LexicalScopeGuard::new(&mut builder);
        let before = effect_snapshot(&builder);

        let error = session.lower_body(&mut builder, body).unwrap_err();

        assert!(format!("{error:?}").contains("RowsUnderPrefix"));
        assert_eq!(effect_snapshot(&builder), before);
        assert_eq!(call_count(&builder), 0);
        assert_eq!(return_count(&builder), 0);
        assert_eq!(
            session.finish(),
            Err(LocatedLegacyLoweringErrorV1::Poisoned)
        );
    }
}

fn call_count(builder: &crate::mir::MirBuilder) -> usize {
    instructions(builder)
        .iter()
        .filter(|instruction| matches!(instruction, MirInstruction::Call { .. }))
        .count()
}

fn return_count(builder: &crate::mir::MirBuilder) -> usize {
    builder
        .function_state
        .current_function
        .as_ref()
        .expect("body-domain function")
        .blocks
        .values()
        .filter_map(|block| block.terminator.as_ref())
        .filter(|instruction| matches!(instruction, MirInstruction::Return { .. }))
        .count()
}
