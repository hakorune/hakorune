use std::collections::HashMap;

use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
use super::normal_source_plan::{
    NormalSourcePlanClassifierV1, PreparedNormalSourcePlanInputV1, SealedNormalScalarRootV1,
    SealedNormalSourcePlanV1, VerifiedNormalMainResolvedSourceUnitV1,
};
use crate::mir::loop_recipe_contract::{LoopNodeKeyV1, LoopRecipeV1};
use crate::mir::resolved_semantics::CallableSemanticSourceLedgerView;

use super::main0_in_body_step_recipe_coseal::{
    issue_main0_in_body_step_recipe_v1, Main0InBodyStepCoSealRejectV1,
    VerifiedMain0InBodyStepRecipeProductV1,
};
use super::main0_in_body_step_source_map::{
    Main0InBodyStepMapRoleV1, Main0InBodyStepSourceMapRejectV1,
    VerifiedMain0InBodyStepSourceMapV1,
};
use super::main0_in_body_step_source_map_issue::issue_main0_in_body_step_source_map_v1;
use super::main0_in_body_step_syntax_facts::{
    issue_main0_in_body_step_syntax_facts_from_ledger_v1, Main0InBodyStepSyntaxFactsRejectV1,
};

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn binary(operator: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn local(name: &str, initializer: ASTNode) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.into()],
        initial_values: vec![Some(Box::new(initializer))],
        declared_type_names: vec![None],
        span: Span::unknown(),
    }
}

fn assignment(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(value),
        span: Span::unknown(),
    }
}

fn step(name: &str, delta: ASTNode) -> ASTNode {
    assignment(name, binary(BinaryOperator::Add, variable(name), delta))
}

/// The bounded profile: `local i; local tmp; loop(i < <bound>) {
/// i = i + 1; tmp = <lit> } return i`.
fn profile_body() -> Vec<ASTNode> {
    vec![
        local("i", integer(0)),
        local("tmp", integer(0)),
        ASTNode::Loop {
            condition: Box::new(binary(BinaryOperator::Less, variable("i"), integer(3))),
            body: vec![step("i", integer(1)), assignment("tmp", integer(1))],
            span: Span::unknown(),
        },
        ASTNode::Return {
            value: Some(Box::new(variable("i"))),
            span: Span::unknown(),
        },
    ]
}

