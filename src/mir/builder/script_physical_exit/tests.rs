use super::{
    LoweredScriptTerminalV1, LoweredScriptUnitPayloadV1, PreparedScriptPhysicalExitCoreV1,
    ScriptPhysicalExitCommitV1, ScriptPhysicalExitErrorV1, ScriptPhysicalExitOpenContractV1,
    ScriptPhysicalResultV1, ScriptRecipeLoweringOperationV1, ScriptSourceCompletionV1,
};
use crate::ast::{BinaryOperator, LiteralValue, Span};
use crate::mir::builder::raw_root_body_exit::RawOpenRootFunctionV1;
use crate::mir::builder::root_batch_slot::RawRootBatchSlotV1;
use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::builder::MirBuilder;
use crate::mir::builder::{canonical_normal_main_entry_target, OpenScriptPhysicalEntrySessionV1};
use crate::mir::raw_root_body_recipe::{
    RawLinearScalarExprV1, RawLinearScalarStmtV1, RawRootBodyEntryContractV1,
    RawRootBodySourceSiteV1, RawScriptBodyRecipeV1, RawScriptTerminalRecipeV1,
    RawScriptUnitOriginV1,
};
use crate::mir::{ConstValue, MirInstruction, MirType};

fn site(path: &[usize]) -> RawRootBodySourceSiteV1 {
    RawRootBodySourceSiteV1::new(path, Span::unknown())
}

fn integer(value: i64, path: &[usize]) -> RawLinearScalarExprV1 {
    RawLinearScalarExprV1::Literal {
        value: LiteralValue::Integer(value),
        site: site(path),
    }
}

fn lower(recipe: &RawScriptBodyRecipeV1) -> Result<LoweredScriptTerminalV1, String> {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("script_terminal_kernel/0".to_string());
    let result = {
        let scope = LexicalScopeGuard::new(&mut builder);
        let result = builder
            .lower_script_body_recipe_v1(recipe)
            .map_err(|error| error.to_string());
        drop(scope);
        result
    };
    builder.exit_function_for_test();
    result
}

fn lower_in_open_script_root(
    recipe: &RawScriptBodyRecipeV1,
) -> (MirBuilder, RawOpenRootFunctionV1, LoweredScriptTerminalV1) {
    let mut builder = MirBuilder::new();
    let open = builder
        .begin_raw_root_function_v1(
            RawRootBatchSlotV1::Main.contract(),
            RawRootBodyEntryContractV1::script(),
        )
        .expect("open Script root");
    let terminal = {
        let scope = LexicalScopeGuard::new(&mut builder);
        let terminal = builder
            .lower_script_body_recipe_v1(recipe)
            .expect("lower Script recipe");
        drop(scope);
        terminal
    };
    (builder, open, terminal)
}

#[test]
fn script_terminal_kernel_classifies_empty_value_and_explicit_void() {
    let empty =
        RawScriptBodyRecipeV1::from_parts(Box::new([]), RawScriptTerminalRecipeV1::EmptyUnit)
            .expect("empty Script recipe");
    assert!(matches!(
        lower(&empty).expect("empty lowering"),
        LoweredScriptTerminalV1::Unit {
            origin: RawScriptUnitOriginV1::EmptyBody,
            payload: LoweredScriptUnitPayloadV1::SyntheticVoid,
        }
    ));

    let value = RawScriptBodyRecipeV1::from_parts(
        Box::new([]),
        RawScriptTerminalRecipeV1::ValueExpression(integer(42, &[0])),
    )
    .expect("value Script recipe");
    assert!(matches!(
        lower(&value).expect("value lowering"),
        LoweredScriptTerminalV1::Value { .. }
    ));

    let explicit_void = RawScriptBodyRecipeV1::from_parts(
        Box::new([]),
        RawScriptTerminalRecipeV1::UnitExpression {
            expression: RawLinearScalarExprV1::Literal {
                value: LiteralValue::Null,
                site: site(&[2]),
            },
            origin: RawScriptUnitOriginV1::VoidExpression,
        },
    )
    .expect("explicit Unit Script recipe");
    assert!(matches!(
        lower(&explicit_void).expect("explicit Unit lowering"),
        LoweredScriptTerminalV1::Unit {
            origin: RawScriptUnitOriginV1::VoidExpression,
            payload: LoweredScriptUnitPayloadV1::ExistingVoid { .. },
        }
    ));
}

