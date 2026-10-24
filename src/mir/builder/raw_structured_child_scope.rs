//! Short-lived exact source scopes for selected structured raw children.
//!
//! This adapter owns no AST policy. Callers prepare every child receipt from
//! the intact parent AST through the neutral role vocabulary before moving
//! syntax into an existing lowering owner.

use std::collections::VecDeque;

use crate::ast::ASTNode;
use crate::mir::{MirBuilder, ValueId};

use crate::mir::resolved_semantics::ExprChildRoleV1;

use super::enum_match_source_demand::EnumMatchSourceDemandPortV1;
use super::normal_script_semantic_lowering_state::{
    ScriptDirectStaticClaimTakeV1, ScriptDirectStaticClaimedRowV1,
};
use super::qmark_source_demand::QMarkPropagationSourceDemandPortV1;
use super::raw_invocation_source_transport::RawInvocationSourceContextV1;
use super::record_literal_source_demand::RecordLiteralSourceDemandPortV1;
use super::recursive_child_lowering::{
    DirectCallDispositionPortV1, RawFunctionHeaderLookupPortV1, RawOrdinaryNewClaimPortV1,
    RecursiveChildLoweringPortV1,
};
use super::recursive_child_lowering_port::{
    QualifiedStaticMethodHandoffIngressV1, QualifiedStaticMethodHandoffPortV1,
    ScriptDirectStaticClaimCompletionErrorV1, ScriptDirectStaticClaimIngressV1,
};
use super::static_result_publication_ingress::{
    StaticResultPublicationIngressErrorV1, StaticResultPublicationIngressPortV1,
    StaticResultPublicationIngressV1,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum PreparedRawChildSourceV1 {
    Preserve,
    Exact(RawInvocationSourceContextV1),
}

impl PreparedRawChildSourceV1 {
    pub(in crate::mir::builder) fn expression_child(
        &self,
        parent: &ASTNode,
        role: ExprChildRoleV1,
    ) -> Result<Self, String> {
        match self {
            Self::Preserve => Ok(Self::Preserve),
            Self::Exact(context) => Ok(Self::Exact(context.child_expression(parent, role)?)),
        }
    }
}

pub(in crate::mir::builder) struct RawStructuredChildScopePortV1<'port, Port> {
    child: &'port mut Port,
    expressions: VecDeque<PreparedRawChildSourceV1>,
    bodies: VecDeque<PreparedRawChildSourceV1>,
    statement_body: Option<(RawInvocationSourceContextV1, usize)>,
}

impl<'port, Port> RawStructuredChildScopePortV1<'port, Port> {
    pub(super) fn child(&self) -> &Port {
        self.child
    }

    pub(super) fn child_mut(&mut self) -> &mut Port {
        self.child
    }

    pub(in crate::mir::builder) fn new(
        child: &'port mut Port,
        expressions: Vec<PreparedRawChildSourceV1>,
        bodies: Vec<PreparedRawChildSourceV1>,
    ) -> Self {
        Self {
            child,
            expressions: expressions.into(),
            bodies: bodies.into(),
            statement_body: None,
        }
    }

    pub(in crate::mir::builder) fn for_block_expression(
        child: &'port mut Port,
        prelude: PreparedRawChildSourceV1,
        tail: PreparedRawChildSourceV1,
    ) -> Self {
        let statement_body = match prelude {
            PreparedRawChildSourceV1::Preserve => None,
            PreparedRawChildSourceV1::Exact(context) => Some((context, 0)),
        };
        Self {
            child,
            expressions: [tail].into(),
            bodies: VecDeque::new(),
            statement_body,
        }
    }

    pub(in crate::mir::builder) fn for_body(
        child: &'port mut Port,
        body: PreparedRawChildSourceV1,
    ) -> Self {
        let statement_body = match &body {
            PreparedRawChildSourceV1::Preserve => None,
            PreparedRawChildSourceV1::Exact(context) => Some((context.clone(), 0)),
        };
        Self {
            child,
            expressions: VecDeque::new(),
            bodies: [body].into(),
            statement_body,
        }
    }

