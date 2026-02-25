use super::*;
use super::temp_dispatch::{
    with_temp_receiver_dispatch, TMP_OUT_FILE_METHOD_BRIDGE, TMP_RECV_FILE_METHOD_BRIDGE,
};

const FILEBOX_OPEN_ARG_ERROR: &str =
    "FileBox.open: requires 1 (path) or 2 (path, mode) arguments";
const FILEBOX_READ_ARG_ERROR: &str = "FileBox.read: requires 0 arguments";
const FILEBOX_CLOSE_ARG_ERROR: &str = "FileBox.close: requires 0 arguments";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum FileBoxDispatchMode {
    BoxCall,
    MethodCall,
}

pub(super) fn try_handle_file_box(
    this: &mut MirInterpreter,
    dst: Option<ValueId>,
    box_val: ValueId,
    method: &str,
    args: &[ValueId],
    mode: FileBoxDispatchMode,
) -> Result<bool, VMError> {
    if !matches!(method, "open" | "read" | "close") {
        return Ok(false);
    }

    let recv = this.reg_load(box_val)?;
    let VMValue::BoxRef(ref recv_box) = recv else {
        return Ok(false);
    };
    let Some(file_box) = recv_box
        .as_any()
        .downcast_ref::<crate::boxes::file::FileBox>()
    else {
        return Ok(false);
    };

    match method {
        "open" => {
            let (path_id, mode_id_opt) = resolve_open_args(this, args, mode)?;
            let path = this.reg_load(path_id)?.to_string();
            let io_mode = mode_id_opt
                .map(|id| this.reg_load(id).map(|v| v.to_string()))
                .transpose()?
                .unwrap_or_else(|| "r".to_string());
            let ok = file_box.ny_open(&path, &io_mode).is_ok();
            this.write_result(dst, VMValue::Integer(if ok { 1 } else { 0 }));
        }
        "read" => {
            validate_read_close_args(this, args, mode, FILEBOX_READ_ARG_ERROR)?;
            match file_box.ny_read_to_string() {
                Ok(s) => this.write_string(dst, s),
                Err(_) => this.write_void(dst),
            }
        }
        "close" => {
            validate_read_close_args(this, args, mode, FILEBOX_CLOSE_ARG_ERROR)?;
            let _ = file_box.ny_close();
            this.write_void(dst);
        }
        _ => return Ok(false),
    }

    Ok(true)
}

pub(super) fn try_handle_file_box_boxcall(
    this: &mut MirInterpreter,
    dst: Option<ValueId>,
    box_val: ValueId,
    method: &str,
    args: &[ValueId],
) -> Result<bool, VMError> {
    try_handle_file_box(this, dst, box_val, method, args, FileBoxDispatchMode::BoxCall)
}

pub(super) fn try_handle_file_box_methodcall(
    this: &mut MirInterpreter,
    receiver: &VMValue,
    method: &str,
    args: &[ValueId],
) -> Result<Option<VMValue>, VMError> {
    with_temp_receiver_dispatch(
        this,
        receiver,
        TMP_RECV_FILE_METHOD_BRIDGE,
        TMP_OUT_FILE_METHOD_BRIDGE,
        |vm, recv_tmp, out_tmp| {
            try_handle_file_box(
                vm,
                Some(out_tmp),
                recv_tmp,
                method,
                args,
                FileBoxDispatchMode::MethodCall,
            )
        },
    )
}

fn resolve_open_args(
    this: &mut MirInterpreter,
    args: &[ValueId],
    mode: FileBoxDispatchMode,
) -> Result<(ValueId, Option<ValueId>), VMError> {
    match mode {
        FileBoxDispatchMode::BoxCall => {
            if args.is_empty() || args.len() > 2 {
                return Err(this.err_invalid(FILEBOX_OPEN_ARG_ERROR));
            }
            if args.len() == 1 {
                return Ok((args[0], None));
            }
            Ok((args[0], Some(args[1])))
        }
        FileBoxDispatchMode::MethodCall => {
            if args.is_empty() || args.len() > 3 {
                return Err(this.err_invalid(FILEBOX_OPEN_ARG_ERROR));
            }
            match args.len() {
                1 => Ok((args[0], None)),
                2 => {
                    if is_filebox_value(this, args[0])? {
                        Ok((args[1], None))
                    } else {
                        Ok((args[0], Some(args[1])))
                    }
                }
                3 => Ok((args[1], Some(args[2]))),
                _ => unreachable!(),
            }
        }
    }
}

fn validate_read_close_args(
    this: &mut MirInterpreter,
    args: &[ValueId],
    mode: FileBoxDispatchMode,
    err_msg: &str,
) -> Result<(), VMError> {
    let max_args = match mode {
        FileBoxDispatchMode::BoxCall => 0,
        FileBoxDispatchMode::MethodCall => 1,
    };
    if args.len() > max_args {
        return Err(this.err_invalid(err_msg));
    }
    Ok(())
}

