//! Phase 285A0: WeakRef handlers - 弱参照の作成とアップグレード
//!
//! SSOT: docs/reference/language/lifecycle.md:179
//!
//! WeakRef は強参照サイクルを避けるための非所有参照です。
//! - `weak(x)` → WeakRef(New): BoxRef から WeakRef を作成
//! - `w.weak_to_strong()` → WeakRef(Load): WeakRef から BoxRef へアップグレード（失敗時は null/Void）

use super::*;

impl MirInterpreter {
    /// WeakRef(New): BoxRef → WeakRef 変換
    ///
    /// # Arguments
    /// * `dst` - 結果を格納する ValueId
    /// * `box_val` - 変換元の Box ValueId
    ///
    /// # Returns
    /// * `Result<(), VMError>` - 成功時は Ok、失敗時は Err
    ///
    /// # Errors
    /// * `box_val` が BoxRef でない場合はエラー
    pub(crate) fn handle_weak_new(
        &mut self,
        dst: ValueId,
        box_val: ValueId,
    ) -> Result<(), VMError> {
        let box_value = self.reg_load(box_val)?;
        let weak_value = box_value
            .downgrade_to_weak()
            .ok_or_else(|| self.err_invalid("WeakRef(New): target is not a Box"))?;
        self.write_reg(dst, weak_value);
        Ok(())
    }

    /// WeakRef(Load): WeakRef → BoxRef | null (= Void) アップグレード
    ///
    /// # Arguments
    /// * `dst` - 結果を格納する ValueId
    /// * `weak_ref` - WeakRef ValueId
    ///
    /// # Returns
    /// * `Result<(), VMError>` - 成功時は Ok、失敗時は Err
    ///
    /// # Note
    /// - SSOT: upgrade failure returns null (= Void in VM) - lifecycle.md:179
    /// - ターゲットが既に drop された場合や Dead 状態の場合は Void を返す
    pub(crate) fn handle_weak_load(
        &mut self,
        dst: ValueId,
        weak_ref: ValueId,
    ) -> Result<(), VMError> {
        let weak_value = self.reg_load(weak_ref)?;
        // upgrade_weak() が None を返した場合は Void（null）
        let result = weak_value.upgrade_weak().unwrap_or(VMValue::Void);
        self.write_reg(dst, result);
        Ok(())
    }
}
