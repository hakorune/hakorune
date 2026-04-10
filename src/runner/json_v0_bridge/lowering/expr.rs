use super::globals::resolve_bridge_global;
use super::match_expr;
use super::ternary;
use super::throw_lower::lower_throw;
use super::BridgeEnv;
use crate::mir::{BasicBlockId, ConstValue, EffectMask, MirFunction, MirInstruction, ValueId};
use std::collections::BTreeMap;

use super::super::ast::ExprV0;

mod access_ops;
mod binary_ops;
mod block_expr;
mod call_ops;
mod sum_ops;

pub(super) trait VarScope {
    fn resolve(
        &mut self,
        env: &BridgeEnv,
        f: &mut MirFunction,
        cur_bb: BasicBlockId,
        name: &str,
    ) -> Result<Option<ValueId>, String>;
}

pub(super) struct NoVars;
impl VarScope for NoVars {
    fn resolve(
        &mut self,
        _env: &BridgeEnv,
        _f: &mut MirFunction,
        _cur_bb: BasicBlockId,
        name: &str,
    ) -> Result<Option<ValueId>, String> {
        Err(format!("undefined variable in this context: {}", name))
    }
}

pub(super) struct MapVars<'a> {
    vars: &'a mut BTreeMap<String, ValueId>,
}
impl<'a> MapVars<'a> {
    fn new(vars: &'a mut BTreeMap<String, ValueId>) -> Self {
        Self { vars }
    }
}
impl<'a> VarScope for MapVars<'a> {
    fn resolve(
        &mut self,
        env: &BridgeEnv,
        f: &mut MirFunction,
        cur_bb: BasicBlockId,
        name: &str,
    ) -> Result<Option<ValueId>, String> {
        if let Some(&vid) = self.vars.get(name) {
            return Ok(Some(vid));
        }

        // Bridge 固有のグローバル解決（imports/hostbridge/env/me dummy）は専用モジュールに委譲
        if let Some(vid) = resolve_bridge_global(name, env, f, cur_bb, self.vars)? {
            return Ok(Some(vid));
        }

        if let Some(suffix) = name.strip_prefix('_') {
            if !suffix.is_empty() {
                let mut matched: Option<ValueId> = None;
                for (candidate, &vid) in self.vars.iter() {
                    if candidate.len() > suffix.len() && candidate.ends_with(name) {
                        if matched.is_some() {
                            return Ok(None);
                        }
                        matched = Some(vid);
                    }
                }
                if matched.is_some() {
                    return Ok(matched);
                }
            }
        }

        Ok(None)
    }
}

