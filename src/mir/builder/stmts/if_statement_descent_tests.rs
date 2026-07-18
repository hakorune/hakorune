use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::function::{
    FastMemBranchConditionProofKind, FastMemRegionMetadata, FastMemRegionOrigin,
};
use crate::mir::instruction::{FastMemRegionId, MemOpKind};
use crate::mir::{BasicBlockId, MirBuilder, MirInstruction, ValueId};

use super::super::recursive_child_lowering::{
    drive_raw_legacy_body_v1, drive_raw_legacy_expression_v1, drive_raw_legacy_statement_v1,
    RecursiveChildLoweringPortV1,
};
use super::if_statement_descent::{
    drive_if_statement_v1, IfStatementDescentPortV1, IfStatementSyntaxViewV1,
    RawLegacyIfStatementInputV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BranchKindV1 {
    Then,
    Else,
}

struct BranchBodyInputV1 {
    kind: BranchKindV1,
    statements: Vec<ASTNode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FailAtV1 {
    ConditionLower,
    ThenInput,
    ElseInput,
    ThenLower,
    ElseLower,
}

struct RecordingIfPortV1 {
    events: RefCell<Vec<&'static str>>,
    fail_at: Option<FailAtV1>,
    condition_value: Option<ValueId>,
}

impl RecordingIfPortV1 {
    fn new(fail_at: Option<FailAtV1>) -> Self {
        Self {
            events: RefCell::new(Vec::new()),
            fail_at,
            condition_value: None,
        }
    }

    fn with_condition_value(mut self, value: ValueId) -> Self {
        self.condition_value = Some(value);
        self
    }

    fn record(&self, event: &'static str) {
        self.events.borrow_mut().push(event);
    }

    fn events(&self) -> Vec<&'static str> {
        self.events.borrow().clone()
    }

    fn fail_if(&self, stage: FailAtV1) -> Result<(), String> {
        if self.fail_at == Some(stage) {
            Err(format!("[if-statement-descent-test/{stage:?}]"))
        } else {
            Ok(())
        }
    }
}

impl RecursiveChildLoweringPortV1 for RecordingIfPortV1 {
    type BodyInput = BranchBodyInputV1;
    type StatementInput = ASTNode;
    type ExpressionInput = ASTNode;

    fn lower_body(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::BodyInput,
    ) -> Result<ValueId, String> {
        match input.kind {
            BranchKindV1::Then => {
                self.record("then-lower");
                self.fail_if(FailAtV1::ThenLower)?;
            }
            BranchKindV1::Else => {
                self.record("else-lower");
                self.fail_if(FailAtV1::ElseLower)?;
            }
        }
        drive_raw_legacy_body_v1(builder, input.statements)
    }

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::StatementInput,
    ) -> Result<ValueId, String> {
        drive_raw_legacy_statement_v1(builder, input)
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::ExpressionInput,
    ) -> Result<ValueId, String> {
        self.record("condition-lower");
        self.fail_if(FailAtV1::ConditionLower)?;
        match self.condition_value {
            Some(value) => Ok(value),
            None => drive_raw_legacy_expression_v1(builder, input),
        }
    }
}

impl IfStatementDescentPortV1 for RecordingIfPortV1 {
    type IfInput = RawLegacyIfStatementInputV1;

    fn if_syntax<'input>(
        &self,
        input: &'input Self::IfInput,
    ) -> Result<IfStatementSyntaxViewV1<'input>, String> {
        self.record("syntax");
        let raw = super::if_statement_descent::raw_syntax_for_tests(input);
        Ok(raw)
    }

    fn if_condition_expression_input(
        &self,
        input: &Self::IfInput,
    ) -> Result<Self::ExpressionInput, String> {
        self.record("condition-input");
        Ok(super::if_statement_descent::raw_condition_for_tests(input))
    }

    fn if_then_body_input(&self, input: &Self::IfInput) -> Result<Self::BodyInput, String> {
        self.record("then-input");
        self.fail_if(FailAtV1::ThenInput)?;
        Ok(BranchBodyInputV1 {
            kind: BranchKindV1::Then,
            statements: super::if_statement_descent::raw_then_body_for_tests(input),
        })
    }

    fn if_else_body_input(&self, input: &Self::IfInput) -> Result<Self::BodyInput, String> {
        self.record("else-input");
        self.fail_if(FailAtV1::ElseInput)?;
        Ok(BranchBodyInputV1 {
            kind: BranchKindV1::Else,
            statements: super::if_statement_descent::raw_else_body_for_tests(input)
                .ok_or_else(|| "[if-statement-descent-test/else-missing]".to_string())?,
        })
    }
}