    fn next_expression(&mut self) -> Result<PreparedRawChildSourceV1, String> {
        self.expressions.pop_front().ok_or_else(|| {
            "[freeze:contract][raw-structured/expression-demand-overflow]".to_owned()
        })
    }

    fn next_body(&mut self) -> Result<PreparedRawChildSourceV1, String> {
        self.bodies
            .pop_front()
            .ok_or_else(|| "[freeze:contract][raw-structured/body-demand-overflow]".to_owned())
    }

    pub(in crate::mir::builder) fn complete_exact_demands_v1(self) -> Result<(), String> {
        if !self.expressions.is_empty() || !self.bodies.is_empty() {
            return Err(format!(
                "[freeze:contract][raw-structured/unconsumed-demands] expressions={} bodies={}",
                self.expressions.len(),
                self.bodies.len()
            ));
        }
        Ok(())
    }

    /// Preserve a child-lowering rejection before checking exact-demand
    /// completion. A failed child may legitimately leave later siblings
    /// unconsumed; reporting that remainder would mask the primary error.
    pub(in crate::mir::builder) fn complete_after_result_v1<T>(
        self,
        result: Result<T, String>,
    ) -> Result<T, String> {
        match result {
            Ok(value) => {
                self.complete_exact_demands_v1()?;
                Ok(value)
            }
            Err(error) => Err(error),
        }
    }
}