pub(super) fn lower_expr_with_scope<S: VarScope>(
    env: &BridgeEnv,
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    e: &ExprV0,
    vars: &mut S,
) -> Result<(ValueId, BasicBlockId), String> {
    match e {
        ExprV0::Int { value } => {
            let ival: i64 = if let Some(n) = value.as_i64() {
                n
            } else if let Some(s) = value.as_str() {
                s.parse().map_err(|_| "invalid int literal")?
            } else {
                return Err("invalid int literal".into());
            };
            let dst = f.next_value_id();
            if let Some(bb) = f.get_block_mut(cur_bb) {
                bb.add_instruction(MirInstruction::Const {
                    dst,
                    value: ConstValue::Integer(ival),
                });
            }
            Ok((dst, cur_bb))
        }
        ExprV0::Str { value } => {
            let dst = f.next_value_id();
            if let Some(bb) = f.get_block_mut(cur_bb) {
                bb.add_instruction(MirInstruction::Const {
                    dst,
                    value: ConstValue::String(value.clone()),
                });
            }
            Ok((dst, cur_bb))
        }
        ExprV0::Bool { value } => {
            let dst = f.next_value_id();
            if let Some(bb) = f.get_block_mut(cur_bb) {
                bb.add_instruction(MirInstruction::Const {
                    dst,
                    value: ConstValue::Bool(*value),
                });
            }
            Ok((dst, cur_bb))
        }
        ExprV0::Null => {
            let dst = f.next_value_id();
            if let Some(bb) = f.get_block_mut(cur_bb) {
                bb.add_instruction(MirInstruction::Const {
                    dst,
                    value: ConstValue::Null,
                });
            }
            Ok((dst, cur_bb))
        }
        ExprV0::Binary { op, lhs, rhs } => {
            binary_ops::lower_binary_expr(env, f, cur_bb, op, lhs, rhs, vars)
        }
        ExprV0::Extern {
            iface,
            method,
            args,
        } => {
            let (arg_ids, cur2) = lower_args_with_scope(env, f, cur_bb, args, vars)?;
            let dst = f.next_value_id();
            if let Some(bb) = f.get_block_mut(cur2) {
                bb.add_instruction(crate::mir::ssot::extern_call::extern_call(
                    Some(dst),
                    iface.clone(),
                    method.clone(),
                    arg_ids,
                    EffectMask::IO,
                ));
            }
            Ok((dst, cur2))
        }
        ExprV0::Compare { op, lhs, rhs } => {
            binary_ops::lower_compare_expr(env, f, cur_bb, op, lhs, rhs, vars)
        }
        ExprV0::Logical { op, lhs, rhs } => {
            binary_ops::lower_logical_expr(env, f, cur_bb, op, lhs, rhs, vars)
        }
        ExprV0::Call { name, args } => {
            call_ops::lower_call_expr(env, f, cur_bb, name, args, vars)
        }
        ExprV0::Method { recv, method, args } => {
            call_ops::lower_method_expr(env, f, cur_bb, recv, method, args, vars)
        }
        ExprV0::Field { recv, field } => {
            access_ops::lower_field_expr(env, f, cur_bb, recv, field, vars)
        }
        ExprV0::New { class, args } => {
            call_ops::lower_new_expr(env, f, cur_bb, class, args, vars)
        }
        ExprV0::EnumCtor {
            enum_name,
            variant,
            payload_type,
            args,
        } => sum_ops::lower_variant_ctor_expr_with_scope(
            env,
            f,
            cur_bb,
            enum_name,
            variant,
            payload_type.as_deref(),
            args,
            vars,
        ),
        ExprV0::Var { name } => access_ops::lower_var_expr(env, f, cur_bb, name, vars),
        ExprV0::Throw { expr } => {
            let (exc, cur) = lower_expr_with_scope(env, f, cur_bb, expr, vars)?;
            Ok(lower_throw(env, f, cur, exc, None))
        }
        ExprV0::BlockExpr { .. } => Err(
            "[freeze:contract][json_v0][blockexpr] BlockExpr requires var scope (use lower_expr_with_vars)"
                .to_string(),
        ),
        ExprV0::Ternary { cond, then, r#else } => {
            ternary::lower_ternary_expr_with_scope(env, f, cur_bb, cond, then, r#else, vars)
        }
        ExprV0::Match {
            scrutinee,
            arms,
            r#else,
        } => match_expr::lower_match_expr_with_scope(env, f, cur_bb, scrutinee, arms, r#else, vars),
        ExprV0::EnumMatch { enum_name, .. } => Err(format!(
            "[freeze:contract][json_v0][enum_match] requires var scope for `{}`",
            enum_name
        )),
    }
}

fn lower_args_with_scope<S: VarScope>(
    env: &BridgeEnv,
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    args: &[ExprV0],
    scope: &mut S,
) -> Result<(Vec<ValueId>, BasicBlockId), String> {
    let mut out = Vec::with_capacity(args.len());
    let mut cur = cur_bb;
    for a in args {
        let (v, c) = lower_expr_with_scope(env, f, cur, a, scope)?;
        out.push(v);
        cur = c;
    }
    Ok((out, cur))
}

#[allow(dead_code)]
fn lower_expr(
    env: &BridgeEnv,
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    e: &ExprV0,
) -> Result<(ValueId, BasicBlockId), String> {
    let mut scope = NoVars;
    lower_expr_with_scope(env, f, cur_bb, e, &mut scope)
}

pub(super) fn lower_expr_with_vars(
    env: &BridgeEnv,
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    e: &ExprV0,
    vars: &mut BTreeMap<String, ValueId>,
) -> Result<(ValueId, BasicBlockId), String> {
    if let ExprV0::BlockExpr { prelude, tail } = e {
        return block_expr::lower_blockexpr_with_vars(env, f, cur_bb, prelude, tail, vars);
    }
    if let ExprV0::EnumMatch {
        enum_name,
        scrutinee,
        arms,
        r#else,
    } = e
    {
        return sum_ops::lower_variant_match_expr_with_vars(
            env,
            f,
            cur_bb,
            enum_name,
            scrutinee,
            arms,
            r#else.as_deref(),
            vars,
        );
    }
    let mut scope = MapVars::new(vars);
    lower_expr_with_scope(env, f, cur_bb, e, &mut scope)
}

#[allow(dead_code)]
fn lower_args(
    env: &BridgeEnv,
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    args: &[ExprV0],
) -> Result<(Vec<ValueId>, BasicBlockId), String> {
    let mut scope = NoVars;
    lower_args_with_scope(env, f, cur_bb, args, &mut scope)
}

pub(super) fn lower_args_with_vars(
    env: &BridgeEnv,
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    args: &[ExprV0],
    vars: &mut BTreeMap<String, ValueId>,
) -> Result<(Vec<ValueId>, BasicBlockId), String> {
    let mut scope = MapVars::new(vars);
    lower_args_with_scope(env, f, cur_bb, args, &mut scope)
}
