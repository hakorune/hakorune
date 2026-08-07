//! Caller-zero contracts for the post-Recipe callable Loop boundary.
//!
//! This module is deliberately test-only while the physical selector is
//! parked.  It joins existing resolver, Recipe, ABI, and completion products
//! without opening a Builder session.  The only new fact is the relation that
//! those products may be executed together; no source meaning is re-resolved.

#![cfg(test)]

use std::ptr;

use crate::mir::exact_trivial_return_abi::ExactTrivialReturnAbiV1;
use crate::mir::loop_recipe_contract::VerifiedLoopPhysicalBoundaryV1;
use crate::mir::resolved_control_flow::{
    DeclaredFunctionResultContractV1, VerifiedFunctionCompletionV1,
};
use crate::mir::resolved_semantics::{
    FunctionOwnerIdV1, RegionId, ResolvedCallableRefV1, VerifiedCallableHeaderV1,
    VerifiedCallableIndexV1,
};

use super::callable_single_loop_prelude_arguments::{
    PreludeArgumentRejectV1, VerifiedCallablePreludeArgumentListV1,
};
use super::callable_single_loop_recipe_coseal::{
    VerifiedCallablePreludeV1, VerifiedCallableSingleLoopRecipeProductV1, VerifiedCallableTailV1,
    VerifiedLoopRecipeCoSealV1,
};
use super::callable_single_loop_source_shapes::SourceReceiverShapeV1;
use super::function_input::ResolvedFunctionLoweringInputV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopPhysicalPrepareRejectReasonV1 {
    MissingCallableIndex,
    MissingCallableHeader,
    ForeignCallableIndex,
    ForeignCallableHeader,
    OwnerHeaderMismatch,
    HeaderIndexMismatch,
    MissingPreludeTarget,
    PreludeTargetHeaderMissing,
    PreludeOwnerMismatch,
    PreludeReceiverMismatch,
    PreludeArityMismatch,
    PreludeResultAbiUnsupported,
    TerminalOwnerMismatch,
    TerminalTargetMismatch,
    TerminalSiteMismatch,
    TerminalNotValue,
    TerminalBindingMismatch,
    TerminalAbiMismatch,
    DeclaredResultAbiUnsupported,
    DeclaredResultAbiMismatch,
    PreludeArgument(PreludeArgumentRejectV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopPhysicalPrepareRejectV1 {
    NoSafeSlice(LoopPhysicalPrepareRejectReasonV1),
}

/// A borrowed input is branded only when its catalog and header are the exact
/// objects already attached to the resolved input.  The brand is intentionally
/// not `Clone`, even though the underlying view is `Copy`.
#[derive(Debug)]
pub(crate) struct VerifiedCallableFunctionLoweringInputV1<'a> {
    input: ResolvedFunctionLoweringInputV1<'a>,
    index: &'a VerifiedCallableIndexV1,
    header: &'a VerifiedCallableHeaderV1,
}

impl<'a> VerifiedCallableFunctionLoweringInputV1<'a> {
    pub(crate) fn issue(
        input: ResolvedFunctionLoweringInputV1<'a>,
        index: &'a VerifiedCallableIndexV1,
        header: &'a VerifiedCallableHeaderV1,
    ) -> Result<Self, LoopPhysicalPrepareRejectV1> {
        let Some(attached_index) = input.callable_index() else {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::MissingCallableIndex,
            ));
        };
        if !ptr::eq(attached_index, index) {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::ForeignCallableIndex,
            ));
        }
        let Some(attached_header) = input.callable_header() else {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::MissingCallableHeader,
            ));
        };
        if !ptr::eq(attached_header, header) {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::ForeignCallableHeader,
            ));
        }
        if input.owner() != header.callable().owner() {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::OwnerHeaderMismatch,
            ));
        }
        let indexed_header = index
            .header_for_callable(header.callable())
            .map_err(|_| no_safe_slice(LoopPhysicalPrepareRejectReasonV1::HeaderIndexMismatch))?;
        if !ptr::eq(indexed_header, header) {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::HeaderIndexMismatch,
            ));
        }
        Ok(Self {
            input,
            index,
            header,
        })
    }

    pub(crate) const fn input(&self) -> ResolvedFunctionLoweringInputV1<'a> {
        self.input
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.input.owner()
    }

    pub(crate) const fn index(&self) -> &'a VerifiedCallableIndexV1 {
        self.index
    }

    pub(crate) const fn header(&self) -> &'a VerifiedCallableHeaderV1 {
        self.header
    }
}

