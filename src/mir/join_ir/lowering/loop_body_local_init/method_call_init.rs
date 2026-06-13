use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::condition_env::ConditionEnv;
use crate::mir::join_ir::lowering::condition_lowerer;
use crate::mir::join_ir::lowering::debug_output_box::DebugOutputBox;
use crate::mir::join_ir::lowering::loop_body_local_env::LoopBodyLocalEnv;
use crate::mir::join_ir::lowering::method_call_lowerer::MethodCallLowerer;
use crate::mir::join_ir::lowering::user_method_policy::UserMethodPolicy;
use crate::mir::join_ir::{JoinInst, MirLikeInst};
use crate::mir::ValueId;

/// Emits a method call in a loop body-local init expression.
///
/// This shelf owns receiver resolution for init expressions and delegates
/// metadata-driven core method lowering to `MethodCallLowerer`. Keep the
/// receiver lookup order unchanged:
///
/// ```text
/// ConditionEnv -> LoopBodyLocalEnv -> CapturedEnv
/// ```
pub(super) fn emit(
    receiver: &ASTNode,
    method: &str,
    args: &[ASTNode],
    cond_env: &ConditionEnv,
    body_local_env: &LoopBodyLocalEnv,
    instructions: &mut Vec<JoinInst>,
    alloc: &mut dyn FnMut() -> ValueId,
    current_static_box_name: Option<&str>,
) -> Result<ValueId, String> {
    let debug = DebugOutputBox::new_dev("loop_body_local_init");
    debug.log(
        "method_call",
        &format!(
            "MethodCall: {}.{}(...)",
            if let ASTNode::Variable { name, .. } = receiver {
                name
            } else {
                "?"
            },
            method
        ),
    );

    let receiver_id = match receiver {
        ASTNode::Variable { name, .. } => {
            resolve_variable_receiver(name, cond_env, body_local_env, &debug)?
        }
        ASTNode::Me { .. } | ASTNode::This { .. } => {
            return emit_static_box_receiver(
                method,
                args,
                cond_env,
                body_local_env,
                instructions,
                alloc,
                current_static_box_name,
                &debug,
            );
        }
        _ => {
            return Err(
                "Complex receiver not supported in init method call (Phase 226 - only simple variables)"
                    .to_string(),
            );
        }
    };

    let mut alloc_wrapper = || alloc();
    let result_id = MethodCallLowerer::lower_for_init(
        receiver_id,
        method,
        args,
        &mut alloc_wrapper,
        cond_env,
        body_local_env,
        instructions,
    )?;

    debug.log(
        "method_call",
        &format!("MethodCallLowerer completed → {:?}", result_id),
    );

    Ok(result_id)
}

fn resolve_variable_receiver(
    name: &str,
    cond_env: &ConditionEnv,
    body_local_env: &LoopBodyLocalEnv,
    debug: &DebugOutputBox,
) -> Result<ValueId, String> {
    if let Some(vid) = cond_env.get(name) {
        debug.log(
            "method_call",
            &format!("Receiver '{}' found in ConditionEnv → {:?}", name, vid),
        );
        Ok(vid)
    } else if let Some(vid) = body_local_env.get(name) {
        debug.log(
            "method_call",
            &format!("Receiver '{}' found in LoopBodyLocalEnv → {:?}", name, vid),
        );
        Ok(vid)
    } else if let Some(&vid) = cond_env.captured.get(name) {
        debug.log(
            "method_call",
            &format!(
                "Receiver '{}' found in CapturedEnv (pinned) → {:?}",
                name, vid
            ),
        );
        Ok(vid)
    } else {
        Err(format!(
            "Method receiver '{}' not found in ConditionEnv / LoopBodyLocalEnv / CapturedEnv (must be loop-outer variable, body-local, or pinned local)",
            name
        ))
    }
}

fn emit_static_box_receiver(
    method: &str,
    args: &[ASTNode],
    cond_env: &ConditionEnv,
    body_local_env: &LoopBodyLocalEnv,
    instructions: &mut Vec<JoinInst>,
    alloc: &mut dyn FnMut() -> ValueId,
    current_static_box_name: Option<&str>,
    debug: &DebugOutputBox,
) -> Result<ValueId, String> {
    let box_name = current_static_box_name.ok_or_else(|| {
        format!(
            "me/this.{}(...) requires current_static_box_name (not in static box context)",
            method
        )
    })?;

    debug.log(
        "method_call",
        &format!("Me/This receiver → box_name={}", box_name),
    );

    if !UserMethodPolicy::allowed_in_init(box_name, method) {
        return Err(format!(
            "User-defined method not allowed in init: {}.{}()",
            box_name, method
        ));
    }

    let mut arg_ids = Vec::new();
    for arg in args {
        let arg_id = condition_lowerer::lower_value_expression(
            arg,
            alloc,
            cond_env,
            Some(body_local_env),
            current_static_box_name,
            instructions,
        )?;
        arg_ids.push(arg_id);
    }

    let result_id = alloc();
    instructions.push(JoinInst::Compute(MirLikeInst::BoxCall {
        dst: Some(result_id),
        box_name: box_name.to_string(),
        method: method.to_string(),
        args: arg_ids,
    }));

    debug.log(
        "method_call",
        &format!("Me/This.{}() emitted BoxCall → {:?}", method, result_id),
    );

    Ok(result_id)
}
