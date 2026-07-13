//! Atomic production lowering for located fallthrough statement `If`.
//!
//! RegionFlow owns every effect and join-source decision. This module only
//! materializes that immutable contract from one post-condition value baseline.

use crate::ast::ASTNode;
use crate::mir::compiler::located::{LocatedBodyV1, LocatedStmtV1};
use crate::mir::compiler::source_view::{BodyChildRoleV1, ExprChildRoleV1};
use crate::mir::resolved_region_flow::{ResolvedElseFallthroughV1, VerifiedResolvedIfFlowV1};
use crate::mir::resolved_semantics::{
    RegionKindV1, ResolvedScopeRegionPairV1, ScopeId, ScopeKindV1,
};

use super::branch_transaction::{ResolvedBranchExitValuesV1, ResolvedBranchTransactionV1};
use super::identity::ResolvedIdentityStateV1;
use super::if_materialization::{
    define_join_phis, DefinedJoinPublishV1, DefinedJoinValueStoreV1, IfCfgSessionV1,
};
use super::lowerer::CanonicalFunctionLowererV1;

#[derive(Debug, Clone, Copy)]
enum ExplicitBranchV1 {
    Then,
    Else,
}

impl<'builder, 'source> CanonicalFunctionLowererV1<'builder, 'source> {
    pub(super) fn lower_statement_if(
        &mut self,
        statement: &LocatedStmtV1<'source>,
    ) -> Result<(), String> {
        let ASTNode::If { else_body, .. } = statement.node() else {
            unreachable!("lower_statement_if is called only for If")
        };
        let row = self.flow.claim_next_if(statement.site())?;
        self.effects.prime_current_effects(
            self.input.function(),
            row.whole_effects().may_rebind_outer(),
        )?;

        let regions = row.regions();
        let surrounding_scope = self
            .input
            .function()
            .scope(regions.then_pair().scope())
            .and_then(|scope| scope.parent())
            .ok_or_else(|| {
                "[freeze:contract][canonical_if/surrounding_scope_missing]".to_string()
            })?;
        self.verify_else_topology(&row, else_body.is_some())?;

        let condition = self
            .input
            .source()
            .child_expr_from_stmt(statement, ExprChildRoleV1::IfCondition)
            .map_err(|error| error.to_string())?;
        let condition_value = self.lower_if_condition(&row, surrounding_scope, &condition)?;

        let join_domain = row
            .join()
            .rows()
            .iter()
            .map(|join| join.binding())
            .collect::<Vec<_>>();
        let then_transaction = ResolvedBranchTransactionV1::snapshot(
            &self.identity,
            &join_domain,
            row.then_port().may_rebind_outer(),
        )?;
        let else_permits = row
            .else_port()
            .explicit_port()
            .map(|port| port.may_rebind_outer())
            .unwrap_or(&[]);
        // This second snapshot happens before either branch and therefore
        // seals the same post-condition baseline for the false route.
        let else_transaction =
            ResolvedBranchTransactionV1::snapshot(&self.identity, &join_domain, else_permits)?;

        let control = self.semantics.enter_region(
            self.input.function(),
            regions.control(),
            RegionKindV1::If,
        )?;
        let opened = match row.else_port() {
            ResolvedElseFallthroughV1::ImplicitIdentity => {
                IfCfgSessionV1::open_implicit_false(self.builder, condition_value)
            }
            ResolvedElseFallthroughV1::Explicit(_) => {
                IfCfgSessionV1::open_explicit_else(self.builder, condition_value)
            }
        };
        let mut cfg = match opened {
            Ok(cfg) => cfg,
            Err(primary) => {
                let close = self.semantics.close_region(control);
                return Err(with_cleanup("canonical_if/open", primary, [close]));
            }
        };

        let primary = self.materialize_open_if(
            statement,
            &row,
            then_transaction,
            else_transaction,
            &mut cfg,
        );
        match primary {
            Ok(()) => self.semantics.close_region(control),
            Err(primary) => {
                let restore = cfg.restore_header_after_error(self.builder);
                let close = self.semantics.close_region(control);
                Err(with_cleanup(
                    "canonical_if/materialize",
                    primary,
                    [restore, close],
                ))
            }
        }
    }

