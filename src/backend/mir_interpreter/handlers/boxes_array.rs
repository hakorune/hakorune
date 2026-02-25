use super::*;
use crate::box_trait::NyashBox;

pub(super) fn try_handle_array_box(
    this: &mut MirInterpreter,
    dst: Option<ValueId>,
    box_val: ValueId,
    method: &str,
    args: &[ValueId],
) -> Result<bool, VMError> {
    let recv = this.reg_load(box_val)?;
    let recv_box_any: Box<dyn NyashBox> = match recv.clone() {
        VMValue::BoxRef(b) => b.share_box(),
        other => other.to_nyash_box(),
    };
    if let Some(ab) = recv_box_any
        .as_any()
        .downcast_ref::<crate::boxes::array::ArrayBox>()
    {
        match method {
            "birth" => {
                // No-op constructor init
                this.write_void(dst);
                return Ok(true);
            }
            "push" => {
                this.validate_args_exact("push", args, 1)?;
                let val = this.load_as_box(args[0])?;
                let _ = ab.push(val);
                this.write_void(dst);
                return Ok(true);
            }
            "pop" => {
                if !args.is_empty() {
                    return Err(this.err_invalid("pop expects 0 args"));
                }
                let ret = ab.pop();
                this.write_result(dst, VMValue::from_nyash_box(ret));
                return Ok(true);
            }
            "len" | "length" | "size" => {
                let ret = ab.length();
                this.write_result(dst, VMValue::from_nyash_box(ret));
                return Ok(true);
            }
            "get" => {
                this.validate_args_exact("get", args, 1)?;
                let idx = this.load_as_box(args[0])?;
                let ret = ab.get(idx);
                this.write_result(dst, VMValue::from_nyash_box(ret));
                return Ok(true);
            }
            "set" => {
                this.validate_args_exact("set", args, 2)?;
                let idx = this.load_as_box(args[0])?;
                let val = this.load_as_box(args[1])?;
                let _ = ab.set(idx, val);
                this.write_void(dst);
                return Ok(true);
            }
            _ => {}
        }
    }
    Ok(false)
}
