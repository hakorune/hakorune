//! Behavior-neutral orchestration for legacy block descent.
//!
//! Source navigation and statement lowering remain caller-owned. This driver
//! owns only scope lifetime, suffix-step sequencing, termination, last-value,
//! and empty-block Void publication.

use crate::mir::builder::MirBuilder;
use crate::mir::utils::is_current_block_terminated;
use crate::mir::ValueId;

use crate::ast::ASTNode;

pub(in crate::mir::builder) trait LegacyBlockDescentPortV1 {
    fn len(&self) -> usize;

    fn suffix_route_input(&self, index: usize) -> Option<&[ASTNode]>;

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        index: usize,
    ) -> Result<ValueId, String>;
}

pub(in crate::mir::builder) fn drive_legacy_block_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
) -> Result<ValueId, String>
where
    Port: LegacyBlockDescentPortV1,
{
    let trace = crate::mir::builder::control_flow::joinir::trace::trace();
    let scope_id = builder.current_block.map(|bb| bb.as_u32()).unwrap_or(0);
    builder.hint_scope_enter(scope_id);
    let _lex_scope = super::super::vars::lexical_scope::LexicalScopeGuard::new(builder);
    let mut last_value = None;
    let total = port.len();

    trace.emit_if(
        "debug",
        "build_block",
        &format!("Processing {} statements", total),
        trace.is_enabled(),
    );

    let mut index = 0;
    while index < total {
        if crate::config::env::joinir_dev_enabled() {
            if let Some(remaining) = port.suffix_route_input(index) {
                let function_name = builder
                    .scope_ctx
                    .current_function
                    .as_ref()
                    .map(|function| function.signature.name.clone())
                    .unwrap_or_else(|| "unknown".to_string());
                let prefix_variables = builder.variable_ctx.variable_map.clone();
                if let Some(consumed) = crate::mir::builder::control_flow::normalization::NormalizedShadowSuffixRouterBox::try_lower_loop_suffix(
                    builder,
                    remaining,
                    &function_name,
                    trace.is_enabled(),
                    Some(&prefix_variables),
                )? {
                    trace.emit_if(
                        "debug",
                        "build_block/suffix_router",
                        &format!(
                            "Phase 142 P0: Suffix router consumed {} statement(s), continuing to process subsequent statements",
                            consumed
                        ),
                        trace.is_enabled(),
                    );
                    index += consumed;
                }
            }
        }

        trace.emit_if(
            "debug",
            "build_block",
            &format!(
                "Statement {}/{}  current_block={:?}  current_function={}",
                index + 1,
                total,
                builder.current_block,
                builder
                    .scope_ctx
                    .current_function
                    .as_ref()
                    .map(|function| function.signature.name.as_str())
                    .unwrap_or("none")
            ),
            trace.is_enabled(),
        );
        last_value = Some(port.lower_statement(builder, index)?);
        index += 1;

        if is_current_block_terminated(builder)? {
            trace.emit_if(
                "debug",
                "build_block",
                &format!("Block terminated after statement {}", index),
                trace.is_enabled(),
            );
            break;
        }
    }

    let output = match last_value {
        Some(value) => value,
        None => crate::mir::builder::emission::constant::emit_void(builder)?,
    };
    if !builder.is_current_block_terminated() {
        builder.hint_scope_leave(scope_id);
    }
    trace.emit_if(
        "debug",
        "build_block",
        &format!("Completed, returning value {:?}", output),
        trace.is_enabled(),
    );
    Ok(output)
}
