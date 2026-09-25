//! Loop Route Router - frozen located-source entry (M10b-I0-R0).
//!
//! # Architecture
//!
//! `route_loop` runs one fixed order with no re-decision:
//!   parser-issued source -> resolved source unit -> exact loop
//!   membership -> winner spine (`issue_loop_node_winner_recipe_v1`) ->
//!   physical admission (`issue_loop_node_physical_admission_v1`) ->
//!   `lower_loop_node_physical_admission_v1` emits MIR.
//!
//! The retired plan/composer ordering machinery is gone; there is no
//! fallback route selection in this file.
//!
//! # Adding New Loop Routes
//!
//! Extend the winner-spine/admission contract under `src/mir/compiler/`;
//! the router only threads resolver products through the fixed order.

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;
use std::rc::Rc;

use crate::mir::loop_route_detection::LoopRouteKind;

// M10b-I0-R0: the frozen entry consumes only resolver products and the
// caller-zero spine/admission/physicalizer edges landed under P1/P2.
use crate::mir::builder::control_flow::lower::Freeze;
use crate::mir::builder::emission::constant::emit_void;
use crate::mir::builder::resolved_lowering::loop_recipe_physicalizer::lower_loop_node_physical_admission_v1;
use crate::mir::compiler::loop_node_physical_admission::issue_loop_node_physical_admission_v1;
use crate::mir::compiler::loop_node_winner_spine::{
    issue_loop_node_winner_recipe_v1, LoopNodeWinnerSpineOutcomeV1,
};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::numeric_substrate::NumericTarget;

/// Context passed to loop route detection/lowering functions.
pub(crate) struct LoopRouteContext<'a> {
    /// Loop condition AST node
    pub condition: &'a ASTNode,

    /// Loop body statements
    pub body: &'a [ASTNode],

    /// Current function name (for routing)
    pub func_name: &'a str,

    /// Debug logging enabled
    pub debug: bool,

    /// In static box context? (affects scan-predicate route behavior)
    pub in_static_box: bool,

    /// Phase 192: Loop route classification based on features.
    pub route_kind: LoopRouteKind,

    /// Phase 200-C: Optional function body AST for capture analysis
    /// None if not available, Some(&[ASTNode]) if function body is accessible
    pub fn_body: Option<&'a [ASTNode]>,

    /// Phase 188.3: Cached StepTree max_loop_depth for nested-loop minimal routes.
    /// None if not computed, Some(depth) when nested-loop candidate is present.
    /// Avoids re-building StepTree in lowering phase
    pub step_tree_max_loop_depth: Option<u32>,
}

impl<'a> LoopRouteContext<'a> {
    /// Create new context from routing parameters
    ///
    /// Automatically detects continue/break statements in body
    /// Extracts features and classifies route kind from AST
    /// Detects infinite loop condition
    /// Uses choose_route_kind() SSOT entry point
    pub(crate) fn new(
        condition: &'a ASTNode,
        body: &'a [ASTNode],
        func_name: &'a str,
        debug: bool,
        in_static_box: bool,
    ) -> Self {
        // Phase 137-6-S1: Use SSOT route selection entry point
        use crate::mir::builder::control_flow::joinir::routing::choose_route_kind;
        let route_kind = choose_route_kind(condition, body);

        Self {
            condition,
            body,
            func_name,
            debug,
            in_static_box,
            route_kind,
            fn_body: None,                  // Phase 200-C: Default to None
            step_tree_max_loop_depth: None, // Phase 188.3: Default to None
        }
    }

    /// Phase 200-C: Create context with fn_body for capture analysis
    pub(crate) fn with_fn_body(
        condition: &'a ASTNode,
        body: &'a [ASTNode],
        func_name: &'a str,
        debug: bool,
        in_static_box: bool,
        fn_body: &'a [ASTNode],
    ) -> Self {
        let mut ctx = Self::new(condition, body, func_name, debug, in_static_box);
        ctx.fn_body = Some(fn_body);
        ctx
    }
}

