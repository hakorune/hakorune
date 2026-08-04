//! If-only topology and physical materialization for the canonical trivial route.
//!
//! This child module owns the existing If lowering mechanics without changing
//! acceptance, caller routing, diagnostics, or CFG/SSA/PHI authority.

use crate::ast::ASTNode;
use crate::mir::builder::resolved_lowering::if_recipe_adapter::CanonicalIfRecipeNodeDemandV1;
use crate::mir::builder::resolved_lowering::trivial_ssa::if_recipe_physicalizer::{
    CanonicalIfPhysicalReceiptV1, CanonicalIfPhysicalValuesV1,
    CanonicalIfRecipeExplicitElseTopologyV1, CanonicalIfRecipeTopologyV1,
};
use crate::mir::compiler::located::LocatedStmtV1;
use crate::mir::compiler::source_view::{BodyChildRoleV1, ExprChildRoleV1};
use crate::mir::resolved_control_flow::if_control::ResolvedIfElsePortV1;
use crate::mir::resolved_semantics::{BindingRefV1, RegionKindV1, ScopeKindV1};
use crate::mir::resolved_value_profile::product::TrivialRepresentationV1;

use super::{require_representation, CanonicalTrivialSsaLowererV1};

enum IfMaterializationTopologyV1 {
    Legacy,
    Selected(CanonicalIfRecipeTopologyV1),
}

impl IfMaterializationTopologyV1 {
    fn selected_binding(&self) -> Option<BindingRefV1> {
        match self {
            Self::Legacy => None,
            Self::Selected(topology) => Some(topology.binding()),
        }
    }
}

enum IfMaterializationOutcomeV1 {
    Legacy,
    Selected(CanonicalIfPhysicalReceiptV1),
}

impl<'builder, 'source> CanonicalTrivialSsaLowererV1<'builder, 'source> {
    pub(super) fn lower_if(&mut self, statement: &LocatedStmtV1<'source>) -> Result<(), String> {
        if self.if_recipe.is_not_selected() {
            return self.lower_if_legacy_unselected(statement);
        }
        let demand = self
            .if_recipe
            .take_if(statement)
            .map_err(|error| format!("[freeze:contract][if_recipe/take] {error:?}"))?;
        match demand {
            CanonicalIfRecipeNodeDemandV1::Single(demand) => {
                super::super::if_recipe_physicalizer::physicalize_if_recipe_v1(
                    self, statement, demand,
                )
                .map(|_| ())
            }
            CanonicalIfRecipeNodeDemandV1::Nested(demand) => {
                super::super::nested_if_proof::lower(self, statement, demand)
            }
        }
    }

    fn lower_if_legacy_unselected(
        &mut self,
        statement: &LocatedStmtV1<'source>,
    ) -> Result<(), String> {
        self.lower_if_materialization_core(statement, IfMaterializationTopologyV1::Legacy)
            .map(|_| ())
    }

    pub(in crate::mir::builder::resolved_lowering::trivial_ssa) fn lower_if_recipe_selected(
        &mut self,
        statement: &LocatedStmtV1<'source>,
        topology: CanonicalIfRecipeTopologyV1,
    ) -> Result<CanonicalIfPhysicalReceiptV1, String> {
        match self.lower_if_materialization_core(
            statement,
            IfMaterializationTopologyV1::Selected(topology),
        )? {
            IfMaterializationOutcomeV1::Selected(receipt) => Ok(receipt),
            IfMaterializationOutcomeV1::Legacy => {
                Err("[freeze:contract][if_recipe/selected_topology_not_materialized]".to_string())
            }
        }
    }

    pub(in crate::mir::builder::resolved_lowering::trivial_ssa) fn lower_nested_if_recipe_selected(
        &mut self,
        statement: &LocatedStmtV1<'source>,
        binding: BindingRefV1,
    ) -> Result<(), String> {
        let topology = CanonicalIfRecipeTopologyV1::ExplicitElse(
            CanonicalIfRecipeExplicitElseTopologyV1::new(binding),
        );
        self.lower_if_recipe_selected(statement, topology)
            .map(|_| ())
    }

