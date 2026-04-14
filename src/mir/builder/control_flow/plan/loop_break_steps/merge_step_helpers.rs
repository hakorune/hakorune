use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

pub(super) fn emit_loop_break_completion_void(
    builder: &mut MirBuilder,
) -> Result<Option<ValueId>, String> {
    use crate::mir::builder::emission::constant::emit_void;

    let void_val = emit_void(builder)?;
    crate::mir::builder::control_flow::joinir::trace::trace().debug(
        "loop_break",
        &format!("Loop complete, returning Void {:?}", void_val),
    );
    Ok(Some(void_val))
}
