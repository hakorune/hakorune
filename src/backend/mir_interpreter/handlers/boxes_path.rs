use super::temp_dispatch::{
    with_temp_receiver_dispatch, TMP_OUT_PATH_METHOD_BRIDGE, TMP_RECV_PATH_METHOD_BRIDGE,
};
use super::*;

const PATHBOX_JOIN_ARG_ERROR: &str = "PathBox.join: requires 2 arguments";
const PATHBOX_DIRNAME_ARG_ERROR: &str = "PathBox.dirname: requires 1 argument";
const PATHBOX_BASENAME_ARG_ERROR: &str = "PathBox.basename: requires 1 argument";
const PATHBOX_EXTNAME_ARG_ERROR: &str = "PathBox.extname: requires 1 argument";
const PATHBOX_ISABS_ARG_ERROR: &str = "PathBox.isAbs: requires 1 argument";
const PATHBOX_NORMALIZE_ARG_ERROR: &str = "PathBox.normalize: requires 1 argument";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum PathBoxDispatchMode {
    BoxCall,
    MethodCall,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PathMethod {
    Join,
    Dirname,
    Basename,
    Extname,
    IsAbs,
    Normalize,
}

fn parse_path_method(method: &str) -> Option<PathMethod> {
    match method {
        "join" => Some(PathMethod::Join),
        "dirname" => Some(PathMethod::Dirname),
        "basename" => Some(PathMethod::Basename),
        "extname" => Some(PathMethod::Extname),
        "isAbs" | "is_abs" | "isabs" => Some(PathMethod::IsAbs),
        "normalize" => Some(PathMethod::Normalize),
        _ => None,
    }
}

pub(super) fn try_handle_path_box(
    this: &mut MirInterpreter,
    dst: Option<ValueId>,
    box_val: ValueId,
    method: &str,
    args: &[ValueId],
    mode: PathBoxDispatchMode,
) -> Result<bool, VMError> {
    let Some(path_method) = parse_path_method(method) else {
        return Ok(false);
    };

    let recv = this.reg_load(box_val)?;
    let VMValue::BoxRef(ref recv_box) = recv else {
        return Ok(false);
    };
    let Some(path_box) = recv_box
        .as_any()
        .downcast_ref::<crate::boxes::path_box::PathBox>()
    else {
        return Ok(false);
    };

    match path_method {
        PathMethod::Join => {
            let argv = resolve_path_args(this, args, mode, 2, PATHBOX_JOIN_ARG_ERROR)?;
            let base = this.reg_load(argv[0])?.to_string();
            let rest = this.reg_load(argv[1])?.to_string();
            this.write_result(dst, VMValue::String(path_box.join(&base, &rest)));
        }
        PathMethod::Dirname => {
            let path = load_unary_path_arg(this, args, mode, PATHBOX_DIRNAME_ARG_ERROR)?;
            this.write_result(dst, VMValue::String(path_box.dirname(&path)));
        }
        PathMethod::Basename => {
            let path = load_unary_path_arg(this, args, mode, PATHBOX_BASENAME_ARG_ERROR)?;
            this.write_result(dst, VMValue::String(path_box.basename(&path)));
        }
        PathMethod::Extname => {
            let path = load_unary_path_arg(this, args, mode, PATHBOX_EXTNAME_ARG_ERROR)?;
            this.write_result(dst, VMValue::String(path_box.extname(&path)));
        }
        PathMethod::IsAbs => {
            let path = load_unary_path_arg(this, args, mode, PATHBOX_ISABS_ARG_ERROR)?;
            this.write_result(dst, VMValue::Bool(path_box.is_abs(&path)));
        }
        PathMethod::Normalize => {
            let path = load_unary_path_arg(this, args, mode, PATHBOX_NORMALIZE_ARG_ERROR)?;
            this.write_result(dst, VMValue::String(path_box.normalize(&path)));
        }
    }
    Ok(true)
}

pub(super) fn try_handle_path_box_boxcall(
    this: &mut MirInterpreter,
    dst: Option<ValueId>,
    box_val: ValueId,
    method: &str,
    args: &[ValueId],
) -> Result<bool, VMError> {
    try_handle_path_box(
        this,
        dst,
        box_val,
        method,
        args,
        PathBoxDispatchMode::BoxCall,
    )
}

pub(super) fn try_handle_path_box_methodcall(
    vm: &mut MirInterpreter,
    receiver: &VMValue,
    method: &str,
    args: &[ValueId],
) -> Result<Option<VMValue>, VMError> {
    with_temp_receiver_dispatch(
        vm,
        receiver,
        TMP_RECV_PATH_METHOD_BRIDGE,
        TMP_OUT_PATH_METHOD_BRIDGE,
        |vm, recv_tmp, out_tmp| {
            try_handle_path_box(
                vm,
                Some(out_tmp),
                recv_tmp,
                method,
                args,
                PathBoxDispatchMode::MethodCall,
            )
        },
    )
}

fn load_unary_path_arg(
    this: &mut MirInterpreter,
    args: &[ValueId],
    mode: PathBoxDispatchMode,
    err_msg: &str,
) -> Result<String, VMError> {
    let argv = resolve_path_args(this, args, mode, 1, err_msg)?;
    Ok(this.reg_load(argv[0])?.to_string())
}

fn resolve_path_args<'a>(
    this: &mut MirInterpreter,
    args: &'a [ValueId],
    mode: PathBoxDispatchMode,
    expected: usize,
    err_msg: &str,
) -> Result<&'a [ValueId], VMError> {
    match mode {
        PathBoxDispatchMode::BoxCall => {
            if args.len() != expected {
                return Err(this.err_invalid(err_msg));
            }
            Ok(args)
        }
        PathBoxDispatchMode::MethodCall => {
            if args.len() == expected {
                return Ok(args);
            }
            if args.len() == expected + 1 && is_pathbox_value(this, args[0])? {
                return Ok(&args[1..]);
            }
            Err(this.err_invalid(err_msg))
        }
    }
}