impl<Port> RecursiveChildLoweringPortV1 for RawStructuredChildScopePortV1<'_, Port>
where
    Port: RecursiveChildLoweringPortV1<
        BodyInput = Vec<ASTNode>,
        StatementInput = ASTNode,
        ExpressionInput = ASTNode,
    >,
{
    type BodyInput = Vec<ASTNode>;
    type StatementInput = ASTNode;
    type ExpressionInput = ASTNode;

    fn complete_short_circuit_expression_v1(
        &mut self, completed: &super::ops::CompletedLogicalShortCircuitV1,
    ) -> Result<(), String> {
        self.child.complete_short_circuit_expression_v1(completed)
    }

    fn complete_ordinary_binary_expression_v1(
        &mut self,
        completed: &super::ops::CompletedOrdinaryBinaryV1,
    ) -> Result<(), String> {
        self.child.complete_ordinary_binary_expression_v1(completed)
    }

    fn observe_borrowed_field_write_guard_binary_v1(&mut self, builder: &mut MirBuilder,
        children: (ValueId, ValueId), completed: &super::ops::CompletedOrdinaryBinaryV1,
    ) -> Result<(), String> { self.child.observe_borrowed_field_write_guard_binary_v1(builder, children, completed) }
    fn prepare_borrowed_field_write_guard_observer_v1(&mut self)
        -> Result<Option<Box<dyn super::if_form::IfFormPhysicalObserverV1>>, String> {
        self.child.prepare_borrowed_field_write_guard_observer_v1()
    }

    fn prepare_borrowed_array_set_v1(&mut self, method: &str, arity: usize)
        -> Result<Option<crate::mir::normal_callable_semantic_package::BorrowedArraySetSourceLoanV1>, String> {
        self.child.prepare_borrowed_array_set_v1(method, arity)
    }
    fn complete_borrowed_array_set_v1(&mut self, builder: &mut MirBuilder,
        source: crate::mir::normal_callable_semantic_package::BorrowedArraySetSourceLoanV1,
        children: [ValueId; 3], write: crate::mir::builder::CompletedArrayElementWriteV1,
    ) -> Result<(), String> { self.child.complete_borrowed_array_set_v1(builder, source, children, write) }

    fn prepare_borrowed_field_write_v1(&mut self) -> Result<Option<crate::mir::normal_callable_semantic_package::FieldWriteSourceLoanV1>, String> {
        self.child.prepare_borrowed_field_write_v1()
    }
    fn read_borrowed_field_write_operand_v1(&mut self, builder: &mut MirBuilder, source: &crate::mir::normal_callable_semantic_package::FieldWriteSourceLoanV1,
        receiver: bool, input: Self::ExpressionInput,
    ) -> Result<crate::mir::normal_callable_semantic_package::FieldWriteValueV1, String> {
        let child_source = self.next_expression()?;
        self.child.with_prepared_child_source_v1(child_source, |child|
            child.read_borrowed_field_write_operand_v1(builder, source, receiver, input))
    }
    fn emit_borrowed_field_write_v1(&mut self, builder: &mut MirBuilder,
        source: crate::mir::normal_callable_semantic_package::FieldWriteSourceLoanV1,
        field: &str, reads: [crate::mir::normal_callable_semantic_package::FieldWriteValueV1; 2],
    ) -> Result<(), String> { self.child.emit_borrowed_field_write_v1(builder, source, field, reads) }

    fn prepare_if_state_observer_v1(&mut self, builder: &MirBuilder)
        -> Result<Option<Box<dyn crate::mir::builder::if_form::IfFormPhysicalObserverV1>>, String> {
        self.child.prepare_if_state_observer_v1(builder)
    }

    fn prepare_checked_bool_condition_v1(&mut self) -> Result<Option<std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedBoolConditionFrameV1>>, String> {
        self.child.prepare_checked_bool_condition_v1()
    }
    fn lower_checked_bool_condition_v1(&mut self, builder: &mut MirBuilder,
        frame: std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedBoolConditionFrameV1>, condition: ASTNode,
    ) -> Result<super::ordinary_new_admission::selected::bool_condition::LoweredBoolConditionV1, String> {
        let source = self.next_expression()?;
        self.child.with_prepared_child_source_v1(source, |child|
            child.lower_checked_bool_condition_v1(builder, frame, condition))
    }

    fn prepare_checked_print_expression_v1(&mut self) -> Result<Option<std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedPrintExpressionFrameV1>>, String> {
        self.child.prepare_checked_print_expression_v1()
    }
    fn lower_checked_print_value_v1(&mut self, builder: &mut MirBuilder,
        frame: std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedPrintExpressionFrameV1>,
    ) -> Result<ValueId, String> {
        let source = self.next_expression()?;
        self.child.with_prepared_child_source_v1(source, |child| child.lower_checked_print_value_v1(builder, frame))
    }

    fn prepare_ordinary_scalar_source_v1(
        &mut self, operator: &crate::ast::BinaryOperator,
    ) -> Result<Option<std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::ExpressionScalarOperationLoanV1>>, String> {
        self.child.prepare_ordinary_scalar_source_v1(operator)
    }

    fn complete_ordinary_scalar_source_v1(
        &mut self, source: std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::ExpressionScalarOperationLoanV1>, children: (ValueId, ValueId),
        relations: [std::rc::Rc<crate::mir::builder::ssa::local::LocalSsaPhysicalRelationV1>; 2],
        completed: &super::ops::CompletedOrdinaryBinaryV1,
        index: &crate::mir::builder::ssa::local::LocalSsaPhysicalIndexV1,
        function: &crate::mir::MirFunction,
    ) -> Result<std::rc::Rc<crate::mir::normal_callable_semantic_package::OriginalScalarAppendV1>, String> {
        self.child.complete_ordinary_scalar_source_v1(source, children, relations, completed, index, function)
    }

    fn prepare_borrowed_compare_source_v1(
        &mut self,
        operator: &crate::ast::BinaryOperator,
    ) -> Result<
        Option<crate::mir::normal_callable_semantic_package::BorrowedCompareSourceLoanV1>,
        String,
    > {
        self.child.prepare_borrowed_compare_source_v1(operator)
    }

    fn prepare_borrowed_compare_operands_v1(
        &mut self,
        builder: &mut MirBuilder,
        source: &crate::mir::normal_callable_semantic_package::BorrowedCompareSourceLoanV1,
        children: (ValueId, ValueId),
    ) -> Result<(), String> {
        self.child
            .prepare_borrowed_compare_operands_v1(builder, source, children)
    }

    fn complete_borrowed_compare_source_v1(
        &mut self,
        loan: crate::mir::normal_callable_semantic_package::BorrowedCompareSourceLoanV1,
        children: (ValueId, ValueId),
        completed: &super::ops::CompletedOrdinaryBinaryV1,
    ) -> Result<
        std::rc::Rc<crate::mir::normal_callable_semantic_package::BorrowedCompareMaterializationV1>,
        String,
    > {
        self.child
            .complete_borrowed_compare_source_v1(loan, children, completed)
    }

    fn prepare_borrowed_mul_source_v1(
        &mut self,
        operator: &crate::ast::BinaryOperator,
    ) -> Result<Option<crate::mir::normal_callable_semantic_package::BorrowedMulSourceLoanV1>, String>
    {
        self.child.prepare_borrowed_mul_source_v1(operator)
    }

    fn complete_borrowed_mul_source_v1(
        &mut self,
        loan: crate::mir::normal_callable_semantic_package::BorrowedMulSourceLoanV1,
        children: (ValueId, ValueId),
        completed: &super::ops::CompletedOrdinaryBinaryV1,
    ) -> Result<
        std::rc::Rc<crate::mir::normal_callable_semantic_package::BorrowedMulMaterializationV1>,
        String,
    > {
        self.child
            .complete_borrowed_mul_source_v1(loan, children, completed)
    }

    fn prepare_terminal_null_literal_v1(
        &mut self,
    ) -> Result<
        Option<crate::mir::normal_callable_semantic_package::TerminalNullReturnSourceLoanV1>,
        String,
    > {
        self.child.prepare_terminal_null_literal_v1()
    }
    fn complete_terminal_null_literal_v1(
        &mut self,
        loan: crate::mir::normal_callable_semantic_package::TerminalNullReturnSourceLoanV1,
        completed: &super::emission::constant::CompletedConstV1,
    ) -> Result<(), String> {
        self.child
            .complete_terminal_null_literal_v1(loan, completed)
    }

    fn prepare_compare_integer_literal_v1(
        &mut self,
        value: i64,
    ) -> Result<
        Option<crate::mir::normal_callable_semantic_package::BorrowedCompareIntegerLiteralLoanV1>,
        String,
    > {
        self.child.prepare_compare_integer_literal_v1(value)
    }

    fn complete_compare_integer_literal_v1(
        &mut self,
        loan: crate::mir::normal_callable_semantic_package::BorrowedCompareIntegerLiteralLoanV1,
        completed: &super::emission::constant::CompletedConstV1,
    ) -> Result<std::rc::Rc<crate::mir::normal_callable_semantic_package::BorrowedCompareIntegerLiteralMaterializationV1>, String>{
        self.child
            .complete_compare_integer_literal_v1(loan, completed)
    }

    fn script_direct_static_claim_ingress_v1(
        &mut self,
        box_name: &str,
        method: &str,
        argument_count: usize,
    ) -> Result<ScriptDirectStaticClaimIngressV1, String> {
        self.child
            .script_direct_static_claim_ingress_v1(box_name, method, argument_count)
    }

    fn take_script_direct_static_claim_v1(
        &mut self,
        box_name: &str,
        method: &str,
        receiver: &ASTNode,
        arguments: &[ASTNode],
    ) -> Result<ScriptDirectStaticClaimTakeV1, String> {
        self.child
            .take_script_direct_static_claim_v1(box_name, method, receiver, arguments)
    }

    fn complete_script_direct_static_claim_v1(
        &mut self,
        claimed: ScriptDirectStaticClaimedRowV1,
    ) -> Result<(), ScriptDirectStaticClaimCompletionErrorV1> {
        self.child.complete_script_direct_static_claim_v1(claimed)
    }

    fn lower_body(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::BodyInput,
    ) -> Result<ValueId, String> {
        let source = self.next_body()?;
        self.child
            .with_prepared_child_source_v1(source, |child| child.lower_body(builder, input))
    }

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::StatementInput,
    ) -> Result<ValueId, String> {
        let Some((body, index)) = self.statement_body.as_mut() else {
            return self.child.lower_statement(builder, input);
        };
        let transport = body.structured_body_statement(input, *index)?;
        *index += 1;
        let (input, context) = RawInvocationSourceContextV1::from_transport(transport);
        self.child
            .with_prepared_child_source_v1(PreparedRawChildSourceV1::Exact(context), |child| {
                child.lower_statement(builder, input)
            })
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::ExpressionInput,
    ) -> Result<ValueId, String> {
        match input {
            ASTNode::Program { statements, span } => {
                let source = self.next_body()?;
                match source {
                    PreparedRawChildSourceV1::Preserve => self
                        .child
                        .lower_expression(builder, ASTNode::Program { statements, span }),
                    source => self.child.with_prepared_child_source_v1(source, |child| {
                        child.lower_body(builder, statements)
                    }),
                }
            }
            input => {
                let source = self.next_expression()?;
                self.child.with_prepared_child_source_v1(source, |child| {
                    child.lower_expression(builder, input)
                })
            }
        }
    }

    fn lower_me_expression_v1(&mut self, builder: &mut MirBuilder) -> Result<ValueId, String> {
        self.child.lower_me_expression_v1(builder)
    }

    fn try_lower_map_read_method_call_v1(
        &mut self,
        builder: &mut MirBuilder,
        receiver: &ASTNode,
        method: &str,
        arguments: &[ASTNode],
        receiver_source: PreparedRawChildSourceV1,
    ) -> Result<Option<ValueId>, String> {
        self.child.try_lower_map_read_method_call_v1(
            builder,
            receiver,
            method,
            arguments,
            receiver_source,
        )
    }

    fn take_construction_store_v1(
        &mut self,
    ) -> Result<
        Option<
            super::normal_callable_semantic_lowering_state::construction::TakenConstructionStore,
        >,
        String,
    > {
        self.child.take_construction_store_v1()
    }

    fn emit_construction_store_v1(
        &mut self,
        builder: &mut MirBuilder,
        store: super::normal_callable_semantic_lowering_state::construction::TakenConstructionStore,
        provider_value: Option<ValueId>,
    ) -> Result<ValueId, String> {
        self.child
            .emit_construction_store_v1(builder, store, provider_value)
    }

    fn complete_construction_stores_v1(&mut self, builder: &MirBuilder) -> Result<(), String> {
        self.child.complete_construction_stores_v1(builder)
    }
}