/// The common Loop demand owns the co-sealed logical product.  It does not
/// duplicate topology, create physical IDs, or retain the source AST.
#[derive(Debug)]
pub(crate) struct VerifiedLoopPhysicalDemandV1 {
    co_seal: VerifiedLoopRecipeCoSealV1,
}

impl VerifiedLoopPhysicalDemandV1 {
    pub(crate) fn issue(co_seal: VerifiedLoopRecipeCoSealV1) -> Self {
        Self { co_seal }
    }

    pub(crate) fn co_seal(&self) -> &VerifiedLoopRecipeCoSealV1 {
        &self.co_seal
    }

    pub(crate) fn into_co_seal(self) -> VerifiedLoopRecipeCoSealV1 {
        self.co_seal
    }

    pub(crate) fn into_physical_boundary(self) -> VerifiedLoopPhysicalBoundaryV1 {
        self.co_seal.into_physical_boundary()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedCallablePreludeCapabilityV1 {
    owner: FunctionOwnerIdV1,
    site: crate::mir::resolved_semantics::SourceExprSiteV1,
    binding: crate::mir::resolved_semantics::BindingRefV1,
    target: ResolvedCallableRefV1,
    receiver: SourceReceiverShapeV1,
    arity: u32,
    result_abi: ExactTrivialReturnAbiV1,
    arguments: VerifiedCallablePreludeArgumentListV1,
}

impl VerifiedCallablePreludeCapabilityV1 {
    fn issue(
        branded: &VerifiedCallableFunctionLoweringInputV1<'_>,
        prelude: &VerifiedCallablePreludeV1,
        expected_receiver: SourceReceiverShapeV1,
    ) -> Result<Self, LoopPhysicalPrepareRejectV1> {
        let Some(target) = prelude.direct_callable() else {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::MissingPreludeTarget,
            ));
        };
        if prelude.owner() != branded.owner() {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::PreludeOwnerMismatch,
            ));
        }
        if prelude.call().receiver() != expected_receiver {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::PreludeReceiverMismatch,
            ));
        }
        let header = branded.index().header_for_callable(target).map_err(|_| {
            no_safe_slice(LoopPhysicalPrepareRejectReasonV1::PreludeTargetHeaderMissing)
        })?;
        if prelude.call().argument_count() as usize != header.signature().arity() {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::PreludeArityMismatch,
            ));
        }
        let result_abi =
            ExactTrivialReturnAbiV1::classify(header.signature().result().source_type_name())
                .ok_or_else(|| {
                    no_safe_slice(LoopPhysicalPrepareRejectReasonV1::PreludeResultAbiUnsupported)
                })?;
        let arguments =
            VerifiedCallablePreludeArgumentListV1::issue(branded.input(), prelude, header)
                .map_err(|reason| {
                    no_safe_slice(LoopPhysicalPrepareRejectReasonV1::PreludeArgument(reason))
                })?;
        Ok(Self {
            owner: prelude.owner(),
            site: prelude.site().clone(),
            binding: prelude.binding(),
            target,
            receiver: prelude.call().receiver(),
            arity: prelude.call().argument_count(),
            result_abi,
            arguments,
        })
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn binding(&self) -> crate::mir::resolved_semantics::BindingRefV1 {
        self.binding
    }

    pub(crate) const fn target(&self) -> ResolvedCallableRefV1 {
        self.target
    }

    pub(crate) const fn receiver(&self) -> SourceReceiverShapeV1 {
        self.receiver
    }

    pub(crate) const fn arity(&self) -> u32 {
        self.arity
    }

    pub(crate) const fn result_abi(&self) -> ExactTrivialReturnAbiV1 {
        self.result_abi
    }

    pub(crate) fn arguments(&self) -> &VerifiedCallablePreludeArgumentListV1 {
        &self.arguments
    }

    #[allow(dead_code)]
    pub(crate) fn site(&self) -> &crate::mir::resolved_semantics::SourceExprSiteV1 {
        &self.site
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedCallableTerminalCompatibilityV1 {
    owner: FunctionOwnerIdV1,
    callable_target: ResolvedCallableRefV1,
    target_function: RegionId,
    statement: crate::mir::resolved_semantics::SourceStmtSiteV1,
    binding: crate::mir::resolved_semantics::BindingRefV1,
    abi: ExactTrivialReturnAbiV1,
}

impl VerifiedCallableTerminalCompatibilityV1 {
    fn issue(
        branded: &VerifiedCallableFunctionLoweringInputV1<'_>,
        prelude: &VerifiedCallablePreludeCapabilityV1,
        tail: &VerifiedCallableTailV1,
        completion: &VerifiedFunctionCompletionV1,
        abi: ExactTrivialReturnAbiV1,
    ) -> Result<Self, LoopPhysicalPrepareRejectV1> {
        if completion.owner() != branded.owner() || tail.owner() != branded.owner() {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::TerminalOwnerMismatch,
            ));
        }
        let expected_target = branded
            .input()
            .function()
            .lowering_roots()
            .function_pair()
            .region();
        if completion.target_function() != expected_target {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::TerminalTargetMismatch,
            ));
        }
        if completion.explicit_site() != Some(tail.statement()) {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::TerminalSiteMismatch,
            ));
        }
        if !completion.returns_value() {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::TerminalNotValue,
            ));
        }
        if prelude.binding() != tail.binding() {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::TerminalBindingMismatch,
            ));
        }
        if prelude.result_abi() != abi {
            return Err(no_safe_slice(
                LoopPhysicalPrepareRejectReasonV1::TerminalAbiMismatch,
            ));
        }
        match completion.function_exit_contract().declared_result() {
            DeclaredFunctionResultContractV1::Annotated(name)
                if name.as_ref() == abi.source_type_name() => {}
            _ => {
                return Err(no_safe_slice(
                    LoopPhysicalPrepareRejectReasonV1::DeclaredResultAbiMismatch,
                ))
            }
        }
        Ok(Self {
            owner: branded.owner(),
            callable_target: prelude.target(),
            target_function: completion.target_function(),
            statement: tail.statement().clone(),
            binding: tail.binding(),
            abi,
        })
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn target_function(&self) -> RegionId {
        self.target_function
    }

    pub(crate) const fn callable_target(&self) -> ResolvedCallableRefV1 {
        self.callable_target
    }

    pub(crate) const fn binding(&self) -> crate::mir::resolved_semantics::BindingRefV1 {
        self.binding
    }

    pub(crate) const fn abi(&self) -> ExactTrivialReturnAbiV1 {
        self.abi
    }

    #[allow(dead_code)]
    pub(crate) fn statement(&self) -> &crate::mir::resolved_semantics::SourceStmtSiteV1 {
        &self.statement
    }
}