    fn lower_if_condition(
        &mut self,
        row: &VerifiedResolvedIfFlowV1,
        surrounding_scope: ScopeId,
        condition: &crate::mir::compiler::located::LocatedExprV1<'source>,
    ) -> Result<crate::mir::ValueId, String> {
        self.flow.begin_condition(row)?;
        if let Err(primary) = self.effects.push_condition(
            self.input.function(),
            surrounding_scope,
            row.condition_effects().may_rebind_outer(),
        ) {
            let abort = self.flow.abort_condition(row.site());
            return Err(with_cleanup(
                "canonical_if/condition_open",
                primary,
                [abort],
            ));
        }

        match self.lower_expr(condition) {
            Ok(value) => {
                if let Err(primary) = self.flow.finish_condition(row.site()) {
                    let abort = self.flow.abort_condition(row.site());
                    let close = self.effects.finish_condition(surrounding_scope);
                    return Err(with_cleanup(
                        "canonical_if/condition_finish",
                        primary,
                        [abort, close],
                    ));
                }
                self.effects.finish_condition(surrounding_scope)?;
                Ok(value)
            }
            Err(primary) => {
                let abort = self.flow.abort_condition(row.site());
                let close = self.effects.finish_condition(surrounding_scope);
                Err(with_cleanup(
                    "canonical_if/condition_error",
                    primary,
                    [abort, close],
                ))
            }
        }
    }

    fn materialize_open_if(
        &mut self,
        statement: &LocatedStmtV1<'source>,
        row: &VerifiedResolvedIfFlowV1,
        then_transaction: ResolvedBranchTransactionV1,
        else_transaction: ResolvedBranchTransactionV1,
        cfg: &mut IfCfgSessionV1,
    ) -> Result<(), String> {
        let then_body = self
            .input
            .source()
            .child_body_from_stmt(statement, BodyChildRoleV1::IfThen)
            .map_err(|error| error.to_string())?;
        let (then_transaction, then_values) = self.lower_explicit_if_branch(
            row,
            ExplicitBranchV1::Then,
            row.regions().then_pair(),
            then_transaction,
            &then_body,
            cfg,
        )?;

        let else_values = match row.else_port() {
            ResolvedElseFallthroughV1::ImplicitIdentity => else_transaction.implicit_false_values(),
            ResolvedElseFallthroughV1::Explicit(_) => {
                let else_body = self
                    .input
                    .source()
                    .child_body_from_stmt(statement, BodyChildRoleV1::IfElse)
                    .map_err(|error| error.to_string())?;
                let else_pair = row.regions().else_pair().ok_or_else(|| {
                    "[freeze:contract][canonical_if/explicit_else_pair_missing]".to_string()
                })?;
                let (_, values) = self.lower_explicit_if_branch(
                    row,
                    ExplicitBranchV1::Else,
                    else_pair,
                    else_transaction,
                    &else_body,
                    cfg,
                )?;
                values
            }
        };

        let predecessors = cfg.verify_actual_predecessors(self.builder)?;
        let join_rows =
            then_transaction.join_rows_for_contract(row.join(), &then_values, &else_values)?;
        let defined = define_join_phis(self.builder, predecessors, &join_rows)?;
        let mut store = EffectAwareJoinStoreV1 {
            identity: &mut self.identity,
            effects: &self.effects,
            product: self.input.function(),
        };
        defined.publish_join_values(&mut store)
    }

    fn lower_explicit_if_branch(
        &mut self,
        row: &VerifiedResolvedIfFlowV1,
        branch: ExplicitBranchV1,
        pair: ResolvedScopeRegionPairV1,
        transaction: ResolvedBranchTransactionV1,
        body: &LocatedBodyV1<'source>,
        cfg: &mut IfCfgSessionV1,
    ) -> Result<(ResolvedBranchTransactionV1, ResolvedBranchExitValuesV1), String> {
        match branch {
            ExplicitBranchV1::Then => cfg.enter_then(self.builder)?,
            ExplicitBranchV1::Else => cfg.enter_else(self.builder)?,
        }
        let (scope_kind, region_kind) = match branch {
            ExplicitBranchV1::Then => (ScopeKindV1::IfThen, RegionKindV1::IfThen),
            ExplicitBranchV1::Else => (ScopeKindV1::IfElse, RegionKindV1::IfElse),
        };
        let semantic = self.semantics.enter_scope_region(
            self.input.function(),
            pair,
            scope_kind,
            region_kind,
        )?;
        if let Err(primary) = self.begin_branch_coverage(row, branch) {
            let close = self
                .semantics
                .close_scope_region_error(semantic, &mut self.identity);
            return Err(with_cleanup(
                "canonical_if/branch_coverage_open",
                primary,
                [close],
            ));
        }
        if let Err(primary) =
            self.effects
                .push_branch(self.input.function(), pair.scope(), transaction)
        {
            let abort = self.abort_branch_coverage(row, branch);
            let close = self
                .semantics
                .close_scope_region_error(semantic, &mut self.identity);
            return Err(with_cleanup(
                "canonical_if/branch_effect_open",
                primary,
                [abort, close],
            ));
        }

        if let Err(primary) = self.lower_body(body) {
            let abort = self.abort_branch_coverage(row, branch);
            let restore = self
                .effects
                .restore_branch(&mut self.identity, pair.scope());
            let close = self
                .semantics
                .close_scope_region_error(semantic, &mut self.identity);
            return Err(with_cleanup(
                "canonical_if/branch_body",
                primary,
                [abort, restore, close],
            ));
        }
        if let Err(primary) = self.finish_branch_coverage(row, branch) {
            let abort = self.abort_branch_coverage(row, branch);
            let restore = self
                .effects
                .restore_branch(&mut self.identity, pair.scope());
            let close = self
                .semantics
                .close_scope_region_error(semantic, &mut self.identity);
            return Err(with_cleanup(
                "canonical_if/branch_coverage_finish",
                primary,
                [abort, restore, close],
            ));
        }
        let (transaction, values) = match self
            .effects
            .capture_branch(&mut self.identity, pair.scope())
        {
            Ok(values) => values,
            Err(primary) => {
                let close = self
                    .semantics
                    .close_scope_region_error(semantic, &mut self.identity);
                return Err(with_cleanup(
                    "canonical_if/branch_capture",
                    primary,
                    [close],
                ));
            }
        };
        self.semantics
            .close_scope_region_success(semantic, &mut self.identity)?;
        match branch {
            ExplicitBranchV1::Then => cfg.close_then(self.builder)?,
            ExplicitBranchV1::Else => cfg.close_else(self.builder)?,
        }
        Ok((transaction, values))
    }

