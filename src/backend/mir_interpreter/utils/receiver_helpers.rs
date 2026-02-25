//! Receiver変換ユーティリティ
//!
//! メソッド呼び出しのreceiverをBoxに変換する共通処理を提供します。

use super::super::*;
use crate::mir::ValueId;
use std::sync::Arc;

impl MirInterpreter {
    /// ReceiverをBoxに変換（エラーハンドリング込み）
    ///
    /// # Arguments
    /// * `receiver` - 変換するValueId
    ///
    /// # Returns
    /// 変換成功時はBox、失敗時はエラー
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn convert_to_box(
        &mut self,
        receiver: ValueId,
    ) -> Result<Arc<dyn crate::box_trait::NyashBox>, VMError> {
        let receiver_value = self.reg_load(receiver)?;
        match receiver_value {
            VMValue::BoxRef(b) => Ok(b),
            _ => Err(VMError::InvalidInstruction("receiver must be Box".into())),
        }
    }
}
