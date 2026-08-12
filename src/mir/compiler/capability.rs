//! Whole-unit capability proof before the first Builder effect.

use crate::ast::{ASTNode, BinaryOperator};
use crate::mir::resolved_control_flow::if_control::{
    verify_resolved_function_if_control_v1, verify_resolved_function_if_control_with_direct_call_v1,
};
use crate::mir::resolved_control_flow::{
    verify_function_completion_v1, VerifiedFunctionCompletionV1,
};
use crate::mir::resolved_region_flow::{
    analyze_resolved_function_flow_v1, VerifiedResolvedFunctionFlowV1,
};
use crate::mir::resolved_semantics::{
    BindingKindV1, RegionKindV1, ResolvedAssignmentTargetV1, ResolvedExitOriginV1,
    ResolvedLexicalRefV1, ScopeKindV1, SourceBindingSiteV1,
};
use crate::mir::resolved_value_profile::{
    analyze_trivial_canonical_main_owner_v1,
    analyze_trivial_canonical_main_owner_with_finite_direct_calls_v1,
    analyze_trivial_canonical_owner_v1,
    analyze_trivial_canonical_owner_with_finite_direct_calls_v1,
    TrivialCanonicalOwnerAnalysisV1,
};

use super::direct_accum_capability::{
    probe_direct_accum_source_unit_v1, DirectAccumSourceUnitProbeV1,
};
use super::function_input::ResolvedFunctionLoweringInputV1;
use super::located::{LocatedBodyV1, LocatedExprV1, LocatedStmtV1};
use super::lowering_input::{CanonicalLoweringErrorV1, VerifiedResolvedSourceUnitV1};
use super::nested_predicate_profile::{
    probe_nested_predicate_source_unit_v1, NestedPredicateSourceUnitProbeV1,
};
use super::source_view::{BodyChildRoleV1, ExprChildRoleV1};

mod first_family_plan;
mod function_role_policy;
mod normal_main_binding;
mod resolved_owner_header;
mod trivial_plan;
pub(crate) use first_family_plan::{
    seal_direct_accum_owner_header_v1, CanonicalFirstFamilyPlanBrandV1, CanonicalFirstFamilyPlanV1,
    CanonicalLoopFamilyPlanV1,
};
use function_role_policy::{CanonicalFunctionRolePolicyV1, DirectCallAdmissionV1};
pub(in crate::mir) use normal_main_binding::bind_sealed_normal_main_parts_v1;
pub(crate) use resolved_owner_header::{
    ResolvedOwnerHeaderFamilyV1, ResolvedOwnerHeaderSealErrorV1, VerifiedResolvedOwnerHeaderV1,
};
pub(crate) use trivial_plan::CanonicalTrivialBindingSsaPlanV1;

#[derive(Debug)]
pub(crate) struct CanonicalCurrentAPlusPlanV1<'a> {
    function: ResolvedFunctionLoweringInputV1<'a>,
    flow: VerifiedResolvedFunctionFlowV1,
    completion: VerifiedFunctionCompletionV1,
    block_expr_count: usize,
}

impl<'a> CanonicalCurrentAPlusPlanV1<'a> {
    pub(crate) fn seal_resolved_owner_header_v1(
        &self,
    ) -> Result<VerifiedResolvedOwnerHeaderV1, ResolvedOwnerHeaderSealErrorV1> {
        VerifiedResolvedOwnerHeaderV1::seal_input(
            CanonicalFirstFamilyPlanBrandV1::from_family(
                ResolvedOwnerHeaderFamilyV1::CurrentCanonicalAPlus,
            ),
            self.function,
        )
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ResolvedFunctionLoweringInputV1<'a>,
        VerifiedResolvedFunctionFlowV1,
        VerifiedFunctionCompletionV1,
        usize,
    ) {
        (
            self.function,
            self.flow,
            self.completion,
            self.block_expr_count,
        )
    }
}

pub(crate) struct CanonicalLoweringPreflightV1;