#[path = "raw_structured_direct_call.rs"]
mod direct_call;

impl<Port> StaticResultPublicationIngressPortV1 for RawStructuredChildScopePortV1<'_, Port>
where
    Port: StaticResultPublicationIngressPortV1,
{
    fn take_static_result_publication_ingress_v1(
        &mut self,
        declarations: Option<&crate::mir::builder::VerifiedSameModuleCallableDeclarationCatalogV1>,
        owner: &str,
        method: &str,
        argument_count: usize,
    ) -> Result<StaticResultPublicationIngressV1, StaticResultPublicationIngressErrorV1> {
        self.child.take_static_result_publication_ingress_v1(
            declarations,
            owner,
            method,
            argument_count,
        )
    }
}

impl<Port> QualifiedStaticMethodHandoffPortV1 for RawStructuredChildScopePortV1<'_, Port>
where
    Port: QualifiedStaticMethodHandoffPortV1,
{
    fn take_qualified_static_method_handoff_v1(
        &mut self,
        receiver: &str,
        method: &str,
        argument_count: usize,
    ) -> Result<QualifiedStaticMethodHandoffIngressV1, String> {
        self.child
            .take_qualified_static_method_handoff_v1(receiver, method, argument_count)
    }
}

impl<Port> RawFunctionHeaderLookupPortV1 for RawStructuredChildScopePortV1<'_, Port>
where
    Port: RawFunctionHeaderLookupPortV1,
{
    fn with_function_headers<R>(
        &mut self,
        observe: impl for<'headers> FnOnce(
            Option<&'headers dyn super::function_signature_lookup::FunctionSignatureLookupV1>,
        ) -> R,
    ) -> R {
        self.child.with_function_headers(observe)
    }
}