/// One pre-effect callable execution product.  Completion is moved into this
/// product exactly once and is not copied into the common Loop demand.
#[derive(Debug)]
pub(crate) struct PreparedCallableLoopPhysicalizationV1<'a> {
    pub(crate) input: VerifiedCallableFunctionLoweringInputV1<'a>,
    pub(crate) demand: VerifiedLoopPhysicalDemandV1,
    pub(crate) prelude: VerifiedCallablePreludeCapabilityV1,
    pub(crate) tail: VerifiedCallableTailV1,
    pub(crate) terminal: VerifiedCallableTerminalCompatibilityV1,
    pub(crate) completion: VerifiedFunctionCompletionV1,
}

impl<'a> PreparedCallableLoopPhysicalizationV1<'a> {
    pub(crate) fn issue(
        input: ResolvedFunctionLoweringInputV1<'a>,
        index: &'a VerifiedCallableIndexV1,
        header: &'a VerifiedCallableHeaderV1,
        product: VerifiedCallableSingleLoopRecipeProductV1,
        completion: VerifiedFunctionCompletionV1,
        // The profile supplies this already-verified source-call shape.  The
        // prepare layer never guesses a receiver kind from a callable name.
        expected_receiver: SourceReceiverShapeV1,
    ) -> Result<Self, LoopPhysicalPrepareRejectV1> {
        let input = VerifiedCallableFunctionLoweringInputV1::issue(input, index, header)?;
        let (co_seal, prelude, tail) = product.into_parts();
        let prelude_capability =
            VerifiedCallablePreludeCapabilityV1::issue(&input, &prelude, expected_receiver)?;
        let abi = declared_result_abi(&input, &completion)?;
        let terminal = VerifiedCallableTerminalCompatibilityV1::issue(
            &input,
            &prelude_capability,
            &tail,
            &completion,
            abi,
        )?;
        Ok(Self {
            input,
            demand: VerifiedLoopPhysicalDemandV1::issue(co_seal),
            prelude: prelude_capability,
            tail,
            terminal,
            completion,
        })
    }