    fn begin_branch_coverage(
        &mut self,
        row: &VerifiedResolvedIfFlowV1,
        branch: ExplicitBranchV1,
    ) -> Result<(), String> {
        match branch {
            ExplicitBranchV1::Then => self.flow.begin_then(row),
            ExplicitBranchV1::Else => self.flow.begin_else(row),
        }
    }

    fn finish_branch_coverage(
        &mut self,
        row: &VerifiedResolvedIfFlowV1,
        branch: ExplicitBranchV1,
    ) -> Result<(), String> {
        match branch {
            ExplicitBranchV1::Then => self.flow.finish_then(row.site()),
            ExplicitBranchV1::Else => self.flow.finish_else(row.site()),
        }
    }

    fn abort_branch_coverage(
        &mut self,
        row: &VerifiedResolvedIfFlowV1,
        branch: ExplicitBranchV1,
    ) -> Result<(), String> {
        match branch {
            ExplicitBranchV1::Then => self.flow.abort_then(row.site()),
            ExplicitBranchV1::Else => self.flow.abort_else(row.site()),
        }
    }

    fn verify_else_topology(
        &self,
        row: &VerifiedResolvedIfFlowV1,
        syntax_has_else: bool,
    ) -> Result<(), String> {
        let semantic_has_else = row.regions().else_pair().is_some();
        let flow_has_else = row.else_port().explicit_port().is_some();
        if syntax_has_else == semantic_has_else && syntax_has_else == flow_has_else {
            Ok(())
        } else {
            Err(format!(
                "[freeze:contract][canonical_if/else_topology_mismatch] syntax={syntax_has_else} semantic={semantic_has_else} flow={flow_has_else}"
            ))
        }
    }
}

struct EffectAwareJoinStoreV1<'a, 'source> {
    identity: &'a mut ResolvedIdentityStateV1<'source>,
    effects: &'a super::branch_transaction::ResolvedActiveEffectStackV1,
    product: &'source crate::mir::resolved_semantics::VerifiedResolvedFunctionV1,
}

impl DefinedJoinValueStoreV1 for EffectAwareJoinStoreV1<'_, '_> {
    fn defined_join_current_value(
        &self,
        binding: crate::mir::resolved_semantics::BindingRefV1,
    ) -> Result<crate::mir::ValueId, String> {
        self.identity.current_value(binding)
    }

    fn publish_defined_join_batch(
        &mut self,
        publishes: Vec<DefinedJoinPublishV1>,
    ) -> Result<(), String> {
        if !self.effects.is_empty() {
            for publish in &publishes {
                self.effects
                    .authorize_current(self.product, publish.binding())?;
            }
        }
        <ResolvedIdentityStateV1<'_> as DefinedJoinValueStoreV1>::publish_defined_join_batch(
            self.identity,
            publishes,
        )
    }
}

fn with_cleanup<const N: usize>(
    owner: &str,
    primary: String,
    cleanups: [Result<(), String>; N],
) -> String {
    let failures = cleanups
        .into_iter()
        .filter_map(Result::err)
        .collect::<Vec<_>>();
    if failures.is_empty() {
        primary
    } else {
        format!(
            "[freeze:contract][{owner}/during_cleanup] primary={primary} cleanup={}",
            failures.join(" | ")
        )
    }
}