impl CanonicalLoweringPreflightV1 {
    pub(crate) fn verify(
        unit: &VerifiedResolvedSourceUnitV1,
    ) -> Result<CanonicalFirstFamilyPlanV1<'_>, CanonicalLoweringErrorV1> {
        match probe_nested_predicate_source_unit_v1(unit)? {
            NestedPredicateSourceUnitProbeV1::Candidate(plan) => return Ok(plan),
            NestedPredicateSourceUnitProbeV1::NotCandidate => {}
        }
        match probe_direct_accum_source_unit_v1(unit)? {
            DirectAccumSourceUnitProbeV1::Candidate(plan) => Ok(plan),
            DirectAccumSourceUnitProbeV1::NotCandidate(function) => Self::verify_function(function),
        }
    }

    pub(crate) fn verify_function<'a>(
        function: ResolvedFunctionLoweringInputV1<'a>,
    ) -> Result<CanonicalFirstFamilyPlanV1<'a>, CanonicalLoweringErrorV1> {
        Self::verify_function_with_policy(
            function,
            DirectCallAdmissionV1::Forbidden,
            CanonicalFunctionRolePolicyV1::OrdinaryFirstFamily,
            None,
        )
    }

    /// Disconnected P0c-F-DX0a facade. Production module admission remains on
    /// `verify_function` until the later atomic P0c-F-I1 cutover.
    pub(crate) fn verify_function_with_finite_direct_calls_v1<'a>(
        function: ResolvedFunctionLoweringInputV1<'a>,
    ) -> Result<CanonicalFirstFamilyPlanV1<'a>, CanonicalLoweringErrorV1> {
        Self::verify_function_with_policy(
            function,
            DirectCallAdmissionV1::FiniteOneOrMore,
            CanonicalFunctionRolePolicyV1::OrdinaryFirstFamily,
            None,
        )
    }

    pub(crate) fn verify_normal_main0_function_v1<'a>(
        function: ResolvedFunctionLoweringInputV1<'a>,
        role: super::normal_source_plan::VerifiedNormalMainRoleV1,
    ) -> Result<CanonicalTrivialBindingSsaPlanV1<'a>, CanonicalLoweringErrorV1> {
        match Self::verify_function_with_policy(
            function,
            DirectCallAdmissionV1::Forbidden,
            CanonicalFunctionRolePolicyV1::NormalMain0,
            Some(role),
        )? {
            CanonicalFirstFamilyPlanV1::Loop(
                super::capability::CanonicalLoopFamilyPlanV1::DirectAccum(plan),
            ) => unsupported(
                "root",
                plan.input().source().root(),
                "direct_accum_not_normal_main",
            ),
            CanonicalFirstFamilyPlanV1::Loop(
                super::capability::CanonicalLoopFamilyPlanV1::NestedPredicate(plan),
            ) => unsupported(
                "root",
                plan.input().source().root(),
                "nested_predicate_not_normal_main",
            ),
            CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) => Ok(plan),
            CanonicalFirstFamilyPlanV1::CurrentCanonicalAPlus(plan) => {
                let (function, ..) = plan.into_parts();
                unsupported(
                    "root",
                    function.source().root(),
                    "normal_main_requires_trivial_binding_ssa",
                )
            }
        }
    }

    pub(crate) fn verify_normal_main0_function_with_finite_direct_calls_v1<'a>(
        function: ResolvedFunctionLoweringInputV1<'a>,
        role: super::normal_source_plan::VerifiedNormalMainRoleV1,
    ) -> Result<CanonicalTrivialBindingSsaPlanV1<'a>, CanonicalLoweringErrorV1> {
        match Self::verify_function_with_policy(
            function,
            DirectCallAdmissionV1::FiniteOneOrMore,
            CanonicalFunctionRolePolicyV1::NormalMainDirectCall0,
            Some(role),
        )? {
            CanonicalFirstFamilyPlanV1::Loop(
                super::capability::CanonicalLoopFamilyPlanV1::DirectAccum(plan),
            ) => unsupported(
                "root",
                plan.input().source().root(),
                "direct_accum_not_normal_main_direct_call",
            ),
            CanonicalFirstFamilyPlanV1::Loop(
                super::capability::CanonicalLoopFamilyPlanV1::NestedPredicate(plan),
            ) => unsupported(
                "root",
                plan.input().source().root(),
                "nested_predicate_not_normal_main_direct_call",
            ),
            CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) => Ok(plan),
            CanonicalFirstFamilyPlanV1::CurrentCanonicalAPlus(plan) => {
                let (function, ..) = plan.into_parts();
                unsupported(
                    "root",
                    function.source().root(),
                    "normal_main_direct_call_requires_trivial_binding_ssa",
                )
            }
        }
    }

    fn verify_function_with_policy<'a>(
        function: ResolvedFunctionLoweringInputV1<'a>,
        direct_call_admission: DirectCallAdmissionV1,
        role: CanonicalFunctionRolePolicyV1,
        main_role: Option<super::normal_source_plan::VerifiedNormalMainRoleV1>,
    ) -> Result<CanonicalFirstFamilyPlanV1<'a>, CanonicalLoweringErrorV1> {
        if function.forest().owner_count() != 1 || !function.forest().upvars().is_empty() {
            return unsupported(
                "source_unit",
                function.source().root(),
                "owner_family_not_closed",
            );
        }
        let ASTNode::FunctionDeclaration {
            name,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            contracts,
            is_static,
            is_override,
            attrs,
            ..
        } = function.source().root()
        else {
            return unsupported("root", function.source().root(), "root_is_not_function");
        };
        let role_error = match role {
            CanonicalFunctionRolePolicyV1::OrdinaryFirstFamily => {
                !*is_static || *is_override || name == "main"
            }
            CanonicalFunctionRolePolicyV1::NormalMain0
            | CanonicalFunctionRolePolicyV1::NormalMainDirectCall0 => {
                !*is_static || *is_override || name != "main" || !params.is_empty()
            }
        };
        if role_error {
            return unsupported("root", function.source().root(), role.rejection_reason());
        }
        if !uses.is_empty() || !contracts.is_empty() || !attrs.is_empty() {
            return unsupported(
                "root",
                function.source().root(),
                "function_metadata_not_activated",
            );
        }
        if (!param_decls.is_empty() && param_decls.len() != params.len())
            || (!param_decls.is_empty()
                && param_decls
                    .iter()
                    .zip(params)
                    .any(|(decl, name)| decl.name != *name))
        {
            return unsupported(
                "root",
                function.source().root(),
                "typed_signature_not_activated",
            );
        }

        let direct_call_count = function.function().direct_call_targets().count();
        let expression_policy = match (direct_call_admission, direct_call_count) {
            (_, 0) => FirstFamilyExpressionPolicyV1::Closed,
            (DirectCallAdmissionV1::FiniteOneOrMore, 1..)
                if function.callable_index().is_some() =>
            {
                FirstFamilyExpressionPolicyV1::ExactDirectCall
            }
            _ => {
                return unsupported(
                    "root",
                    function.source().root(),
                    "direct_call_cardinality_not_activated",
                )
            }
        };
        if expression_policy == FirstFamilyExpressionPolicyV1::ExactDirectCall
            && params.is_empty()
            && !role.allows_zero_parameter_direct_call()
        {
            return unsupported(
                "root",
                function.source().root(),
                "zero_parameter_direct_call_not_activated",
            );
        }

        let located_body = function.source().root_body().map_err(source_navigation)?;
        debug_assert_eq!(body.len(), located_body.statements().len());
        let block_expr_count = verify_body(
            function,
            &located_body,
            ReturnPolicyV1::FinalOnly,
            expression_policy,
        )?;
        let completion = verify_function_completion_v1(function).map_err(|error| {
            CanonicalLoweringErrorV1::ResolvedFunctionCompletion {
                detail: format!("{error:?}"),
            }
        })?;
        let if_control = match expression_policy {
            FirstFamilyExpressionPolicyV1::Closed => {
                verify_resolved_function_if_control_v1(function, &completion)
            }
            FirstFamilyExpressionPolicyV1::ExactDirectCall => {
                verify_resolved_function_if_control_with_direct_call_v1(function, &completion)
            }
        }
        .map_err(|error| CanonicalLoweringErrorV1::ResolvedRegionFlow {
            detail: format!("if_control_contract={error:?}"),
        })?;
        verify_product_shape(
            function,
            if_control.row_count(),
            if_control.explicit_else_count(),
            block_expr_count,
        )?;

        let profile = match (role, expression_policy, main_role) {
            (
                CanonicalFunctionRolePolicyV1::NormalMain0,
                FirstFamilyExpressionPolicyV1::Closed,
                Some(main_role),
            ) => analyze_trivial_canonical_main_owner_v1(
                function,
                &completion,
                &if_control,
                main_role,
            ),
            (
                CanonicalFunctionRolePolicyV1::NormalMainDirectCall0,
                FirstFamilyExpressionPolicyV1::ExactDirectCall,
                Some(main_role),
            ) => analyze_trivial_canonical_main_owner_with_finite_direct_calls_v1(
                function,
                &completion,
                &if_control,
                main_role,
            ),
            (
                CanonicalFunctionRolePolicyV1::OrdinaryFirstFamily,
                FirstFamilyExpressionPolicyV1::Closed,
                None,
            ) => analyze_trivial_canonical_owner_v1(function, &completion, &if_control),
            (
                CanonicalFunctionRolePolicyV1::OrdinaryFirstFamily,
                FirstFamilyExpressionPolicyV1::ExactDirectCall,
                None,
            ) => {
                debug_assert_eq!(
                    direct_call_admission,
                    DirectCallAdmissionV1::FiniteOneOrMore
                );
                analyze_trivial_canonical_owner_with_finite_direct_calls_v1(
                    function,
                    &completion,
                    &if_control,
                )
            }
            _ => {
                return unsupported(
                    "root",
                    function.source().root(),
                    "function_role_capability_mismatch",
                )
            }
        }
        .map_err(|error| CanonicalLoweringErrorV1::ResolvedRegionFlow {
            detail: format!("trivial_profile_contract={error:?}"),
        })?;
        match profile {
            TrivialCanonicalOwnerAnalysisV1::Admitted(profile) => {
                return Ok(CanonicalFirstFamilyPlanV1::TrivialBindingSsa(
                    CanonicalTrivialBindingSsaPlanV1 {
                        function,
                        if_control,
                        completion,
                        profile,
                        block_expr_count,
                    },
                ));
            }
            TrivialCanonicalOwnerAnalysisV1::NotAdmitted(_) => {
                if return_type_name.is_some() {
                    return unsupported(
                        "root",
                        function.source().root(),
                        "typed_return_profile_not_activated",
                    );
                }
                if param_decls
                    .iter()
                    .any(|declaration| declaration.declared_type_name.is_some())
                {
                    return unsupported(
                        "root",
                        function.source().root(),
                        "typed_parameter_profile_not_activated",
                    );
                }
            }
        }

        // Temporary A+ is selected only from an explicit whole-owner profile
        // stop. Contract failures above are canonical errors and never reach
        // this branch. The legacy RegionFlow analysis is therefore absent
        // from every admitted Binding-SSA route.
        let flow = analyze_resolved_function_flow_v1(function, &completion).map_err(|error| {
            CanonicalLoweringErrorV1::ResolvedRegionFlow {
                detail: format!("{error:?}"),
            }
        })?;
        Ok(CanonicalFirstFamilyPlanV1::CurrentCanonicalAPlus(
            CanonicalCurrentAPlusPlanV1 {
                function,
                flow,
                completion,
                block_expr_count,
            },
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReturnPolicyV1 {
    FinalOnly,
    Forbidden,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FirstFamilyExpressionPolicyV1 {
    Closed,
    ExactDirectCall,
}

fn verify_body(
    input: ResolvedFunctionLoweringInputV1<'_>,
    body: &LocatedBodyV1<'_>,
    return_policy: ReturnPolicyV1,
    expression_policy: FirstFamilyExpressionPolicyV1,
) -> Result<usize, CanonicalLoweringErrorV1> {
    let mut block_expr_count = 0;
    for index in 0..body.statements().len() {
        let statement = input
            .source()
            .body_stmt(body, index)
            .map_err(source_navigation)?;
        let is_last = index + 1 == body.statements().len();
        block_expr_count +=
            verify_statement(input, &statement, return_policy, is_last, expression_policy)?;
    }
    Ok(block_expr_count)
}

fn verify_statement(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &LocatedStmtV1<'_>,
    return_policy: ReturnPolicyV1,
    is_last: bool,
    expression_policy: FirstFamilyExpressionPolicyV1,
) -> Result<usize, CanonicalLoweringErrorV1> {
    let site = format!("{:?}", statement.site());
    match statement.node() {
        ASTNode::Local {
            variables,
            initial_values,
            declared_type_names,
            ..
        } => {
            if variables.is_empty()
                || initial_values.len() != variables.len()
                || declared_type_names.len() != variables.len()
                || declared_type_names.iter().any(Option::is_some)
            {
                return unsupported(site, statement.node(), "local_shape_not_closed");
            }
            let mut block_expr_count = 0;
            for (index, initial) in initial_values.iter().enumerate() {
                if initial.is_some() {
                    let initial = input
                        .source()
                        .child_expr_from_stmt(
                            statement,
                            ExprChildRoleV1::LocalInitializer(index as u32),
                        )
                        .map_err(source_navigation)?;
                    block_expr_count += verify_expression(input, &initial, expression_policy)?;
                }
            }
            Ok(block_expr_count)
        }
        ASTNode::Outbox {
            variables,
            initial_values,
            ..
        } => {
            if variables.is_empty()
                || initial_values.len() != variables.len()
                || initial_values.iter().any(Option::is_some)
            {
                return unsupported(site, statement.node(), "outbox_shape_not_closed");
            }
            Ok(0)
        }
        ASTNode::Assignment { target, .. } => {
            if !matches!(target.as_ref(), ASTNode::Variable { .. }) {
                return unsupported(site, target, "target_is_not_binding_rebind");
            }
            let value = input
                .source()
                .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentValue)
                .map_err(source_navigation)?;
            verify_expression(input, &value, expression_policy)
        }
        ASTNode::If { else_body, .. } => {
            let condition = input
                .source()
                .child_expr_from_stmt(statement, ExprChildRoleV1::IfCondition)
                .map_err(source_navigation)?;
            let mut block_expr_count = verify_expression(input, &condition, expression_policy)?;
            let then_body = input
                .source()
                .child_body_from_stmt(statement, BodyChildRoleV1::IfThen)
                .map_err(source_navigation)?;
            block_expr_count += verify_body(
                input,
                &then_body,
                ReturnPolicyV1::Forbidden,
                expression_policy,
            )?;
            if else_body.is_some() {
                let else_body = input
                    .source()
                    .child_body_from_stmt(statement, BodyChildRoleV1::IfElse)
                    .map_err(source_navigation)?;
                block_expr_count += verify_body(
                    input,
                    &else_body,
                    ReturnPolicyV1::Forbidden,
                    expression_policy,
                )?;
            }
            Ok(block_expr_count)
        }
        ASTNode::Return { value, .. } => {
            if return_policy == ReturnPolicyV1::Forbidden || !is_last {
                return unsupported(site, statement.node(), "return_not_allowed_here");
            }
            if value.is_some() {
                let value = input
                    .source()
                    .child_expr_from_stmt(statement, ExprChildRoleV1::ReturnValue)
                    .map_err(source_navigation)?;
                return verify_expression(input, &value, expression_policy);
            }
            Ok(0)
        }
        ASTNode::Literal { .. }
        | ASTNode::Variable { .. }
        | ASTNode::BinaryOp { .. }
        | ASTNode::BlockExpr { .. } => {
            let expression = input
                .source()
                .statement_expression(statement)
                .map_err(source_navigation)?;
            verify_expression(input, &expression, expression_policy)
        }
        _ => unsupported(site, statement.node(), "statement_not_in_first_family"),
    }
}

fn verify_expression(
    input: ResolvedFunctionLoweringInputV1<'_>,
    expression: &LocatedExprV1<'_>,
    expression_policy: FirstFamilyExpressionPolicyV1,
) -> Result<usize, CanonicalLoweringErrorV1> {
    let site = format!("{:?}", expression.site());
    match expression.node() {
        ASTNode::Literal { .. } | ASTNode::Variable { .. } => Ok(0),
        ASTNode::BinaryOp { operator, .. }
            if !matches!(operator, BinaryOperator::And | BinaryOperator::Or) =>
        {
            let left = input
                .source()
                .child_expr_from_expr(expression, ExprChildRoleV1::BinaryLeft)
                .map_err(source_navigation)?;
            let right = input
                .source()
                .child_expr_from_expr(expression, ExprChildRoleV1::BinaryRight)
                .map_err(source_navigation)?;
            Ok(verify_expression(input, &left, expression_policy)?
                + verify_expression(input, &right, expression_policy)?)
        }
        ASTNode::BlockExpr { .. } => {
            input
                .function()
                .block_expr_scope_region_pair(expression.owner(), expression.site())
                .map_err(|_| CanonicalLoweringErrorV1::UnsupportedFirstFamilyShape {
                    site: site.clone(),
                    actual: expression.node().node_type(),
                    reason: "blockexpr_pair_not_closed",
                })?;
            let prelude = input
                .source()
                .child_body_from_expr(expression, BodyChildRoleV1::BlockExprPrelude)
                .map_err(source_navigation)?;
            let prelude_count = verify_body(
                input,
                &prelude,
                ReturnPolicyV1::Forbidden,
                expression_policy,
            )?;
            let tail = input
                .source()
                .child_expr_from_expr(expression, ExprChildRoleV1::BlockExprTail)
                .map_err(source_navigation)?;
            Ok(1 + prelude_count + verify_expression(input, &tail, expression_policy)?)
        }
        ASTNode::FunctionCall { arguments, .. }
            if expression_policy == FirstFamilyExpressionPolicyV1::ExactDirectCall
                && input
                    .function()
                    .direct_call_target(expression.site())
                    .is_some() =>
        {
            let mut block_expr_count = 0;
            for index in 0..arguments.len() {
                let index = u32::try_from(index).map_err(|_| {
                    CanonicalLoweringErrorV1::UnsupportedFirstFamilyShape {
                        site: site.clone(),
                        actual: expression.node().node_type(),
                        reason: "direct_call_argument_index_overflow",
                    }
                })?;
                let argument = input
                    .source()
                    .child_expr_from_expr(expression, ExprChildRoleV1::CallArgument(index))
                    .map_err(source_navigation)?;
                block_expr_count += verify_expression(input, &argument, expression_policy)?;
            }
            Ok(block_expr_count)
        }
        _ => unsupported(site, expression.node(), "expression_not_in_first_family"),
    }
}

fn verify_product_shape(
    input: ResolvedFunctionLoweringInputV1<'_>,
    if_count: usize,
    explicit_else_count: usize,
    block_expr_count: usize,
) -> Result<(), CanonicalLoweringErrorV1> {
    let product = input.function();
    if product.owner() != input.owner()
        || input.forest().owner(input.owner()).is_none()
        || product
            .declaration_binding(&SourceBindingSiteV1::Receiver)
            .is_some()
    {
        return unsupported("product", input.source().root(), "owner_product_mismatch");
    }
    for (_, binding) in product.bindings() {
        if !matches!(
            binding.kind(),
            BindingKindV1::Parameter { .. }
                | BindingKindV1::Local { .. }
                | BindingKindV1::Outbox { .. }
        ) {
            return unsupported(
                "product.binding",
                input.source().root(),
                "binding_kind_not_closed",
            );
        }
    }
    let product_block_expr_scopes = product
        .scopes()
        .filter(|(_, scope)| scope.kind() == ScopeKindV1::BlockExpr)
        .count();
    let product_block_expr_regions = product
        .regions()
        .filter(|(_, region)| region.kind() == RegionKindV1::BlockExpr)
        .count();
    let expected_scope_count = 2 + block_expr_count + if_count + explicit_else_count;
    let expected_region_count = 2 + block_expr_count + (2 * if_count) + explicit_else_count;
    if product.scope_count() != expected_scope_count
        || product.region_count() != expected_region_count
        || product_block_expr_scopes != block_expr_count
        || product_block_expr_regions != block_expr_count
        || product
            .variable_refs()
            .any(|(_, reference)| !matches!(reference, ResolvedLexicalRefV1::Local(_)))
        || product
            .assignment_targets()
            .any(|(_, target)| !matches!(target, ResolvedAssignmentTargetV1::BindingRebind(_)))
        || product
            .resolved_exits()
            .any(|(_, exit)| exit.origin() != ResolvedExitOriginV1::ExplicitReturn)
        || product.scopes().any(|(_, scope)| {
            !matches!(
                scope.kind(),
                ScopeKindV1::Function
                    | ScopeKindV1::LexicalBlock
                    | ScopeKindV1::BlockExpr
                    | ScopeKindV1::IfThen
                    | ScopeKindV1::IfElse
            )
        })
        || product.regions().any(|(_, region)| {
            !matches!(
                region.kind(),
                RegionKindV1::Function
                    | RegionKindV1::Sequence
                    | RegionKindV1::BlockExpr
                    | RegionKindV1::If
                    | RegionKindV1::IfThen
                    | RegionKindV1::IfElse
            )
        })
    {
        return unsupported(
            "product",
            input.source().root(),
            "semantic_shape_not_closed",
        );
    }
    Ok(())
}

fn source_navigation(error: impl ToString) -> CanonicalLoweringErrorV1 {
    CanonicalLoweringErrorV1::SourceNavigation {
        detail: error.to_string(),
    }
}

fn unsupported<T>(
    site: impl Into<String>,
    node: &ASTNode,
    reason: &'static str,
) -> Result<T, CanonicalLoweringErrorV1> {
    Err(CanonicalLoweringErrorV1::UnsupportedFirstFamilyShape {
        site: site.into(),
        actual: node.node_type(),
        reason,
    })
}
