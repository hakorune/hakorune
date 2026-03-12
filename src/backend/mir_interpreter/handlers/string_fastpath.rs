use super::*;

pub(super) fn try_handle_fast_string_len(
    this: &mut MirInterpreter,
    dst: Option<ValueId>,
    box_val: ValueId,
    method: &str,
    args: &[ValueId],
) -> Result<bool, VMError> {
    if !this.vm_fast_enabled || !args.is_empty() || (method != "length" && method != "size") {
        return Ok(false);
    }
    if let Some(recv) = this.regs.get(&box_val) {
        if let Some(n) = string_len_from_vmvalue(recv, this.string_cp_mode) {
            this.write_result(dst, VMValue::Integer(n));
            return Ok(true);
        }
        return Ok(false);
    }
    let recv = this.reg_load(box_val)?;
    if let Some(n) = string_len_from_vmvalue(&recv, this.string_cp_mode) {
        this.write_result(dst, VMValue::Integer(n));
        return Ok(true);
    }
    Ok(false)
}

pub(super) fn string_len_from_vmvalue(recv: &VMValue, use_cp: bool) -> Option<i64> {
    match recv {
        VMValue::String(s) => Some(if use_cp {
            s.chars().count() as i64
        } else {
            s.len() as i64
        }),
        VMValue::BoxRef(b) => b
            .as_any()
            .downcast_ref::<crate::box_trait::StringBox>()
            .map(|sb| {
                if use_cp {
                    sb.value.chars().count() as i64
                } else {
                    sb.value.len() as i64
                }
            }),
        _ => None,
    }
}
