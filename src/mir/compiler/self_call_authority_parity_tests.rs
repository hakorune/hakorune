#![cfg(feature = "vm-reference")]

//! R0-P0 test-only normalized parity between the retiring bare-function
//! self-call authority and the disconnected singleton Program authority.

use std::collections::BTreeMap;

use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, ParamDecl, Span};
use crate::backend::{MirInterpreter, VMValue};
use crate::mir::resolved_value_profile::product::{
    TrivialProfileCoverageSubjectV1, TrivialTerminalProfileV1,
};
use crate::mir::{BasicBlockId, EdgeArgs, MirFunction, MirInstruction, ValueId};

use super::capability::{CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1};
use super::recursive_callable_module_plan::VerifiedRecursiveCallableModulePlanV1;
use super::{
    CanonicalFinishScheduleV1, CanonicalLoweringErrorV1, CanonicalModuleLoweringSessionV1,
    MirCompileResult, MirCompiler, VerifiedResolvedCallableProgramV1, VerifiedResolvedSourceUnitV1,
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

fn call(name: &str, argument: ASTNode) -> ASTNode {
    ASTNode::FunctionCall {
        name: name.into(),
        arguments: vec![argument],
        span: Span::unknown(),
    }
}

fn local(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.into()],
        initial_values: vec![Some(Box::new(value))],
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

fn return_(value: ASTNode) -> ASTNode {
    ASTNode::Return {
        value: Some(Box::new(value)),
        span: Span::unknown(),
    }
}

fn function(name: &str, body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.into(),
        params: vec!["n".into()],
        param_decls: vec![ParamDecl {
            name: "n".into(),
            declared_type_name: Some("i64".into()),
        }],
        return_type_name: Some("i64".into()),
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn program(function: ASTNode) -> ASTNode {
    ASTNode::Program {
        statements: vec![function],
        span: Span::unknown(),
    }
}

fn countdown(name: &str) -> ASTNode {
    function(
        name,
        vec![
            local("result", variable("n")),
            ASTNode::If {
                condition: Box::new(binary(BinaryOperator::Greater, variable("n"), integer(0))),
                then_body: vec![assignment(
                    "result",
                    call(
                        name,
                        binary(BinaryOperator::Subtract, variable("n"), integer(1)),
                    ),
                )],
                else_body: None,
                span: Span::unknown(),
            },
            return_(variable("result")),
        ],
    )
}

fn countdown_phi(name: &str) -> ASTNode {
    function(
        name,
        vec![
            local("result", variable("n")),
            ASTNode::If {
                condition: Box::new(binary(BinaryOperator::Greater, variable("n"), integer(0))),
                then_body: vec![
                    local("next", variable("n")),
                    ASTNode::If {
                        condition: Box::new(binary(
                            BinaryOperator::Greater,
                            variable("n"),
                            integer(1),
                        )),
                        then_body: vec![assignment(
                            "next",
                            binary(BinaryOperator::Subtract, variable("n"), integer(1)),
                        )],
                        else_body: Some(vec![assignment("next", integer(0))]),
                        span: Span::unknown(),
                    },
                    local("recursive", call(name, variable("next"))),
                    assignment("result", variable("recursive")),
                ],
                else_body: None,
                span: Span::unknown(),
            },
            return_(variable("result")),
        ],
    )
}

impl MirCompiler {
    fn compile_singleton_program_for_r0_p0(
        &mut self,
        source: &VerifiedResolvedCallableProgramV1,
    ) -> Result<MirCompileResult, CanonicalLoweringErrorV1> {
        let input = source.lowering_input();
        let plan = VerifiedRecursiveCallableModulePlanV1::verify_one_or_more_for_r0(
            input.program().module(),
        )
        .map_err(|error| super::callable_program_stage_error("r0_p0_parity", error))?;
        let mut session = CanonicalModuleLoweringSessionV1::open(&self.builder);
        let candidate = session
            .builder_mut()
            .build_recursive_callable_module_candidate(plan)
            .map_err(|error| super::callable_program_stage_error("r0_p0_transaction", error))?;
        let result = self.finish_built_canonical_module(
            candidate,
            CanonicalFinishScheduleV1::TrivialBindingSsa,
        )?;
        session.commit(&mut self.builder);
        Ok(result)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct AuthoritySnapshot {
    header: String,
    semantic_shape: String,
    parameter_rows: Vec<String>,
    call_rows: Vec<String>,
    definition_representations: Vec<String>,
    merge_representations: Vec<String>,
    terminal: String,
    coverage_order: Vec<&'static str>,
}

fn authority_snapshot(
    input: super::function_input::ResolvedFunctionLoweringInputV1<'_>,
    plan: super::capability::CanonicalTrivialBindingSsaPlanV1<'_>,
) -> AuthoritySnapshot {
    let header = input.callable_header().expect("call-enabled input");
    let graph = input.function().normalized_graph();
    let semantic_shape = format!(
        "bindings={:?};scopes={:?};regions={:?};decls={};uses={};assigns={};exits={}",
        graph
            .bindings()
            .iter()
            .map(|row| row.kind)
            .collect::<Vec<_>>(),
        graph
            .scopes()
            .iter()
            .map(|row| row.key.kind)
            .collect::<Vec<_>>(),
        graph
            .regions()
            .iter()
            .map(|row| row.key.kind)
            .collect::<Vec<_>>(),
        graph.declarations().len(),
        graph.variable_uses().len(),
        graph.assignments().len(),
        graph.exits().len(),
    );
    let (_, _, _, profile, _) = plan.into_parts();
    AuthoritySnapshot {
        header: format!(
            "{:?}:{}:{}:{:?}->{:?}:{}",
            header.source_key().namespace(),
            header.source_key().name(),
            header.source_key().arity(),
            header.signature().params(),
            header.signature().result(),
            header.symbol().as_mir_name(),
        ),
        semantic_shape,
        parameter_rows: profile
            .parameter_entries()
            .iter()
            .map(|row| {
                format!(
                    "{}:{}:{:?}:{:?}",
                    row.source_name(),
                    row.declared_type_name(),
                    row.abi(),
                    row.representation()
                )
            })
            .collect(),
        call_rows: profile
            .direct_calls()
            .iter()
            .map(|row| {
                format!(
                    "{}:{:?}->{:?}:args={}:result={:?}:effect={:?}",
                    row.target().symbol().as_mir_name(),
                    row.target().signature().params(),
                    row.target().signature().result(),
                    row.arguments().len(),
                    row.result(),
                    row.effect()
                )
            })
            .collect(),
        definition_representations: profile
            .definitions()
            .iter()
            .map(|row| format!("{:?}", row.representation()))
            .collect(),
        merge_representations: profile
            .merge_profiles()
            .iter()
            .map(|row| format!("{:?}", row.representation()))
            .collect(),
        terminal: match profile.terminal() {
            TrivialTerminalProfileV1::ExplicitValue { representation, .. } => {
                format!(
                    "explicit:{representation:?}:{:?}",
                    profile.function_return().map(|r| r.abi())
                )
            }
            other => format!("{other:?}"),
        },
        coverage_order: profile
            .coverage()
            .ordered_subjects()
            .iter()
            .map(|subject| match subject {
                TrivialProfileCoverageSubjectV1::Value(_) => "value",
                TrivialProfileCoverageSubjectV1::DirectCall(_) => "call",
                TrivialProfileCoverageSubjectV1::Definition { .. } => "definition",
                TrivialProfileCoverageSubjectV1::IfMergeProfile { .. } => "merge",
                TrivialProfileCoverageSubjectV1::ExplicitValueTerminal(_) => "return-value",
                TrivialProfileCoverageSubjectV1::ExplicitNoValueTerminal(_) => "return-void",
                TrivialProfileCoverageSubjectV1::ImplicitNoValueTerminal { .. } => "fallthrough",
            })
            .collect(),
    }
}

fn old_authority(root: ASTNode) -> (VerifiedResolvedSourceUnitV1, AuthoritySnapshot) {
    let source = VerifiedResolvedSourceUnitV1::resolve_function_with_root_callable(root).unwrap();
    let input = source.root_function_input().unwrap();
    let CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) =
        CanonicalLoweringPreflightV1::verify(&source).unwrap()
    else {
        panic!("old self-call route selected non-trivial plan")
    };
    let snapshot = authority_snapshot(input, plan);
    (source, snapshot)
}

fn new_authority(root: ASTNode) -> (VerifiedResolvedCallableProgramV1, AuthoritySnapshot) {
    let source = VerifiedResolvedCallableProgramV1::resolve(program(root)).unwrap();
    let plan =
        VerifiedRecursiveCallableModulePlanV1::verify_one_or_more_for_r0(source.module()).unwrap();
    let (_, _, mut plans) = plan.into_parts();
    let (key, plan) = plans.pop_first().expect("singleton plan");
    assert!(plans.is_empty());
    let snapshot = authority_snapshot(source.module().function_input(&key).unwrap(), plan);
    (source, snapshot)
}

#[derive(Debug, PartialEq, Eq)]
struct MirSnapshot {
    signature: String,
    blocks: Vec<String>,
    declared_parameters: String,
    parameter_contracts: String,
    declared_return: Option<String>,
    return_contract: String,
    direct_call_capabilities: usize,
    ownership_operations: usize,
}

fn normalized_edge_args(args: &Option<EdgeArgs>, values: &BTreeMap<ValueId, usize>) -> String {
    match args {
        None => "-".into(),
        Some(args) => format!(
            "{:?}:{:?}",
            args.layout,
            args.values
                .iter()
                .map(|value| values[value])
                .collect::<Vec<_>>()
        ),
    }
}

fn instruction_dst(instruction: &MirInstruction) -> Option<ValueId> {
    match instruction {
        MirInstruction::Const { dst, .. }
        | MirInstruction::BinOp { dst, .. }
        | MirInstruction::Compare { dst, .. }
        | MirInstruction::Copy { dst, .. }
        | MirInstruction::Phi { dst, .. } => Some(*dst),
        MirInstruction::Call { dst, .. } => *dst,
        _ => None,
    }
}

fn normalized_instruction(
    instruction: &MirInstruction,
    blocks: &BTreeMap<BasicBlockId, usize>,
    values: &BTreeMap<ValueId, usize>,
) -> String {
    let value = |id: &ValueId| values[id];
    match instruction {
        MirInstruction::Const {
            dst,
            value: constant,
        } => {
            format!("v{}=const:{constant:?}", value(dst))
        }
        MirInstruction::BinOp { dst, op, lhs, rhs } => {
            format!("v{}=bin:{op:?}:v{}:v{}", value(dst), value(lhs), value(rhs))
        }
        MirInstruction::Compare { dst, op, lhs, rhs } => {
            format!("v{}=cmp:{op:?}:v{}:v{}", value(dst), value(lhs), value(rhs))
        }
        MirInstruction::Copy { dst, src } => format!("v{}=copy:v{}", value(dst), value(src)),
        MirInstruction::Phi {
            dst,
            inputs,
            type_hint,
        } => {
            let mut normalized = inputs
                .iter()
                .map(|(block, input)| (blocks[block], value(input)))
                .collect::<Vec<_>>();
            normalized.sort_unstable();
            format!("v{}=phi:{normalized:?}:{type_hint:?}", value(dst))
        }
        MirInstruction::Call {
            dst,
            func,
            callee,
            args,
            effects,
        } => format!(
            "v{}=call:func={}:callee={callee:?}:args={:?}:effects={effects:?}",
            value(dst.as_ref().expect("exact call result")),
            if *func == ValueId::INVALID {
                "invalid".into()
            } else {
                format!("v{}", value(func))
            },
            args.iter().map(value).collect::<Vec<_>>(),
        ),
        MirInstruction::Branch {
            condition,
            then_bb,
            else_bb,
            then_edge_args,
            else_edge_args,
        } => format!(
            "branch:v{}:b{}:{}:b{}:{}",
            value(condition),
            blocks[then_bb],
            normalized_edge_args(then_edge_args, values),
            blocks[else_bb],
            normalized_edge_args(else_edge_args, values),
        ),
        MirInstruction::Jump { target, edge_args } => format!(
            "jump:b{}:{}",
            blocks[target],
            normalized_edge_args(edge_args, values)
        ),
        MirInstruction::Return { value: result } => {
            format!("return:{:?}", result.map(|id| value(&id)))
        }
        other => panic!("unexpected P0 exact-i64 MIR instruction: {other:?}"),
    }
}

fn mir_snapshot(function: &MirFunction) -> MirSnapshot {
    let block_ids = {
        let mut ids = function.blocks.keys().copied().collect::<Vec<_>>();
        ids.sort_unstable();
        ids
    };
    let blocks = block_ids
        .iter()
        .enumerate()
        .map(|(index, id)| (*id, index))
        .collect::<BTreeMap<_, _>>();
    let mut values = function
        .params
        .iter()
        .enumerate()
        .map(|(index, value)| (*value, index))
        .collect::<BTreeMap<_, _>>();
    for block_id in &block_ids {
        let block = &function.blocks[block_id];
        for instruction in &block.instructions {
            if let Some(dst) = instruction_dst(instruction) {
                let next = values.len();
                values.entry(dst).or_insert(next);
            }
        }
    }
    let normalized_blocks = block_ids
        .iter()
        .map(|block_id| {
            let block = &function.blocks[block_id];
            let mut rows = block
                .instructions
                .iter()
                .map(|instruction| normalized_instruction(instruction, &blocks, &values))
                .collect::<Vec<_>>();
            rows.push(normalized_instruction(
                block.terminator.as_ref().expect("sealed block terminator"),
                &blocks,
                &values,
            ));
            format!("b{}={rows:?}", blocks[block_id])
        })
        .collect();
    let ownership_operations = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter(|instruction| {
            matches!(
                instruction,
                MirInstruction::CopyOwned { .. }
                    | MirInstruction::DestroyOwned { .. }
                    | MirInstruction::ReleaseStrong { .. }
            )
        })
        .count();
    MirSnapshot {
        signature: format!("{:?}", function.signature),
        blocks: normalized_blocks,
        declared_parameters: format!("{:?}", function.metadata.declared_param_decls),
        parameter_contracts: format!("{:?}", function.metadata.parameter_entry_contracts),
        declared_return: function.metadata.declared_return_type_name.clone(),
        return_contract: format!("{:?}", function.metadata.return_exit_contract),
        direct_call_capabilities: function
            .metadata
            .canonical_direct_static_call_capabilities
            .len(),
        ownership_operations,
    }
}

fn compile_old(
    compiler: &mut MirCompiler,
    source: &VerifiedResolvedSourceUnitV1,
) -> MirCompileResult {
    compiler
        .compile_resolved(source.lowering_input(), None)
        .unwrap()
}

fn compile_new(
    compiler: &mut MirCompiler,
    source: &VerifiedResolvedCallableProgramV1,
) -> MirCompileResult {
    compiler
        .compile_singleton_program_for_r0_p0(source)
        .unwrap()
}

fn assert_backend_parity(old: &MirCompileResult, new: &MirCompileResult) {
    for result in [old, new] {
        crate::mir::backend_capability::enforce_mir_backend_supported(
            &result.module,
            "mir-interpreter",
        )
        .unwrap();
        for backend in ["wasm", "ny-llvmc-exe"] {
            assert!(
                crate::mir::backend_capability::enforce_mir_backend_supported(
                    &result.module,
                    backend,
                )
                .is_err()
            );
            let error = crate::mir::canonical_direct_static_call_backend_capability::enforce(
                &result.module,
                backend,
            )
            .unwrap_err();
            assert!(error.contains("silent_fallback_allowed=false"), "{error}");
        }
    }
    let recursive_error =
        crate::mir::canonical_recursive_callable_module_backend_capability::enforce(
            &new.module,
            "wasm",
        )
        .unwrap_err();
    assert!(
        recursive_error.contains("silent_fallback_allowed=false"),
        "{recursive_error}"
    );
}

#[test]
fn singleton_program_matches_old_authority_and_mir_for_both_schedules() {
    for optimize in [false, true] {
        let root = countdown("countdown_parity");
        let (old_source, old_authority) = old_authority(root.clone());
        let (new_source, new_authority) = new_authority(root);
        assert_eq!(old_authority, new_authority);

        let old = compile_old(&mut MirCompiler::with_options(optimize), &old_source);
        let new = compile_new(&mut MirCompiler::with_options(optimize), &new_source);
        assert_eq!(
            mir_snapshot(&old.module.functions["countdown_parity/1"]),
            mir_snapshot(&new.module.functions["countdown_parity/1"])
        );
        assert!(old
            .module
            .metadata
            .canonical_recursive_callable_module_capability
            .is_none());
        assert!(new
            .module
            .metadata
            .canonical_recursive_callable_module_capability
            .is_some());
        assert_backend_parity(&old, &new);
        for input in [0, 1, 6] {
            let args = [VMValue::Integer(input)];
            let old_value = MirInterpreter::new()
                .execute_function_with_args(&old.module, "countdown_parity/1", &args)
                .unwrap();
            let new_value = MirInterpreter::new()
                .execute_function_with_args(&new.module, "countdown_parity/1", &args)
                .unwrap();
            assert_eq!(old_value, new_value);
        }
    }
}

#[test]
fn post_if_phi_and_local_assignment_relations_match() {
    let root = countdown_phi("countdown_phi_parity");
    let (old_source, old_authority) = old_authority(root.clone());
    let (new_source, new_authority) = new_authority(root);
    assert_eq!(old_authority, new_authority);
    let old = compile_old(&mut MirCompiler::with_options(false), &old_source);
    let new = compile_new(&mut MirCompiler::with_options(false), &new_source);
    assert_eq!(
        mir_snapshot(&old.module.functions["countdown_phi_parity/1"]),
        mir_snapshot(&new.module.functions["countdown_phi_parity/1"])
    );
    assert_eq!(
        MirInterpreter::new()
            .execute_function_with_args(
                &new.module,
                "countdown_phi_parity/1",
                &[VMValue::Integer(5)],
            )
            .unwrap(),
        VMValue::Integer(0)
    );
}

#[test]
fn final_call_result_return_relation_matches_without_execution() {
    let root = function(
        "return_self_parity",
        vec![return_(call("return_self_parity", variable("n")))],
    );
    let (old_source, old_authority) = old_authority(root.clone());
    let (new_source, new_authority) = new_authority(root);
    assert_eq!(old_authority, new_authority);
    let old = compile_old(&mut MirCompiler::with_options(false), &old_source);
    let new = compile_new(&mut MirCompiler::with_options(false), &new_source);
    assert_eq!(
        mir_snapshot(&old.module.functions["return_self_parity/1"]),
        mir_snapshot(&new.module.functions["return_self_parity/1"])
    );
}

#[test]
fn disconnected_singleton_rejection_does_not_poison_compiler() {
    let invalid = VerifiedResolvedCallableProgramV1::resolve(program(function(
        "invalid_singleton",
        vec![return_(variable("n"))],
    )))
    .unwrap();
    let mut compiler = MirCompiler::with_options(false);
    let error = compiler
        .compile_singleton_program_for_r0_p0(&invalid)
        .unwrap_err()
        .to_string();
    assert!(error.contains("DirectCallCardinality"), "{error}");

    let valid =
        VerifiedResolvedCallableProgramV1::resolve(program(countdown("after_reject"))).unwrap();
    assert!(compiler.compile_singleton_program_for_r0_p0(&valid).is_ok());
}