// M10b-I0-R0: frozen located-source entry.
//
// One fixed order with no re-decision:
//   parser-issued FunctionDeclaration -> resolved source unit -> exact loop
//   membership -> one policy winner -> verified recipe -> one physical
//   admission -> one canonical physicalizer -> outer publication.
//
// Every failure is a typed terminal Freeze. There is no ordered schedule,
// no retry, no `Ok(None)` tail, and no second physical owner.
pub(crate) fn route_loop(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
) -> Result<Option<ValueId>, String> {
    // Source authority: the FunctionDeclaration recorded at function entry.
    // A missing declaration means the walked loop has no canonical source
    // identity — a contract freeze, never a silent skip.
    let Some(declaration) = builder
        .function_state
        .compilation
        .fn_declaration_ast
        .clone()
    else {
        return Err(Freeze::contract(
            "route_loop requires a parser-issued FunctionDeclaration in the lowering session",
        )
        .to_string());
    };

    // Resolve the function's source unit once; sibling loops share it.
    let unit = if let Some(unit) = &builder
        .function_state
        .compilation
        .resolved_loop_source_unit
    {
        Rc::clone(unit)
    } else {
        let unit = Rc::new(
            VerifiedResolvedSourceUnitV1::resolve_function(declaration)
                .map_err(|error| error.to_string())?,
        );
        builder
            .function_state
            .compilation
            .resolved_loop_source_unit = Some(Rc::clone(&unit));
        unit
    };
    let input = unit
        .root_function_input()
        .map_err(|error| error.to_string())?;

    // Exact membership: the walked loop must equal exactly one resolver-
    // inventoried Loop statement by structure. Zero or two matches are
    // terminal — no index, span, or ordinal re-derivation.
    let mut matched = None;
    for site in input.function().loop_sites() {
        let located = input
            .source()
            .exact_stmt(site)
            .map_err(|error| error.to_string())?;
        let ASTNode::Loop {
            condition, body, ..
        } = located.node()
        else {
            return Err(Freeze::contract(
                "resolver loop inventory projected a non-Loop statement",
            )
            .to_string());
        };
        if condition.as_ref() == ctx.condition && body.as_slice() == ctx.body {
            if matched.replace(located).is_some() {
                return Err(Freeze::contract(
                    "ambiguous loop membership: two resolver sites match one walked loop",
                )
                .to_string());
            }
        }
    }
    let Some(loop_stmt) = matched else {
        return Err(Freeze::contract(
            "walked loop is absent from the resolver loop inventory",
        )
        .to_string());
    };

    // One policy winner -> one verified recipe.
    let issued = match issue_loop_node_winner_recipe_v1(input, loop_stmt, NumericTarget::host()) {
        LoopNodeWinnerSpineOutcomeV1::Issued(issued) => issued,
        LoopNodeWinnerSpineOutcomeV1::Declined(_) => {
            return Err(Freeze::contract(
                "loop winner selection declined: zero selected family candidates",
            )
            .to_string())
        }
        LoopNodeWinnerSpineOutcomeV1::Unresolved(failure)
        | LoopNodeWinnerSpineOutcomeV1::Rejected(failure) => {
            return Err(Freeze::contract(&format!(
                "loop winner spine terminal: {failure:?}"
            ))
            .to_string())
        }
    };

    // One physical admission -> one canonical physicalizer.
    let admission = issue_loop_node_physical_admission_v1(input, issued).map_err(|error| {
        Freeze::contract(&format!("loop node physical admission terminal: {error:?}")).to_string()
    })?;
    let continuation = lower_loop_node_physical_admission_v1(builder, input, admission)?;

    // Outer publication: the walk adopts post-loop values by resolver
    // binding name and continues at the sealed root After block.
    for (binding, value) in continuation.writebacks() {
        let record = input.function().binding(*binding).ok_or_else(|| {
            Freeze::contract("loop writeback binding unknown to resolver inventory").to_string()
        })?;
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert(record.diagnostic_name().to_owned(), *value);
    }
    builder.function_state.current_block = Some(continuation.root_after());
    let result = emit_void(builder)?;
    Ok(Some(result))
}
