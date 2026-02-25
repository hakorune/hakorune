use super::*;
use crate::backend::runtime_type_spec::{matches_spec, RuntimeTypeSpec};
use crate::mir::types::MirType;

/// MirType → RuntimeTypeSpec 変換（呼び出し側で行う）
/// MirType 依存を type_ops.rs に閉じ込め、runtime_type_spec.rs は MirType に依存しない
fn spec_from_mir_type(ty: &MirType) -> RuntimeTypeSpec {
    match ty {
        MirType::Unknown => RuntimeTypeSpec::Unknown,
        MirType::Void => RuntimeTypeSpec::Void,
        MirType::Bool => RuntimeTypeSpec::Bool,
        MirType::Integer => RuntimeTypeSpec::Integer,
        MirType::Float => RuntimeTypeSpec::Float,
        MirType::String => RuntimeTypeSpec::String,
        MirType::WeakRef => RuntimeTypeSpec::WeakRef,
        MirType::Future(_) => RuntimeTypeSpec::Future,
        MirType::Array(_) => RuntimeTypeSpec::Array,
        MirType::Box(name) => RuntimeTypeSpec::Box(name.clone()),
    }
}

impl MirInterpreter {
    pub(crate) fn handle_type_op(
        &mut self,
        dst: ValueId,
        op: TypeOpKind,
        value: ValueId,
        ty: &MirType,
    ) -> Result<(), VMError> {
        let v = self.reg_load(value)?;
        let spec = spec_from_mir_type(ty); // MirType → RuntimeTypeSpec 変換
        match op {
            TypeOpKind::Check => {
                let ok = matches_spec(&v, &spec); // MirType に依存しない
                self.write_reg(dst, VMValue::Bool(ok));
                Ok(())
            }
            TypeOpKind::Cast => {
                if matches_spec(&v, &spec) {
                    // MirType に依存しない
                    self.write_reg(dst, v);
                    Ok(())
                } else {
                    Err(VMError::TypeError(format!(
                        "type cast failed: expected {:?}, got {:?}", // 文言維持
                        ty, v
                    )))
                }
            }
        }
    }
}