#[test]
fn script_terminal_kernel_preserves_unit_statement_origins() {
    let cases = [
        (
            RawLinearScalarStmtV1::Print {
                expression: integer(1, &[0, 0]),
                site: site(&[0]),
            },
            RawScriptUnitOriginV1::PrintStatement,
        ),
        (
            RawLinearScalarStmtV1::Local {
                variables: vec!["x".into()].into_boxed_slice(),
                initialized: vec![Some(integer(1, &[1, 0]))].into_boxed_slice(),
                site: site(&[1]),
            },
            RawScriptUnitOriginV1::LocalStatement,
        ),
    ];

    for (statement, origin) in cases {
        let recipe = RawScriptBodyRecipeV1::from_parts(
            Box::new([]),
            RawScriptTerminalRecipeV1::UnitStatement { statement, origin },
        )
        .expect("Unit statement Script recipe");
        assert!(matches!(
            lower(&recipe).expect("Unit statement lowering"),
            LoweredScriptTerminalV1::Unit {
                origin: actual_origin,
                payload: LoweredScriptUnitPayloadV1::SyntheticVoid,
            } if actual_origin == origin
        ));
    }

    for (statement, origin) in [
        (
            RawLinearScalarStmtV1::Assignment {
                target: "x".into(),
                value: integer(2, &[3, 0]),
                site: site(&[3]),
            },
            RawScriptUnitOriginV1::AssignmentStatement,
        ),
        (
            RawLinearScalarStmtV1::CompoundAssignment {
                target: "x".into(),
                operator: BinaryOperator::Add,
                value: integer(2, &[4, 0]),
                site: site(&[4]),
            },
            RawScriptUnitOriginV1::CompoundAssignmentStatement,
        ),
    ] {
        let recipe = RawScriptBodyRecipeV1::from_parts(
            vec![RawLinearScalarStmtV1::Local {
                variables: vec!["x".into()].into_boxed_slice(),
                initialized: vec![Some(integer(1, &[2, 0]))].into_boxed_slice(),
                site: site(&[2]),
            }]
            .into_boxed_slice(),
            RawScriptTerminalRecipeV1::UnitStatement { statement, origin },
        )
        .expect("bound Unit statement Script recipe");
        assert!(matches!(
            lower(&recipe).expect("bound Unit statement lowering"),
            LoweredScriptTerminalV1::Unit {
                origin: actual_origin,
                payload: LoweredScriptUnitPayloadV1::SyntheticVoid,
            } if actual_origin == origin
        ));
    }
}

#[test]
fn script_terminal_kernel_reports_exact_terminal_failure() {
    let recipe = RawScriptBodyRecipeV1::from_parts(
        Box::new([]),
        RawScriptTerminalRecipeV1::ValueExpression(RawLinearScalarExprV1::Variable {
            name: "missing".into(),
            site: site(&[7]),
        }),
    )
    .expect("failing Script recipe");

    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("script_terminal_failure/0".to_string());
    let scope = LexicalScopeGuard::new(&mut builder);
    let error = builder
        .lower_script_body_recipe_v1(&recipe)
        .expect_err("undefined terminal variable must fail");
    drop(scope);
    builder.exit_function_for_test();

    assert_eq!(
        error.operation(),
        ScriptRecipeLoweringOperationV1::TerminalValueExpression
    );
    assert_eq!(error.site().path(), &[7]);
    assert!(!error.detail().is_empty());
}

