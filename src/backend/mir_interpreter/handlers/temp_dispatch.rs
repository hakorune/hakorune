use super::*;

// Temporary register IDs reserved for method fallback bridges.
// Keep these in one place to avoid accidental overlap when new bridges are added.
pub(super) const TMP_RECV_INSTANCE_FIELD_OP: ValueId = ValueId(999_999_992);
pub(super) const TMP_OUT_INSTANCE_FIELD_OP: ValueId = ValueId(999_999_993);
pub(super) const TMP_RECV_INSTANCE_METHOD_BRIDGE: ValueId = ValueId(999_999_994);
pub(super) const TMP_OUT_INSTANCE_METHOD_BRIDGE: ValueId = ValueId(999_999_995);
pub(super) const TMP_RECV_OBJECT_FIELD_METHOD_BRIDGE: ValueId = ValueId(999_999_996);
pub(super) const TMP_OUT_OBJECT_FIELD_METHOD_BRIDGE: ValueId = ValueId(999_999_997);
pub(super) const TMP_RECV_FILE_METHOD_BRIDGE: ValueId = ValueId(999_999_998);
pub(super) const TMP_OUT_FILE_METHOD_BRIDGE: ValueId = ValueId(999_999_999);
pub(super) const TMP_RECV_PATH_METHOD_BRIDGE: ValueId = ValueId(999_999_990);
pub(super) const TMP_OUT_PATH_METHOD_BRIDGE: ValueId = ValueId(999_999_991);
pub(super) const TMP_RECV_BUFFER_METHOD_BRIDGE: ValueId = ValueId(999_999_988);
pub(super) const TMP_OUT_BUFFER_METHOD_BRIDGE: ValueId = ValueId(999_999_989);

pub(super) fn with_temp_receiver_dispatch<F>(
    vm: &mut MirInterpreter,
    receiver: &VMValue,
    recv_tmp: ValueId,
    out_tmp: ValueId,
    dispatch: F,
) -> Result<Option<VMValue>, VMError>
where
    F: FnOnce(&mut MirInterpreter, ValueId, ValueId) -> Result<bool, VMError>,
{
    let prev_recv = vm.regs.insert(recv_tmp, receiver.clone());
    let prev_out = vm.regs.remove(&out_tmp);

    let dispatch_result = dispatch(vm, recv_tmp, out_tmp);

    let restore =
        |this: &mut MirInterpreter, prev_recv: Option<VMValue>, prev_out: Option<VMValue>| {
            if let Some(v) = prev_recv {
                this.regs.insert(recv_tmp, v);
            } else {
                this.regs.remove(&recv_tmp);
            }
            if let Some(v) = prev_out {
                this.regs.insert(out_tmp, v);
            } else {
                this.regs.remove(&out_tmp);
            }
        };

    let handled = match dispatch_result {
        Ok(handled) => handled,
        Err(e) => {
            restore(vm, prev_recv, prev_out);
            return Err(e);
        }
    };

    let out = if handled {
        Some(vm.reg_load(out_tmp).unwrap_or(VMValue::Void))
    } else {
        None
    };

    restore(vm, prev_recv, prev_out);
    Ok(out)
}