    pub(crate) const fn input(&self) -> &VerifiedCallableFunctionLoweringInputV1<'a> {
        &self.input
    }

    pub(crate) fn demand(&self) -> &VerifiedLoopPhysicalDemandV1 {
        &self.demand
    }

    pub(crate) fn into_demand(self) -> VerifiedLoopPhysicalDemandV1 {
        self.demand
    }

    pub(crate) const fn prelude(&self) -> &VerifiedCallablePreludeCapabilityV1 {
        &self.prelude
    }

    pub(crate) fn tail(&self) -> &VerifiedCallableTailV1 {
        &self.tail
    }

    pub(crate) const fn terminal(&self) -> &VerifiedCallableTerminalCompatibilityV1 {
        &self.terminal
    }

    pub(crate) fn completion(&self) -> &VerifiedFunctionCompletionV1 {
        &self.completion
    }
}

pub(crate) fn issue_callable_loop_physicalization_v1<'a>(
    input: ResolvedFunctionLoweringInputV1<'a>,
    index: &'a VerifiedCallableIndexV1,
    header: &'a VerifiedCallableHeaderV1,
    product: VerifiedCallableSingleLoopRecipeProductV1,
    completion: VerifiedFunctionCompletionV1,
    expected_receiver: SourceReceiverShapeV1,
) -> Result<PreparedCallableLoopPhysicalizationV1<'a>, LoopPhysicalPrepareRejectV1> {
    PreparedCallableLoopPhysicalizationV1::issue(
        input,
        index,
        header,
        product,
        completion,
        expected_receiver,
    )
}

fn declared_result_abi(
    branded: &VerifiedCallableFunctionLoweringInputV1<'_>,
    completion: &VerifiedFunctionCompletionV1,
) -> Result<ExactTrivialReturnAbiV1, LoopPhysicalPrepareRejectV1> {
    let DeclaredFunctionResultContractV1::Annotated(name) =
        completion.function_exit_contract().declared_result()
    else {
        return Err(no_safe_slice(
            LoopPhysicalPrepareRejectReasonV1::DeclaredResultAbiUnsupported,
        ));
    };
    let completion_abi = ExactTrivialReturnAbiV1::classify(name).ok_or_else(|| {
        no_safe_slice(LoopPhysicalPrepareRejectReasonV1::DeclaredResultAbiUnsupported)
    })?;
    let header_abi =
        ExactTrivialReturnAbiV1::classify(branded.header().signature().result().source_type_name())
            .ok_or_else(|| {
                no_safe_slice(LoopPhysicalPrepareRejectReasonV1::DeclaredResultAbiMismatch)
            })?;
    if completion_abi != header_abi {
        return Err(no_safe_slice(
            LoopPhysicalPrepareRejectReasonV1::DeclaredResultAbiMismatch,
        ));
    }
    Ok(completion_abi)
}

