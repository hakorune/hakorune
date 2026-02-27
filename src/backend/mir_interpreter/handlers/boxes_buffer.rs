use super::*;
use super::temp_dispatch::{
    with_temp_receiver_dispatch, TMP_OUT_BUFFER_METHOD_BRIDGE, TMP_RECV_BUFFER_METHOD_BRIDGE,
};
use crate::boxes::math_box::FloatBox;

pub(super) fn try_handle_buffer_box(
    this: &mut MirInterpreter,
    dst: Option<ValueId>,
    box_val: ValueId,
    method: &str,
    args: &[ValueId],
) -> Result<bool, VMError> {
    let recv = this.reg_load(box_val)?;
    let VMValue::BoxRef(ref recv_box) = recv else {
        return Ok(false);
    };
    let Some(buffer_box) = recv_box
        .as_any()
        .downcast_ref::<crate::boxes::buffer::BufferBox>()
    else {
        return Ok(false);
    };

    match method {
        "writeU8" => {
            this.validate_args_exact(method, args, 1)?;
            let v = this.load_as_int(args[0])?;
            match buffer_box.write_u8(v) {
                Ok(len) => this.write_result(dst, VMValue::Integer(len as i64)),
                Err(e) => return Err(this.err_invalid(e)),
            }
            Ok(true)
        }
        "writeU16" => {
            this.validate_args_exact(method, args, 1)?;
            let v = this.load_as_int(args[0])?;
            match buffer_box.write_u16(v) {
                Ok(len) => this.write_result(dst, VMValue::Integer(len as i64)),
                Err(e) => return Err(this.err_invalid(e)),
            }
            Ok(true)
        }
        "writeU32" => {
            this.validate_args_exact(method, args, 1)?;
            let v = this.load_as_int(args[0])?;
            match buffer_box.write_u32(v) {
                Ok(len) => this.write_result(dst, VMValue::Integer(len as i64)),
                Err(e) => return Err(this.err_invalid(e)),
            }
            Ok(true)
        }
        "writeU64" => {
            this.validate_args_exact(method, args, 1)?;
            let v = this.load_as_int(args[0])?;
            match buffer_box.write_u64(v) {
                Ok(len) => this.write_result(dst, VMValue::Integer(len as i64)),
                Err(e) => return Err(this.err_invalid(e)),
            }
            Ok(true)
        }
        "writeF32" => {
            this.validate_args_exact(method, args, 1)?;
            let v = load_as_f64(this, args[0])?;
            let len = buffer_box.write_f32(v);
            this.write_result(dst, VMValue::Integer(len as i64));
            Ok(true)
        }
        "writeF64" => {
            this.validate_args_exact(method, args, 1)?;
            let v = load_as_f64(this, args[0])?;
            let len = buffer_box.write_f64(v);
            this.write_result(dst, VMValue::Integer(len as i64));
            Ok(true)
        }
        "readU8" => {
            this.validate_args_exact(method, args, 1)?;
            let off = this.load_as_int(args[0])?;
            let out = buffer_box.read_u8(off).map_err(|e| this.err_invalid(e))?;
            this.write_result(dst, VMValue::Integer(out as i64));
            Ok(true)
        }
        "readU16" => {
            this.validate_args_exact(method, args, 1)?;
            let off = this.load_as_int(args[0])?;
            let out = buffer_box.read_u16(off).map_err(|e| this.err_invalid(e))?;
            this.write_result(dst, VMValue::Integer(out as i64));
            Ok(true)
        }
        "readU32" => {
            this.validate_args_exact(method, args, 1)?;
            let off = this.load_as_int(args[0])?;
            let out = buffer_box.read_u32(off).map_err(|e| this.err_invalid(e))?;
            this.write_result(dst, VMValue::Integer(out as i64));
            Ok(true)
        }
        "readU64" => {
            this.validate_args_exact(method, args, 1)?;
            let off = this.load_as_int(args[0])?;
            let out = buffer_box.read_u64(off).map_err(|e| this.err_invalid(e))?;
            if out > i64::MAX as u64 {
                return Err(this.err_invalid("readU64 overflow: value does not fit i64"));
            }
            this.write_result(dst, VMValue::Integer(out as i64));
            Ok(true)
        }
        "readF32" => {
            this.validate_args_exact(method, args, 1)?;
            let off = this.load_as_int(args[0])?;
            let out = buffer_box.read_f32(off).map_err(|e| this.err_invalid(e))?;
            this.write_result(dst, VMValue::Float(out as f64));
            Ok(true)
        }
        "readF64" => {
            this.validate_args_exact(method, args, 1)?;
            let off = this.load_as_int(args[0])?;
            let out = buffer_box.read_f64(off).map_err(|e| this.err_invalid(e))?;
            this.write_result(dst, VMValue::Float(out));
            Ok(true)
        }
        _ => Ok(false),
    }
}

pub(super) fn try_handle_buffer_box_methodcall(
    this: &mut MirInterpreter,
    receiver: &VMValue,
    method: &str,
    args: &[ValueId],
) -> Result<Option<VMValue>, VMError> {
    with_temp_receiver_dispatch(
        this,
        receiver,
        TMP_RECV_BUFFER_METHOD_BRIDGE,
        TMP_OUT_BUFFER_METHOD_BRIDGE,
        |vm, recv_tmp, out_tmp| try_handle_buffer_box(vm, Some(out_tmp), recv_tmp, method, args),
    )
}

fn load_as_f64(this: &mut MirInterpreter, arg: ValueId) -> Result<f64, VMError> {
    let value = this.reg_load(arg)?;
    match value {
        VMValue::Float(f) => Ok(f),
        VMValue::Integer(i) => Ok(i as f64),
        VMValue::BoxRef(bx) => {
            if let Some(fb) = bx.as_any().downcast_ref::<FloatBox>() {
                Ok(fb.value)
            } else if let Some(ib) = bx.as_any().downcast_ref::<crate::box_trait::IntegerBox>() {
                Ok(ib.value as f64)
            } else {
                Err(this.err_invalid("Expected numeric argument"))
            }
        }
        _ => Err(this.err_invalid("Expected numeric argument")),
    }
}

#[cfg(test)]
mod tests {
    use super::try_handle_buffer_box;
    use crate::backend::mir_interpreter::{MirInterpreter, VMValue};
    use crate::boxes::buffer::BufferBox;
    use crate::mir::ValueId;

    #[test]
    fn bufferbox_boxcall_typed_rw_contract() {
        let recv_id = ValueId(40);
        let dst_id = ValueId(41);
        let arg_id = ValueId(42);
        let off_id = ValueId(43);

        let mut interp = MirInterpreter::new();
        interp
            .regs
            .insert(recv_id, VMValue::from_nyash_box(Box::new(BufferBox::new())));
        interp.regs.insert(arg_id, VMValue::Integer(0x3456));
        interp.regs.insert(off_id, VMValue::Integer(0));

        let handled = try_handle_buffer_box(
            &mut interp,
            Some(dst_id),
            recv_id,
            "writeU16",
            &[arg_id],
        )
        .expect("writeU16 should not error");
        assert!(handled);

        let handled = try_handle_buffer_box(
            &mut interp,
            Some(dst_id),
            recv_id,
            "readU16",
            &[off_id],
        )
        .expect("readU16 should not error");
        assert!(handled);
        assert_eq!(
            interp.reg_load(dst_id).expect("dst must exist"),
            VMValue::Integer(0x3456)
        );
    }
}
