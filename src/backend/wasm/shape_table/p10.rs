use crate::mir::{ConstValue, MirInstruction, MirModule};

const P10_LOOP_EXTERN_CANDIDATE_ID: &str = "wsm.p10.main_loop_extern_call.v0";
const P10_MIN4_NATIVE_SHAPE_ID: &str = "wsm.p10.main_loop_extern_call.fixed3.v0";
const P10_MIN6_WARN_NATIVE_SHAPE_ID: &str = "wsm.p10.main_loop_extern_call.warn.fixed4.v0";
const P10_MIN7_INFO_NATIVE_SHAPE_ID: &str = "wsm.p10.main_loop_extern_call.info.fixed4.v0";
const P10_MIN8_ERROR_NATIVE_SHAPE_ID: &str = "wsm.p10.main_loop_extern_call.error.fixed4.v0";
const P10_MIN9_DEBUG_NATIVE_SHAPE_ID: &str = "wsm.p10.main_loop_extern_call.debug.fixed4.v0";
const P10_MIN5_WARN_INVENTORY_ID: &str = "wsm.p10.main_loop_extern_call.warn.fixed3.inventory.v0";
const P10_MIN5_INFO_INVENTORY_ID: &str = "wsm.p10.main_loop_extern_call.info.fixed3.inventory.v0";
const P10_MIN5_ERROR_INVENTORY_ID: &str = "wsm.p10.main_loop_extern_call.error.fixed3.inventory.v0";
const P10_MIN5_DEBUG_INVENTORY_ID: &str = "wsm.p10.main_loop_extern_call.debug.fixed3.inventory.v0";

/// Analysis-only candidate detector for WSM-P10.
/// This does not alter route planning and remains bridge-only.
pub(crate) fn detect_p10_loop_extern_call_candidate(
    mir_module: &MirModule,
) -> Option<&'static str> {
    let main = mir_module.get_function("main")?;
    if main.blocks.len() < 2 {
        return None;
    }

    let mut has_extern_call = false;
    let mut has_branch = false;
    let mut has_jump = false;

    for block in main.blocks.values() {
        for inst in &block.instructions {
            if matches!(
                inst,
                MirInstruction::Call {
                    callee: Some(crate::mir::Callee::Extern(_)),
                    ..
                }
            ) {
                has_extern_call = true;
            }
        }

        if let Some(term) = &block.terminator {
            match term {
                MirInstruction::Branch { .. } => has_branch = true,
                MirInstruction::Jump { .. } => has_jump = true,
                _ => {}
            }
        }
    }

    (has_extern_call && has_branch && has_jump).then_some(P10_LOOP_EXTERN_CANDIDATE_ID)
}

/// WSM-P10-min4 native promotion matcher.
/// Keep this conservative so existing bridge contracts stay stable.
pub(crate) fn detect_p10_min4_native_promotable_shape(
    mir_module: &MirModule,
) -> Option<&'static str> {
    let main = mir_module.get_function("main")?;
    if main.blocks.len() < 2 {
        return None;
    }

    let mut has_branch = false;
    let mut has_jump = false;
    let mut extern_log_calls = 0usize;
    let mut has_other_call = false;
    let mut has_const_3 = false;

    for block in main.blocks.values() {
        for inst in &block.instructions {
            match inst {
                MirInstruction::Const {
                    value: ConstValue::Integer(3),
                    ..
                } => has_const_3 = true,
                MirInstruction::Call {
                    callee: Some(callee),
                    ..
                } => match callee {
                    crate::mir::Callee::Extern(name) => {
                        if name == "env.console.log" {
                            extern_log_calls += 1;
                        } else {
                            has_other_call = true;
                        }
                    }
                    crate::mir::Callee::Method {
                        box_name, method, ..
                    } => {
                        if box_name == "console" && method == "log" {
                            extern_log_calls += 1;
                        } else {
                            has_other_call = true;
                        }
                    }
                    _ => has_other_call = true,
                },
                MirInstruction::Call { .. } => has_other_call = true,
                _ => {}
            }
        }

        if let Some(term) = &block.terminator {
            match term {
                MirInstruction::Branch { .. } => has_branch = true,
                MirInstruction::Jump { .. } => has_jump = true,
                _ => {}
            }
        }
    }

    if has_branch && has_jump && has_const_3 && extern_log_calls == 1 && !has_other_call {
        Some(P10_MIN4_NATIVE_SHAPE_ID)
    } else {
        None
    }
}

/// WSM-P10-min6 warn-family native promotion matcher.
/// Keep strict boundary so min5 inventory fixtures remain bridge-only.
pub(crate) fn detect_p10_min6_warn_native_promotable_shape(
    mir_module: &MirModule,
) -> Option<&'static str> {
    detect_p10_fixed4_console_method_native_shape(
        mir_module,
        "warn",
        "env.console.warn",
        P10_MIN6_WARN_NATIVE_SHAPE_ID,
    )
}

/// WSM-P10-min7 info-family native promotion matcher.
/// Keep strict boundary so min5 inventory fixtures remain bridge-only.
pub(crate) fn detect_p10_min7_info_native_promotable_shape(
    mir_module: &MirModule,
) -> Option<&'static str> {
    detect_p10_fixed4_console_method_native_shape(
        mir_module,
        "info",
        "env.console.info",
        P10_MIN7_INFO_NATIVE_SHAPE_ID,
    )
}

