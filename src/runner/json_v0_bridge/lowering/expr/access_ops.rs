use super::super::super::ast::ExprV0;
use super::super::BridgeEnv;
use super::VarScope;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, MirType, ValueId};

pub(super) fn lower_var_expr<S: VarScope>(
    env: &BridgeEnv,
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    name: &str,
    vars: &mut S,
) -> Result<(ValueId, BasicBlockId), String> {
    match vars.resolve(env, f, cur_bb, name)? {
        Some(value_id) => Ok((value_id, cur_bb)),
        None => Err(format!("undefined variable: {}", name)),
    }
}

pub(super) fn lower_field_expr<S: VarScope>(
    env: &BridgeEnv,
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    recv: &ExprV0,
    field: &str,
    vars: &mut S,
) -> Result<(ValueId, BasicBlockId), String> {
    let (base, cur2) = super::lower_expr_with_scope(env, f, cur_bb, recv, vars)?;
    let declared_type = declared_field_type(env, f, base, field);
    let dst = f.next_value_id();
    if let Some(bb) = f.get_block_mut(cur2) {
        bb.add_instruction(MirInstruction::FieldGet {
            dst,
            base,
            field: field.to_string(),
            declared_type: declared_type.clone(),
        });
    }
    if let Some(ty) = declared_type {
        f.metadata.value_types.insert(dst, ty);
    }
    Ok((dst, cur2))
}

fn declared_field_type(
    env: &BridgeEnv,
    f: &MirFunction,
    base: ValueId,
    field: &str,
) -> Option<MirType> {
    let MirType::Box(box_name) = f.metadata.value_types.get(&base)? else {
        return None;
    };
    let decl = env.user_box_decls.get(box_name)?;
    let field_decl = decl.field_decls.iter().find(|decl| decl.name == field)?;
    type_name_hint_to_mir(field_decl.declared_type.as_deref())
}

fn type_name_hint_to_mir(raw: Option<&str>) -> Option<MirType> {
    let raw = raw?;
    let lower = raw.to_ascii_lowercase();
    match lower.as_str() {
        "integer" | "int" | "i64" | "integerbox" => Some(MirType::Integer),
        "float" | "f64" | "floatbox" => Some(MirType::Float),
        "bool" | "boolean" | "boolbox" => Some(MirType::Bool),
        "string" | "str" | "stringbox" => Some(MirType::String),
        "void" | "null" | "voidbox" | "nullbox" => Some(MirType::Void),
        _ if looks_like_generic_type_param(raw) => None,
        _ => Some(MirType::Box(raw.to_string())),
    }
}

fn looks_like_generic_type_param(raw: &str) -> bool {
    !raw.is_empty()
        && raw
            .chars()
            .all(|ch| ch.is_ascii_uppercase() || ch.is_ascii_digit())
}
