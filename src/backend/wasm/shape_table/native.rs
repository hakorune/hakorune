use crate::mir::BinaryOp;
use crate::mir::{ConstValue, MirInstruction, MirModule};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NativeShape {
    MainReturnI32Const,
    MainReturnI32ConstViaCopy,
    MainReturnI32ConstBinOp,
}

impl NativeShape {
    pub(crate) fn id(self) -> &'static str {
        match self {
            NativeShape::MainReturnI32Const => "wsm.p4.main_return_i32_const.v0",
            NativeShape::MainReturnI32ConstViaCopy => "wsm.p5.main_return_i32_const_via_copy.v0",
            NativeShape::MainReturnI32ConstBinOp => "wsm.p9.main_return_i32_const_binop.v0",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NativeMatch {
    pub(crate) shape: NativeShape,
    pub(crate) value: i32,
}

type ShapeMatcher = fn(&MirModule) -> Option<NativeMatch>;

const NATIVE_SHAPE_TABLE: &[ShapeMatcher] = &[
    match_main_return_i32_const,
    match_main_return_i32_const_via_copy,
    match_main_return_i32_const_binop,
];

pub(crate) fn match_native_shape(mir_module: &MirModule) -> Option<NativeMatch> {
    for matcher in NATIVE_SHAPE_TABLE {
        if let Some(found) = matcher(mir_module) {
            return Some(found);
        }
    }
    None
}

pub(crate) fn match_main_return_i32_const(mir_module: &MirModule) -> Option<NativeMatch> {
    let main = mir_module.get_function("main")?;
    if main.blocks.len() != 1 {
        return None;
    }

    let entry = main.blocks.get(&main.entry_block)?;
    if entry.instructions.len() != 1 {
        return None;
    }

    let MirInstruction::Const { dst, value } = &entry.instructions[0] else {
        return None;
    };
    let MirInstruction::Return {
        value: Some(ret_val),
    } = entry.terminator.as_ref()?
    else {
        return None;
    };
    if ret_val != dst {
        return None;
    }

    let ConstValue::Integer(n) = value else {
        return None;
    };
    let value = i32::try_from(*n).ok()?;
    Some(NativeMatch {
        shape: NativeShape::MainReturnI32Const,
        value,
    })
}

pub(crate) fn match_main_return_i32_const_via_copy(mir_module: &MirModule) -> Option<NativeMatch> {
    let main = mir_module.get_function("main")?;
    if main.blocks.len() != 1 {
        return None;
    }

    let entry = main.blocks.get(&main.entry_block)?;
    if entry.instructions.len() != 2 {
        return None;
    }

    let MirInstruction::Const { dst, value } = &entry.instructions[0] else {
        return None;
    };
    let MirInstruction::Copy { dst: copy_dst, src } = &entry.instructions[1] else {
        return None;
    };
    if src != dst {
        return None;
    }

    let MirInstruction::Return {
        value: Some(ret_val),
    } = entry.terminator.as_ref()?
    else {
        return None;
    };
    if ret_val != copy_dst {
        return None;
    }

    let ConstValue::Integer(n) = value else {
        return None;
    };
    let value = i32::try_from(*n).ok()?;
    Some(NativeMatch {
        shape: NativeShape::MainReturnI32ConstViaCopy,
        value,
    })
}

pub(crate) fn match_main_return_i32_const_binop(mir_module: &MirModule) -> Option<NativeMatch> {
    let main = mir_module.get_function("main")?;
    if main.blocks.len() != 1 {
        return None;
    }

    let entry = main.blocks.get(&main.entry_block)?;
    if entry.instructions.len() != 3 {
        return None;
    }

    let MirInstruction::Const {
        dst: lhs_dst,
        value: lhs_value,
    } = &entry.instructions[0]
    else {
        return None;
    };
    let MirInstruction::Const {
        dst: rhs_dst,
        value: rhs_value,
    } = &entry.instructions[1]
    else {
        return None;
    };
    let MirInstruction::BinOp {
        dst: binop_dst,
        op,
        lhs,
        rhs,
    } = &entry.instructions[2]
    else {
        return None;
    };
    if lhs != lhs_dst || rhs != rhs_dst {
        return None;
    }

    let MirInstruction::Return {
        value: Some(ret_val),
    } = entry.terminator.as_ref()?
    else {
        return None;
    };
    if ret_val != binop_dst {
        return None;
    }

    let ConstValue::Integer(lhs_n) = lhs_value else {
        return None;
    };
    let ConstValue::Integer(rhs_n) = rhs_value else {
        return None;
    };

    let folded = fold_i64_binop(*op, *lhs_n, *rhs_n)?;
    let value = i32::try_from(folded).ok()?;
    Some(NativeMatch {
        shape: NativeShape::MainReturnI32ConstBinOp,
        value,
    })
}

pub(crate) fn fold_i64_binop(op: BinaryOp, lhs: i64, rhs: i64) -> Option<i64> {
    match op {
        BinaryOp::Add => lhs.checked_add(rhs),
        BinaryOp::Sub => lhs.checked_sub(rhs),
        BinaryOp::Mul => lhs.checked_mul(rhs),
        BinaryOp::Div => lhs.checked_div(rhs),
        BinaryOp::Mod => lhs.checked_rem(rhs),
        _ => None,
    }
}
