//! Phase 285A0: WeakRef handlers - 弱参照の作成とアップグレード
//!
//! SSOT: docs/reference/language/lifecycle.md:179
//!
//! WeakRef は強参照サイクルを避けるための非所有参照です。
//! - `weak(x)` → WeakRef(New): BoxRef から WeakRef を作成
//! - `w.weak_to_strong()` → WeakRef(Load): WeakRef から BoxRef へアップグレード（失敗時は null/Void）

use super::*;

impl MirInterpreter {
    pub(crate) fn execute_weak_field_write(
        &mut self,
        function: &crate::mir::MirFunction,
        site_id: crate::mir::WeakFieldWriteSiteId,
        contract_id: &str,
        base: ValueId,
        field_index: u32,
        value: ValueId,
    ) -> Result<(), VMError> {
        let contract = function
            .metadata
            .weak_field_write_contracts
            .iter()
            .find(|contract| contract.site_id == site_id && contract.contract_id == contract_id)
            .ok_or_else(|| {
                self.err_invalid(format!(
                    "[type/weak_field_contract_carrier_missing] site={} contract={}",
                    site_id.0, contract_id
                ))
            })?;
        if contract.base_value_id != base
            || contract.value_id != value
            || contract.field_index != field_index
        {
            return Err(self.err_invalid(format!(
                "[type/weak_field_contract_stale_carrier] site={}",
                site_id.0
            )));
        }
        let base_value = self.reg_load(base)?;
        let VMValue::BoxRef(base_box) = base_value else {
            return Err(self.err_invalid(format!(
                "{} actual={:?}",
                crate::runtime::weak_field::BASE_NOT_INSTANCE_TAG,
                base_value
            )));
        };
        let instance = base_box
            .as_any()
            .downcast_ref::<crate::instance_v2::InstanceBox>()
            .ok_or_else(|| {
                self.err_invalid(crate::runtime::weak_field::BASE_NOT_INSTANCE_TAG.to_string())
            })?;
        let slot_value = self.weak_slot_value(value)?;
        crate::runtime::weak_field::WeakFieldRuntime::write_contract(
            instance,
            &contract.box_schema_fingerprint,
            field_index,
            slot_value,
        )
        .map_err(|reason| self.err_invalid(reason))
    }

    pub(super) fn weak_slot_value(
        &self,
        value: ValueId,
    ) -> Result<crate::runtime::weak_field::WeakSlotState, VMError> {
        match self.reg_load(value)? {
            VMValue::Void => Ok(crate::runtime::weak_field::WeakSlotState::Empty),
            VMValue::WeakBox(weak) => Ok(crate::runtime::weak_field::WeakSlotState::Occupied(weak)),
            other => Err(self.err_invalid(format!(
                "{} actual={:?}",
                crate::runtime::weak_field::CONTRACT_VIOLATION_TAG,
                other
            ))),
        }
    }

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
        let result = match weak_value {
            VMValue::WeakBox(_) => weak_value.upgrade_weak().unwrap_or(VMValue::Void),
            VMValue::Void => VMValue::Void,
            other => {
                return Err(self.err_invalid(format!(
                    "{} actual={:?}",
                    crate::runtime::weak_ref_value::WEAKREF_LOAD_INVALID_INPUT_TAG,
                    other
                )))
            }
        };
        self.write_reg(dst, result);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::box_trait::NyashBox;
    use std::collections::HashMap;
    use std::sync::Arc;

    #[test]
    fn weak_load_keeps_void_total_case() {
        let mut vm = MirInterpreter::new();
        let source = ValueId::new(1);
        let result = ValueId::new(2);
        vm.regs.insert(source, VMValue::Void);

        vm.handle_weak_load(result, source)
            .expect("Void should load");

        assert_eq!(vm.regs.get(&result), Some(&VMValue::Void));
    }

    #[test]
    fn weak_load_rejects_non_weak_values() {
        let mut vm = MirInterpreter::new();
        let source = ValueId::new(1);
        vm.regs.insert(source, VMValue::Integer(42));

        let error = vm
            .handle_weak_load(ValueId::new(2), source)
            .expect_err("Integer must not pass WeakRef.Load");

        assert!(error
            .to_string()
            .contains(crate::runtime::weak_ref_value::WEAKREF_LOAD_INVALID_INPUT_TAG));
    }

    #[test]
    fn weak_load_rejects_strong_box_values() {
        let mut vm = MirInterpreter::new();
        let source = ValueId::new(1);
        let strong: Arc<dyn NyashBox> = Arc::new(crate::box_trait::IntegerBox::new(42));
        vm.regs.insert(source, VMValue::BoxRef(strong));

        let error = vm
            .handle_weak_load(ValueId::new(2), source)
            .expect_err("strong BoxRef must not pass WeakRef.Load");

        assert!(error
            .to_string()
            .contains(crate::runtime::weak_ref_value::WEAKREF_LOAD_INVALID_INPUT_TAG));
    }

    #[test]
    fn weak_load_rejects_logically_finalized_target() {
        let instance = Arc::new(crate::instance_v2::InstanceBox::from_declaration(
            "FinalizedTarget".to_string(),
            Vec::new(),
            HashMap::new(),
        ));
        let target: Arc<dyn NyashBox> = instance.clone();
        let weak = Arc::downgrade(&target);
        instance.fini().expect("fini should succeed");

        let mut vm = MirInterpreter::new();
        let source = ValueId::new(1);
        let result = ValueId::new(2);
        vm.regs.insert(source, VMValue::WeakBox(weak));
        vm.handle_weak_load(result, source)
            .expect("dead upgrade should return Void");

        assert_eq!(vm.regs.get(&result), Some(&VMValue::Void));
    }
}
