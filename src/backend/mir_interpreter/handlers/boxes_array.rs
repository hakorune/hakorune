use super::*;
use crate::box_trait::NyashBox;
use crate::boxes::array::{ArrayMethodId, ArraySurfaceInvokeResult};

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
            _ => {}
        }
        if let Some(method_id) = ArrayMethodId::from_name(method) {
            this.validate_args_exact(method, args, method_id.arity())?;
            let surface_args = args
                .iter()
                .map(|arg| this.load_as_box(*arg))
                .collect::<Result<Vec<_>, _>>()?;
            let result = ab
                .invoke_surface(method_id, surface_args)
                .map_err(|err| this.err_invalid(err.to_string()))?;
            match result {
                ArraySurfaceInvokeResult::Value(value) => {
                    this.write_result(dst, VMValue::from_nyash_box(value));
                }
                ArraySurfaceInvokeResult::Void => {
                    this.write_void(dst);
                }
            }
            return Ok(true);
        }
    }
    Ok(false)
}