fn bool_lit(value: bool) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(value),
        span: Span::unknown(),
    }
}

fn int_lit(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn return_int(value: i64) -> ASTNode {
    ASTNode::Return {
        value: Some(Box::new(int_lit(value))),
        span: Span::unknown(),
    }
}

fn assignment(name: &str, value: i64) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }),
        value: Box::new(int_lit(value)),
        span: Span::unknown(),
    }
}

fn local(name: &str, value: i64) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.to_string()],
        initial_values: vec![Some(Box::new(int_lit(value)))],
        declared_type_names: vec![None],
        span: Span::unknown(),
    }
}

fn if_input(
    then_body: Vec<ASTNode>,
    else_body: Option<Vec<ASTNode>>,
) -> RawLegacyIfStatementInputV1 {
    RawLegacyIfStatementInputV1::new(bool_lit(true), then_body, else_body)
}

fn builder(name: &str) -> MirBuilder {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test(name.to_string());
    builder
}

fn terminators(builder: &MirBuilder) -> Vec<MirInstruction> {
    builder
        .scope_ctx
        .current_function
        .as_ref()
        .expect("If driver function")
        .blocks
        .values()
        .filter_map(|block| block.terminator.clone())
        .collect()
}

fn cfg_snapshot(builder: &MirBuilder) -> Vec<String> {
    let mut rows: Vec<_> = builder
        .scope_ctx
        .current_function
        .as_ref()
        .expect("If driver function")
        .blocks
        .iter()
        .map(|(id, block)| {
            format!(
                "{id:?}|{:?}|{:?}|{:?}",
                block.instructions, block.terminator, block.predecessors
            )
        })
        .collect();
    rows.sort();
    rows
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IfCfgShapeSnapshotV1 {
    current_block: Option<BasicBlockId>,
    blocks: Vec<(BasicBlockId, String, BTreeSet<BasicBlockId>)>,
}

fn cfg_shape_snapshot(builder: &MirBuilder) -> IfCfgShapeSnapshotV1 {
    let function = builder
        .scope_ctx
        .current_function
        .as_ref()
        .expect("If driver function");
    IfCfgShapeSnapshotV1 {
        current_block: builder.current_block,
        blocks: {
            let mut blocks: Vec<_> = function
                .blocks
                .iter()
                .map(|(id, block)| {
                    (
                        *id,
                        format!("{:?}", block.terminator),
                        block.predecessors.clone(),
                    )
                })
                .collect();
            blocks.sort_by_key(|(id, _, _)| *id);
            blocks
        },
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IfPartialStateSnapshotV1 {
    cfg: Vec<String>,
    current_block: Option<BasicBlockId>,
    variables: BTreeMap<String, ValueId>,
    lexical_scope_depth: usize,
    if_merge_stack: Vec<BasicBlockId>,
    debug_scope_stack: Vec<String>,
}

fn partial_state_snapshot(builder: &MirBuilder) -> IfPartialStateSnapshotV1 {
    IfPartialStateSnapshotV1 {
        cfg: cfg_snapshot(builder),
        current_block: builder.current_block,
        variables: builder.variable_ctx.variable_map.clone(),
        lexical_scope_depth: builder.scope_ctx.lexical_scope_stack.len(),
        if_merge_stack: builder.scope_ctx.if_merge_stack.clone(),
        debug_scope_stack: builder.scope_ctx.debug_scope_stack.clone(),
    }
}

fn assert_simple_if_termination_shape(
    builder: &MirBuilder,
    then_returns: bool,
    else_returns: bool,
) -> (BasicBlockId, BasicBlockId, BasicBlockId) {
    let function = builder.scope_ctx.current_function.as_ref().unwrap();
    let (then_bb, else_bb) = function
        .blocks
        .values()
        .find_map(|block| match block.terminator.as_ref() {
            Some(MirInstruction::Branch {
                then_bb, else_bb, ..
            }) => Some((*then_bb, *else_bb)),
            _ => None,
        })
        .expect("one If header Branch");
    let merge_bb = builder.current_block.expect("If merge current block");

    for (label, block_id, returns) in [
        ("then", then_bb, then_returns),
        ("else", else_bb, else_returns),
    ] {
        let block = function.blocks.get(&block_id).expect("If branch block");
        if returns {
            assert!(
                matches!(block.terminator, Some(MirInstruction::Return { .. })),
                "{label}: {:?}",
                block.terminator
            );
        } else {
            assert!(
                matches!(block.terminator, Some(MirInstruction::Jump { target, .. }) if target == merge_bb),
                "{label}: {:?}",
                block.terminator
            );
        }
    }

    // Existing IfForm publishes merge predecessors while constructing a
    // two-input result/variable PHI. A one-reachable-branch merge carries its
    // edge solely in the Jump terminator and deliberately has no PHI rows.
    let expected_predecessors = if !then_returns && !else_returns {
        BTreeSet::from([then_bb, else_bb])
    } else {
        BTreeSet::new()
    };
    assert_eq!(
        function.blocks[&merge_bb].predecessors,
        expected_predecessors,
        "merge={merge_bb:?} cfg={:?}",
        cfg_snapshot(builder)
    );
    (then_bb, else_bb, merge_bb)
}

#[test]
fn if_driver_demands_condition_then_and_else_in_exact_order() {
    let input = if_input(vec![int_lit(1)], Some(vec![int_lit(2)]));
    let mut port = RecordingIfPortV1::new(None);
    let mut builder = builder("if_driver_order/0");
    let _scope = LexicalScopeGuard::new(&mut builder);

    drive_if_statement_v1(&mut builder, &mut port, &input).unwrap();

    assert_eq!(
        port.events(),
        vec![
            "syntax",
            "condition-input",
            "condition-lower",
            "then-input",
            "then-lower",
            "else-input",
            "else-lower",
        ]
    );
    assert!(terminators(&builder)
        .iter()
        .any(|term| matches!(term, MirInstruction::Branch { .. })));
    assert_eq!(builder.recursion_depth, 0);
}

#[test]
fn if_driver_condition_failure_precedes_ifform_effects() {
    let input = if_input(vec![int_lit(1)], Some(vec![int_lit(2)]));
    let mut port = RecordingIfPortV1::new(Some(FailAtV1::ConditionLower));
    let mut builder = builder("if_driver_condition_failure/0");
    let _scope = LexicalScopeGuard::new(&mut builder);
    let before = cfg_shape_snapshot(&builder);

    let error = drive_if_statement_v1(&mut builder, &mut port, &input).unwrap_err();

    assert!(error.contains("ConditionLower"), "{error}");
    assert_eq!(
        port.events(),
        vec!["syntax", "condition-input", "condition-lower"]
    );
    assert_eq!(cfg_shape_snapshot(&builder), before);
    assert_eq!(builder.recursion_depth, 0);
}

#[test]
fn if_driver_fastmem_failure_requests_no_branch_or_cfg() {
    let input = if_input(vec![int_lit(1)], Some(vec![int_lit(2)]));
    let mut port = RecordingIfPortV1::new(None);
    let mut builder = builder("if_driver_fastmem_failure/0");
    let _scope = LexicalScopeGuard::new(&mut builder);
    builder.push_fastmem_region(FastMemRegionId(17));

    let error = drive_if_statement_v1(&mut builder, &mut port, &input).unwrap_err();

    assert!(error.contains("fastmem/branch_cfg_requires_owner_eq_condition"));
    assert_eq!(
        port.events(),
        vec!["syntax", "condition-input", "condition-lower"]
    );
    assert!(terminators(&builder).is_empty());
    assert!(builder
        .scope_ctx
        .current_function
        .as_ref()
        .unwrap()
        .metadata
        .fastmem_branch_condition_facts
        .is_empty());
    assert_eq!(builder.recursion_depth, 0);
}

#[test]
fn if_driver_fastmem_success_publishes_one_existing_condition_fact() {
    let input = if_input(vec![int_lit(1)], Some(vec![int_lit(2)]));
    let mut builder = builder("if_driver_fastmem_success/0");
    let _scope = LexicalScopeGuard::new(&mut builder);
    let region = FastMemRegionId(0);
    builder
        .scope_ctx
        .current_function
        .as_mut()
        .unwrap()
        .metadata
        .fastmem_regions
        .push(FastMemRegionMetadata {
            id: region,
            contract: "IfDriverFastMemV1".to_string(),
            source_span: Span::unknown(),
            origin: FastMemRegionOrigin::SourceFastMemBlock,
            body_statement_count: 1,
            emitted_memop_count: 0,
        });
    builder.push_fastmem_region(region);
    let lhs = crate::mir::builder::emission::constant::emit_integer(&mut builder, 1).unwrap();
    let rhs = crate::mir::builder::emission::constant::emit_integer(&mut builder, 1).unwrap();
    let condition_value = builder
        .emit_fastmem_value_memop(region, MemOpKind::OwnerEq, vec![lhs, rhs])
        .unwrap();
    let mut port = RecordingIfPortV1::new(None).with_condition_value(condition_value);

    drive_if_statement_v1(&mut builder, &mut port, &input).unwrap();

    assert_eq!(
        port.events(),
        vec![
            "syntax",
            "condition-input",
            "condition-lower",
            "then-input",
            "then-lower",
            "else-input",
            "else-lower",
        ]
    );
    let facts = &builder
        .scope_ctx
        .current_function
        .as_ref()
        .unwrap()
        .metadata
        .fastmem_branch_condition_facts;
    assert_eq!(facts.len(), 1);
    assert_eq!(facts[0].region, region);
    assert_eq!(facts[0].condition_value, condition_value);
    assert_eq!(
        facts[0].proof_kind,
        FastMemBranchConditionProofKind::SourceAssumeOwnerEq
    );
    assert!(facts[0].owner_eq_required);
}

#[test]
fn if_driver_branch_failures_preserve_exact_demand_boundary() {
    fn failed_state(failure: FailAtV1) -> (Vec<&'static str>, IfPartialStateSnapshotV1) {
        let input = if_input(vec![int_lit(1)], Some(vec![int_lit(2)]));
        let mut port = RecordingIfPortV1::new(Some(failure));
        let mut builder = builder(&format!("if_driver_branch_failure_{failure:?}/0"));
        let _scope = LexicalScopeGuard::new(&mut builder);

        let error = drive_if_statement_v1(&mut builder, &mut port, &input).unwrap_err();

        assert!(error.contains(&format!("{failure:?}")), "{error}");
        assert_eq!(builder.recursion_depth, 0);
        (port.events(), partial_state_snapshot(&builder))
    }

    let (then_input_events, then_input_state) = failed_state(FailAtV1::ThenInput);
    let (then_lower_events, then_lower_state) = failed_state(FailAtV1::ThenLower);
    assert_eq!(
        then_input_events,
        vec!["syntax", "condition-input", "condition-lower", "then-input"]
    );
    assert_eq!(
        then_lower_events,
        vec![
            "syntax",
            "condition-input",
            "condition-lower",
            "then-input",
            "then-lower",
        ]
    );
    assert_eq!(then_input_state, then_lower_state);
    assert_eq!(then_input_state.cfg.len(), 2);
    assert!(then_input_state
        .debug_scope_stack
        .last()
        .is_some_and(|region| region.ends_with("/then")));
    assert!(then_input_state.if_merge_stack.is_empty());

    let (else_input_events, else_input_state) = failed_state(FailAtV1::ElseInput);
    let (else_lower_events, else_lower_state) = failed_state(FailAtV1::ElseLower);
    assert_eq!(
        else_input_events,
        vec![
            "syntax",
            "condition-input",
            "condition-lower",
            "then-input",
            "then-lower",
            "else-input",
        ]
    );
    assert_eq!(
        else_lower_events,
        vec![
            "syntax",
            "condition-input",
            "condition-lower",
            "then-input",
            "then-lower",
            "else-input",
            "else-lower",
        ]
    );
    assert_eq!(else_input_state, else_lower_state);
    assert_eq!(else_input_state.cfg.len(), 3);
    assert!(else_input_state
        .debug_scope_stack
        .last()
        .is_some_and(|region| region.ends_with("/else")));
    assert!(else_input_state.if_merge_stack.is_empty());
    assert_ne!(
        then_input_state.current_block,
        else_input_state.current_block
    );

    let input = if_input(vec![int_lit(1)], Some(vec![int_lit(2)]));
    let mut fresh_port = RecordingIfPortV1::new(None);
    let mut fresh_builder = builder("if_driver_fresh_after_failure/0");
    let _fresh_scope = LexicalScopeGuard::new(&mut fresh_builder);
    drive_if_statement_v1(&mut fresh_builder, &mut fresh_port, &input).unwrap();
    assert!(terminators(&fresh_builder)
        .iter()
        .any(|term| matches!(term, MirInstruction::Branch { .. })));
}

#[test]
fn if_driver_implicit_false_never_requests_else() {
    let input = if_input(vec![int_lit(1)], None);
    let mut port = RecordingIfPortV1::new(None);
    let mut builder = builder("if_driver_implicit_false/0");
    let _scope = LexicalScopeGuard::new(&mut builder);

    drive_if_statement_v1(&mut builder, &mut port, &input).unwrap();

    assert_eq!(
        port.events(),
        vec![
            "syntax",
            "condition-input",
            "condition-lower",
            "then-input",
            "then-lower",
        ]
    );
    let (then_bb, else_bb, merge_bb) = assert_simple_if_termination_shape(&builder, false, false);
    let function = builder.scope_ctx.current_function.as_ref().unwrap();
    assert!(function.blocks[&then_bb].successors.contains(&merge_bb));
    assert!(function.blocks[&else_bb].successors.contains(&merge_bb));
}

#[test]
fn if_driver_preserves_termination_and_variable_phi_shapes() {
    for (label, then_body, else_body, then_returns, else_returns) in [
        (
            "both_fallthrough",
            vec![int_lit(1)],
            vec![int_lit(2)],
            false,
            false,
        ),
        (
            "then_return",
            vec![return_int(1)],
            vec![int_lit(2)],
            true,
            false,
        ),
        (
            "else_return",
            vec![int_lit(1)],
            vec![return_int(2)],
            false,
            true,
        ),
        (
            "both_return",
            vec![return_int(1)],
            vec![return_int(2)],
            true,
            true,
        ),
    ] {
        let input = if_input(then_body, Some(else_body));
        let mut port = RecordingIfPortV1::new(None);
        let mut builder = builder(&format!("if_driver_termination_{label}/0"));
        let _scope = LexicalScopeGuard::new(&mut builder);

        drive_if_statement_v1(&mut builder, &mut port, &input).unwrap();

        let (then_bb, else_bb, merge_bb) =
            assert_simple_if_termination_shape(&builder, then_returns, else_returns);
        assert_eq!(
            terminators(&builder)
                .iter()
                .filter(|term| matches!(term, MirInstruction::Return { .. }))
                .count(),
            usize::from(then_returns) + usize::from(else_returns),
            "{label}: {:?}",
            cfg_snapshot(&builder)
        );
        if label == "both_fallthrough" {
            let function = builder.scope_ctx.current_function.as_ref().unwrap();
            assert!(function.blocks[&merge_bb].instructions.iter().any(
                |instruction| matches!(instruction, MirInstruction::Phi { inputs, .. }
                    if inputs.iter().map(|(pred, _)| *pred).collect::<BTreeSet<_>>()
                        == BTreeSet::from([then_bb, else_bb]))
            ));
        }
    }

    let mut builder = builder("if_driver_variable_phi/0");
    let _scope = LexicalScopeGuard::new(&mut builder);
    builder.build_statement(local("x", 0)).unwrap();
    let input = if_input(vec![assignment("x", 1)], Some(vec![assignment("x", 2)]));
    let mut port = RecordingIfPortV1::new(None);
    drive_if_statement_v1(&mut builder, &mut port, &input).unwrap();
    let x_value = builder.variable_ctx.variable_map["x"];
    let merge_bb = builder.current_block.unwrap();
    let function = builder.scope_ctx.current_function.as_ref().unwrap();
    let merge = &function.blocks[&merge_bb];
    let x_phi_inputs = merge
        .instructions
        .iter()
        .find_map(|instruction| match instruction {
            MirInstruction::Phi { dst, inputs, .. } if *dst == x_value => Some(inputs),
            _ => None,
        });
    let x_phi_inputs = x_phi_inputs.expect("merged x must be the merge-block Phi");
    assert_eq!(x_phi_inputs.len(), 2);
    assert_eq!(
        x_phi_inputs
            .iter()
            .map(|(predecessor, _)| *predecessor)
            .collect::<BTreeSet<_>>(),
        merge.predecessors
    );
}
