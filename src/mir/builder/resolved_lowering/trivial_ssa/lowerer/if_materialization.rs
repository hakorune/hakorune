//! Common located If materialization; old selected Recipe wire path retired.
use super::{require_representation, CanonicalTrivialSsaLowererV1};
use crate::ast::ASTNode;
use crate::mir::compiler::located::LocatedStmtV1;
use crate::mir::compiler::source_view::{BodyChildRoleV1, ExprChildRoleV1};
use crate::mir::resolved_control_flow::if_control::ResolvedIfElsePortV1;
use crate::mir::resolved_semantics::{RegionKindV1, ScopeKindV1};
use crate::mir::resolved_value_profile::product::TrivialRepresentationV1;

impl<'builder, 'source> CanonicalTrivialSsaLowererV1<'builder, 'source> {
    pub(super) fn lower_if(&mut self, statement: &LocatedStmtV1<'source>) -> Result<(), String> {
        let ASTNode::If { else_body, .. } = statement.node() else {
            unreachable!("If helper requires If")
        };
        let mut row = self
            .session
            .claim_if_control(statement)
            .map_err(|error| format!("[freeze:contract][if_control/claim] {error:?}"))?;
        row.claim_statement(statement)
            .map_err(|error| format!("[freeze:contract][if_control/statement] {error:?}"))?;
        let condition = self
            .input
            .source()
            .child_expr_from_stmt(statement, ExprChildRoleV1::IfCondition)
            .map_err(|error| error.to_string())?;
        let (condition, representation) = self.lower_expr(&condition, Some(&mut row))?;
        require_representation(
            representation,
            TrivialRepresentationV1::InlineBool,
            "if_condition",
        )?;
        let regions = row.regions();
        let control = self.session.semantics.enter_region(
            self.input.function(),
            regions.control(),
            RegionKindV1::If,
        )?;
        let header = self.current_block()?;
        let then_block = self.builder.next_block_id();
        let explicit_else = matches!(row.else_port(), ResolvedIfElsePortV1::Explicit(_));
        let else_block = explicit_else.then(|| self.builder.next_block_id());
        let merge = self.builder.next_block_id();
        self.builder.ensure_block_exists(then_block)?;
        if let Some(block) = else_block {
            self.builder.ensure_block_exists(block)?;
        }
        self.builder.ensure_block_exists(merge)?;
        let false_target = else_block.unwrap_or(merge);
        {
            let cfg = &self.session.cfg()?;
            let function = self
                .builder
                .function_state
                .current_function
                .as_mut()
                .ok_or_else(|| {
                    "[freeze:contract][canonical_binding_ssa/function_missing]".to_string()
                })?;
            cfg.emit_branch(function, header, condition, then_block, false_target)
                .map_err(|error| error.to_string())?;
        }
        self.seal_block_if_needed(header)?;
        self.seal_block_if_needed(then_block)?;
        self.builder.start_new_block(then_block)?;
        let then_body = self
            .input
            .source()
            .child_body_from_stmt(statement, BodyChildRoleV1::IfThen)
            .map_err(|error| error.to_string())?;
        row.claim_body(&then_body)
            .map_err(|error| format!("[freeze:contract][if_control/then_body] {error:?}"))?;
        let then_scope = self.session.semantics.enter_scope_region(
            self.input.function(),
            regions.then_pair(),
            ScopeKindV1::IfThen,
            RegionKindV1::IfThen,
        )?;
        self.lower_body(&then_body, Some(&mut row))?;
        self.session
            .semantics
            .close_scope_region_success(then_scope, &mut self.session.identity)?;
        let then_exit = self.current_block()?;
        self.emit_jump(then_exit, merge)?;
        if let Some(else_block_id) = else_block {
            self.seal_block_if_needed(else_block_id)?;
            self.builder.start_new_block(else_block_id)?;
            let else_body = self
                .input
                .source()
                .child_body_from_stmt(statement, BodyChildRoleV1::IfElse)
                .map_err(|error| error.to_string())?;
            row.claim_body(&else_body)
                .map_err(|error| format!("[freeze:contract][if_control/else_body] {error:?}"))?;
            let pair = regions.else_pair().ok_or_else(|| {
                "[freeze:contract][canonical_binding_ssa/else_pair_missing]".to_string()
            })?;
            let else_scope = self.session.semantics.enter_scope_region(
                self.input.function(),
                pair,
                ScopeKindV1::IfElse,
                RegionKindV1::IfElse,
            )?;
            self.lower_body(&else_body, Some(&mut row))?;
            self.session
                .semantics
                .close_scope_region_success(else_scope, &mut self.session.identity)?;
            let branch_exit = self.current_block()?;
            self.emit_jump(branch_exit, merge)?;
        } else if else_body.is_some() || regions.else_pair().is_some() {
            return Err("[freeze:contract][canonical_binding_ssa/else_topology]".to_string());
        }
        self.seal_block_if_needed(merge)?;
        self.builder.start_new_block(merge)?;
        row.finish_coverage()
            .map_err(|error| format!("[freeze:contract][if_control/coverage] {error:?}"))?;
        let _representation_only = self.profile.claim_if_merges(statement.site())?;
        self.session.semantics.close_region(control)?;
        Ok(())
    }
}