/// WSM-P10-min8 error-family native promotion matcher.
/// Keep strict boundary so min5 inventory fixtures remain bridge-only.
pub(crate) fn detect_p10_min8_error_native_promotable_shape(
    mir_module: &MirModule,
) -> Option<&'static str> {
    detect_p10_fixed4_console_method_native_shape(
        mir_module,
        "error",
        "env.console.error",
        P10_MIN8_ERROR_NATIVE_SHAPE_ID,
    )
}

/// WSM-P10-min9 debug-family native promotion matcher.
/// Keep strict boundary so min5 inventory fixtures remain bridge-only.
pub(crate) fn detect_p10_min9_debug_native_promotable_shape(
    mir_module: &MirModule,
) -> Option<&'static str> {
    detect_p10_fixed4_console_method_native_shape(
        mir_module,
        "debug",
        "env.console.debug",
        P10_MIN9_DEBUG_NATIVE_SHAPE_ID,
    )
}

pub(crate) fn detect_p10_fixed4_console_method_native_shape(
    mir_module: &MirModule,
    method_name: &str,
    extern_name: &str,
    shape_id: &'static str,
) -> Option<&'static str> {
    let main = mir_module.get_function("main")?;
    if main.blocks.len() < 2 {
        return None;
    }

    let mut has_branch = false;
    let mut has_jump = false;
    let mut has_const_4 = false;
    let mut method_calls = 0usize;
    let mut has_other_call = false;

    for block in main.blocks.values() {
        for inst in &block.instructions {
            match inst {
                MirInstruction::Const {
                    value: ConstValue::Integer(4),
                    ..
                } => has_const_4 = true,
                MirInstruction::Call {
                    callee: Some(callee),
                    ..
                } => match callee {
                    crate::mir::Callee::Extern(name) => {
                        if name == extern_name {
                            method_calls += 1;
                        } else {
                            has_other_call = true;
                        }
                    }
                    crate::mir::Callee::Method {
                        box_name, method, ..
                    } => {
                        if box_name == "console" && method == method_name {
                            method_calls += 1;
                        } else {
                            has_other_call = true;
                        }
                    }
                    _ => has_other_call = true,
                },
                MirInstruction::Call { .. } => has_other_call = true,
                _ => {}
            }
        }

        if let Some(term) = &block.terminator {
            match term {
                MirInstruction::Branch { .. } => has_branch = true,
                MirInstruction::Jump { .. } => has_jump = true,
                _ => {}
            }
        }
    }

    if has_branch && has_jump && has_const_4 && method_calls == 1 && !has_other_call {
        Some(shape_id)
    } else {
        None
    }
}

/// WSM-P10-min5 expansion inventory matcher.
/// Analysis-only: records adjacent loop/extern shapes that are still bridge-only.
pub(crate) fn detect_p10_min5_expansion_inventory_shape(
    mir_module: &MirModule,
) -> Option<&'static str> {
    let main = mir_module.get_function("main")?;
    if main.blocks.len() < 2 {
        return None;
    }

    let mut has_branch = false;
    let mut has_jump = false;
    let mut has_const_3 = false;
    let mut call_method: Option<&'static str> = None;
    let mut has_other_call = false;

    for block in main.blocks.values() {
        for inst in &block.instructions {
            match inst {
                MirInstruction::Const {
                    value: ConstValue::Integer(3),
                    ..
                } => has_const_3 = true,
                MirInstruction::Call {
                    callee: Some(callee),
                    ..
                } => {
                    let found = match callee {
                        crate::mir::Callee::Extern(name) => match name.as_str() {
                            "env.console.warn" => Some(P10_MIN5_WARN_INVENTORY_ID),
                            "env.console.info" => Some(P10_MIN5_INFO_INVENTORY_ID),
                            "env.console.error" => Some(P10_MIN5_ERROR_INVENTORY_ID),
                            "env.console.debug" => Some(P10_MIN5_DEBUG_INVENTORY_ID),
                            _ => None,
                        },
                        crate::mir::Callee::Method {
                            box_name, method, ..
                        } => {
                            if box_name != "console" {
                                None
                            } else {
                                match method.as_str() {
                                    "warn" => Some(P10_MIN5_WARN_INVENTORY_ID),
                                    "info" => Some(P10_MIN5_INFO_INVENTORY_ID),
                                    "error" => Some(P10_MIN5_ERROR_INVENTORY_ID),
                                    "debug" => Some(P10_MIN5_DEBUG_INVENTORY_ID),
                                    _ => None,
                                }
                            }
                        }
                        _ => None,
                    };
                    if let Some(shape_id) = found {
                        if call_method.replace(shape_id).is_some() {
                            has_other_call = true;
                        }
                    } else {
                        has_other_call = true;
                    }
                }
                MirInstruction::Call { .. } => has_other_call = true,
                _ => {}
            }
        }

        if let Some(term) = &block.terminator {
            match term {
                MirInstruction::Branch { .. } => has_branch = true,
                MirInstruction::Jump { .. } => has_jump = true,
                _ => {}
            }
        }
    }

    if has_branch && has_jump && has_const_3 && !has_other_call {
        call_method
    } else {
        None
    }
}