fn is_pathbox_value(this: &mut MirInterpreter, value_id: ValueId) -> Result<bool, VMError> {
    let value = this.reg_load(value_id)?;
    if let VMValue::BoxRef(boxed) = value {
        return Ok(boxed.type_name() == "PathBox");
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::{try_handle_path_box, PathBoxDispatchMode};
    use crate::backend::mir_interpreter::{MirInterpreter, VMValue};
    use crate::boxes::path_box::PathBox;
    use crate::mir::ValueId;
    use crate::providers::ring1::path::Ring1PathService;
    use crate::runtime::provider_lock;
    use std::sync::Arc;

    fn build_interpreter_with_pathbox(recv_id: ValueId) -> MirInterpreter {
        let _ = provider_lock::set_pathbox_provider(Arc::new(Ring1PathService::new()));
        let mut interp = MirInterpreter::new();
        let pb = PathBox::new();
        interp
            .regs
            .insert(recv_id, VMValue::from_nyash_box(Box::new(pb)));
        interp
    }

    #[test]
    fn pathbox_boxcall_join_contract() {
        let recv_id = ValueId(10);
        let dst_id = ValueId(11);
        let mut interp = build_interpreter_with_pathbox(recv_id);

        let base_id = ValueId(12);
        let rel_id = ValueId(13);
        interp
            .regs
            .insert(base_id, VMValue::String("apps".to_string()));
        interp
            .regs
            .insert(rel_id, VMValue::String("tests".to_string()));

        let handled = try_handle_path_box(
            &mut interp,
            Some(dst_id),
            recv_id,
            "join",
            &[base_id, rel_id],
            PathBoxDispatchMode::BoxCall,
        )
        .expect("PathBox.join should succeed");
        assert!(handled);
        assert_eq!(
            interp.reg_load(dst_id).unwrap_or(VMValue::Void).to_string(),
            "apps/tests"
        );
    }

    #[test]
    fn pathbox_methodcall_accepts_recv_dup_shape() {
        let recv_id = ValueId(20);
        let dst_id = ValueId(21);
        let mut interp = build_interpreter_with_pathbox(recv_id);

        let recv_dup_id = ValueId(22);
        let recv_dup = interp.reg_load(recv_id).expect("recv must exist");
        interp.regs.insert(recv_dup_id, recv_dup);

        let path_id = ValueId(23);
        interp
            .regs
            .insert(path_id, VMValue::String("apps/tests/main.hako".to_string()));

        let handled = try_handle_path_box(
            &mut interp,
            Some(dst_id),
            recv_id,
            "dirname",
            &[recv_dup_id, path_id],
            PathBoxDispatchMode::MethodCall,
        )
        .expect("methodcall dirname([recv,path]) should not error");
        assert!(handled);
        assert_eq!(
            interp.reg_load(dst_id).unwrap_or(VMValue::Void).to_string(),
            "apps/tests"
        );
    }
}