    fn lower_if_materialization_core(
        &mut self,
        statement: &LocatedStmtV1<'source>,
        topology: IfMaterializationTopologyV1,
    ) -> Result<IfMaterializationOutcomeV1, String> {
        let ASTNode::If { else_body, .. } = statement.node() else {
            unreachable!("If helper requires If")
        };
        let mut row = self
            .session
            .if_control
            .claim(statement)
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
        let selected_binding = topology.selected_binding();
        let selected_baseline = match (&topology, selected_binding) {
            (IfMaterializationTopologyV1::Selected(topology), Some(binding))
                if !topology.is_explicit_else() =>
            {
                Some(self.session.identity.read_entry(
                    self.builder,
                    &mut self.session.phis,
                    header,
                    binding,
                )?)
            }
            _ => None,
        };
        let explicit_else = match &topology {
            IfMaterializationTopologyV1::Legacy => {
                matches!(row.else_port(), ResolvedIfElsePortV1::Explicit(_))
            }
            IfMaterializationTopologyV1::Selected(topology) => topology.is_explicit_else(),
        };
        let else_block = explicit_else.then(|| self.builder.next_block_id());
        let merge = self.builder.next_block_id();
        self.builder.ensure_block_exists(then_block)?;
        if let Some(block) = else_block {
            self.builder.ensure_block_exists(block)?;
        }
        self.builder.ensure_block_exists(merge)?;
        let false_target = else_block.unwrap_or(merge);
        {
            let cfg = &self.session.cfg;
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
        let then_value = selected_binding
            .map(|binding| {
                self.session.identity.read_entry(
                    self.builder,
                    &mut self.session.phis,
                    then_exit,
                    binding,
                )
            })
            .transpose()?;
        self.emit_jump(then_exit, merge)?;
        let mut else_exit = None;
        let mut else_value = None;
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
            let branch_value = selected_binding
                .map(|binding| {
                    self.session.identity.read_entry(
                        self.builder,
                        &mut self.session.phis,
                        branch_exit,
                        binding,
                    )
                })
                .transpose()?;
            self.emit_jump(branch_exit, merge)?;
            else_exit = Some(branch_exit);
            else_value = branch_value;
        } else if else_body.is_some() || regions.else_pair().is_some() {
            return Err("[freeze:contract][canonical_binding_ssa/else_topology]".to_string());
        }
        self.seal_block_if_needed(merge)?;
        self.builder.start_new_block(merge)?;
        row.finish_coverage()
            .map_err(|error| format!("[freeze:contract][if_control/coverage] {error:?}"))?;
        let _representation_only = self.profile.claim_if_merges(statement.site())?;
        self.session.semantics.close_region(control)?;
        if let (IfMaterializationTopologyV1::Selected(topology), Some(then_value)) =
            (topology, then_value)
        {
            let values = match (selected_baseline, else_block, else_exit, else_value) {
                (Some(baseline_value), None, None, None) => {
                    CanonicalIfPhysicalValuesV1::ImplicitFallthrough { baseline_value }
                }
                (None, Some(else_block), Some(else_exit), Some(else_value)) => {
                    CanonicalIfPhysicalValuesV1::ExplicitElse {
                        else_block,
                        else_exit,
                        else_value,
                    }
                }
                _ => return Err("[freeze:contract][if_recipe/physical_values_shape]".to_string()),
            };
            let receipt = super::super::if_recipe_physicalizer::selected_receipt(
                topology, header, condition, then_block, then_exit, merge, then_value, values,
            )?;
            return Ok(IfMaterializationOutcomeV1::Selected(receipt));
        }
        Ok(IfMaterializationOutcomeV1::Legacy)
    }
}
