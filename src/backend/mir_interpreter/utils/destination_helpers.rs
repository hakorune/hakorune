//! Destination書き込みユーティリティ
//!
//! MIR命令の結果をdestinationレジスタに書き込む共通処理を提供します。

use super::super::*;
use crate::mir::ValueId;
use std::sync::Arc;

impl MirInterpreter {
    /// Box結果をdestinationに書き込む
    ///
    /// # Arguments
    /// * `dst` - 書き込み先のValueId (Noneの場合は何もしない)
    /// * `result` - 書き込むBox
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn write_box_result(
        &mut self,
        dst: Option<ValueId>,
        result: Arc<dyn crate::box_trait::NyashBox>,
    ) {
        if let Some(d) = dst {
            self.write_reg(d, VMValue::BoxRef(result));
        }
    }

    /// Voidをdestinationに書き込む
    ///
    /// # Arguments
    /// * `dst` - 書き込み先のValueId (Noneの場合は何もしない)
    #[inline]
    pub(crate) fn write_void(&mut self, dst: Option<ValueId>) {
        if let Some(d) = dst {
            self.write_reg(d, VMValue::Void);
        }
    }

    /// 値をそのままdestinationに書き込む（汎用）
    ///
    /// # Arguments
    /// * `dst` - 書き込み先のValueId (Noneの場合は何もしない)
    /// * `value` - 書き込む値
    #[inline]
    pub(crate) fn write_result(&mut self, dst: Option<ValueId>, value: VMValue) {
        if let Some(d) = dst {
            self.write_reg(d, value);
        }
    }

    /// String値をdestinationに書き込む
    ///
    /// # Arguments
    /// * `dst` - 書き込み先のValueId (Noneの場合は何もしない)
    /// * `value` - 書き込むString
    #[inline]
    pub(crate) fn write_string(&mut self, dst: Option<ValueId>, value: String) {
        if let Some(d) = dst {
            self.write_reg(d, VMValue::String(value));
        }
    }

    /// Box<dyn NyashBox>をVMValueに変換してdestinationに書き込む
    ///
    /// # Arguments
    /// * `dst` - 書き込み先のValueId (Noneの場合は何もしない)
    /// * `nyash_box` - 書き込むBox (NullBox→Void, IntegerBox→Integer等に自動変換)
    #[inline]
    pub(crate) fn write_from_box(
        &mut self,
        dst: Option<ValueId>,
        nyash_box: Box<dyn crate::box_trait::NyashBox>,
    ) {
        if let Some(d) = dst {
            self.write_reg(d, VMValue::from_nyash_box(nyash_box));
        }
    }
}