fn is_filebox_value(this: &mut MirInterpreter, value_id: ValueId) -> Result<bool, VMError> {
    let value = this.reg_load(value_id)?;
    if let VMValue::BoxRef(boxed) = value {
        return Ok(boxed.type_name() == "FileBox");
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::{try_handle_file_box, FileBoxDispatchMode};
    use crate::backend::mir_interpreter::{MirInterpreter, VMValue};
    use crate::boxes::file::core_ro::CoreRoFileIo;
    use crate::boxes::file::FileBox;
    use crate::mir::ValueId;
    use std::sync::Arc;

    fn build_interpreter_with_filebox(recv_id: ValueId) -> MirInterpreter {
        let mut interp = MirInterpreter::new();
        let fb = FileBox::with_provider(Arc::new(CoreRoFileIo::new()));
        interp
            .regs
            .insert(recv_id, VMValue::from_nyash_box(Box::new(fb)));
        interp
    }

    #[test]
    fn filebox_boxcall_open_contract_accepts_1_or_2_args() {
        let recv_id = ValueId(10);
        let dst_id = ValueId(11);
        let mut interp = build_interpreter_with_filebox(recv_id);

        let path_id = ValueId(12);
        interp
            .regs
            .insert(path_id, VMValue::String("/__missing__.txt".to_string()));
        let handled = try_handle_file_box(
            &mut interp,
            Some(dst_id),
            recv_id,
            "open",
            &[path_id],
            FileBoxDispatchMode::BoxCall,
        )
        .expect("boxcall open(1) should not error");
        assert!(handled);
        assert!(matches!(
            interp.reg_load(dst_id).unwrap_or(VMValue::Void),
            VMValue::Integer(_)
        ));

        let mode_id = ValueId(13);
        interp
            .regs
            .insert(mode_id, VMValue::String("r".to_string()));
        let handled = try_handle_file_box(
            &mut interp,
            Some(dst_id),
            recv_id,
            "open",
            &[path_id, mode_id],
            FileBoxDispatchMode::BoxCall,
        )
        .expect("boxcall open(2) should not error");
        assert!(handled);
    }

    #[test]
    fn filebox_methodcall_open_contract_accepts_recv_dup_shape() {
        let recv_id = ValueId(20);
        let dst_id = ValueId(21);
        let mut interp = build_interpreter_with_filebox(recv_id);

        let recv_dup_id = ValueId(22);
        let recv_dup = interp.reg_load(recv_id).expect("recv must exist");
        interp.regs.insert(recv_dup_id, recv_dup);

        let path_id = ValueId(23);
        interp
            .regs
            .insert(path_id, VMValue::String("/__missing__.txt".to_string()));

        let handled = try_handle_file_box(
            &mut interp,
            Some(dst_id),
            recv_id,
            "open",
            &[recv_dup_id, path_id],
            FileBoxDispatchMode::MethodCall,
        )
        .expect("methodcall open([recv,path]) should not error");
        assert!(handled);
        assert!(matches!(
            interp.reg_load(dst_id).unwrap_or(VMValue::Void),
            VMValue::Integer(_)
        ));
    }

    #[test]
    fn filebox_read_close_arg_contract_is_enforced_per_mode() {
        let recv_id = ValueId(30);
        let dst_id = ValueId(31);
        let mut interp = build_interpreter_with_filebox(recv_id);

        let recv_dup_id = ValueId(32);
        let recv_dup = interp.reg_load(recv_id).expect("recv must exist");
        interp.regs.insert(recv_dup_id, recv_dup);

        let path_id = ValueId(33);
        interp
            .regs
            .insert(path_id, VMValue::String("/tmp/x".to_string()));

        // MethodCall: allow [] and [recv]
        let handled = try_handle_file_box(
            &mut interp,
            Some(dst_id),
            recv_id,
            "read",
            &[recv_dup_id],
            FileBoxDispatchMode::MethodCall,
        )
        .expect("methodcall read([recv]) should not error");
        assert!(handled);

        // MethodCall: reject [recv, path]
        let err = try_handle_file_box(
            &mut interp,
            Some(dst_id),
            recv_id,
            "close",
            &[recv_dup_id, path_id],
            FileBoxDispatchMode::MethodCall,
        )
        .expect_err("methodcall close([recv,path]) must error");
        assert!(format!("{:?}", err).contains("FileBox.close: requires 0 arguments"));

        // BoxCall: reject [recv]
        let err = try_handle_file_box(
            &mut interp,
            Some(dst_id),
            recv_id,
            "read",
            &[recv_dup_id],
            FileBoxDispatchMode::BoxCall,
        )
        .expect_err("boxcall read([recv]) must error");
        assert!(format!("{:?}", err).contains("FileBox.read: requires 0 arguments"));
    }
}