fn no_safe_slice(reason: LoopPhysicalPrepareRejectReasonV1) -> LoopPhysicalPrepareRejectV1 {
    LoopPhysicalPrepareRejectV1::NoSafeSlice(reason)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl, Span};
    use crate::mir::compiler::callable_single_loop_recipe_coseal::issue_callable_single_loop_recipe_v1;
    use crate::mir::compiler::callable_single_loop_source_map::issue_callable_single_loop_source_map_v1;
    use crate::mir::compiler::callable_single_loop_source_shapes::SourceReceiverShapeV1;
    use crate::mir::compiler::callable_single_loop_static_fixture_tests::static_fixture_for_test;
    use crate::mir::compiler::callable_single_loop_syntax_facts::issue_callable_single_loop_syntax_facts_v1;
    use crate::mir::compiler::callable_single_loop_syntax_facts::tests::{
        input_loop_and_context, unit,
    };
    use crate::mir::compiler::resolved_callable_module::VerifiedResolvedCallableModuleV1;
    use crate::mir::resolved_control_flow::verify_function_completion_v1;
    use crate::mir::resolved_semantics::{
        CallableCatalogSealOutcomeV1, CallableSemanticSourceLedgerView, CanonicalCallableKeyV1,
        ExprChildRoleV1, OwnedExprSiteV1, VerifiedCallableHeaderSourceUnitV1,
        VerifiedOwnerFreeCallableCatalogSourceUnitV1,
    };

    fn variable(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.into(),
            span: Span::unknown(),
        }
    }

    fn scalar_function(name: &str, params: &[&str]) -> ASTNode {
        ASTNode::FunctionDeclaration {
            name: name.into(),
            params: params.iter().map(|name| (*name).into()).collect(),
            param_decls: params
                .iter()
                .map(|name| ParamDecl {
                    name: (*name).into(),
                    declared_type_name: Some("i64".into()),
                })
                .collect(),
            return_type_name: Some("i64".into()),
            body: vec![ASTNode::Return {
                value: Some(Box::new(variable(params[0]))),
                span: Span::unknown(),
            }],
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: true,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }
    }

    fn loop_function() -> ASTNode {
        let integer = |value: i64| ASTNode::Literal {
            value: crate::ast::LiteralValue::Integer(value),
            span: Span::unknown(),
        };
        let assignment = |name: &str, value: ASTNode| ASTNode::Assignment {
            target: Box::new(variable(name)),
            value: Box::new(value),
            span: Span::unknown(),
        };
        ASTNode::FunctionDeclaration {
            name: "int_to_str".into(),
            params: vec!["n".into(), "helper".into()],
            param_decls: vec![
                ParamDecl {
                    name: "n".into(),
                    declared_type_name: Some("i64".into()),
                },
                ParamDecl {
                    name: "helper".into(),
                    declared_type_name: Some("i64".into()),
                },
            ],
            return_type_name: Some("i64".into()),
            body: vec![
                ASTNode::Local {
                    variables: vec!["value".into()],
                    initial_values: vec![Some(Box::new(ASTNode::MethodCall {
                        object: Box::new(variable("helper")),
                        method: "to_i64".into(),
                        arguments: vec![variable("n")],
                        span: Span::unknown(),
                    }))],
                    declared_type_names: vec![None],
                    span: Span::unknown(),
                },
                ASTNode::Local {
                    variables: vec!["i".into()],
                    initial_values: vec![Some(Box::new(integer(0)))],
                    declared_type_names: vec![None],
                    span: Span::unknown(),
                },
                ASTNode::Loop {
                    condition: Box::new(ASTNode::BinaryOp {
                        operator: crate::ast::BinaryOperator::Less,
                        left: Box::new(variable("i")),
                        right: Box::new(integer(1)),
                        span: Span::unknown(),
                    }),
                    body: vec![assignment(
                        "i",
                        ASTNode::BinaryOp {
                            operator: crate::ast::BinaryOperator::Add,
                            left: Box::new(variable("i")),
                            right: Box::new(integer(1)),
                            span: Span::unknown(),
                        },
                    )],
                    span: Span::unknown(),
                },
                ASTNode::Return {
                    value: Some(Box::new(variable("value"))),
                    span: Span::unknown(),
                },
            ],
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: true,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }
    }

    fn loop_module() -> VerifiedResolvedCallableModuleV1 {
        let source = VerifiedCallableHeaderSourceUnitV1::seal_header_surface(ASTNode::Program {
            statements: vec![scalar_function("helper", &["n"]), loop_function()],
            span: Span::unknown(),
        })
        .unwrap();
        let owner_free = VerifiedOwnerFreeCallableCatalogSourceUnitV1::seal(source).unwrap();
        let catalog = CallableCatalogSealOutcomeV1::seal(owner_free, 41).unwrap();
        VerifiedResolvedCallableModuleV1::resolve(catalog).unwrap()
    }

    fn loop_product<'a>(
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'a>,
    ) -> (
        CallableSemanticSourceLedgerView<'a>,
        super::super::callable_single_loop_recipe_coseal::VerifiedCallableSingleLoopRecipeProductV1,
    ) {
        let body = input.source().root_body().unwrap();
        let loop_stmt = input.source().body_stmt(&body, 2).unwrap();
        let ledger = input
            .forest()
            .callable_source_ledger(input.owner())
            .unwrap();
        let context = ledger.resolved_loop_source(loop_stmt.site()).unwrap();
        let syntax = issue_callable_single_loop_syntax_facts_v1(input, loop_stmt, context).unwrap();
        let map = issue_callable_single_loop_source_map_v1(&ledger, syntax).unwrap();
        let product = issue_callable_single_loop_recipe_v1(&ledger, map).unwrap();
        (ledger, product)
    }

    #[test]
    fn demand_owns_the_co_seal_after_source_views_are_dropped() {
        let demand = {
            let unit = unit(
                None,
                ASTNode::Literal {
                    value: crate::ast::LiteralValue::Integer(1),
                    span: Span::unknown(),
                },
            );
            let (input, _, _) = input_loop_and_context(&unit);
            let (_, product) = loop_product(input);
            let (co_seal, _, _) = product.into_parts();
            VerifiedLoopPhysicalDemandV1::issue(co_seal)
        };
        assert_eq!(demand.co_seal().operations().len(), 7);
        assert_eq!(demand.co_seal().continuation().loop_key().raw(), 0);
    }

    #[test]
    fn input_brand_rejects_a_root_view_before_any_product_is_opened() {
        let unit = unit(
            None,
            ASTNode::Literal {
                value: crate::ast::LiteralValue::Integer(1),
                span: Span::unknown(),
            },
        );
        let input = unit.root_function_input().unwrap();
        let module = loop_module();
        let key = CanonicalCallableKeyV1::free_static_for_test("int_to_str", 2);
        let index = module.source().catalog().index();
        let header = index.lookup(&key).unwrap();
        assert!(matches!(
            VerifiedCallableFunctionLoweringInputV1::issue(input, index, header),
            Err(LoopPhysicalPrepareRejectV1::NoSafeSlice(
                LoopPhysicalPrepareRejectReasonV1::MissingCallableIndex
            ))
        ));
    }

    #[test]
    fn input_brand_accepts_the_exact_catalog_view() {
        let module = loop_module();
        let key = CanonicalCallableKeyV1::free_static_for_test("int_to_str", 2);
        let input = module.function_input(&key).unwrap();
        let index = module.source().catalog().index();
        let header = index.lookup(&key).unwrap();
        let brand = VerifiedCallableFunctionLoweringInputV1::issue(input, index, header)
            .expect("exact callable brand");
        assert_eq!(brand.owner(), header.callable().owner());
        assert_eq!(brand.header().source_key(), &key);
    }

    #[test]
    fn input_brand_rejects_foreign_catalog_and_header_views() {
        let first = loop_module();
        let second = loop_module();
        let key = CanonicalCallableKeyV1::free_static_for_test("int_to_str", 2);
        let input = first.function_input(&key).unwrap();
        let first_index = first.source().catalog().index();
        let second_index = second.source().catalog().index();
        let first_header = first_index.lookup(&key).unwrap();
        let second_header = second_index.lookup(&key).unwrap();
        assert!(matches!(
            VerifiedCallableFunctionLoweringInputV1::issue(input, second_index, second_header),
            Err(LoopPhysicalPrepareRejectV1::NoSafeSlice(
                LoopPhysicalPrepareRejectReasonV1::ForeignCallableIndex
            ))
        ));
        assert!(matches!(
            VerifiedCallableFunctionLoweringInputV1::issue(input, first_index, second_header),
            Err(LoopPhysicalPrepareRejectV1::NoSafeSlice(
                LoopPhysicalPrepareRejectReasonV1::ForeignCallableHeader
            ))
        ));
        assert!(first_header.callable().owner() != second_header.callable().owner());
    }

    #[test]
    fn current_method_call_fixture_is_a_typed_missing_target_boundary() {
        let module = loop_module();
        let key = CanonicalCallableKeyV1::free_static_for_test("int_to_str", 2);
        let input = module.function_input(&key).unwrap();
        let index = module.source().catalog().index();
        let header = index.lookup(&key).unwrap();
        let completion = verify_function_completion_v1(input).unwrap();
        let (_, product) = loop_product(input);
        assert!(matches!(
            issue_callable_loop_physicalization_v1(
                input,
                index,
                header,
                product,
                completion,
                SourceReceiverShapeV1::Other,
            ),
            Err(LoopPhysicalPrepareRejectV1::NoSafeSlice(
                LoopPhysicalPrepareRejectReasonV1::MissingPreludeTarget
            ))
        ));
    }

    #[test]
    fn resolver_static_fixture_produces_declaration_backed_prepared_positive() {
        let module = static_fixture_for_test();
        let key = CanonicalCallableKeyV1::free_static_for_test("int_to_str", 1);
        let input = module.function_input(&key).unwrap();
        let index = module.source().catalog().index();
        let header = index.lookup(&key).unwrap();
        let completion = verify_function_completion_v1(input).unwrap();
        let (_, product) = loop_product(input);
        let prepared = issue_callable_loop_physicalization_v1(
            input,
            index,
            header,
            product,
            completion,
            SourceReceiverShapeV1::FreeStatic,
        )
        .expect("declaration-backed Prepared product");

        assert_eq!(
            prepared.prelude().result_abi(),
            ExactTrivialReturnAbiV1::I64
        );
        let arguments = prepared.prelude().arguments().rows();
        assert_eq!(arguments.len(), 1);
        assert_eq!(arguments[0].ordinal(), 0);
        assert_eq!(arguments[0].abi(), ExactTrivialReturnAbiV1::I64);
        assert_eq!(arguments[0].binding().owner(), input.owner());
        let call_site = OwnedExprSiteV1::new(input.owner(), prepared.prelude().site().clone());
        let call = input
            .source()
            .expr_at(&call_site)
            .expect("prepared call site");
        let argument = input
            .source()
            .child_expr_from_expr(&call, ExprChildRoleV1::CallArgument(0))
            .expect("prepared argument site");
        assert_eq!(arguments[0].site(), argument.site());
        assert_eq!(prepared.terminal().abi(), ExactTrivialReturnAbiV1::I64);
        assert_eq!(
            prepared
                .completion()
                .function_exit_contract()
                .declared_result(),
            &DeclaredFunctionResultContractV1::Annotated("i64".into())
        );
        assert_eq!(
            prepared.prelude().target(),
            module
                .source()
                .catalog()
                .index()
                .resolve_free_static_source_call("to_i64", 1)
                .unwrap()
                .callable()
        );
    }
}