#[test]
fn script_physical_exit_kernel_commits_existing_scalar_return_once() {
    let recipe = RawScriptBodyRecipeV1::from_parts(
        Box::new([]),
        RawScriptTerminalRecipeV1::ValueExpression(integer(42, &[0])),
    )
    .expect("scalar Script recipe");
    let (mut builder, _open, terminal) = lower_in_open_script_root(&recipe);

    let prepared = PreparedScriptPhysicalExitCoreV1::prepare(
        &builder,
        terminal,
        ScriptPhysicalExitOpenContractV1::ProvisionalUnknown,
    )
    .expect("prepare scalar exit");
    let completed = ScriptPhysicalExitCommitV1::commit_projected(&mut builder, prepared);

    assert_eq!(completed.source(), ScriptSourceCompletionV1::Value);
    let ScriptPhysicalResultV1::ExistingOperand { value, ty } = completed.physical() else {
        panic!("scalar Script must preserve its exact lowered operand");
    };
    assert_eq!(*ty, MirType::Integer);
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .expect("open function after core commit");
    assert_eq!(function.signature.return_type, MirType::Integer);
    assert!(matches!(
        function.get_block(completed.block()).and_then(|block| block.terminator.as_ref()),
        Some(MirInstruction::Return { value: Some(actual) }) if actual == value
    ));
}

#[test]
fn script_physical_exit_kernel_materializes_exact_synthetic_unit() {
    let recipe =
        RawScriptBodyRecipeV1::from_parts(Box::new([]), RawScriptTerminalRecipeV1::EmptyUnit)
            .expect("empty Script recipe");
    let (mut builder, _open, terminal) = lower_in_open_script_root(&recipe);

    let prepared = PreparedScriptPhysicalExitCoreV1::prepare(
        &builder,
        terminal,
        ScriptPhysicalExitOpenContractV1::ProvisionalUnknown,
    )
    .expect("prepare Unit exit");
    let completed = ScriptPhysicalExitCommitV1::commit_projected(&mut builder, prepared);

    assert_eq!(
        completed.source(),
        ScriptSourceCompletionV1::Unit {
            origin: RawScriptUnitOriginV1::EmptyBody,
        }
    );
    let ScriptPhysicalResultV1::SyntheticVoid { value } = completed.physical() else {
        panic!("empty Script must use one synthetic Void");
    };
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .expect("open function after core commit");
    let block = function.get_block(completed.block()).expect("entry block");
    assert_eq!(function.signature.return_type, MirType::Void);
    assert!(block.instructions.iter().any(|instruction| {
        matches!(
            instruction,
            MirInstruction::Const {
                dst,
                value: ConstValue::Void,
            } if dst == value
        )
    }));
    assert!(matches!(
        block.terminator.as_ref(),
        Some(MirInstruction::Return { value: Some(actual) }) if actual == value
    ));
}

#[test]
fn script_physical_exit_kernel_rejects_void_value_before_commit() {
    let recipe = RawScriptBodyRecipeV1::from_parts(
        Box::new([]),
        RawScriptTerminalRecipeV1::ValueExpression(RawLinearScalarExprV1::Literal {
            value: LiteralValue::Null,
            site: site(&[0]),
        }),
    )
    .expect("drift fixture recipe");
    let (builder, _open, terminal) = lower_in_open_script_root(&recipe);

    assert!(matches!(
        PreparedScriptPhysicalExitCoreV1::prepare(
            &builder,
            terminal,
            ScriptPhysicalExitOpenContractV1::ProvisionalUnknown,
        ),
        Err(ScriptPhysicalExitErrorV1::ValueExpressionCannotBeVoid { .. })
    ));
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .expect("open function after rejected preparation");
    let block = function
        .get_block(builder.function_state.current_block.expect("current block"))
        .expect("entry block");
    assert!(block.terminator.is_none());
}

#[test]
fn normal_script_physical_session_opens_one_detached_unknown_main() {
    let live = MirBuilder::new();
    let session =
        OpenScriptPhysicalEntrySessionV1::open(&live, canonical_normal_main_entry_target())
            .expect("open detached Script session");
    let function = session
        .builder()
        .function_state
        .current_function
        .as_ref()
        .expect("candidate function");
    assert_eq!(function.signature.name, "main");
    assert_eq!(function.signature.return_type, MirType::Unknown);
    assert!(live.function_state.current_function.is_none());
    assert_eq!(session.entry_block(), builder_current_block(&session));
}

fn builder_current_block(session: &OpenScriptPhysicalEntrySessionV1) -> crate::mir::BasicBlockId {
    session
        .builder()
        .function_state
        .current_block
        .expect("candidate current block")
}