fn program(body: Vec<ASTNode>) -> ASTNode {
    let mut methods = HashMap::new();
    methods.insert(
        "main".to_owned(),
        ASTNode::FunctionDeclaration {
            name: "main".to_owned(),
            params: Vec::new(),
            param_decls: Vec::new(),
            return_type_name: None,
            body,
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: true,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        },
    );
    ASTNode::Program {
        statements: vec![ASTNode::BoxDeclaration {
            name: "Main".to_owned(),
            fields: Vec::new(),
            field_decls: Vec::new(),
            public_fields: Vec::new(),
            private_fields: Vec::new(),
            methods: crate::ast::BoxMethodInventoryV1::from_legacy_ast_map(methods),
            constructors: HashMap::new(),
            init_fields: Vec::new(),
            weak_fields: Vec::new(),
            delegates: Vec::new(),
            invariants: Vec::new(),
            transitions: Vec::new(),
            is_interface: false,
            is_sync: false,
            is_record: false,
            type_parameters: Vec::new(),
            extends: Vec::new(),
            implements: Vec::new(),
            is_static: true,
            static_init: None,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    }
}

fn resolved_main(body: Vec<ASTNode>) -> VerifiedNormalMainResolvedSourceUnitV1 {
    let input = PreparedNormalSourcePlanInputV1::new(program(body), "main0-in-body-step-test");
    let plan = NormalSourcePlanClassifierV1::seal(input).expect("valid Main0");
    let SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Main0(main)) = plan else {
        panic!("expected Main0");
    };
    main.prepare_function_source()
        .expect("exact Main source")
        .prepare_embedded_resolved_main()
        .expect("embedded Main resolution")
}

fn issue_map(
    unit: &VerifiedNormalMainResolvedSourceUnitV1,
) -> (
    CallableSemanticSourceLedgerView<'_>,
    VerifiedMain0InBodyStepSourceMapV1,
) {
    let input = unit.borrow_function_input().expect("resolved input");
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .expect("callable ledger");
    let facts = issue_main0_in_body_step_syntax_facts_from_ledger_v1(input, &ledger)
        .expect("syntax facts");
    let map = issue_main0_in_body_step_source_map_v1(&ledger, facts).expect("source map");
    (ledger, map)
}

fn issue_product(
    unit: &VerifiedNormalMainResolvedSourceUnitV1,
) -> (
    CallableSemanticSourceLedgerView<'_>,
    VerifiedMain0InBodyStepRecipeProductV1,
) {
    let (ledger, map) = issue_map(unit);
    let product = issue_main0_in_body_step_recipe_v1(&ledger, map).expect("co-seal");
    (ledger, product)
}

fn item_keys(recipe: &LoopRecipeV1, block: u32) -> Vec<u32> {
    recipe.blocks[block as usize]
        .items
        .iter()
        .map(|key| key.raw())
        .collect()
}

#[test]
fn issues_main0_in_body_step_semantic_program_once() {
    let unit = resolved_main(profile_body());
    let (_ledger, product) = issue_product(&unit);
    let recipe = product.operations().core().recipe().as_recipe();

    // One loop, two carriers (`i` and the write-only `tmp`), no declared
    // exits: the predicate's false edge is the only loop exit.
    assert_eq!(recipe.loops.len(), 1);
    assert_eq!(recipe.blocks.len(), 2);
    assert_eq!(recipe.bindings.len(), 2);
    assert_eq!(recipe.carriers.len(), 2);
    assert_eq!(recipe.inputs.len(), 2);
    assert_eq!(recipe.exits.len(), 0);

    // Condition block: read + bound const + compare. Body block: step read +
    // delta const + add + carrier write + effect const + effect write.
    assert_eq!(item_keys(recipe, 0), vec![0, 1, 2]);
    assert_eq!(item_keys(recipe, 1), vec![3, 4, 5, 6, 7, 8]);

    // Every operation item carries its co-recorded source evidence.
    assert_eq!(product.operations().evidence().len(), 9);

    // Initialized inputs: both locals enter the loop as carriers.
    assert_eq!(product.input().rows().len(), 2);

    // The continuation contract names the carrier binding's After port on
    // the root loop, derived from the verified Recipe rather than a token.
    assert_eq!(product.continuation().loop_key(), LoopNodeKeyV1::new(0));
    assert_eq!(product.continuation().after().binding().raw(), 0);

    // Source receipts: the write-only effect site and the tail return stay
    // source-bound and outside the Recipe.
    assert_eq!(product.control().owner(), product.tail().owner());
    assert_ne!(product.control().effect_site(), product.tail().statement());
}

#[test]
fn product_survives_source_unit_drop() {
    let product = {
        let unit = resolved_main(profile_body());
        let (_ledger, product) = issue_product(&unit);
        product
    };
    assert_eq!(product.operations().evidence().len(), 9);
}

#[test]
fn facts_reject_extra_loop_body_statement() {
    let mut body = profile_body();
    let ASTNode::Loop {
        body: loop_body, ..
    } = &mut body[2]
    else {
        unreachable!()
    };
    loop_body.push(step("i", integer(1)));
    let unit = resolved_main(body);
    let input = unit.borrow_function_input().expect("input");
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .expect("ledger");
    assert_eq!(
        issue_main0_in_body_step_syntax_facts_from_ledger_v1(input, &ledger),
        Err(Main0InBodyStepSyntaxFactsRejectV1::LoopBodyShape)
    );
}

#[test]
fn facts_reject_non_literal_condition_bound() {
    let mut body = profile_body();
    let ASTNode::Loop { condition, .. } = &mut body[2] else {
        unreachable!()
    };
    **condition = binary(BinaryOperator::Less, variable("i"), variable("tmp"));
    let unit = resolved_main(body);
    let input = unit.borrow_function_input().expect("input");
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .expect("ledger");
    assert_eq!(
        issue_main0_in_body_step_syntax_facts_from_ledger_v1(input, &ledger),
        Err(Main0InBodyStepSyntaxFactsRejectV1::ConditionBoundNotLiteral)
    );
}

#[test]
fn facts_reject_non_variable_condition_operand() {
    let mut body = profile_body();
    let ASTNode::Loop { condition, .. } = &mut body[2] else {
        unreachable!()
    };
    **condition = binary(BinaryOperator::Less, integer(0), integer(3));
    let unit = resolved_main(body);
    let input = unit.borrow_function_input().expect("input");
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .expect("ledger");
    assert_eq!(
        issue_main0_in_body_step_syntax_facts_from_ledger_v1(input, &ledger),
        Err(Main0InBodyStepSyntaxFactsRejectV1::ConditionOperandShape)
    );
}

#[test]
fn facts_reject_non_literal_effect_value() {
    let mut body = profile_body();
    let ASTNode::Loop {
        body: loop_body, ..
    } = &mut body[2]
    else {
        unreachable!()
    };
    loop_body[1] = assignment("tmp", variable("i"));
    let unit = resolved_main(body);
    let input = unit.borrow_function_input().expect("input");
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .expect("ledger");
    assert_eq!(
        issue_main0_in_body_step_syntax_facts_from_ledger_v1(input, &ledger),
        Err(Main0InBodyStepSyntaxFactsRejectV1::EffectValueNotLiteral)
    );
}

#[test]
fn facts_reject_extra_root_statement() {
    let mut body = profile_body();
    body.insert(3, local("extra", integer(9)));
    let unit = resolved_main(body);
    let input = unit.borrow_function_input().expect("input");
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .expect("ledger");
    assert_eq!(
        issue_main0_in_body_step_syntax_facts_from_ledger_v1(input, &ledger),
        Err(Main0InBodyStepSyntaxFactsRejectV1::RootBodyShape)
    );
}

#[test]
fn facts_reject_second_loop() {
    let mut body = profile_body();
    let ASTNode::Loop {
        body: loop_body, ..
    } = &mut body[2]
    else {
        unreachable!()
    };
    loop_body.push(ASTNode::Loop {
        condition: Box::new(integer(1)),
        body: Vec::new(),
        span: Span::unknown(),
    });
    let unit = resolved_main(body);
    let input = unit.borrow_function_input().expect("input");
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .expect("ledger");
    assert_eq!(
        issue_main0_in_body_step_syntax_facts_from_ledger_v1(input, &ledger),
        Err(Main0InBodyStepSyntaxFactsRejectV1::LoopCardinality)
    );
}

#[test]
fn facts_reject_literal_tail() {
    let mut body = profile_body();
    let ASTNode::Return { value, .. } = &mut body[3] else {
        unreachable!()
    };
    *value = Some(Box::new(integer(3)));
    let unit = resolved_main(body);
    let input = unit.borrow_function_input().expect("input");
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .expect("ledger");
    assert_eq!(
        issue_main0_in_body_step_syntax_facts_from_ledger_v1(input, &ledger),
        Err(Main0InBodyStepSyntaxFactsRejectV1::TailShape)
    );
}

#[test]
fn map_rejects_non_less_condition_operator() {
    let mut body = profile_body();
    let ASTNode::Loop { condition, .. } = &mut body[2] else {
        unreachable!()
    };
    **condition = binary(BinaryOperator::LessEqual, variable("i"), integer(3));
    let unit = resolved_main(body);
    let input = unit.borrow_function_input().expect("input");
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .expect("ledger");
    let facts = issue_main0_in_body_step_syntax_facts_from_ledger_v1(input, &ledger)
        .expect("syntax facts");
    assert_eq!(
        issue_main0_in_body_step_source_map_v1(&ledger, facts),
        Err(Main0InBodyStepSourceMapRejectV1::UnsupportedOperator(
            Main0InBodyStepMapRoleV1::ConditionOperator
        ))
    );
}

#[test]
fn map_rejects_tail_read_of_effect_local() {
    let mut body = profile_body();
    let ASTNode::Return { value, .. } = &mut body[3] else {
        unreachable!()
    };
    *value = Some(Box::new(variable("tmp")));
    let unit = resolved_main(body);
    let input = unit.borrow_function_input().expect("input");
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .expect("ledger");
    let facts = issue_main0_in_body_step_syntax_facts_from_ledger_v1(input, &ledger)
        .expect("syntax facts");
    assert_eq!(
        issue_main0_in_body_step_source_map_v1(&ledger, facts),
        Err(Main0InBodyStepSourceMapRejectV1::BindingMismatch(
            Main0InBodyStepMapRoleV1::TailReturnRead
        ))
    );
}

#[test]
fn map_rejects_effect_write_to_carrier() {
    // `i = 1` as the second body statement: the write-only rebind targets
    // the carrier itself, which the carrier/effect separation must refuse.
    let mut body = profile_body();
    let ASTNode::Loop {
        body: loop_body, ..
    } = &mut body[2]
    else {
        unreachable!()
    };
    loop_body[1] = assignment("i", integer(1));
    let unit = resolved_main(body);
    let input = unit.borrow_function_input().expect("input");
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .expect("ledger");
    let facts = issue_main0_in_body_step_syntax_facts_from_ledger_v1(input, &ledger)
        .expect("syntax facts");
    assert_eq!(
        issue_main0_in_body_step_source_map_v1(&ledger, facts),
        Err(Main0InBodyStepSourceMapRejectV1::CarrierIsEffect)
    );
}

#[test]
fn map_rejects_foreign_ledger() {
    let unit = resolved_main(profile_body());
    let other = resolved_main(profile_body());
    let input = unit.borrow_function_input().expect("input");
    let foreign_ledger = other
        .borrow_function_input()
        .expect("foreign input")
        .forest()
        .callable_source_ledger(other.borrow_function_input().unwrap().owner())
        .expect("foreign ledger");
    assert_eq!(
        issue_main0_in_body_step_syntax_facts_from_ledger_v1(input, &foreign_ledger),
        Err(Main0InBodyStepSyntaxFactsRejectV1::ForeignOwner)
    );
}

#[test]
fn co_seal_rejects_missing_map_row() {
    let unit = resolved_main(profile_body());
    let (ledger, map) = issue_map(&unit);
    let (
        owner,
        origin,
        source_kind,
        loop_source,
        frame,
        scope_region,
        effect_site,
        rows,
    ) = map.into_parts();
    let kept: Vec<_> = rows
        .into_vec()
        .into_iter()
        .filter(|row| row.role() != Main0InBodyStepMapRoleV1::EffectWrite)
        .collect();
    let map = VerifiedMain0InBodyStepSourceMapV1::rebuild_for_test(
        owner,
        origin,
        source_kind,
        loop_source,
        frame,
        scope_region,
        effect_site,
        kept,
    );
    match issue_main0_in_body_step_recipe_v1(&ledger, map) {
        Err(Main0InBodyStepCoSealRejectV1::MissingRole(
            Main0InBodyStepMapRoleV1::EffectWrite,
        )) => {}
        _ => panic!("effect-less source map must be rejected as missing the effect write row"),
    }
}