impl<Port> RawOrdinaryNewClaimPortV1 for RawStructuredChildScopePortV1<'_, Port>
where
    Port: RawOrdinaryNewClaimPortV1,
{
    fn prepare_terminal_field_read(
        &mut self,
        object: ASTNode,
        field: &str,
    ) -> Result<Option<super::fields::PreparedRawFieldReadV1>, String> {
        self.child.prepare_terminal_field_read(object, field)
    }
    fn prepare_root_home_exit(&mut self, builder: &MirBuilder) -> Result<bool, String> {
        self.child.prepare_root_home_exit(builder)
    }
    fn emit_terminal_i64_call_exit(
        &mut self,
        builder: &mut MirBuilder,
    ) -> Result<Option<ValueId>, String> {
        self.child.emit_terminal_i64_call_exit(builder)
    }
    fn emit_terminal_i64_add_return(
        &mut self,
        builder: &mut MirBuilder,
    ) -> Result<Option<ValueId>, String> {
        self.child.emit_terminal_i64_add_return(builder)
    }

    fn emit_terminal_i64_field_return(
        &mut self,
        builder: &mut MirBuilder,
    ) -> Result<Option<crate::mir::builder::recursive_child_lowering::TerminalFieldReturnV1>, String> {
        self.child.emit_terminal_i64_field_return(builder)
    }

    fn emit_terminal_map_get_return(
        &mut self,
        builder: &mut MirBuilder,
    ) -> Result<Option<ValueId>, String> {
        self.child.emit_terminal_map_get_return(builder)
    }

    fn emit_terminal_scalar_literal_return(
        &mut self,
        builder: &mut MirBuilder,
    ) -> Result<Option<ValueId>, String> {
        self.child.emit_terminal_scalar_literal_return(builder)
    }

    fn complete_terminal_scalar_literal_return(&mut self, builder: &MirBuilder) -> Result<(), String> {
        self.child.complete_terminal_scalar_literal_return(builder)
    }

    fn emit_root_home_exit(
        &mut self,
        builder: &mut MirBuilder,
        value: ValueId,
    ) -> Result<ValueId, String> {
        self.child.emit_root_home_exit(builder, value)
    }
    fn emit_root_home_unit_exit(&mut self, builder: &mut MirBuilder) -> Result<ValueId, String> {
        self.child.emit_root_home_unit_exit(builder)
    }
    fn prepare_ordinary_new_emission(
        &mut self,
        builder: &MirBuilder,
        claim: &crate::mir::normal_callable_semantic_package::OrdinaryNewAdmissionClaimV1,
    ) -> Result<bool, String> {
        self.child.prepare_ordinary_new_emission(builder, claim)
    }

    fn emit_ordinary_new_claim(
        &mut self,
        builder: &mut MirBuilder,
        claim: crate::mir::normal_callable_semantic_package::OrdinaryNewAdmissionClaimV1,
    ) -> Result<ValueId, String> {
        self.child.emit_ordinary_new_claim(builder, claim)
    }

    fn try_take_result_new_claim(
        &mut self,
        class: &str,
        argument_count: usize,
    ) -> Result<
        Option<crate::mir::normal_callable_semantic_package::OrdinaryNewResultClaimV1>,
        String,
    > {
        self.child.try_take_result_new_claim(class, argument_count)
    }

    fn prepare_result_new_emission(
        &mut self,
        builder: &MirBuilder,
        claim: &crate::mir::normal_callable_semantic_package::OrdinaryNewResultClaimV1,
    ) -> Result<bool, String> {
        self.child.prepare_result_new_emission(builder, claim)
    }

    fn emit_result_new_claim(
        &mut self,
        builder: &mut MirBuilder,
        claim: crate::mir::normal_callable_semantic_package::OrdinaryNewResultClaimV1,
    ) -> Result<ValueId, String> {
        self.child.emit_result_new_claim(builder, claim)
    }

    fn try_emit_unit_discard(&mut self, builder: &mut MirBuilder, method: &str, arity: usize) -> Result<bool, String> {
        self.child.try_emit_unit_discard(builder, method, arity)
    }

    fn try_emit_owned_array_append(&mut self, builder: &mut MirBuilder, method: &str, arity: usize) -> Result<bool, String> {
        self.child.try_emit_owned_array_append(builder, method, arity)
    }

    fn prepare_owned_array_construction(
        &mut self, class: &str, argument_count: usize, field_count: usize,
    ) -> Result<Option<crate::mir::normal_callable_semantic_package::PreparedOwnedArrayConstructionV1>, String> {
        self.child.prepare_owned_array_construction(class, argument_count, field_count)
    }

    fn emit_owned_array_construction(
        &mut self, builder: &mut MirBuilder,
        source: crate::mir::normal_callable_semantic_package::PreparedOwnedArrayConstructionV1,
    ) -> Result<ValueId, String> {
        self.child.emit_owned_array_construction(builder, source)
    }

    fn validate_named_array_construction_route(&self, named_route: bool) -> Result<(), String> {
        self.child
            .validate_named_array_construction_route(named_route)
    }

    fn complete_ordinary_new_expression(
        &mut self,
        class: &str,
        value: ValueId,
    ) -> Result<(), String> {
        self.child.complete_ordinary_new_expression(class, value)
    }

    fn named_array_field_provider_recording(
        &mut self,
    ) -> Result<
        Option<(
            crate::mir::resolved_semantics::SourceExprSiteV1,
            hakorune_mir_defs::CanonicalFieldRefV1,
        )>,
        String,
    > {
        self.child.named_array_field_provider_recording()
    }
    fn try_take_ordinary_new_claim(
        &mut self,
        class: &str,
        argument_count: usize,
    ) -> Result<
        Option<crate::mir::normal_callable_semantic_package::OrdinaryNewAdmissionClaimV1>,
        String,
    > {
        self.child
            .try_take_ordinary_new_claim(class, argument_count)
    }

    fn try_take_ordinary_new_birth_recipe(
        &mut self,
        class: &str,
        argument_count: usize,
    ) -> Result<
        Option<crate::mir::normal_callable_semantic_package::VerifiedOrdinaryNewBirthRecipeV1>,
        String,
    > {
        self.child
            .try_take_ordinary_new_birth_recipe(class, argument_count)
    }
}

impl<Port> RecordLiteralSourceDemandPortV1 for RawStructuredChildScopePortV1<'_, Port>
where
    Port: RecordLiteralSourceDemandPortV1,
{
    fn record_literal_explicit_field_count_v1(
        &self,
        literal: &ASTNode,
    ) -> Result<Option<u32>, String> {
        self.child.record_literal_explicit_field_count_v1(literal)
    }
}

impl<Port> QMarkPropagationSourceDemandPortV1 for RawStructuredChildScopePortV1<'_, Port>
where
    Port: QMarkPropagationSourceDemandPortV1,
{
    fn has_qmark_propagation_receipt_v1(&self, qmark: &ASTNode) -> Result<bool, String> {
        self.child.has_qmark_propagation_receipt_v1(qmark)
    }
}

impl<Port> EnumMatchSourceDemandPortV1 for RawStructuredChildScopePortV1<'_, Port>
where
    Port: EnumMatchSourceDemandPortV1,
{
    fn has_enum_match_scrutinee_receipt_v1(&self, expression: &ASTNode) -> Result<bool, String> {
        self.child.has_enum_match_scrutinee_receipt_v1(expression)
    }
}

#[cfg(test)]
#[path = "raw_structured_child_scope_tests.rs"]
mod tests;
